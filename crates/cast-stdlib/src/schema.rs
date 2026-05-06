//! Data modeling & schema evolution patterns.
//!
//! Each concept lives in its own submodule. This module file holds
//! only the group sentinel and the category rule.

pub mod backward_compatible_schema;
pub mod canonical_domain_model;
pub mod command_schema;
pub mod denormalization;
pub mod dual_write_migration;
pub mod entity_value_object_split;
pub mod event_schema;
pub mod explicit_nullability;
pub mod forward_compatible_schema;
pub mod lazy_migration;
pub mod migration_checkpoint;
pub mod normalization;
pub mod projection_model;
pub mod read_repair;
pub mod schema_definition;
pub mod schema_migration;
pub mod schema_registry;
pub mod semantic_versioning_for_data;
pub mod sum_type_modeling;

cast::concept! {
    name: "schema",
    summary: "Umbrella for the schema stdlib category. Data modeling & \
              schema evolution patterns.",
    anchors: [
        crate::schema::backward_compatible_schema,
        crate::schema::canonical_domain_model,
        crate::schema::command_schema,
        crate::schema::denormalization,
        crate::schema::dual_write_migration,
        crate::schema::entity_value_object_split,
        crate::schema::event_schema,
        crate::schema::explicit_nullability,
        crate::schema::forward_compatible_schema,
        crate::schema::lazy_migration,
        crate::schema::migration_checkpoint,
        crate::schema::normalization,
        crate::schema::projection_model,
        crate::schema::read_repair,
        crate::schema::schema_definition,
        crate::schema::schema_migration,
        crate::schema::schema_registry,
        crate::schema::semantic_versioning_for_data,
        crate::schema::sum_type_modeling,
    ],
    tags: ["cast_stdlib", "schema"],
}

/// Sentinel for the schema stdlib group.
pub struct SchemaGroup;

cast::rule! {
    rule: "Design for old and new versions of a schema to coexist.",
    why:  "'We'll migrate everything at deploy time' is adorable in \
           the same way juggling knives is adorable — production \
           traffic doesn't pause for your migration, and any rollback \
           leaves you straddling formats anyway.",
    governs: [cast_stdlib::schema::SchemaGroup],
    tags: ["cast_stdlib", "schema"],
}
