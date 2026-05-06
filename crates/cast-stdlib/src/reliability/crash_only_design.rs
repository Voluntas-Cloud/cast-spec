//! `crash_only_design` — recovery by restart from durable state.

/// Sentinel for `crash_only_design`.
pub struct CrashOnlyDesign;

cast::concept! {
    name: "crash_only_design",
    summary: "Recovery by restart from durable state. The shutdown \
              path is not a separate code path — every restart is a \
              cold restart, exercised constantly.",
    anchors: [cast_stdlib::reliability::crash_only_design::CrashOnlyDesign],
    tags: ["cast_stdlib", "reliability"],
}
