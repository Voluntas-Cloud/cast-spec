//! `module_visibility_rule` — restrict internal imports.

/// Sentinel for `module_visibility_rule`.
pub struct ModuleVisibilityRule;

cast::concept! {
    name: "module_visibility_rule",
    summary: "Restrict which modules can import which. Encodes \
              architectural intent — \"the storage layer must not \
              call the API layer\" — as something the compiler \
              enforces, not something the next reviewer has to spot.",
    anchors: [cast_stdlib::project_layout::module_visibility_rule::ModuleVisibilityRule],
    tags: ["cast_stdlib", "project_layout"],
}
