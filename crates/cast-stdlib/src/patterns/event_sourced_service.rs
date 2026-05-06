//! `event_sourced_service` — state derived from an append-only history of domain events.

/// Sentinel for `event_sourced_service`.
pub struct EventSourcedService;

cast::concept! {
    name: "event_sourced_service",
    summary: "A service where state is derived from an append-only \
              history of domain events. Composes append_only_log, \
              event_message, monotonic_sequence_id, snapshot_storage, \
              idempotent_operation, temporal_query, projection_model, \
              and rebuild_from_history. Used for financial ledgers, \
              audit-heavy business systems, workflow history, user \
              activity timelines, and cluster control-plane history.",
    anchors: [cast_stdlib::patterns::event_sourced_service::EventSourcedService],
    tags: ["cast_stdlib", "patterns"],
}
