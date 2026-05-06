//! `property_based_test` — generate cases from invariants.

/// Sentinel for `property_based_test`.
pub struct PropertyBasedTest;

cast::concept! {
    name: "property_based_test",
    summary: "Generate cases from invariants. The developer states a \
              property that must hold; the framework generates inputs \
              and shrinks failing cases to a minimal counterexample.",
    anchors: [cast_stdlib::testing::property_based_test::PropertyBasedTest],
    tags: ["cast_stdlib", "testing"],
}
