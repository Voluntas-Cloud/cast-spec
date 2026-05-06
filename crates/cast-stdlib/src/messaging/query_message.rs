//! `query_message` — request for information, side-effect-free.

/// Sentinel for `query_message`.
pub struct QueryMessage;

cast::concept! {
    name: "query_message",
    summary: "Request for information. Side-effect-free; the response \
              is the only effect that matters.",
    anchors: [cast_stdlib::messaging::query_message::QueryMessage],
    tags: ["cast_stdlib", "messaging"],
}
