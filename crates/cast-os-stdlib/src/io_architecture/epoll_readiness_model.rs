//! `epoll_readiness_model` — Linux readiness notification for file descriptors.

/// Sentinel for `epoll_readiness_model`.
pub struct EpollReadinessModel;

cast::concept! {
    name: "epoll_readiness_model",
    summary: "Linux readiness notification for file descriptors.",
    anchors: [cast_os_stdlib::io_architecture::epoll_readiness_model::EpollReadinessModel],
    tags: ["cast_os_stdlib", "io_architecture"],
}
