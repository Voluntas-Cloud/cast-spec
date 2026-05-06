//! `strangler_fig_migration` — replace legacy gradually, named after the strangler fig tree.

/// Sentinel for `strangler_fig_migration`.
pub struct StranglerFigMigration;

cast::concept! {
    name: "strangler_fig_migration",
    summary: "Replace legacy system gradually. New system grows around \
              the old; the old shrinks until removed. Named after the \
              strangler fig tree.",
    anchors: [cast_stdlib::architecture::strangler_fig_migration::StranglerFigMigration],
    tags: ["cast_stdlib", "architecture"],
}
