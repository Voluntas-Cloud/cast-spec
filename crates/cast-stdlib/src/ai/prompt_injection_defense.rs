//! `prompt_injection_defense` — prevent untrusted text from changing rules.

/// Sentinel for `prompt_injection_defense`.
pub struct PromptInjectionDefense;

cast::concept! {
    name: "prompt_injection_defense",
    summary: "Prevent untrusted text from rewriting the agent's \
              rules. Treat retrieved content, tool output, and user \
              messages as data, not instructions; the moment those \
              channels can override system prompts, the security \
              model is over.",
    anchors: [cast_stdlib::ai::prompt_injection_defense::PromptInjectionDefense],
    tags: ["cast_stdlib", "ai"],
}
