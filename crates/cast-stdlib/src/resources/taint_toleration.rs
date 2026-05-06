//! `taint_toleration` — restrict which workloads can use nodes.

/// Sentinel for `taint_toleration`.
pub struct TaintToleration;

cast::concept! {
    name: "taint_toleration",
    summary: "Restrict which workloads can use nodes. A taint repels \
              everything by default; only workloads that explicitly \
              tolerate it can land — useful for dedicated, expensive, \
              or quirky hardware.",
    anchors: [cast_stdlib::resources::taint_toleration::TaintToleration],
    tags: ["cast_stdlib", "resources"],
}
