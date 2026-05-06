//! `example_driven_documentation` — teach by concrete examples.

/// Sentinel for `example_driven_documentation`.
pub struct ExampleDrivenDocumentation;

cast::concept! {
    name: "example_driven_documentation",
    summary: "Teach by concrete examples. A worked example pinned next \
              to the abstract description gets read; the abstract \
              description on its own usually doesn't.",
    anchors: [cast_stdlib::docs::example_driven_documentation::ExampleDrivenDocumentation],
    tags: ["cast_stdlib", "docs"],
}
