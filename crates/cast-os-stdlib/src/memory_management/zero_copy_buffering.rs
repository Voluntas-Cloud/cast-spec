//! `zero_copy_buffering` — avoid copying data between layers.

/// Sentinel for `zero_copy_buffering`.
pub struct ZeroCopyBuffering;

cast::concept! {
    name: "zero_copy_buffering",
    summary: "avoid copying data between layers.",
    anchors: [cast_os_stdlib::memory_management::zero_copy_buffering::ZeroCopyBuffering],
    tags: ["cast_os_stdlib", "memory_management"],
}
