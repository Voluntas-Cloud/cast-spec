//! `end_to_end_test` — test whole workflow.

/// Sentinel for `end_to_end_test`.
pub struct EndToEndTest;

cast::concept! {
    name: "end_to_end_test",
    summary: "Test whole workflow. Real services, real data flow, real \
              network — slow and flaky compared to unit tests but the \
              only way to catch integration bugs.",
    anchors: [cast_stdlib::testing::end_to_end_test::EndToEndTest],
    tags: ["cast_stdlib", "testing"],
}
