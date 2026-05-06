//! `confirmation_for_destructive_action` — require explicit confirmation.

/// Sentinel for `confirmation_for_destructive_action`.
pub struct ConfirmationForDestructiveAction;

cast::concept! {
    name: "confirmation_for_destructive_action",
    summary: "Require explicit confirmation before something \
              irreversible. The confirmation needs to make the cost \
              specific (\"delete 412 records\") rather than generic \
              (\"are you sure?\"), or users will rubber-stamp it.",
    anchors: [cast_stdlib::ux::confirmation_for_destructive_action::ConfirmationForDestructiveAction],
    tags: ["cast_stdlib", "ux"],
}
