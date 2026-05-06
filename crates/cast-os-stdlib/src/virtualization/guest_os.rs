//! `guest_os` — OS running inside VM.

/// Sentinel for `guest_os`.
pub struct GuestOs;

cast::concept! {
    name: "guest_os",
    summary: "OS running inside VM.",
    anchors: [cast_os_stdlib::virtualization::guest_os::GuestOs],
    tags: ["cast_os_stdlib", "virtualization"],
}
