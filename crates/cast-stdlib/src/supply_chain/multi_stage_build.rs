//! `multi_stage_build` — separate build environment from runtime image.

/// Sentinel for `multi_stage_build`.
pub struct MultiStageBuild;

cast::concept! {
    name: "multi_stage_build",
    summary: "Separate build environment from runtime image. Compilers, \
              package managers, and intermediate files stay in the \
              builder; only the produced binary ships.",
    anchors: [cast_stdlib::supply_chain::multi_stage_build::MultiStageBuild],
    tags: ["cast_stdlib", "supply_chain"],
}
