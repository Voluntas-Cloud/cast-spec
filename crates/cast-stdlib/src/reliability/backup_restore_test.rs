//! `backup_restore_test` — prove backups actually work.

/// Sentinel for `backup_restore_test`.
pub struct BackupRestoreTest;

cast::concept! {
    name: "backup_restore_test",
    summary: "Prove backups actually work. Periodically restore into \
              an isolated environment and verify the result; until \
              that succeeds the backup is a hypothesis.",
    anchors: [cast_stdlib::reliability::backup_restore_test::BackupRestoreTest],
    tags: ["cast_stdlib", "reliability"],
}
