//! `live_migration` — move VM while running.

/// Sentinel for `live_migration`.
pub struct LiveMigration;

cast::concept! {
    name: "live_migration",
    summary: "move VM while running.",
    anchors: [cast_os_stdlib::virtualization::live_migration::LiveMigration],
    tags: ["cast_os_stdlib", "virtualization"],
}
