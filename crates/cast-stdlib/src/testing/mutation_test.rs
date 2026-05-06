//! `mutation_test` — verify tests catch intentional code mutations.

/// Sentinel for `mutation_test`.
pub struct MutationTest;

cast::concept! {
    name: "mutation_test",
    summary: "Verify tests catch intentional code mutations. The \
              tooling makes small changes to the code under test; \
              if no test fails, the test suite has a hole exactly \
              where the mutation lives.",
    anchors: [cast_stdlib::testing::mutation_test::MutationTest],
    tags: ["cast_stdlib", "testing"],
}
