//! `backup_restore_and_rebuild_system` — prove the system can recover from loss.

/// Sentinel for `backup_restore_and_rebuild_system`.
pub struct BackupRestoreAndRebuildSystem;

cast::concept! {
    name: "backup_restore_and_rebuild_system",
    summary: "The system proves it can recover from loss — not just \
              that backups exist, but that they restore correctly. \
              Composes snapshot_storage, append_only_log, \
              backup_restore_test, disaster_recovery_plan, \
              rebuild_from_history, content_addressed_cache, \
              encrypted_at_rest_storage, and retention_policy. Used \
              for personal cloud backup, database recovery, \
              configuration restore, node-loss recovery, and \
              audit/history preservation.",
    anchors: [cast_stdlib::patterns::backup_restore_and_rebuild_system::BackupRestoreAndRebuildSystem],
    tags: ["cast_stdlib", "patterns"],
}
