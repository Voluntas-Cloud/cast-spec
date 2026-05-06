//! `chaos_test` — test failure behavior.

/// Sentinel for `chaos_test`.
pub struct ChaosTest;

cast::concept! {
    name: "chaos_test",
    summary: "Test failure behavior. Inject latency, kill processes, \
              partition networks — verify the system degrades the \
              way the design says it does.",
    anchors: [cast_stdlib::testing::chaos_test::ChaosTest],
    tags: ["cast_stdlib", "testing"],
}
