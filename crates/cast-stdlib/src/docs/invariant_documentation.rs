//! `invariant_documentation` — record what must always be true.

/// Sentinel for `invariant_documentation`.
pub struct InvariantDocumentation;

cast::concept! {
    name: "invariant_documentation",
    summary: "Record the conditions that must always hold. Invariants \
              are the assertions a refactor must preserve; if nobody \
              wrote them down, the refactor will discover them by \
              breaking them.",
    anchors: [cast_stdlib::docs::invariant_documentation::InvariantDocumentation],
    tags: ["cast_stdlib", "docs"],
}
