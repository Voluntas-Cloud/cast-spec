//! `hermetic_build` — build does not depend on ambient machine state.

/// Sentinel for `hermetic_build`.
pub struct HermeticBuild;

cast::concept! {
    name: "hermetic_build",
    summary: "Build does not depend on ambient machine state. No \
              network access mid-build, no leaking host paths, no \
              implicit environment variables — inputs are explicit.",
    anchors: [cast_stdlib::supply_chain::hermetic_build::HermeticBuild],
    tags: ["cast_stdlib", "supply_chain"],
}
