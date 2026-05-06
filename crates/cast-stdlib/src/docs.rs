//! Documentation & knowledge patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod architecture_decision_record;
pub mod concept_glossary;
pub mod decision_log;
pub mod deprecation_notice;
pub mod documentation_tests;
pub mod example_driven_documentation;
pub mod failure_mode_documentation;
pub mod interface_documentation;
pub mod invariant_documentation;
pub mod living_documentation;
pub mod migration_guide;
pub mod onboarding_guide;
pub mod playbook;
pub mod runbook;
pub mod schema_documentation;

cast::concept! {
    name: "docs",
    summary: "Umbrella for the docs stdlib category. Documentation & \
              knowledge patterns.",
    anchors: [
        crate::docs::architecture_decision_record,
        crate::docs::concept_glossary,
        crate::docs::decision_log,
        crate::docs::deprecation_notice,
        crate::docs::documentation_tests,
        crate::docs::example_driven_documentation,
        crate::docs::failure_mode_documentation,
        crate::docs::interface_documentation,
        crate::docs::invariant_documentation,
        crate::docs::living_documentation,
        crate::docs::migration_guide,
        crate::docs::onboarding_guide,
        crate::docs::playbook,
        crate::docs::runbook,
        crate::docs::schema_documentation,
    ],
    tags: ["cast_stdlib", "docs"],
}

/// Sentinel for the docs stdlib group.
pub struct DocsGroup;

cast::rule! {
    rule: "Document decisions, not just final shapes.",
    why:  "The \"why\" is the part future-you will actually need; the \
           shape is recoverable from the code, the reasoning is not.",
    governs: [cast_stdlib::docs::DocsGroup],
    tags: ["cast_stdlib", "docs"],
}
