//! `lease_ownership` — temporary ownership with expiry.

/// Sentinel for `lease_ownership`.
pub struct LeaseOwnership;

cast::concept! {
    name: "lease_ownership",
    summary: "Temporary ownership with expiry. The holder must renew \
              before the lease ends; if they crash or partition away, \
              the lease lapses without manual intervention.",
    anchors: [cast_stdlib::distributed::lease_ownership::LeaseOwnership],
    tags: ["cast_stdlib", "distributed"],
}
