//! Testing & verification patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod acceptance_test;
pub mod chaos_test;
pub mod contract_test;
pub mod end_to_end_test;
pub mod formal_verification;
pub mod fuzz_test;
pub mod golden_file_test;
pub mod integration_test;
pub mod load_test;
pub mod model_checking;
pub mod mutation_test;
pub mod property_based_test;
pub mod regression_test;
pub mod security_test;
pub mod smoke_test;
pub mod snapshot_test;
pub mod soak_test;
pub mod static_analysis;
pub mod stress_test;
pub mod unit_test;

cast::concept! {
    name: "testing",
    summary: "Umbrella for the testing stdlib category. Testing & \
              verification patterns.",
    anchors: [
        crate::testing::acceptance_test,
        crate::testing::chaos_test,
        crate::testing::contract_test,
        crate::testing::end_to_end_test,
        crate::testing::formal_verification,
        crate::testing::fuzz_test,
        crate::testing::golden_file_test,
        crate::testing::integration_test,
        crate::testing::load_test,
        crate::testing::model_checking,
        crate::testing::mutation_test,
        crate::testing::property_based_test,
        crate::testing::regression_test,
        crate::testing::security_test,
        crate::testing::smoke_test,
        crate::testing::snapshot_test,
        crate::testing::soak_test,
        crate::testing::static_analysis,
        crate::testing::stress_test,
        crate::testing::unit_test,
    ],
    tags: ["cast_stdlib", "testing"],
}

/// Sentinel for the testing stdlib group.
pub struct TestingGroup;

cast::rule! {
    rule: "Test properties and boundaries, not just happy paths.",
    why:  "Happy paths are what demos use to lie politely. The bugs \
           that ship live in the empty inputs, the off-by-one indices, \
           the concurrent edits, and the network partitions — none of \
           which appear on the demo script.",
    governs: [cast_stdlib::testing::TestingGroup],
    tags: ["cast_stdlib", "testing"],
}
