//! `regression_test` — prevent old bugs returning.

/// Sentinel for `regression_test`.
pub struct RegressionTest;

cast::concept! {
    name: "regression_test",
    summary: "Prevent old bugs returning. Each fixed bug leaves a test \
              behind that would have caught it; the test suite grows \
              into a memory of every mistake the project has made.",
    anchors: [cast_stdlib::testing::regression_test::RegressionTest],
    tags: ["cast_stdlib", "testing"],
}
