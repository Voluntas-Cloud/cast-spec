//! `sync_engine` — reconcile data between systems.

/// Sentinel for `sync_engine`.
pub struct SyncEngine;

cast::concept! {
    name: "sync_engine",
    summary: "Reconcile data between systems on an ongoing basis. \
              The hard parts are conflict resolution, identity \
              mapping, and \"how do we recover when sync has been \
              broken for two weeks\" — all of which are real \
              questions the engine has to have answers for.",
    anchors: [cast_stdlib::integration::sync_engine::SyncEngine],
    tags: ["cast_stdlib", "integration"],
}
