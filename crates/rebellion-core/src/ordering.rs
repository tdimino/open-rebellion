//! Shared deterministic ordering primitives used across crate boundaries.

/// Sort values by a Unicode-lowercased display name, then a stable numeric
/// identity. The owned key keeps this generic across data and renderer DTOs.
pub fn sort_by_case_folded_name<T>(values: &mut [T], key: impl Fn(&T) -> (String, u32)) {
    values.sort_by_key(key);
}
