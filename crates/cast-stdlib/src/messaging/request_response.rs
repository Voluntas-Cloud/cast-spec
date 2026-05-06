//! `request_response` — caller waits for direct synchronous reply.

/// Sentinel for `request_response`.
pub struct RequestResponse;

cast::concept! {
    name: "request_response",
    summary: "Caller waits for direct response. Synchronous interaction; \
              the caller knows when the work is done.",
    anchors: [cast_stdlib::messaging::request_response::RequestResponse],
    tags: ["cast_stdlib", "messaging"],
}
