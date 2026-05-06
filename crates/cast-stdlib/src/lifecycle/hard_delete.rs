//! `hard_delete` — physically remove data, irreversible.

/// Sentinel for `hard_delete`.
pub struct HardDelete;

cast::concept! {
    name: "hard_delete",
    summary: "Physically remove data. Irreversible; usually required \
              for compliance (right-to-erasure) or quota recovery.",
    anchors: [cast_stdlib::lifecycle::hard_delete::HardDelete],
    tags: ["cast_stdlib", "lifecycle"],
}
