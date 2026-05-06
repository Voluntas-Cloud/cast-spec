//! `preempt_rt_kernel` — Linux real-time preemption model.

/// Sentinel for `preempt_rt_kernel`.
pub struct PreemptRtKernel;

cast::concept! {
    name: "preempt_rt_kernel",
    summary: "Linux real-time preemption model.",
    anchors: [cast_os_stdlib::realtime::preempt_rt_kernel::PreemptRtKernel],
    tags: ["cast_os_stdlib", "realtime"],
}
