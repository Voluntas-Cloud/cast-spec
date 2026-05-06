//! I/O architecture.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod asynchronous_io;
pub mod backpressure_from_io_layer;
pub mod buffered_io;
pub mod completion_event;
pub mod direct_io;
pub mod dma_io_path;
pub mod epoll_readiness_model;
pub mod event_driven_io;
pub mod hybrid_poll_interrupt_io;
pub mod interrupt_driven_io;
pub mod io_completion_queue;
pub mod io_scheduler;
pub mod io_submission_queue;
pub mod kqueue_event_model;
pub mod linux_io_uring_model;
pub mod nonblocking_io;
pub mod polling_io;
pub mod request_queue;
pub mod scatter_gather_io;
pub mod select_poll_model;
pub mod synchronous_io;
pub mod windows_iocp_model;
pub mod zero_copy_io;

cast::concept! {
    name: "io_architecture",
    summary: "Umbrella for the io_architecture stdlib category. I/O \
              architecture.",
    anchors: [
        crate::io_architecture::asynchronous_io,
        crate::io_architecture::backpressure_from_io_layer,
        crate::io_architecture::buffered_io,
        crate::io_architecture::completion_event,
        crate::io_architecture::direct_io,
        crate::io_architecture::dma_io_path,
        crate::io_architecture::epoll_readiness_model,
        crate::io_architecture::event_driven_io,
        crate::io_architecture::hybrid_poll_interrupt_io,
        crate::io_architecture::interrupt_driven_io,
        crate::io_architecture::io_completion_queue,
        crate::io_architecture::io_scheduler,
        crate::io_architecture::io_submission_queue,
        crate::io_architecture::kqueue_event_model,
        crate::io_architecture::linux_io_uring_model,
        crate::io_architecture::nonblocking_io,
        crate::io_architecture::polling_io,
        crate::io_architecture::request_queue,
        crate::io_architecture::scatter_gather_io,
        crate::io_architecture::select_poll_model,
        crate::io_architecture::synchronous_io,
        crate::io_architecture::windows_iocp_model,
        crate::io_architecture::zero_copy_io,
    ],
    tags: ["cast_os_stdlib", "io_architecture"],
}

/// Sentinel for the io_architecture stdlib group.
pub struct IoArchitectureGroup;
