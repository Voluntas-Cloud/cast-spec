//! `partial_response` — client requests subset of fields.

/// Sentinel for `partial_response`.
pub struct PartialResponse;

cast::concept! {
    name: "partial_response",
    summary: "Client requests subset of fields. Reduces payload size and \
              lets the server skip expensive joins for unrequested fields.",
    anchors: [cast_stdlib::api::partial_response::PartialResponse],
    tags: ["cast_stdlib", "api"],
}
