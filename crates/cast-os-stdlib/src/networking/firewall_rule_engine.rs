//! `firewall_rule_engine` — rules controlling network flow.

/// Sentinel for `firewall_rule_engine`.
pub struct FirewallRuleEngine;

cast::concept! {
    name: "firewall_rule_engine",
    summary: "rules controlling network flow.",
    anchors: [cast_os_stdlib::networking::firewall_rule_engine::FirewallRuleEngine],
    tags: ["cast_os_stdlib", "networking"],
}
