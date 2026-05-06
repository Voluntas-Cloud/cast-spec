//! `scaffold_template` — repeatable project structure.

/// Sentinel for `scaffold_template`.
pub struct ScaffoldTemplate;

cast::concept! {
    name: "scaffold_template",
    summary: "A repeatable starting structure for new projects, \
              services, or modules. Encodes the conventions the team \
              actually wants — not the ones in the README that \
              nobody updated since 2019.",
    anchors: [cast_stdlib::project_layout::scaffold_template::ScaffoldTemplate],
    tags: ["cast_stdlib", "project_layout"],
}
