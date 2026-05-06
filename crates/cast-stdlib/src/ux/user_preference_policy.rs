//! `user_preference_policy` — behavior shaped by user choices.

/// Sentinel for `user_preference_policy`.
pub struct UserPreferencePolicy;

cast::concept! {
    name: "user_preference_policy",
    summary: "Behaviour the user has chosen — theme, density, \
              notifications, defaults. Preferences should persist \
              across devices and survive upgrades; rediscovering them \
              at every login teaches users the system doesn't \
              remember them.",
    anchors: [cast_stdlib::ux::user_preference_policy::UserPreferencePolicy],
    tags: ["cast_stdlib", "ux"],
}
