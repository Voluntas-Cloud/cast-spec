//! `command_message` — instruction to perform work.

/// Sentinel for `command_message`.
pub struct CommandMessage;

cast::concept! {
    name: "command_message",
    summary: "Instruction to perform work. Imperative; may be rejected; \
              usually targeted at a specific principal or service.",
    anchors: [cast_stdlib::messaging::command_message::CommandMessage],
    tags: ["cast_stdlib", "messaging"],
}
