//! `transactional_package_update` — update applied atomically.

/// Sentinel for `transactional_package_update`.
pub struct TransactionalPackageUpdate;

cast::concept! {
    name: "transactional_package_update",
    summary: "update applied atomically.",
    anchors: [cast_os_stdlib::update_evolution::transactional_package_update::TransactionalPackageUpdate],
    tags: ["cast_os_stdlib", "update_evolution"],
}
