//! IPC and local communication.
//!
//! Each concept lives in its own submodule (one sentinel + one
//! `cast::concept!` block per file). This module file holds only
//! the group sentinel and the `pub mod` declarations.

pub mod binder_ipc;
pub mod brokered_ipc;
pub mod capability_ipc_endpoint;
pub mod copy_based_ipc;
pub mod dbus_message_bus;
pub mod eventfd_notification;
pub mod futex_synchronization;
pub mod mach_port_ipc;
pub mod memory_mapped_ipc;
pub mod message_queue_ipc;
pub mod named_pipe_fifo;
pub mod namespaced_ipc;
pub mod pipe_ipc;
pub mod rpc_local_call;
pub mod semaphore_ipc;
pub mod shared_memory_ipc;
pub mod signal_ipc;
pub mod unix_domain_socket;
pub mod windows_alpc_ipc;
pub mod zero_copy_ipc;

cast::concept! {
    name: "ipc",
    summary: "Umbrella for the ipc stdlib category. IPC and local \
              communication.",
    anchors: [
        crate::ipc::binder_ipc,
        crate::ipc::brokered_ipc,
        crate::ipc::capability_ipc_endpoint,
        crate::ipc::copy_based_ipc,
        crate::ipc::dbus_message_bus,
        crate::ipc::eventfd_notification,
        crate::ipc::futex_synchronization,
        crate::ipc::mach_port_ipc,
        crate::ipc::memory_mapped_ipc,
        crate::ipc::message_queue_ipc,
        crate::ipc::named_pipe_fifo,
        crate::ipc::namespaced_ipc,
        crate::ipc::pipe_ipc,
        crate::ipc::rpc_local_call,
        crate::ipc::semaphore_ipc,
        crate::ipc::shared_memory_ipc,
        crate::ipc::signal_ipc,
        crate::ipc::unix_domain_socket,
        crate::ipc::windows_alpc_ipc,
        crate::ipc::zero_copy_ipc,
    ],
    tags: ["cast_os_stdlib", "ipc"],
}

/// Sentinel for the ipc stdlib group.
pub struct IpcGroup;
