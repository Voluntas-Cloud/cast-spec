//! Directory watcher → in-process broadcast bus.
//!
//! Lives under `mailbox/` for historical reasons but is generic
//! enough for any append-only directory (it's also used by the
//! `watcher_log` plane). `start` takes a path predicate so the
//! consumer decides which files matter; only Create / rename-into
//! events fire, so each new file shows up exactly once.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use notify::event::{ModifyKind, RenameMode};
use notify::{EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use tokio::sync::broadcast;
use tracing::warn;

#[derive(Debug, Clone)]
pub struct DirectoryEvent {
    /// Filename relative to the watched dir, e.g. `005-reply.md`.
    pub filename: String,
}

pub struct DirectoryWatch {
    pub events: broadcast::Sender<DirectoryEvent>,
    /// The notify watcher must outlive the subscription set; held
    /// here so the caller can drop the whole bundle in one go.
    _watcher: RecommendedWatcher,
}

pub type PathFilter = Box<dyn Fn(&Path) -> bool + Send + 'static>;

/// Start a watch on `dir`. `accept` is a predicate that decides
/// which paths to broadcast (others are silently dropped).
pub fn start(dir: &Path, accept: PathFilter) -> Result<DirectoryWatch> {
    let (tx, _rx) = broadcast::channel::<DirectoryEvent>(64);
    let dir_owned: PathBuf = dir.to_path_buf();
    let tx_for_handler = tx.clone();

    let mut watcher = notify::recommended_watcher(move |res: notify::Result<notify::Event>| {
        let ev = match res {
            Ok(e) => e,
            Err(e) => {
                warn!(err = %e, "directory watcher error");
                return;
            }
        };
        if !is_relevant_event(&ev.kind) {
            return;
        }
        for path in &ev.paths {
            if !accept(path) {
                continue;
            }
            let Ok(rel) = path.strip_prefix(&dir_owned) else {
                continue;
            };
            let Some(name) = rel.to_str().map(str::to_string) else {
                continue;
            };
            // Receiver-not-yet-subscribed is a normal startup state;
            // ignore the error.
            let _ = tx_for_handler.send(DirectoryEvent { filename: name });
        }
    })?;

    watcher
        .watch(dir, RecursiveMode::NonRecursive)
        .with_context(|| format!("watch dir {}", dir.display()))?;

    Ok(DirectoryWatch {
        events: tx,
        _watcher: watcher,
    })
}

/// Predicate matching mailbox message files: `\d+-*.md` plus
/// `meta.json`.
pub fn mailbox_filter() -> PathFilter {
    Box::new(|path: &Path| {
        path.extension().map(|e| e == "md").unwrap_or(false)
            || path.file_name().map(|n| n == "meta.json").unwrap_or(false)
    })
}

/// Predicate matching numbered JSON files in the watcher-log dir.
pub fn json_filter() -> PathFilter {
    Box::new(|path: &Path| path.extension().map(|e| e == "json").unwrap_or(false))
}

/// Emit one event per *file appearing* — Create (O_CREAT|O_EXCL) and
/// Modify::Name (mv/rename into the dir). Modify::Data writes that
/// follow a Create are deliberately dropped so each new file fires
/// exactly once.
fn is_relevant_event(kind: &EventKind) -> bool {
    matches!(
        kind,
        EventKind::Create(_)
            | EventKind::Modify(ModifyKind::Name(RenameMode::To))
            | EventKind::Modify(ModifyKind::Name(RenameMode::Both))
    )
}
