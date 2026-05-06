//! `keepalive_probe` — detect dead connections.

/// Sentinel for `keepalive_probe`.
pub struct KeepaliveProbe;

cast::concept! {
    name: "keepalive_probe",
    summary: "Detect dead connections. Periodic small messages or TCP \
              keepalive expose silently-broken peers before the next \
              real request hits a 30-second timeout.",
    anchors: [cast_stdlib::networking::keepalive_probe::KeepaliveProbe],
    tags: ["cast_stdlib", "networking"],
}
