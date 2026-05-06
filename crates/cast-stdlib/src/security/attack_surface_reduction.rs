//! `attack_surface_reduction` — minimize exposed functionality.

/// Sentinel for `attack_surface_reduction`.
pub struct AttackSurfaceReduction;

cast::concept! {
    name: "attack_surface_reduction",
    summary: "Minimize exposed functionality. Endpoints, ports, debug \
              hooks, and admin tooling that aren't needed are removed; \
              every reachable feature is a feature an attacker can \
              study.",
    anchors: [cast_stdlib::security::attack_surface_reduction::AttackSurfaceReduction],
    tags: ["cast_stdlib", "security"],
}
