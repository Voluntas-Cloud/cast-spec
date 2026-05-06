//! `load_test` — measure under expected traffic.

/// Sentinel for `load_test`.
pub struct LoadTest;

cast::concept! {
    name: "load_test",
    summary: "Measure under expected traffic. Verifies the system \
              meets latency and throughput targets at the load it is \
              actually built for.",
    anchors: [cast_stdlib::testing::load_test::LoadTest],
    tags: ["cast_stdlib", "testing"],
}
