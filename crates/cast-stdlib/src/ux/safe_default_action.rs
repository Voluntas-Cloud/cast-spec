//! `safe_default_action` — default does not cause harm.

/// Sentinel for `safe_default_action`.
pub struct SafeDefaultAction;

cast::concept! {
    name: "safe_default_action",
    summary: "The default action — Enter, the highlighted button, the \
              top of the menu — must not destroy data. Make the safe \
              path the easy path, and the destructive path the \
              deliberate one.",
    anchors: [cast_stdlib::ux::safe_default_action::SafeDefaultAction],
    tags: ["cast_stdlib", "ux"],
}
