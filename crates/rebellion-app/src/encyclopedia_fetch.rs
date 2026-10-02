//! Bounded, status-aware browser fetches for loose encyclopedia content.
//!
//! Miniquad's general asset loader buffers complete responses and does not
//! preserve non-200 HTTP statuses. This adapter is deliberately separate: its
//! JavaScript half checks status before reading and bounds streamed bytes, then
//! this module allocates the final Rust buffer only after that validation.

use std::fmt;

const STATE_PENDING: i32 = 0;
const STATE_READY: i32 = 1;
const STATE_NOT_FOUND: i32 = 2;
const STATE_HTTP_STATUS: i32 = 3;
const STATE_TRANSPORT: i32 = 4;
const STATE_RESOURCE_LIMIT: i32 = 5;
const STATE_UNSUPPORTED_STREAMING: i32 = 6;

/// Failure classes preserved by [`fetch_encyclopedia_file`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum EncyclopediaFetchError {
    NotFound,
    HttpStatus(u16),
    Transport,
    ResourceLimit { max_bytes: usize },
    UnsupportedStreaming,
}

impl fmt::Display for EncyclopediaFetchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => formatter.write_str("encyclopedia file was not found (HTTP 404)"),
            Self::HttpStatus(status) => {
                write!(formatter, "encyclopedia fetch returned HTTP {status}")
            }
            Self::Transport => formatter.write_str("encyclopedia fetch transport failed"),
            Self::ResourceLimit { max_bytes } => write!(
                formatter,
                "encyclopedia file exceeds the {max_bytes}-byte fetch limit"
            ),
            Self::UnsupportedStreaming => formatter.write_str(
                "browser does not provide streaming response bodies for encyclopedia fetches",
            ),
        }
    }
}

impl std::error::Error for EncyclopediaFetchError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BridgeOutcome {
    Pending,
    Ready(usize),
}

fn classify_bridge_outcome(
    state: i32,
    detail: u32,
    max_bytes: usize,
) -> Result<BridgeOutcome, EncyclopediaFetchError> {
    match state {
        STATE_PENDING => Ok(BridgeOutcome::Pending),
        STATE_READY => {
            let length = detail as usize;
            if length > max_bytes {
                Err(EncyclopediaFetchError::ResourceLimit { max_bytes })
            } else {
                Ok(BridgeOutcome::Ready(length))
            }
        }
        STATE_NOT_FOUND => Err(EncyclopediaFetchError::NotFound),
        STATE_HTTP_STATUS => match u16::try_from(detail) {
            Ok(status) => Err(EncyclopediaFetchError::HttpStatus(status)),
            Err(_) => Err(EncyclopediaFetchError::Transport),
        },
        STATE_RESOURCE_LIMIT => Err(EncyclopediaFetchError::ResourceLimit { max_bytes }),
        STATE_UNSUPPORTED_STREAMING => Err(EncyclopediaFetchError::UnsupportedStreaming),
        _ => Err(EncyclopediaFetchError::Transport),
    }
}

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn open_rebellion_encyclopedia_fetch_start(
        path_ptr: *const u8,
        path_len: usize,
        max_bytes: usize,
    ) -> u32;
    fn open_rebellion_encyclopedia_fetch_poll(handle: u32) -> i32;
    fn open_rebellion_encyclopedia_fetch_detail(handle: u32) -> u32;
    fn open_rebellion_encyclopedia_fetch_copy(
        handle: u32,
        destination_ptr: *mut u8,
        destination_len: usize,
    ) -> i32;
    fn open_rebellion_encyclopedia_fetch_release(handle: u32) -> i32;
}

#[cfg(target_arch = "wasm32")]
struct RequestGuard {
    handle: u32,
}

#[cfg(target_arch = "wasm32")]
impl Drop for RequestGuard {
    fn drop(&mut self) {
        // The JavaScript release operation is idempotent and also aborts any
        // pending fetch/reader. This is what makes cancellation of this future
        // release browser-side request and chunk state.
        unsafe {
            open_rebellion_encyclopedia_fetch_release(self.handle);
        }
    }
}

/// Fetch one loose encyclopedia file without using Miniquad's buffered loader.
///
/// The JavaScript bridge streams and bounds the response before reporting it
/// ready. Rust then reserves exactly the validated length and copies once into
/// the result. Dropping this future releases and aborts the browser request.
#[cfg(target_arch = "wasm32")]
pub(crate) async fn fetch_encyclopedia_file(
    path: &str,
    max_bytes: usize,
) -> Result<Vec<u8>, EncyclopediaFetchError> {
    let handle =
        unsafe { open_rebellion_encyclopedia_fetch_start(path.as_ptr(), path.len(), max_bytes) };
    if handle == 0 {
        return Err(EncyclopediaFetchError::Transport);
    }
    let _request = RequestGuard { handle };

    loop {
        let state = unsafe { open_rebellion_encyclopedia_fetch_poll(handle) };
        let detail = if state == STATE_PENDING {
            0
        } else {
            unsafe { open_rebellion_encyclopedia_fetch_detail(handle) }
        };
        match classify_bridge_outcome(state, detail, max_bytes)? {
            BridgeOutcome::Pending => macroquad::prelude::next_frame().await,
            BridgeOutcome::Ready(length) => {
                let mut bytes = Vec::new();
                bytes
                    .try_reserve_exact(length)
                    .map_err(|_| EncyclopediaFetchError::ResourceLimit { max_bytes })?;
                bytes.resize(length, 0);
                if length != 0 {
                    let copied = unsafe {
                        open_rebellion_encyclopedia_fetch_copy(
                            handle,
                            bytes.as_mut_ptr(),
                            bytes.len(),
                        )
                    };
                    if copied != 1 {
                        return Err(EncyclopediaFetchError::Transport);
                    }
                }
                return Ok(bytes);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_errors_describe_their_distinct_failure_classes() {
        assert_eq!(
            EncyclopediaFetchError::NotFound.to_string(),
            "encyclopedia file was not found (HTTP 404)"
        );
        assert_eq!(
            EncyclopediaFetchError::HttpStatus(500).to_string(),
            "encyclopedia fetch returned HTTP 500"
        );
        assert_eq!(
            EncyclopediaFetchError::Transport.to_string(),
            "encyclopedia fetch transport failed"
        );
        assert_eq!(
            EncyclopediaFetchError::ResourceLimit { max_bytes: 32 }.to_string(),
            "encyclopedia file exceeds the 32-byte fetch limit"
        );
        assert_eq!(
            EncyclopediaFetchError::UnsupportedStreaming.to_string(),
            "browser does not provide streaming response bodies for encyclopedia fetches"
        );
    }

    #[test]
    fn pending_bridge_result_remains_pending_without_using_detail() {
        assert_eq!(
            classify_bridge_outcome(STATE_PENDING, u32::MAX, 0),
            Ok(BridgeOutcome::Pending)
        );
    }

    #[test]
    fn bridge_result_preserves_http_failure_classes() {
        assert_eq!(
            classify_bridge_outcome(STATE_NOT_FOUND, 404, 8),
            Err(EncyclopediaFetchError::NotFound)
        );
        assert_eq!(
            classify_bridge_outcome(STATE_HTTP_STATUS, 403, 8),
            Err(EncyclopediaFetchError::HttpStatus(403))
        );
        assert_eq!(
            classify_bridge_outcome(STATE_HTTP_STATUS, u16::MAX as u32 + 1, 8),
            Err(EncyclopediaFetchError::Transport)
        );
    }

    #[test]
    fn bridge_result_checks_ready_length_before_allocation() {
        assert_eq!(
            classify_bridge_outcome(STATE_READY, 8, 8),
            Ok(BridgeOutcome::Ready(8))
        );
        assert_eq!(
            classify_bridge_outcome(STATE_READY, 9, 8),
            Err(EncyclopediaFetchError::ResourceLimit { max_bytes: 8 })
        );
        assert_eq!(
            classify_bridge_outcome(STATE_RESOURCE_LIMIT, 0, 8),
            Err(EncyclopediaFetchError::ResourceLimit { max_bytes: 8 })
        );
    }

    #[test]
    fn bridge_result_rejects_unknown_and_unsupported_states() {
        assert_eq!(
            classify_bridge_outcome(STATE_TRANSPORT, 0, 8),
            Err(EncyclopediaFetchError::Transport)
        );
        assert_eq!(
            classify_bridge_outcome(STATE_UNSUPPORTED_STREAMING, 0, 8),
            Err(EncyclopediaFetchError::UnsupportedStreaming)
        );
        assert_eq!(
            classify_bridge_outcome(-1, 0, 8),
            Err(EncyclopediaFetchError::Transport)
        );
    }
}
