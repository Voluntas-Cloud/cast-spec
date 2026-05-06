//! `supply_chain_integrity` — verify dependencies/artifacts.

/// Sentinel for `supply_chain_integrity`.
pub struct SupplyChainIntegrity;

cast::concept! {
    name: "supply_chain_integrity",
    summary: "Verify dependencies and artifacts. Every input to the \
              build is pinned and signature-checked; trust shifts from \
              \"we trust the registry\" to \"we trust this exact \
              digest\".",
    anchors: [cast_stdlib::security::supply_chain_integrity::SupplyChainIntegrity],
    tags: ["cast_stdlib", "security"],
}
