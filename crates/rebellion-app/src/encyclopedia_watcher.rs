//! Native filesystem-event adapter for encyclopedia content-only refreshes.
//!
//! The watcher owns no world or panel-action handle. At the main-loop polling
//! boundary it drains/coalesces filesystem events, refreshes discovery once,
//! resolves the enabled order once, and gives that exact order to E50's
//! content-only entry. Manual reload remains responsible for world overlays and
//! explicitly rearms a root that was missing or unwatchable at startup.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

#[cfg(test)]
use std::collections::VecDeque;

use rebellion_data::mods::{ModRuntime, ModWatchPoll, ModWatcher};

use crate::encyclopedia_lifecycle::{EncyclopediaContentRefresh, EncyclopediaLifecycle};

/// A short quiet period coalesces the remove/create halves of an editor's
/// atomic save. Continuous event streams cannot postpone publication beyond
/// the separate maximum latency.
const WATCH_QUIET_PERIOD: Duration = Duration::from_millis(100);
const WATCH_MAX_LATENCY: Duration = Duration::from_millis(500);

#[derive(Debug, Clone, Copy)]
struct PendingChange {
    first_seen: Duration,
    last_seen: Duration,
}

/// Result of one defined main-loop watcher boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncyclopediaWatchOutcome {
    /// Present at most once per poll even when the backend drained many events.
    pub refresh: Option<EncyclopediaContentRefresh>,
    /// Bounded backend diagnostics observed during this drain.
    pub diagnostics: Vec<String>,
}

/// One retained native watcher for the discovered mods root.
pub struct EncyclopediaWatcher {
    root: PathBuf,
    watcher: Option<ModWatcher>,
    diagnostic: Option<String>,
    clock_start: Instant,
    pending: Option<PendingChange>,
    #[cfg(test)]
    injected_polls: VecDeque<ModWatchPoll>,
    #[cfg(test)]
    refresh_count: u64,
    #[cfg(test)]
    last_resolved_order: Vec<String>,
}

impl EncyclopediaWatcher {
    /// Arms the native recursive watcher when possible. A missing or
    /// unwatchable root is non-fatal: the base encyclopedia remains usable and
    /// [`Self::rearm`] can be called by explicit Reload Mods after creation.
    #[must_use]
    pub fn new(root: &Path) -> Self {
        match ModWatcher::new(root) {
            Ok(watcher) => Self {
                root: root.to_path_buf(),
                watcher: Some(watcher),
                diagnostic: None,
                clock_start: Instant::now(),
                pending: None,
                #[cfg(test)]
                injected_polls: VecDeque::new(),
                #[cfg(test)]
                refresh_count: 0,
                #[cfg(test)]
                last_resolved_order: Vec::new(),
            },
            Err(error) => Self {
                root: root.to_path_buf(),
                watcher: None,
                diagnostic: Some(bounded_diagnostic(error.to_string())),
                clock_start: Instant::now(),
                pending: None,
                #[cfg(test)]
                injected_polls: VecDeque::new(),
                #[cfg(test)]
                refresh_count: 0,
                #[cfg(test)]
                last_resolved_order: Vec::new(),
            },
        }
    }

    /// Replaces the current watcher with one newly armed against the same root.
    /// Exactly one backend watcher is retained after a successful call.
    pub fn rearm(&mut self) -> bool {
        match ModWatcher::new(&self.root) {
            Ok(watcher) => {
                self.watcher = Some(watcher);
                self.diagnostic = None;
                self.pending = None;
                true
            }
            Err(error) => {
                self.watcher = None;
                self.diagnostic = Some(bounded_diagnostic(error.to_string()));
                self.pending = None;
                false
            }
        }
    }

    #[cfg(test)]
    #[must_use]
    pub const fn is_armed(&self) -> bool {
        self.watcher.is_some()
    }

    #[must_use]
    pub fn diagnostic(&self) -> Option<&str> {
        self.diagnostic.as_deref()
    }

    /// Polls once at the app's defined frame boundary. Automatic events can
    /// only call E50's content entry; this type deliberately has no `GameWorld`,
    /// `PanelAction`, or world-application API.
    pub fn poll_and_refresh(
        &mut self,
        runtime: &mut ModRuntime,
        lifecycle: &mut EncyclopediaLifecycle,
    ) -> EncyclopediaWatchOutcome {
        self.poll_and_refresh_at(runtime, lifecycle, self.clock_start.elapsed())
    }

    fn poll_and_refresh_at(
        &mut self,
        runtime: &mut ModRuntime,
        lifecycle: &mut EncyclopediaLifecycle,
        now: Duration,
    ) -> EncyclopediaWatchOutcome {
        if let Some(error) = self
            .watcher
            .as_ref()
            .and_then(|watcher| watcher.validate_root().err())
        {
            self.watcher = None;
            self.pending = None;
            let diagnostic = bounded_diagnostic(format!(
                "watch root unavailable at {}: {error}; recreate it and use Reload Mods to rearm",
                self.root.display()
            ));
            self.diagnostic = Some(diagnostic.clone());
            return EncyclopediaWatchOutcome {
                refresh: None,
                diagnostics: vec![diagnostic],
            };
        }
        let poll = self.poll_backend();
        if let Some(diagnostic) = poll.diagnostics.last() {
            self.diagnostic = Some(bounded_diagnostic(diagnostic.clone()));
        }
        if poll.changed {
            if let Some(pending) = &mut self.pending {
                pending.last_seen = now;
            } else {
                self.pending = Some(PendingChange {
                    first_seen: now,
                    last_seen: now,
                });
            }
        }
        let stable = self.pending.is_some_and(|pending| {
            now.saturating_sub(pending.last_seen) >= WATCH_QUIET_PERIOD
                || now.saturating_sub(pending.first_seen) >= WATCH_MAX_LATENCY
        });
        if !stable {
            return EncyclopediaWatchOutcome {
                refresh: None,
                diagnostics: poll.diagnostics,
            };
        }
        self.pending = None;

        runtime.refresh();
        let ordered = runtime.enabled_sorted();
        #[cfg(test)]
        {
            self.last_resolved_order = ordered.iter().map(|item| item.name.clone()).collect();
            self.refresh_count = self
                .refresh_count
                .checked_add(1)
                .expect("a finite test cannot overflow the refresh counter");
        }
        let refresh = lifecycle.refresh_content_only(&ordered);
        EncyclopediaWatchOutcome {
            refresh: Some(refresh),
            diagnostics: poll.diagnostics,
        }
    }

    fn poll_backend(&mut self) -> ModWatchPoll {
        #[cfg(test)]
        if let Some(poll) = self.injected_polls.pop_front() {
            return poll;
        }
        self.watcher
            .as_ref()
            .map_or_else(ModWatchPoll::default, ModWatcher::poll)
    }

    #[cfg(test)]
    pub(crate) fn from_test_polls(
        root: &Path,
        polls: impl IntoIterator<Item = ModWatchPoll>,
    ) -> Self {
        Self {
            root: root.to_path_buf(),
            watcher: None,
            diagnostic: None,
            clock_start: Instant::now(),
            pending: None,
            injected_polls: polls.into_iter().collect(),
            refresh_count: 0,
            last_resolved_order: Vec::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn poll_and_refresh_at_for_test(
        &mut self,
        runtime: &mut ModRuntime,
        lifecycle: &mut EncyclopediaLifecycle,
        now: Duration,
    ) -> EncyclopediaWatchOutcome {
        self.poll_and_refresh_at(runtime, lifecycle, now)
    }

    #[cfg(test)]
    pub(crate) const fn has_pending_for_test(&self) -> bool {
        self.pending.is_some()
    }

    #[cfg(test)]
    pub(crate) const fn refresh_count_for_test(&self) -> u64 {
        self.refresh_count
    }

    #[cfg(test)]
    pub(crate) fn last_resolved_order_for_test(&self) -> Vec<&str> {
        self.last_resolved_order
            .iter()
            .map(String::as_str)
            .collect()
    }
}

fn bounded_diagnostic(mut message: String) -> String {
    const LIMIT: usize = 256;
    if message.len() > LIMIT {
        let mut end = LIMIT;
        while !message.is_char_boundary(end) {
            end -= 1;
        }
        message.truncate(end);
    }
    message
}

#[cfg(test)]
mod tests {
    use super::bounded_diagnostic;

    #[test]
    fn watcher_diagnostic_bound_preserves_utf8_boundaries() {
        let message = format!("{}é{}", "x".repeat(255), "z".repeat(16));
        let bounded = bounded_diagnostic(message);
        assert_eq!(bounded.len(), 255);
        assert!(bounded.chars().all(|character| character == 'x'));
    }
}
