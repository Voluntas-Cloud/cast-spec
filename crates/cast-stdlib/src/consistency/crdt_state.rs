//! `crdt_state` — conflict-free replicated data type.

/// Sentinel for `crdt_state`.
pub struct CrdtState;

cast::concept! {
    name: "crdt_state",
    summary: "Conflict-free replicated data type. State whose merge \
              function is associative, commutative, and idempotent — \
              no coordination needed.",
    anchors: [cast_stdlib::consistency::crdt_state::CrdtState],
    tags: ["cast_stdlib", "consistency"],
}
