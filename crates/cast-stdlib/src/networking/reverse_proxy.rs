//! `reverse_proxy` — accept client traffic and forward internally.

/// Sentinel for `reverse_proxy`.
pub struct ReverseProxy;

cast::concept! {
    name: "reverse_proxy",
    summary: "Accept client traffic and forward internally. Clients see \
              the proxy as the service; the real backend is hidden \
              behind it and can change shape without breaking callers.",
    anchors: [cast_stdlib::networking::reverse_proxy::ReverseProxy],
    tags: ["cast_stdlib", "networking"],
}
