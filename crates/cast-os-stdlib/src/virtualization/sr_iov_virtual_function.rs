//! `sr_iov_virtual_function` — hardware-assisted device sharing.

/// Sentinel for `sr_iov_virtual_function`.
pub struct SrIovVirtualFunction;

cast::concept! {
    name: "sr_iov_virtual_function",
    summary: "hardware-assisted device sharing.",
    anchors: [cast_os_stdlib::virtualization::sr_iov_virtual_function::SrIovVirtualFunction],
    tags: ["cast_os_stdlib", "virtualization"],
}
