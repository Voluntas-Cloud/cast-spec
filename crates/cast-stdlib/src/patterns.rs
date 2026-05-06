//! Higher-level architectural patterns.
//!
//! Each pattern is a recurring composition of foundational concepts —
//! the shape of a system rather than a single primitive. Each lives
//! in its own submodule with a sentinel struct anchored at
//! `cast_stdlib::patterns::<pattern>::<Sentinel>`. The summary names
//! the constituent foundational concepts in prose so readers can walk
//! to the building blocks without the crate having to enumerate every
//! cross-anchor.

pub mod anti_corruption_boundary;
pub mod api_gateway_with_contract_enforcement;
pub mod append_only_audit_trail;
pub mod backup_restore_and_rebuild_system;
pub mod bounded_ai_agent_execution;
pub mod cache_projection_layer;
pub mod capability_based_access_system;
pub mod command_queue_with_idempotent_handlers;
pub mod consent_based_data_access;
pub mod content_addressed_artifact_pipeline;
pub mod cost_aware_cloud_bursting;
pub mod cqrs_read_write_split;
pub mod data_lineage_and_provenance_system;
pub mod declarative_infrastructure_system;
pub mod derived_state_rebuild_system;
pub mod distributed_replicated_log;
pub mod domain_event_automation_system;
pub mod durable_workflow_engine;
pub mod event_driven_service_platform;
pub mod event_sourced_service;
pub mod expand_contract_migration;
pub mod fault_tolerant_cluster_control;
pub mod human_approved_automation_flow;
pub mod incident_response_system;
pub mod integration_hub;
pub mod leader_elected_controller;
pub mod local_first_encrypted_personal_store;
pub mod message_bus_backbone;
pub mod multi_tenant_isolation_platform;
pub mod observability_fabric;
pub mod offline_first_sync_engine;
pub mod operator_console_and_control_surface;
pub mod plugin_extension_platform;
pub mod policy_driven_resource_scheduler;
pub mod privacy_rights_workflow;
pub mod progressive_delivery_system;
pub mod rag_knowledge_system;
pub mod replay_protected_pairing_flow;
pub mod reproducible_build_supply_chain;
pub mod sealed_secret_management_system;
pub mod search_and_indexing_pipeline;
pub mod secure_bootstrap_chain;
pub mod secure_signed_command_system;
pub mod self_describing_platform;
pub mod self_healing_reconciler_control_plane;
pub mod temporal_state_system;
pub mod tiered_data_lifecycle_system;
pub mod versioned_configuration_store;
pub mod webhook_delivery_system;
pub mod zero_trust_internal_platform;

cast::concept! {
    name: "patterns",
    summary: "Umbrella for the patterns stdlib category. Higher-level \
              architectural patterns.",
    anchors: [
        crate::patterns::anti_corruption_boundary,
        crate::patterns::api_gateway_with_contract_enforcement,
        crate::patterns::append_only_audit_trail,
        crate::patterns::backup_restore_and_rebuild_system,
        crate::patterns::bounded_ai_agent_execution,
        crate::patterns::cache_projection_layer,
        crate::patterns::capability_based_access_system,
        crate::patterns::command_queue_with_idempotent_handlers,
        crate::patterns::consent_based_data_access,
        crate::patterns::content_addressed_artifact_pipeline,
        crate::patterns::cost_aware_cloud_bursting,
        crate::patterns::cqrs_read_write_split,
        crate::patterns::data_lineage_and_provenance_system,
        crate::patterns::declarative_infrastructure_system,
        crate::patterns::derived_state_rebuild_system,
        crate::patterns::distributed_replicated_log,
        crate::patterns::domain_event_automation_system,
        crate::patterns::durable_workflow_engine,
        crate::patterns::event_driven_service_platform,
        crate::patterns::event_sourced_service,
        crate::patterns::expand_contract_migration,
        crate::patterns::fault_tolerant_cluster_control,
        crate::patterns::human_approved_automation_flow,
        crate::patterns::incident_response_system,
        crate::patterns::integration_hub,
        crate::patterns::leader_elected_controller,
        crate::patterns::local_first_encrypted_personal_store,
        crate::patterns::message_bus_backbone,
        crate::patterns::multi_tenant_isolation_platform,
        crate::patterns::observability_fabric,
        crate::patterns::offline_first_sync_engine,
        crate::patterns::operator_console_and_control_surface,
        crate::patterns::plugin_extension_platform,
        crate::patterns::policy_driven_resource_scheduler,
        crate::patterns::privacy_rights_workflow,
        crate::patterns::progressive_delivery_system,
        crate::patterns::rag_knowledge_system,
        crate::patterns::replay_protected_pairing_flow,
        crate::patterns::reproducible_build_supply_chain,
        crate::patterns::sealed_secret_management_system,
        crate::patterns::search_and_indexing_pipeline,
        crate::patterns::secure_bootstrap_chain,
        crate::patterns::secure_signed_command_system,
        crate::patterns::self_describing_platform,
        crate::patterns::self_healing_reconciler_control_plane,
        crate::patterns::temporal_state_system,
        crate::patterns::tiered_data_lifecycle_system,
        crate::patterns::versioned_configuration_store,
        crate::patterns::webhook_delivery_system,
        crate::patterns::zero_trust_internal_platform,
    ],
    tags: ["cast_stdlib", "patterns"],
}

/// Sentinel for the patterns stdlib group.
pub struct PatternsGroup;

cast::rule! {
    rule: "Document a pattern by what it composes, not how it's implemented.",
    why:  "A pattern's value is the relationship it names between \
           primitives. Describe the composition and a reader can swap \
           one ingredient without tearing up the recipe; describe the \
           implementation and you've written prose that goes stale \
           the first time the implementation changes.",
    governs: [cast_stdlib::patterns::PatternsGroup],
    tags: ["cast_stdlib", "patterns"],
}
