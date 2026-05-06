//! `http_routing` — route based on HTTP metadata.

/// Sentinel for `http_routing`.
pub struct HttpRouting;

cast::concept! {
    name: "http_routing",
    summary: "Route based on HTTP metadata. Host, path, headers, and \
              method pick the backend; the proxy is application-aware \
              and can rewrite, fan out, or shadow requests.",
    anchors: [cast_stdlib::networking::http_routing::HttpRouting],
    tags: ["cast_stdlib", "networking"],
}
