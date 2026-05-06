//! `capability_discovery` — client asks what server supports.

/// Sentinel for `capability_discovery`.
pub struct CapabilityDiscovery;

cast::concept! {
    name: "capability_discovery",
    summary: "Client asks what server supports. Lets clients gracefully \
              handle servers at different versions without hard-coding \
              capability matrices.",
    anchors: [cast_stdlib::api::capability_discovery::CapabilityDiscovery],
    tags: ["cast_stdlib", "api"],
}
