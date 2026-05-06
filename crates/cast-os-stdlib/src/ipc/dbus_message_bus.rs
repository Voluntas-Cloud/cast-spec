//! `dbus_message_bus` — desktop/system IPC bus.

/// Sentinel for `dbus_message_bus`.
pub struct DbusMessageBus;

cast::concept! {
    name: "dbus_message_bus",
    summary: "desktop/system IPC bus.",
    anchors: [cast_os_stdlib::ipc::dbus_message_bus::DbusMessageBus],
    tags: ["cast_os_stdlib", "ipc"],
}
