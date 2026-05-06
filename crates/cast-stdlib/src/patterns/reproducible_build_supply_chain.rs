//! `reproducible_build_supply_chain` — build artifacts can be reproduced, verified, signed, and traced.

/// Sentinel for `reproducible_build_supply_chain`.
pub struct ReproducibleBuildSupplyChain;

cast::concept! {
    name: "reproducible_build_supply_chain",
    summary: "Build artifacts can be reproduced, verified, signed, \
              and traced. Composes reproducible_build, \
              hermetic_build, pinned_dependency, \
              lockfile_dependency_graph, software_bill_of_materials, \
              signed_artifact, provenance_attestation, and \
              artifact_registry. Used for open-source releases, \
              secure platform builds, Voluntas image building, \
              regulated deployments, and defense against \
              supply-chain attacks.",
    anchors: [cast_stdlib::patterns::reproducible_build_supply_chain::ReproducibleBuildSupplyChain],
    tags: ["cast_stdlib", "patterns"],
}
