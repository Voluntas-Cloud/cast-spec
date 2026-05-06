//! `lint_rule_set` — static style/correctness rules.

/// Sentinel for `lint_rule_set`.
pub struct LintRuleSet;

cast::concept! {
    name: "lint_rule_set",
    summary: "Static checks for style and correctness, codified and \
              enforced in CI. Linting moves debates from review \
              comments into a config file, where they can be \
              resolved once instead of weekly.",
    anchors: [cast_stdlib::project_layout::lint_rule_set::LintRuleSet],
    tags: ["cast_stdlib", "project_layout"],
}
