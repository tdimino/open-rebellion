//! One focus across the galaxy view's child windows. The original's System,
//! Fleet, Defenses and Missions windows are children of one galaxy window,
//! so a click raises the child clicked whatever its kind (Win32 z-order).
//! Each kind's state remembers when one of its windows last took focus; the
//! newest kind is in front, and only its last window paints on top.

use std::sync::atomic::{AtomicU64, Ordering};

static NEXT: AtomicU64 = AtomicU64::new(1);

/// A number larger than every one handed out before.
pub(crate) fn next_serial() -> u64 {
    NEXT.fetch_add(1, Ordering::Relaxed)
}
