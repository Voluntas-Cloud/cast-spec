//! `liveness_check` — determine if a service should be restarted.

/// Sentinel for `liveness_check`.
pub struct LivenessCheck;

cast::concept! {
    name: "liveness_check",
    summary: "Determine if service should be restarted. Failing means \
              the process is wedged; the platform reacts by killing \
              and restarting it.",
    anchors: [cast_stdlib::reliability::liveness_check::LivenessCheck],
    tags: ["cast_stdlib", "reliability"],
}
