//! `context_isolation` — separate unrelated tasks/conversations.

/// Sentinel for `context_isolation`.
pub struct ContextIsolation;

cast::concept! {
    name: "context_isolation",
    summary: "Separate unrelated tasks and conversations so they \
              don't bleed into each other. Cross-talk between \
              contexts is how secrets leak, instructions leak, and \
              the agent suddenly thinks it's still doing yesterday's \
              task.",
    anchors: [cast_stdlib::ai::context_isolation::ContextIsolation],
    tags: ["cast_stdlib", "ai"],
}
