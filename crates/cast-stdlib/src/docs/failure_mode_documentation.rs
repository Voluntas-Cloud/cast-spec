//! `failure_mode_documentation` — document known failures.

/// Sentinel for `failure_mode_documentation`.
pub struct FailureModeDocumentation;

cast::concept! {
    name: "failure_mode_documentation",
    summary: "Document known failure modes and what they look like. \
              \"This symptom usually means X\" turns oncall \
              archaeology into oncall lookup.",
    anchors: [cast_stdlib::docs::failure_mode_documentation::FailureModeDocumentation],
    tags: ["cast_stdlib", "docs"],
}
