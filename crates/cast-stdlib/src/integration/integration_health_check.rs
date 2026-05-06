//! `integration_health_check` — verify dependency connectivity.

/// Sentinel for `integration_health_check`.
pub struct IntegrationHealthCheck;

cast::concept! {
    name: "integration_health_check",
    summary: "Verify the dependency is reachable and behaving. The \
              check needs to exercise the actual contract, not just \
              ping the host — \"the TCP port answers\" is a famously \
              misleading definition of healthy.",
    anchors: [cast_stdlib::integration::integration_health_check::IntegrationHealthCheck],
    tags: ["cast_stdlib", "integration"],
}
