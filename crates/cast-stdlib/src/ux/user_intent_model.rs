//! `user_intent_model` — capture what user wants, not just clicked.

/// Sentinel for `user_intent_model`.
pub struct UserIntentModel;

cast::concept! {
    name: "user_intent_model",
    summary: "Model what the user is trying to accomplish, not just \
              the literal events they fired. \"Clicked submit\" is an \
              event; \"wanted to publish\" is the intent — and \
              recovery, undo, and confirmation should reason about the \
              latter.",
    anchors: [cast_stdlib::ux::user_intent_model::UserIntentModel],
    tags: ["cast_stdlib", "ux"],
}
