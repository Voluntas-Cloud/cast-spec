//! `golden_file_test` — compare against canonical fixture.

/// Sentinel for `golden_file_test`.
pub struct GoldenFileTest;

cast::concept! {
    name: "golden_file_test",
    summary: "Compare against canonical fixture. Useful for output that \
              is large and structured; the test fails when output \
              diverges from the checked-in golden file.",
    anchors: [cast_stdlib::testing::golden_file_test::GoldenFileTest],
    tags: ["cast_stdlib", "testing"],
}
