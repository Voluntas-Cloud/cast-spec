//! `generated_code_boundary` — separate generated and human-written code.

/// Sentinel for `generated_code_boundary`.
pub struct GeneratedCodeBoundary;

cast::concept! {
    name: "generated_code_boundary",
    summary: "Keep generated files in clearly marked locations. \
              Without the boundary, someone hand-edits a generated \
              file, the next regeneration overwrites it, and the \
              fix is rediscovered every six months.",
    anchors: [cast_stdlib::project_layout::generated_code_boundary::GeneratedCodeBoundary],
    tags: ["cast_stdlib", "project_layout"],
}
