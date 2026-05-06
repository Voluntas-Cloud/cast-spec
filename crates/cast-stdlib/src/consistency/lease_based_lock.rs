//! `lease_based_lock` — lock that expires automatically.

/// Sentinel for `lease_based_lock`.
pub struct LeaseBasedLock;

cast::concept! {
    name: "lease_based_lock",
    summary: "Lock that expires automatically. Bounds the damage from \
              a holder that crashes or partitions away.",
    anchors: [cast_stdlib::consistency::lease_based_lock::LeaseBasedLock],
    tags: ["cast_stdlib", "consistency"],
}
