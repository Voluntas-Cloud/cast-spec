//! `retry_with_backoff` — retry after increasing delay.

/// Sentinel for `retry_with_backoff`.
pub struct RetryWithBackoff;

cast::concept! {
    name: "retry_with_backoff",
    summary: "Retry after increasing delay. Transient failures often \
              resolve given a moment; backoff prevents the retry storm \
              that would otherwise pile on a struggling dependency.",
    anchors: [cast_stdlib::reliability::retry_with_backoff::RetryWithBackoff],
    tags: ["cast_stdlib", "reliability"],
}
