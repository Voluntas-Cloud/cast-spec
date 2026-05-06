//! `stable_logical_name` — name that survives relocation.

/// Sentinel for `stable_logical_name`.
pub struct StableLogicalName;

cast::concept! {
    name: "stable_logical_name",
    summary: "Human or system-readable name that survives relocation. \
              Decoupled from physical address; resolved through a \
              naming layer that can be re-pointed without renaming.",
    anchors: [cast_stdlib::identity::stable_logical_name::StableLogicalName],
    tags: ["cast_stdlib", "identity"],
}
