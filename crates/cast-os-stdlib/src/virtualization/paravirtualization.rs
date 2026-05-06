//! `paravirtualization` — guest cooperates with hypervisor.

/// Sentinel for `paravirtualization`.
pub struct Paravirtualization;

cast::concept! {
    name: "paravirtualization",
    summary: "guest cooperates with hypervisor.",
    anchors: [cast_os_stdlib::virtualization::paravirtualization::Paravirtualization],
    tags: ["cast_os_stdlib", "virtualization"],
}
