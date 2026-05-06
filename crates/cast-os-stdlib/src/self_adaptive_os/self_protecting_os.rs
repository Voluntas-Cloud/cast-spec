//! `self_protecting_os` — OS adapts to threats.

/// Sentinel for `self_protecting_os`.
pub struct SelfProtectingOs;

cast::concept! {
    name: "self_protecting_os",
    summary: "OS adapts to threats.",
    anchors: [cast_os_stdlib::self_adaptive_os::self_protecting_os::SelfProtectingOs],
    tags: ["cast_os_stdlib", "self_adaptive_os"],
}
