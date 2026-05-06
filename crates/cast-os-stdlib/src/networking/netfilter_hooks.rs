//! `netfilter_hooks` — Linux packet processing hooks.

/// Sentinel for `netfilter_hooks`.
pub struct NetfilterHooks;

cast::concept! {
    name: "netfilter_hooks",
    summary: "Linux packet processing hooks.",
    anchors: [cast_os_stdlib::networking::netfilter_hooks::NetfilterHooks],
    tags: ["cast_os_stdlib", "networking"],
}
