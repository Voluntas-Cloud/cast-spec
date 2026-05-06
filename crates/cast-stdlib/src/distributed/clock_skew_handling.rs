//! `clock_skew_handling` — tolerate unsynchronized clocks.

/// Sentinel for `clock_skew_handling`.
pub struct ClockSkewHandling;

cast::concept! {
    name: "clock_skew_handling",
    summary: "Tolerate unsynchronized clocks. Wall-clock time across \
              nodes drifts; protocols that rely on it must define a \
              tolerated skew and behave defensibly when reality \
              exceeds it.",
    anchors: [cast_stdlib::distributed::clock_skew_handling::ClockSkewHandling],
    tags: ["cast_stdlib", "distributed"],
}
