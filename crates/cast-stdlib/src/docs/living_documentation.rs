//! `living_documentation` — generated or validated from source.

/// Sentinel for `living_documentation`.
pub struct LivingDocumentation;

cast::concept! {
    name: "living_documentation",
    summary: "Documentation generated from, or validated against, the \
              source. Prose that drifts from the code is a liability; \
              prose that is checked by the build can be trusted.",
    anchors: [cast_stdlib::docs::living_documentation::LivingDocumentation],
    tags: ["cast_stdlib", "docs"],
}
