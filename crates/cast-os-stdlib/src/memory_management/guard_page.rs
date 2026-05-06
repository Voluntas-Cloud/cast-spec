//! `guard_page` — unmapped page used to catch overflow.

/// Sentinel for `guard_page`.
pub struct GuardPage;

cast::concept! {
    name: "guard_page",
    summary: "unmapped page used to catch overflow.",
    anchors: [cast_os_stdlib::memory_management::guard_page::GuardPage],
    tags: ["cast_os_stdlib", "memory_management"],
}
