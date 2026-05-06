//! `mutable_index_over_immutable_data` — small mutable control plane over append-only data plane.

/// Sentinel for `mutable_index_over_immutable_data`.
pub struct MutableIndexOverImmutableData;

cast::concept! {
    name: "mutable_index_over_immutable_data",
    summary: "Mutable pointers/indexes over immutable payloads. The \
              data plane is append-only; the control plane (which \
              version is current, which blobs are referenced) is \
              mutable and small.",
    anchors: [cast_stdlib::storage::mutable_index_over_immutable_data::MutableIndexOverImmutableData],
    tags: ["cast_stdlib", "storage"],
}
