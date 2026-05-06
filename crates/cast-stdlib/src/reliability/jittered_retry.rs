//! `jittered_retry` — randomize retry timing to avoid thundering herd.

/// Sentinel for `jittered_retry`.
pub struct JitteredRetry;

cast::concept! {
    name: "jittered_retry",
    summary: "Randomize retries to avoid thundering herd. Without \
              jitter, every client whose call failed at the same \
              instant retries at the same instant.",
    anchors: [cast_stdlib::reliability::jittered_retry::JitteredRetry],
    tags: ["cast_stdlib", "reliability"],
}
