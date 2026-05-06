//! `expand_contract_migration` — schema or API migrations happen in safe stages without downtime.

/// Sentinel for `expand_contract_migration`.
pub struct ExpandContractMigration;

cast::concept! {
    name: "expand_contract_migration",
    summary: "Schema or API migrations happen in safe stages \
              without downtime. Composes backward_compatible_schema, \
              forward_compatible_schema, dual_write_migration, \
              lazy_migration, read_repair, database_expand_contract, \
              deprecation_lifecycle, and compatibility_test_suite. \
              Used for database schema changes, API evolution, event \
              schema migration, stored document migration, and \
              multi-version client support.",
    anchors: [cast_stdlib::patterns::expand_contract_migration::ExpandContractMigration],
    tags: ["cast_stdlib", "patterns"],
}
