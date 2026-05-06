//! `rolling_update` — gradually replace instances.

/// Sentinel for `rolling_update`.
pub struct RollingUpdate;

cast::concept! {
    name: "rolling_update",
    summary: "Gradually replace instances. New replicas come up, old \
              replicas drain and stop; the service stays available \
              throughout, paid for by running mixed versions for a \
              window.",
    anchors: [cast_stdlib::deployment::rolling_update::RollingUpdate],
    tags: ["cast_stdlib", "deployment"],
}
