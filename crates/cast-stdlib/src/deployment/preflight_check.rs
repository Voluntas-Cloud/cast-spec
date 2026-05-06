//! `preflight_check` — validate environment before change.

/// Sentinel for `preflight_check`.
pub struct PreflightCheck;

cast::concept! {
    name: "preflight_check",
    summary: "Validate environment before change. Capacity, config, \
              dependencies, and quotas verified before the deployer \
              starts moving things; catches problems while rollback \
              is cheap.",
    anchors: [cast_stdlib::deployment::preflight_check::PreflightCheck],
    tags: ["cast_stdlib", "deployment"],
}
