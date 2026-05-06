//! `interface_documentation` — describe API/tool behavior.

/// Sentinel for `interface_documentation`.
pub struct InterfaceDocumentation;

cast::concept! {
    name: "interface_documentation",
    summary: "Describe what an API or tool does, including what it \
              promises and what it doesn't. The contract is what \
              callers depend on; the implementation is allowed to \
              change underneath.",
    anchors: [cast_stdlib::docs::interface_documentation::InterfaceDocumentation],
    tags: ["cast_stdlib", "docs"],
}
