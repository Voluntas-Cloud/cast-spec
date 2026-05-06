//! `documentation_tests` — verify examples still work.

/// Sentinel for `documentation_tests`.
pub struct DocumentationTests;

cast::concept! {
    name: "documentation_tests",
    summary: "Verify that documented examples still compile and run. \
              The point isn't elegance; it's that an example which \
              silently goes stale teaches the wrong thing to the next \
              person who copy-pastes it.",
    anchors: [cast_stdlib::docs::documentation_tests::DocumentationTests],
    tags: ["cast_stdlib", "docs"],
}
