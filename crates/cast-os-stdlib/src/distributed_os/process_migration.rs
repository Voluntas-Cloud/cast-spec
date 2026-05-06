//! `process_migration` — move running process between machines.

/// Sentinel for `process_migration`.
pub struct ProcessMigration;

cast::concept! {
    name: "process_migration",
    summary: "move running process between machines.",
    anchors: [cast_os_stdlib::distributed_os::process_migration::ProcessMigration],
    tags: ["cast_os_stdlib", "distributed_os"],
}
