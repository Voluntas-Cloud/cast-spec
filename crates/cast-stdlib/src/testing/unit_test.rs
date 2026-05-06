//! `unit_test` — test isolated logic.

/// Sentinel for `unit_test`.
pub struct UnitTest;

cast::concept! {
    name: "unit_test",
    summary: "Test isolated logic. Single function or component, \
              dependencies stubbed; fast, deterministic, and the bulk \
              of the suite by count.",
    anchors: [cast_stdlib::testing::unit_test::UnitTest],
    tags: ["cast_stdlib", "testing"],
}
