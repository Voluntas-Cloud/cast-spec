//! `memory_layering` — split short-term, long-term, project, user memory.

/// Sentinel for `memory_layering`.
pub struct MemoryLayering;

cast::concept! {
    name: "memory_layering",
    summary: "Different kinds of memory live in different layers: the \
              current turn, the conversation, the project, and the \
              user across projects. Each has different retention, \
              redaction, and trust rules.",
    anchors: [cast_stdlib::ai::memory_layering::MemoryLayering],
    tags: ["cast_stdlib", "ai"],
}
