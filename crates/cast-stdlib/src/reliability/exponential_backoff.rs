//! `exponential_backoff` — delay grows exponentially across retries.

/// Sentinel for `exponential_backoff`.
pub struct ExponentialBackoff;

cast::concept! {
    name: "exponential_backoff",
    summary: "Delay grows exponentially. Each successive retry waits \
              roughly twice as long; total work converges and the \
              dependency gets time to recover.",
    anchors: [cast_stdlib::reliability::exponential_backoff::ExponentialBackoff],
    tags: ["cast_stdlib", "reliability"],
}
