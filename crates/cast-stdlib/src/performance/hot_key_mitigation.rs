//! `hot_key_mitigation` — prevent one key from overloading the system.

/// Sentinel for `hot_key_mitigation`.
pub struct HotKeyMitigation;

cast::concept! {
    name: "hot_key_mitigation",
    summary: "Prevent one key from overloading system. Sharding by \
              user, request coalescing, local caches — anything that \
              keeps a single popular key from monopolizing one \
              partition.",
    anchors: [cast_stdlib::performance::hot_key_mitigation::HotKeyMitigation],
    tags: ["cast_stdlib", "performance"],
}
