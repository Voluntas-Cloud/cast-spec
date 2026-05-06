//! `dependency_based_boot_order` — startup ordered by dependencies.

/// Sentinel for `dependency_based_boot_order`.
pub struct DependencyBasedBootOrder;

cast::concept! {
    name: "dependency_based_boot_order",
    summary: "startup ordered by dependencies.",
    anchors: [cast_os_stdlib::boot_init::dependency_based_boot_order::DependencyBasedBootOrder],
    tags: ["cast_os_stdlib", "boot_init"],
}
