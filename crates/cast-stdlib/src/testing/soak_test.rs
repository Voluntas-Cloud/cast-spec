//! `soak_test` — run for extended duration.

/// Sentinel for `soak_test`.
pub struct SoakTest;

cast::concept! {
    name: "soak_test",
    summary: "Run for extended duration. Memory leaks, slow resource \
              exhaustion, and accumulating clock drift only show up \
              after hours or days; soaks expose them.",
    anchors: [cast_stdlib::testing::soak_test::SoakTest],
    tags: ["cast_stdlib", "testing"],
}
