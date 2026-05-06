//! `traffic_control_qdisc` — Linux queueing discipline layer.

/// Sentinel for `traffic_control_qdisc`.
pub struct TrafficControlQdisc;

cast::concept! {
    name: "traffic_control_qdisc",
    summary: "Linux queueing discipline layer.",
    anchors: [cast_os_stdlib::networking::traffic_control_qdisc::TrafficControlQdisc],
    tags: ["cast_os_stdlib", "networking"],
}
