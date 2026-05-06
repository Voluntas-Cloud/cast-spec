//! `cqrs_split` — separate command and query models.

/// Sentinel for `cqrs_split`.
pub struct CqrsSplit;

cast::concept! {
    name: "cqrs_split",
    summary: "Separate the command (write) and query (read) models. \
              Each can scale and evolve independently; the cost is \
              the bookkeeping that keeps them in sync, which is \
              real and not free.",
    anchors: [cast_stdlib::state_data::cqrs_split::CqrsSplit],
    tags: ["cast_stdlib", "state_data"],
}
