//! `database_expand_contract` — add new schema, migrate, then remove old.

/// Sentinel for `database_expand_contract`.
pub struct DatabaseExpandContract;

cast::concept! {
    name: "database_expand_contract",
    summary: "Add new schema, migrate, then remove old. Phase 1 adds \
              new columns/tables; phase 2 dual-writes; phase 3 reads \
              from new; phase 4 drops old. No phase requires \
              simultaneous code+data change.",
    anchors: [cast_stdlib::deployment::database_expand_contract::DatabaseExpandContract],
    tags: ["cast_stdlib", "deployment"],
}
