//! `orphan_process_reparenting` — abandoned process adopted by init/supervisor.

/// Sentinel for `orphan_process_reparenting`.
pub struct OrphanProcessReparenting;

cast::concept! {
    name: "orphan_process_reparenting",
    summary: "abandoned process adopted by init/supervisor.",
    anchors: [cast_os_stdlib::execution_model::orphan_process_reparenting::OrphanProcessReparenting],
    tags: ["cast_os_stdlib", "execution_model"],
}
