//! `compatibility_test_suite` — contract tests as executable contract.

/// Sentinel for `compatibility_test_suite`.
pub struct CompatibilityTestSuite;

cast::concept! {
    name: "compatibility_test_suite",
    summary: "Contract tests for implementations. Any implementation \
              must pass the suite; the suite is the executable contract.",
    anchors: [cast_stdlib::api::compatibility_test_suite::CompatibilityTestSuite],
    tags: ["cast_stdlib", "api"],
}
