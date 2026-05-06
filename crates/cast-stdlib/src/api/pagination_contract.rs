//! `pagination_contract` — bounded retrieval of large result sets.

/// Sentinel for `pagination_contract`.
pub struct PaginationContract;

cast::concept! {
    name: "pagination_contract",
    summary: "Bounded retrieval of large result sets. Cursor-based or \
              offset-based; the contract makes the iteration termination \
              condition explicit.",
    anchors: [cast_stdlib::api::pagination_contract::PaginationContract],
    tags: ["cast_stdlib", "api"],
}
