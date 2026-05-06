//! `undoable_action` — user can reverse mistakes.

/// Sentinel for `undoable_action`.
pub struct UndoableAction;

cast::concept! {
    name: "undoable_action",
    summary: "Let the user reverse a mistake. Soft-delete and \
              \"recently changed\" beat a confirmation modal, because \
              users will train themselves to dismiss the modal long \
              before they stop misclicking.",
    anchors: [cast_stdlib::ux::undoable_action::UndoableAction],
    tags: ["cast_stdlib", "ux"],
}
