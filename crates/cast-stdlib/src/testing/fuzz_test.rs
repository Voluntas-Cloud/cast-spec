//! `fuzz_test` — feed unexpected/random input.

/// Sentinel for `fuzz_test`.
pub struct FuzzTest;

cast::concept! {
    name: "fuzz_test",
    summary: "Feed unexpected/random input. Coverage-guided generation \
              finds the inputs the developer never thought of; \
              especially good at finding crashes and assertion \
              violations.",
    anchors: [cast_stdlib::testing::fuzz_test::FuzzTest],
    tags: ["cast_stdlib", "testing"],
}
