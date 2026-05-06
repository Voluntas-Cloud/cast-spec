//! `data_lineage_and_provenance_system` — track where data came from and what produced it.

/// Sentinel for `data_lineage_and_provenance_system`.
pub struct DataLineageAndProvenanceSystem;

cast::concept! {
    name: "data_lineage_and_provenance_system",
    summary: "The system knows where data came from, how it changed, \
              and what produced it. Composes data_lineage, \
              provenance_attestation, content_hash_id, event_stream, \
              correlation_id, schema_versioned_storage, audit_log, \
              and source_grounding. Used for AI citations, \
              compliance, build provenance, analytics pipelines, and \
              document processing.",
    anchors: [cast_stdlib::patterns::data_lineage_and_provenance_system::DataLineageAndProvenanceSystem],
    tags: ["cast_stdlib", "patterns"],
}
