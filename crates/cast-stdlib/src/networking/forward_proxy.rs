//! `forward_proxy` — client-side proxy to external services.

/// Sentinel for `forward_proxy`.
pub struct ForwardProxy;

cast::concept! {
    name: "forward_proxy",
    summary: "Client-side proxy to external services. Outbound traffic \
              is funnelled through a known waypoint that can authenticate, \
              filter, and audit who reached what.",
    anchors: [cast_stdlib::networking::forward_proxy::ForwardProxy],
    tags: ["cast_stdlib", "networking"],
}
