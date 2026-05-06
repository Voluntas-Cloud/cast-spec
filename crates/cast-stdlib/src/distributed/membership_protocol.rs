//! `membership_protocol` — determine participating nodes.

/// Sentinel for `membership_protocol`.
pub struct MembershipProtocol;

cast::concept! {
    name: "membership_protocol",
    summary: "Determine participating nodes. The set of nodes that \
              count toward quorum, replication, and broadcast; \
              changes as nodes join and leave.",
    anchors: [cast_stdlib::distributed::membership_protocol::MembershipProtocol],
    tags: ["cast_stdlib", "distributed"],
}
