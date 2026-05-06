//! `implementation_first_api` — implementation generates the contract.

/// Sentinel for `implementation_first_api`.
pub struct ImplementationFirstApi;

cast::concept! {
    name: "implementation_first_api",
    summary: "Implementation generates contract. The code is the source \
              of truth; the schema is generated from it.",
    anchors: [cast_stdlib::api::implementation_first_api::ImplementationFirstApi],
    tags: ["cast_stdlib", "api"],
}
