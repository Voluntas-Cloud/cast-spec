//! `spawn_model` — create process directly from executable.

/// Sentinel for `spawn_model`.
pub struct SpawnModel;

cast::concept! {
    name: "spawn_model",
    summary: "create process directly from executable.",
    anchors: [cast_os_stdlib::execution_model::spawn_model::SpawnModel],
    tags: ["cast_os_stdlib", "execution_model"],
}
