//! Shared wire boundary for deterministic browser-interface fixtures.

use serde::Serialize;

extern "C" {
    fn open_rebellion_interface_fixture_code() -> u32;
    fn open_rebellion_interface_fixture_emit(ptr: *const u8, len: usize);
}

pub(crate) fn requested_code() -> u32 {
    // The fixture host owns this import and returns zero when no fixture is requested.
    unsafe { open_rebellion_interface_fixture_code() }
}

pub(crate) fn emit<T: Serialize + ?Sized>(payload: &T) {
    let bytes = serde_json::to_vec(payload).expect("serialize interface fixture payload");
    emit_bytes(&bytes);
}

pub(crate) fn emit_some<T: Serialize>(payload: Option<T>) {
    if let Some(payload) = payload {
        emit(&payload);
    }
}

pub(crate) fn emit_bytes(bytes: &[u8]) {
    // The host copies the payload during the call; `bytes` remains live for its duration.
    unsafe { open_rebellion_interface_fixture_emit(bytes.as_ptr(), bytes.len()) };
}
