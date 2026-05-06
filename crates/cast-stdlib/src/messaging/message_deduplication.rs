//! `message_deduplication` — receiver discards messages it has already handled.

/// Sentinel for `message_deduplication`.
pub struct MessageDeduplication;

cast::concept! {
    name: "message_deduplication",
    summary: "Avoid duplicate processing. Either by idempotency key, \
              content hash, or sequence number — receiver discards \
              messages it has already handled.",
    anchors: [cast_stdlib::messaging::message_deduplication::MessageDeduplication],
    tags: ["cast_stdlib", "messaging"],
}
