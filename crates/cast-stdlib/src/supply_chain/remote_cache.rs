//! `remote_cache` — shared cache across machines.

/// Sentinel for `remote_cache`.
pub struct RemoteCache;

cast::concept! {
    name: "remote_cache",
    summary: "Shared cache across machines. CI runners and developer \
              machines hit the same cache; correctness depends on \
              the cache being a function of inputs, not of the \
              machine.",
    anchors: [cast_stdlib::supply_chain::remote_cache::RemoteCache],
    tags: ["cast_stdlib", "supply_chain"],
}
