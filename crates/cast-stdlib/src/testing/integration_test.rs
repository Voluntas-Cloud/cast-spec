//! `integration_test` — test components together.

/// Sentinel for `integration_test`.
pub struct IntegrationTest;

cast::concept! {
    name: "integration_test",
    summary: "Test components together. Crosses the boundaries unit \
              tests stub out — real database, real queue, real \
              filesystem — catches bugs that live at interfaces.",
    anchors: [cast_stdlib::testing::integration_test::IntegrationTest],
    tags: ["cast_stdlib", "testing"],
}
