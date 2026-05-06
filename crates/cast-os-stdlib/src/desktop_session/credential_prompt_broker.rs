//! `credential_prompt_broker` — OS-mediated privilege prompt.

/// Sentinel for `credential_prompt_broker`.
pub struct CredentialPromptBroker;

cast::concept! {
    name: "credential_prompt_broker",
    summary: "OS-mediated privilege prompt.",
    anchors: [cast_os_stdlib::desktop_session::credential_prompt_broker::CredentialPromptBroker],
    tags: ["cast_os_stdlib", "desktop_session"],
}
