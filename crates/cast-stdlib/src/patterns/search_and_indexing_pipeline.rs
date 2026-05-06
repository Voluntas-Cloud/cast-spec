//! `search_and_indexing_pipeline` — raw data is transformed into searchable indexes.

/// Sentinel for `search_and_indexing_pipeline`.
pub struct SearchAndIndexingPipeline;

cast::concept! {
    name: "search_and_indexing_pipeline",
    summary: "Raw data is transformed into searchable indexes. \
              Composes import_pipeline, incremental_rebuild, \
              content_hash_id, embedding_index, hybrid_search, \
              precomputed_index, source_grounding, and \
              schema_versioned_storage. Used for document search, \
              code search, recording transcript search, AI/RAG \
              context, and personal knowledge retrieval.",
    anchors: [cast_stdlib::patterns::search_and_indexing_pipeline::SearchAndIndexingPipeline],
    tags: ["cast_stdlib", "patterns"],
}
