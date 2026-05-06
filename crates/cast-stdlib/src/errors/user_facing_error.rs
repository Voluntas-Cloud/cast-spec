//! `user_facing_error` — safe explanation for user.

/// Sentinel for `user_facing_error`.
pub struct UserFacingError;

cast::concept! {
    name: "user_facing_error",
    summary: "Safe explanation for the user. Tells them what they can \
              do next without leaking implementation; correlation IDs \
              connect what the user sees to the operator-facing detail.",
    anchors: [cast_stdlib::errors::user_facing_error::UserFacingError],
    tags: ["cast_stdlib", "errors"],
}
