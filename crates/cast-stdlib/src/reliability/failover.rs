//! `failover` — switch to backup when primary fails.

/// Sentinel for `failover`.
pub struct Failover;

cast::concept! {
    name: "failover",
    summary: "Switch to backup when primary fails. The detection step \
              and the cutover step are separately load-bearing — both \
              are common sources of split-brain.",
    anchors: [cast_stdlib::reliability::failover::Failover],
    tags: ["cast_stdlib", "reliability"],
}
