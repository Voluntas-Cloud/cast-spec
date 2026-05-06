//! `workspace_manifest` — declares project members.

/// Sentinel for `workspace_manifest`.
pub struct WorkspaceManifest;

cast::concept! {
    name: "workspace_manifest",
    summary: "Declares which packages or projects belong to the \
              workspace. The manifest is what makes the build system \
              and the tooling agree on the set of members; without \
              it, each tool grows its own definition.",
    anchors: [cast_stdlib::project_layout::workspace_manifest::WorkspaceManifest],
    tags: ["cast_stdlib", "project_layout"],
}
