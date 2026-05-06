//! `formal_verification` — prove properties mathematically.

/// Sentinel for `formal_verification`.
pub struct FormalVerification;

cast::concept! {
    name: "formal_verification",
    summary: "Prove properties mathematically. Stronger than testing — \
              shows the property holds for every input, not just the \
              ones the test sampled. Expensive; reserved for parts \
              that must not fail.",
    anchors: [cast_stdlib::testing::formal_verification::FormalVerification],
    tags: ["cast_stdlib", "testing"],
}
