//! `contract_test` — test API/provider compatibility.

/// Sentinel for `contract_test`.
pub struct ContractTest;

cast::concept! {
    name: "contract_test",
    summary: "Test API/provider compatibility. The contract is \
              expressed as a runnable suite; consumers and providers \
              both run it to confirm they agree on the surface.",
    anchors: [cast_stdlib::testing::contract_test::ContractTest],
    tags: ["cast_stdlib", "testing"],
}
