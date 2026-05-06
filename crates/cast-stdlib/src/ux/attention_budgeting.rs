//! `attention_budgeting` — avoid overwhelming the user.

/// Sentinel for `attention_budgeting`.
pub struct AttentionBudgeting;

cast::concept! {
    name: "attention_budgeting",
    summary: "Treat the user's attention as a finite resource the \
              system spends. Banners, badges, modals, and red dots \
              each charge against the budget; when everything is \
              urgent, nothing is.",
    anchors: [cast_stdlib::ux::attention_budgeting::AttentionBudgeting],
    tags: ["cast_stdlib", "ux"],
}
