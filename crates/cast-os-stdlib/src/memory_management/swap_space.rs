//! `swap_space` — disk-backed extension of memory.

/// Sentinel for `swap_space`.
pub struct SwapSpace;

cast::concept! {
    name: "swap_space",
    summary: "disk-backed extension of memory.",
    anchors: [cast_os_stdlib::memory_management::swap_space::SwapSpace],
    tags: ["cast_os_stdlib", "memory_management"],
}
