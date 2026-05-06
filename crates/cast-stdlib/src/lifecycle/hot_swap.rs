//! `hot_swap` — atomic in-process replacement of state or behavior without restart.

/// Sentinel for `hot_swap`.
pub struct HotSwap;

cast::concept! {
    name: "hot_swap",
    summary: "Replace the live version of state or behavior in a \
              running process such that no observer ever sees a \
              partial, half-applied, or in-between version. Readers \
              either see the prior version to completion or the new \
              version going forward; there is no torn read. Common \
              implementations: pointer swap behind an Arc, generation \
              counter + lookup, double-buffer with index flip. \
              Distinct from `compare_and_swap` (the primitive) — \
              hot_swap is the architectural pattern that uses CAS to \
              avoid the restart cost.",
    anchors: [cast_stdlib::lifecycle::hot_swap::HotSwap],
    tags: ["cast_stdlib", "lifecycle"],
}
