//! Build, packaging & supply-chain patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod artifact_manifest;
pub mod artifact_registry;
pub mod build_cache;
pub mod container_image_digest_pin;
pub mod dependency_vulnerability_scan;
pub mod hermetic_build;
pub mod license_compliance_scan;
pub mod lockfile_dependency_graph;
pub mod minimal_runtime_image;
pub mod multi_stage_build;
pub mod pinned_dependency;
pub mod promotion_pipeline;
pub mod provenance_attestation;
pub mod release_candidate;
pub mod remote_cache;
pub mod reproducible_build;
pub mod rollback_artifact;
pub mod semantic_versioning;
pub mod signed_artifact;
pub mod software_bill_of_materials;

cast::concept! {
    name: "supply_chain",
    summary: "Umbrella for the supply_chain stdlib category. Build, \
              packaging & supply-chain patterns.",
    anchors: [
        crate::supply_chain::artifact_manifest,
        crate::supply_chain::artifact_registry,
        crate::supply_chain::build_cache,
        crate::supply_chain::container_image_digest_pin,
        crate::supply_chain::dependency_vulnerability_scan,
        crate::supply_chain::hermetic_build,
        crate::supply_chain::license_compliance_scan,
        crate::supply_chain::lockfile_dependency_graph,
        crate::supply_chain::minimal_runtime_image,
        crate::supply_chain::multi_stage_build,
        crate::supply_chain::pinned_dependency,
        crate::supply_chain::promotion_pipeline,
        crate::supply_chain::provenance_attestation,
        crate::supply_chain::release_candidate,
        crate::supply_chain::remote_cache,
        crate::supply_chain::reproducible_build,
        crate::supply_chain::rollback_artifact,
        crate::supply_chain::semantic_versioning,
        crate::supply_chain::signed_artifact,
        crate::supply_chain::software_bill_of_materials,
    ],
    tags: ["cast_stdlib", "supply_chain"],
}

/// Sentinel for the supply-chain stdlib group.
pub struct SupplyChainGroup;

cast::rule! {
    rule: "Build once, promote many times.",
    why:  "Rebuilding per environment is a creative way to deploy \
           different software by accident. The artifact you tested \
           in staging must be the artifact that ships to production \
           — bit-for-bit, not 'logically equivalent'.",
    governs: [cast_stdlib::supply_chain::SupplyChainGroup],
    tags: ["cast_stdlib", "supply_chain"],
}
