//! `acceptance_test` — validate user/business requirement.

/// Sentinel for `acceptance_test`.
pub struct AcceptanceTest;

cast::concept! {
    name: "acceptance_test",
    summary: "Validate user/business requirement. Phrased in domain \
              terms, not implementation terms; passing means the \
              behavior the customer asked for actually exists.",
    anchors: [cast_stdlib::testing::acceptance_test::AcceptanceTest],
    tags: ["cast_stdlib", "testing"],
}
