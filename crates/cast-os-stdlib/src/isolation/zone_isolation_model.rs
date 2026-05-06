//! `zone_isolation_model` — Solaris-style OS virtualization.

/// Sentinel for `zone_isolation_model`.
pub struct ZoneIsolationModel;

cast::concept! {
    name: "zone_isolation_model",
    summary: "Solaris-style OS virtualization.",
    anchors: [cast_os_stdlib::isolation::zone_isolation_model::ZoneIsolationModel],
    tags: ["cast_os_stdlib", "isolation"],
}
