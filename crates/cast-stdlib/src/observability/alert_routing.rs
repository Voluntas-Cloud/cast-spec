//! `alert_routing` — send alerts to responsible humans/systems.

/// Sentinel for `alert_routing`.
pub struct AlertRouting;

cast::concept! {
    name: "alert_routing",
    summary: "Send alerts to responsible humans/systems. The right \
              alert must reach the right responder at the right time; \
              everything else is paging into the void.",
    anchors: [cast_stdlib::observability::alert_routing::AlertRouting],
    tags: ["cast_stdlib", "observability"],
}
