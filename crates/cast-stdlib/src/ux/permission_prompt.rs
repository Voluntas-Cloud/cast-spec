//! `permission_prompt` — ask before sensitive action.

/// Sentinel for `permission_prompt`.
pub struct PermissionPrompt;

cast::concept! {
    name: "permission_prompt",
    summary: "Ask the user before doing something sensitive on their \
              behalf. The prompt should describe what is being \
              requested specifically; \"allow access?\" without scope \
              is consent theater.",
    anchors: [cast_stdlib::ux::permission_prompt::PermissionPrompt],
    tags: ["cast_stdlib", "ux"],
}
