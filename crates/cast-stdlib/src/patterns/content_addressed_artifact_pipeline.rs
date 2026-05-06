//! `content_addressed_artifact_pipeline` — artifacts identified by content hash, not mutable name.

/// Sentinel for `content_addressed_artifact_pipeline`.
pub struct ContentAddressedArtifactPipeline;

cast::concept! {
    name: "content_addressed_artifact_pipeline",
    summary: "Artifacts are identified by content hash rather than \
              mutable names. Composes content_hash_id, \
              content_addressed_cache, immutable_blob_store, \
              artifact_manifest, signed_artifact, \
              provenance_attestation, and container_image_digest_pin. \
              Used for build systems, container image pipelines, AI \
              model artifact storage, reproducible deployment, and \
              software supply-chain integrity.",
    anchors: [cast_stdlib::patterns::content_addressed_artifact_pipeline::ContentAddressedArtifactPipeline],
    tags: ["cast_stdlib", "patterns"],
}
