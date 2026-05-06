//! `declarative_infrastructure_system` — users declare desired state, the platform realises it.

/// Sentinel for `declarative_infrastructure_system`.
pub struct DeclarativeInfrastructureSystem;

cast::concept! {
    name: "declarative_infrastructure_system",
    summary: "Users declare what should exist; the platform figures \
              out how to create and maintain it. Composes \
              desired_state_model, configuration_schema, \
              policy_as_code, reconciliation_loop, drift_detection, \
              resource_finalizer, idempotent_operation, and \
              rollback_operation. Used for cluster management, home \
              cloud setup, infrastructure as code, service \
              installation, and storage/network provisioning.",
    anchors: [cast_stdlib::patterns::declarative_infrastructure_system::DeclarativeInfrastructureSystem],
    tags: ["cast_stdlib", "patterns"],
}
