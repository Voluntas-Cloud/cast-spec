//! `dual_write_migration` — temporarily write old and new formats.

/// Sentinel for `dual_write_migration`.
pub struct DualWriteMigration;

cast::concept! {
    name: "dual_write_migration",
    summary: "Temporarily write old and new formats. Reader can switch \
              over once the new format has full coverage; old format \
              is removed last.",
    anchors: [cast_stdlib::schema::dual_write_migration::DualWriteMigration],
    tags: ["cast_stdlib", "schema"],
}
