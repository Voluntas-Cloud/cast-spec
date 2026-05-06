//! `stress_test` — push beyond expected limits.

/// Sentinel for `stress_test`.
pub struct StressTest;

cast::concept! {
    name: "stress_test",
    summary: "Push beyond expected limits. Reveals the failure mode \
              the system enters under overload — graceful degradation \
              vs cascading collapse.",
    anchors: [cast_stdlib::testing::stress_test::StressTest],
    tags: ["cast_stdlib", "testing"],
}
