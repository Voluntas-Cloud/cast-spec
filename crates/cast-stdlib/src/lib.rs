//! cast-stdlib — a curated library of architectural concepts.
//!
//! Each `cast::concept!` block in this crate names a recurring
//! architectural pattern (copy-on-write overlay, capability token,
//! MVCC generation log, …) with a short summary, and is paired with
//! a `cast::rule!` block carrying the category's load-bearing
//! best-practice. Downstream projects pull cast-stdlib in and
//! reference these concepts through `cast::continues_in!`,
//! inheriting the survey work rather than re-deriving it.
//!
//! The Rust-level surface is intentionally minimal. Each concept has
//! a zero-sized sentinel struct purely so the cast analyzer has a
//! real path to anchor at. Nothing in this crate executes.

pub mod ai;
pub mod algorithms;
pub mod api;
pub mod architecture;
pub mod bugs;
pub mod config;
pub mod consistency;
pub mod deployment;
pub mod distributed;
pub mod docs;
pub mod errors;
pub mod function_properties;
pub mod identity;
pub mod type_properties;
pub mod integration;
pub mod lifecycle;
pub mod messaging;
pub mod networking;
pub mod observability;
pub mod patterns;
pub mod performance;
pub mod privacy;
pub mod project_layout;
pub mod reliability;
pub mod resources;
pub mod schema;
pub mod security;
pub mod state_data;
pub mod storage;
pub mod supply_chain;
pub mod testing;
pub mod time_ordering;
pub mod trust;
pub mod ux;
pub mod workflow;

cast::concept! {
    name: "cast_stdlib",
    summary: "Per-crate umbrella for the architectural concept catalog. \
              Pulls every concept under one of the 32 category modules \
              (storage, identity, trust, lifecycle, messaging, …) \
              underneath this node by longest-prefix-match on the \
              module-level anchors below. Without this umbrella, \
              cast-stdlib concepts are orphans in the canonical tree: \
              their anchors strict-prefix-match no other concept's \
              anchor, so `place_zero_anchor` falls back to picking an \
              alphabetically-first sibling and the resulting placement \
              cycle disconnects the whole crate from the workspace root.",
    anchors: [
        crate::ai,
        crate::api,
        crate::architecture,
        crate::bugs,
        crate::config,
        crate::consistency,
        crate::deployment,
        crate::distributed,
        crate::docs,
        crate::errors,
        crate::identity,
        crate::integration,
        crate::lifecycle,
        crate::messaging,
        crate::networking,
        crate::observability,
        crate::patterns,
        crate::performance,
        crate::privacy,
        crate::project_layout,
        crate::reliability,
        crate::resources,
        crate::schema,
        crate::security,
        crate::state_data,
        crate::storage,
        crate::supply_chain,
        crate::testing,
        crate::time_ordering,
        crate::trust,
        crate::ux,
        crate::workflow,
    ],
    tags: ["cast_stdlib"],
}
