//! `graceful_degradation` — reduced capability instead of total failure.

/// Sentinel for `graceful_degradation`.
pub struct GracefulDegradation;

cast::concept! {
    name: "graceful_degradation",
    summary: "Reduce capability instead of total failure. Read-only \
              mode, queue requests, serve cached responses — \
              anything but a hard outage.",
    anchors: [cast_stdlib::lifecycle::graceful_degradation::GracefulDegradation],
    tags: ["cast_stdlib", "lifecycle"],
}
