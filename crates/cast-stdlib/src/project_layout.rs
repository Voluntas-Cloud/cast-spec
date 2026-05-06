//! Repository, project & code organization patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod branching_strategy;
pub mod change_proposal;
pub mod code_owner_mapping;
pub mod commit_convention;
pub mod dependency_graph;
pub mod formatting_standard;
pub mod generated_code_boundary;
pub mod issue_as_code;
pub mod lint_rule_set;
pub mod module_visibility_rule;
pub mod monorepo;
pub mod polyrepo;
pub mod scaffold_template;
pub mod source_of_truth_file;
pub mod workspace_manifest;

cast::concept! {
    name: "project_layout",
    summary: "Umbrella for the project_layout stdlib category. Repository, \
              project & code organization patterns.",
    anchors: [
        crate::project_layout::branching_strategy,
        crate::project_layout::change_proposal,
        crate::project_layout::code_owner_mapping,
        crate::project_layout::commit_convention,
        crate::project_layout::dependency_graph,
        crate::project_layout::formatting_standard,
        crate::project_layout::generated_code_boundary,
        crate::project_layout::issue_as_code,
        crate::project_layout::lint_rule_set,
        crate::project_layout::module_visibility_rule,
        crate::project_layout::monorepo,
        crate::project_layout::polyrepo,
        crate::project_layout::scaffold_template,
        crate::project_layout::source_of_truth_file,
        crate::project_layout::workspace_manifest,
    ],
    tags: ["cast_stdlib", "project_layout"],
}

/// Sentinel for the project_layout stdlib group.
pub struct ProjectLayoutGroup;

cast::rule! {
    rule: "Make the repo navigable by machines and tired humans.",
    why:  "Both are your users, and one of them drinks coffee. \
           Conventions, ownership, and module boundaries make a repo \
           something a new contributor can survive on day one rather \
           than something they have to be initiated into.",
    governs: [cast_stdlib::project_layout::ProjectLayoutGroup],
    tags: ["cast_stdlib", "project_layout"],
}
