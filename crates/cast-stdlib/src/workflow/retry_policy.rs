//! `retry_policy` — structured retry rules.

/// Sentinel for `retry_policy`.
pub struct RetryPolicy;

cast::concept! {
    name: "retry_policy",
    summary: "Structured retry rules. The policy specifies which errors \
              retry, how many times, with what backoff and jitter; \
              ad-hoc per-call retries are how thundering herds are born.",
    anchors: [cast_stdlib::workflow::retry_policy::RetryPolicy],
    tags: ["cast_stdlib", "workflow"],
}
