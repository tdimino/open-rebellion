//! Shared bounded native-file reads for Encyclopedia assets.

#![cfg(not(target_arch = "wasm32"))]

use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub(crate) enum BoundedReadError {
    LimitDoesNotFitU64,
    LimitOverflow,
    Read(std::io::Error),
    Exceeded,
}

pub(crate) fn bounded_read_limit(max_bytes: usize) -> Result<u64, BoundedReadError> {
    u64::try_from(max_bytes)
        .map_err(|_| BoundedReadError::LimitDoesNotFitU64)?
        .checked_add(1)
        .ok_or(BoundedReadError::LimitOverflow)
}

pub(crate) fn read_open_file_bounded(
    file: File,
    capacity: usize,
    max_bytes: usize,
) -> Result<Vec<u8>, BoundedReadError> {
    let mut bytes = Vec::with_capacity(capacity.min(max_bytes));
    let mut bounded = file.take(bounded_read_limit(max_bytes)?);
    bounded
        .read_to_end(&mut bytes)
        .map_err(BoundedReadError::Read)?;
    if bytes.len() > max_bytes {
        return Err(BoundedReadError::Exceeded);
    }
    Ok(bytes)
}
