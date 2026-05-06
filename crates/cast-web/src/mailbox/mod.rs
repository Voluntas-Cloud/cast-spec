//! Filesystem-backed mailbox: a directory of numbered markdown
//! messages plus a `meta.json` heartbeat file. Each message is a
//! single file; appending = creating the next-numbered file. No
//! delimiter parsing, no partial-write corruption — the filesystem
//! gives us atomic message granularity for free.
//!
//! See `examples/mailbox/` for the on-disk shape illustrated.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::message::{Frontmatter, Role};

cast::concept! {
    name: "numbered_file_protocol",
    summary: "Cross-cutting filesystem-as-protocol pattern: a directory \
              holds files named `<NNN>-<suffix>.<ext>`; listing reads \
              the dir and sorts by leading integer; appending = \
              `next_number()` reads the dir then `O_CREAT|O_EXCL` \
              allocates the next filename (racing writers retry on \
              `AlreadyExists`); clearing/trimming removes by leading-\
              integer key. The filesystem provides atomic per-message \
              granularity for free — no in-process lock is needed \
              because the directory IS the coordination protocol, so \
              an external editor (a Claude session) participates as a \
              peer simply by writing the next file. Realised in two \
              places: the user/assistant `Mailbox` and the assistant ↔ \
              cast-watch `WatcherLog`. Downstream concepts that rest \
              on the protocol primitive-anchor `next_number` to name \
              the mechanism without re-describing it.",
    anchors: [
        crate::mailbox::leading_number,
        crate::mailbox::next_number,
        crate::mailbox::write_exclusive,
    ],
    tags: ["cast_web"],
}

cast::continues_in! {
    target:  cast_stdlib::consistency::atomic_operation::AtomicOperation,
    concept: "numbered_file_protocol",
    why:     "Append uses O_CREAT|O_EXCL — the kernel guarantees \
              atomicity of the create-or-fail step. Two racing writers \
              get two different numbers, both succeed, never an \
              overwrite. The filesystem is the atomic primitive.",
    tags:    ["cast_web"],
}

cast::continues_in! {
    target:  cast_stdlib::identity::monotonic_sequence_id::MonotonicSequenceId,
    concept: "numbered_file_protocol",
    why:     "The leading `<NNN>` is a monotonic sequence ID issued by \
              the directory itself (max + 1). Comparable, dense, \
              issued in strictly increasing order — the same role \
              that's elsewhere served by a database autoincrement \
              column.",
    tags:    ["cast_web"],
}

cast::concept! {
    name: "cast_web_mailbox",
    summary: "On-disk thread of numbered markdown messages. Each file \
              is one message with YAML frontmatter and a markdown body \
              that may embed fenced JSON payloads (`json draft`, \
              `json applied`, `json question`, `json alternatives`). \
              `meta.json` carries `last_heartbeat_at`; the UI grays \
              the send button when the value is older than 90s, on \
              the assumption that no Claude session is attached. \
              Cast-web is just a viewer / appender — the editing \
              actor (a Claude session) lives outside this process \
              and writes new files into the same directory.",
    anchors: [
        crate::mailbox::Mailbox,
        crate::mailbox::Message,
        CAST::AS_PRIMITIVE::crate::mailbox::watch_directory,
        CAST::AS_PRIMITIVE::crate::mailbox::next_number,
    ],
    tags: ["cast_web"],
}

cast::io::continues_in! {
    target:  "crates/cast-web/examples/mailbox/001-prompt.md",
    lang:    external,
    concept: "cast_web_mailbox",
    why:     "Worked example of a `role: user` message — the simplest \
              shape, freeform body, no fenced JSON payload. A Claude \
              session reading the directory would treat this as the \
              initial prompt of a thread.",
    tags:     ["cast_web"],
}

cast::io::continues_in! {
    target:  "crates/cast-web/examples/mailbox/002-reply.md",
    lang:    external,
    concept: "cast_web_mailbox",
    why:     "Worked example of a `role: assistant` reply. \
              `in_reply_to` chains it to the previous message; the \
              body is conversational markdown.",
    tags:     ["cast_web"],
}

cast::io::continues_in! {
    target:  "crates/cast-web/examples/mailbox/003-draft.md",
    lang:    external,
    concept: "cast_web_mailbox",
    why:     "Worked example of a `role: user` message that embeds \
              a `json draft` fenced block — a structured proposed \
              edit to a cast data object. Demonstrates the open-set \
              JSON fence-tag pattern.",
    tags:     ["cast_web"],
}

cast::continues_in! {
    target:  cast_stdlib::storage::append_only_log::AppendOnlyLog,
    concept: "cast_web_mailbox",
    why:     "The mailbox IS an append-only log: numbered files are \
              never rewritten, the directory's max-id-plus-one is the \
              next sequence, and concurrent reads of historical \
              messages are inherently safe. The atomic-O_EXCL \
              constraint on append is what keeps the log durable in \
              the face of a racing external editor (a Claude session) \
              that holds no in-process lock.",
    tags:    ["cast_web"],
}

cast::rule! {
    rule:    "Mailbox::append must allocate the next-numbered \
              filename via O_CREAT|O_EXCL — never read-then-write. \
              Two appenders racing must produce two different \
              numbers, both succeeding, never an overwrite.",
    why:     "The mailbox is consumed by an external process \
              (a Claude session) that holds no lock; the filesystem \
              IS the coordination protocol. A non-atomic max-and-\
              increment would let two POSTs allocate the same id, \
              and one's content would silently win. O_EXCL turns \
              that race into a retry.",
    governs: [
        crate::mailbox::Mailbox,
    ],
    tags:     ["cast_web"],
}

cast::continues_in! {
    target:  cast_stdlib::bugs::unknown_locus_bug::UnknownLocusBug,
    concept: "cast_web_mailbox",
    why:     "OBSERVED 2026-05-04: id collision when the in-browser \
              writer (POST /mailbox/send, role=user) and the external \
              editing actor (a Claude session writing role=assistant \
              files via the Write tool) commit at near-identical \
              instants. Each independently picks `max(NNN)+1` from a \
              directory listing without observing the other's pending \
              write, and both land on the same NNN. The in-process \
              `Mailbox::append` already uses O_EXCL (rule above), but \
              that rule `governs: [crate::mailbox::Mailbox]` — it \
              constrains the shim, not the external actor. The Write \
              tool does not use O_EXCL.\n\n\
              Open question — architectural vs implementation:\n\
              - architectural: monotonic numbered ids \
              (cast_stdlib::identity::monotonic_sequence_id) are the \
              wrong primitive when writers are uncoordinated. The right \
              primitive is uuid_identity, or a partitioned-counter \
              keyed by role (`user-001`, `assistant-001` so the two \
              namespaces never collide).\n\
              - implementation: keep monotonic ids; require the \
              external actor to use O_EXCL+retry too (i.e. extend the \
              O_EXCL rule's governs:-set, or change the protocol so \
              the external actor goes through a coordinator that owns \
              the id allocator).\n\n\
              Both are defensible until we pick — re-classify this \
              edge to architectural_bug or implementation_bug once the \
              decision lands.",
    tags:    ["cast_web"],
}

cast::concept! {
    name: "cast_web_mailbox_clear",
    summary: "Wipe-the-thread affordance: the chat panel exposes a \
              `clear` button that removes every numbered `*-user.md` \
              and `*-assistant.md` from the mailbox directory while \
              leaving `meta.json` (session_id + heartbeat) intact. \
              The next message posted starts numbering from 001 again. \
              Implemented as `Mailbox::clear` server-side and \
              wired through `POST /mailbox/clear`. Intended for \
              dev-loop hygiene: when a thread is exhausted or got \
              noisy, clearing is faster than restarting the watcher \
              just to scrub state. The directory watcher only emits \
              create / rename events, so the clearing client must \
              call `refreshMessages` itself; other tabs viewing the \
              same mailbox won't auto-update until they reload.",
    anchors: [
        crate::mailbox::Mailbox,
        crate::shim::mailbox_routes::clear,
        CAST::AS_PRIMITIVE::crate::mailbox::leading_number,
    ],
    tags: ["cast_web"],
}

pub mod message;
pub mod watch_directory;

pub use message::Message;

/// Handle to a mailbox directory on disk.
#[derive(Clone)]
pub struct Mailbox {
    pub root: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    #[serde(default)]
    pub session_id: Option<String>,
    #[serde(default)]
    pub started_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub last_heartbeat_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Liveness {
    pub attached: bool,
    pub age_seconds: Option<i64>,
    pub last_heartbeat_at: Option<DateTime<Utc>>,
}

const HEARTBEAT_STALE_SECONDS: i64 = 90;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppendRequest {
    pub role: Role,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub in_reply_to: Option<String>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppendResponse {
    pub id: String,
    pub filename: String,
}

impl Mailbox {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub async fn ensure(&self) -> Result<()> {
        tokio::fs::create_dir_all(&self.root)
            .await
            .with_context(|| format!("create mailbox dir {}", self.root.display()))
    }

    /// Read every numbered .md file in the dir, parse each, and
    /// return them sorted by leading number.
    pub async fn list_messages(&self) -> Result<Vec<Message>> {
        let mut entries = tokio::fs::read_dir(&self.root)
            .await
            .with_context(|| format!("list mailbox dir {}", self.root.display()))?;
        let mut numbered = Vec::<(u64, PathBuf)>::new();
        while let Some(e) = entries.next_entry().await? {
            let path = e.path();
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            if let Some(n) = leading_number(&path) {
                numbered.push((n, path));
            }
        }
        numbered.sort_by_key(|(n, _)| *n);

        let mut out = Vec::with_capacity(numbered.len());
        for (_, path) in numbered {
            let text = tokio::fs::read_to_string(&path)
                .await
                .with_context(|| format!("read {}", path.display()))?;
            match Message::parse(&text) {
                Ok(m) => out.push(m),
                Err(e) => tracing::warn!(file = %path.display(), err = %e, "skip unparseable mailbox file"),
            }
        }
        Ok(out)
    }

    /// Atomically allocate the next filename and write the message.
    /// Retries up to 8 times under contention.
    pub async fn append(&self, req: &AppendRequest) -> Result<AppendResponse> {
        let next = next_number(&self.root).await?;
        for offset in 0..8u64 {
            let n = next + offset;
            let id = format!("{n:03}");
            let suffix = match req.role {
                Role::User => "user",
                Role::Assistant => "assistant",
            };
            let filename = format!("{id}-{suffix}.md");
            let path = self.root.join(&filename);

            let frontmatter = Frontmatter {
                id: id.clone(),
                role: req.role,
                in_reply_to: req.in_reply_to.clone(),
                created: Utc::now().to_rfc3339(),
            };
            let m = Message {
                frontmatter,
                body: req.body.clone(),
                payloads: Vec::new(),
            };
            let rendered = m.render()?;

            match write_exclusive(&path, &rendered).await {
                Ok(()) => return Ok(AppendResponse { id, filename }),
                Err(e) if is_already_exists(&e) => continue,
                Err(e) => return Err(e),
            }
        }
        Err(anyhow!("could not allocate next mailbox filename after 8 retries"))
    }

    /// Remove every numbered `.md` file in the directory. `meta.json`
    /// is left in place. Returns the number of files removed.
    ///
    /// Not atomic across files — a concurrent `append` may land
    /// between the read_dir snapshot and the remove pass, in which
    /// case the new file survives (its number is past `max` of the
    /// snapshot). That's the right behavior: a fresh write should not
    /// be silently swallowed by a clear that started before it.
    pub async fn clear(&self) -> Result<usize> {
        let mut entries = match tokio::fs::read_dir(&self.root).await {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
            Err(e) => return Err(e.into()),
        };
        let mut victims = Vec::<PathBuf>::new();
        while let Some(e) = entries.next_entry().await? {
            let path = e.path();
            if path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            if leading_number(&path).is_some() {
                victims.push(path);
            }
        }
        let mut removed = 0usize;
        for path in victims {
            match tokio::fs::remove_file(&path).await {
                Ok(()) => removed += 1,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    return Err(anyhow::Error::from(e)
                        .context(format!("remove {}", path.display())));
                }
            }
        }
        Ok(removed)
    }

    pub async fn read_meta(&self) -> Result<Option<Meta>> {
        let path = self.root.join("meta.json");
        match tokio::fs::read(&path).await {
            Ok(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn liveness(&self) -> Result<Liveness> {
        let meta = self.read_meta().await?;
        let last = meta.as_ref().and_then(|m| m.last_heartbeat_at);
        let age = last.map(|t| (Utc::now() - t).num_seconds());
        let attached = matches!(age, Some(a) if a < HEARTBEAT_STALE_SECONDS);
        Ok(Liveness {
            attached,
            age_seconds: age,
            last_heartbeat_at: last,
        })
    }
}

pub(crate) fn leading_number(path: &Path) -> Option<u64> {
    let name = path.file_name()?.to_str()?;
    let digits: String = name.chars().take_while(|c| c.is_ascii_digit()).collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse().ok()
}

pub(crate) async fn next_number(root: &Path) -> Result<u64> {
    let mut entries = match tokio::fs::read_dir(root).await {
        Ok(e) => e,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(1),
        Err(e) => return Err(e.into()),
    };
    let mut max = 0u64;
    while let Some(e) = entries.next_entry().await? {
        if let Some(n) = leading_number(&e.path()) {
            max = max.max(n);
        }
    }
    Ok(max + 1)
}

pub(crate) async fn write_exclusive(path: &Path, content: &str) -> Result<()> {
    use tokio::fs::OpenOptions;
    use tokio::io::AsyncWriteExt;
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .await?;
    f.write_all(content.as_bytes()).await?;
    f.flush().await?;
    Ok(())
}

fn is_already_exists(err: &anyhow::Error) -> bool {
    err.downcast_ref::<std::io::Error>()
        .map(|e| e.kind() == std::io::ErrorKind::AlreadyExists)
        .unwrap_or(false)
}
