//! `local_first_encrypted_personal_store` — user data lives locally first, syncs securely, stays user-controlled.

/// Sentinel for `local_first_encrypted_personal_store`.
pub struct LocalFirstEncryptedPersonalStore;

cast::concept! {
    name: "local_first_encrypted_personal_store",
    summary: "User data lives locally first, syncs securely, and \
              remains user-controlled. Composes \
              encrypted_at_rest_storage, content_addressed_cache, \
              sync_engine, hardware_backed_key, data_minimization, \
              capability_token, append_only_log, and \
              conflict_resolution. Used for password managers, \
              personal knowledge bases, health/finance records, \
              mobile-first personal clouds, and the Voluntas user \
              data layer.",
    anchors: [cast_stdlib::patterns::local_first_encrypted_personal_store::LocalFirstEncryptedPersonalStore],
    tags: ["cast_stdlib", "patterns"],
}
