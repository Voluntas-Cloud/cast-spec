//! `full_virtualization` — guest runs mostly unaware.

/// Sentinel for `full_virtualization`.
pub struct FullVirtualization;

cast::concept! {
    name: "full_virtualization",
    summary: "guest runs mostly unaware.",
    anchors: [cast_os_stdlib::virtualization::full_virtualization::FullVirtualization],
    tags: ["cast_os_stdlib", "virtualization"],
}
