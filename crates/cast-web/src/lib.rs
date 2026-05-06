//! cast-web — a small Rust shim plus a plain-TypeScript frontend that
//! lets a human (and optionally a Claude session) browse and refine
//! the cast concept graph served by `cast-watch`. To cast what
//! `gitweb` / `cgit` are to git: single-purpose, readable, useful out
//! of the box, and explicitly not a framework. If someone wants a
//! Forgejo-shaped product over the same data plane, they fork or
//! build a sibling project that also speaks cast-watch — they do not
//! extend cast-web in place.
//!
//! Three responsibilities, no more:
//!
//! 1. Data plane — relay JSON-line traffic between the browser and a
//!    single cast-watch unix socket.
//! 2. Mailbox — serve a directory of numbered markdown messages over
//!    SSE and append the next-numbered file on POST.
//! 3. Static — serve the embedded compiled-from-TypeScript bundle.
//!
//! The crate is its own KICKOFF.md (kept at the repo root for the
//! moment) and Cast.cast in lib.rs: every cast-web module owns a
//! `cast::concept!` block that declares its anchors, and the
//! cast-watch's `paths_unresolved` query is the live impl punch
//! list. As modules land, the count drops. As new sub-areas are
//! sketched, the count climbs — that is the design pattern, not a
//! bug.

cast::concept! {
    name: "cast_web",
    summary: "Read-only browser UI over a cast-watch concept graph, \
              plus a filesystem mailbox surface for optional LLM-\
              mediated edits. cast-web has no graph store of its own \
              — the browser is a thin renderer of the watcher's \
              `manual` + `query` + `walk` + `concepts_for_path` + \
              `subscribe` responses, and the Rust shim is one Unix \
              socket connection per WebSocket client. The mailbox is \
              a separate plane: a directory of numbered markdown \
              files, served over SSE and appended on POST. It is \
              optional — if no Claude session is attached, the AI \
              controls gray out and the data plane keeps working.",
    anchors: [
        crate::shim,
        crate::mailbox,
        crate::draft,
        crate::frontend,
    ],
    tags: ["cast_web"],
}

// The crate doc comment enumerates three responsibilities. This
// matrix pins how each maps to a route prefix, a transport, and the
// place its state lives. Adding a fourth surface to cast-web means
// adding a row here.
cast::matrix! {
    columns: [http_route, transport, persistent_state],
    rows: {
        data_plane @ crate::shim::ws_server: [
            "`GET /ws` (long-lived) and `POST /watcher/query` (one-shot)",
            "WebSocket frames for /ws; HTTP request/response for /watcher/query — both wrap one Unix-socket round-trip per call",
            "None — the shim holds no graph state between requests; cast-watch owns the snapshot",
        ],
        mailbox @ crate::mailbox::Mailbox: [
            "`POST /mailbox/send`, `GET /mailbox/stream`, `POST /mailbox/clear`, `GET /mailbox/messages`, `GET /mailbox/meta`",
            "Server-Sent Events for /stream; HTTP for the rest",
            "Filesystem: a directory of numbered `*-user.md` / `*-assistant.md` files plus `meta.json` for heartbeat + session id",
        ],
        static_assets @ crate::frontend: [
            "`GET /` and `GET /static/*`",
            "Plain HTTP response carrying the embedded byte slice",
            "Compiled-in: `rust-embed` bakes `web/dist/*` into the binary at build time",
        ],
    },
    tags: ["cast_web"],
    note: "Three rows = three responsibilities = three reasons cast-web \
           exists. `cast-watch` owns the analyzer; `Mailbox` owns the \
           on-disk thread; `frontend` owns the bytes the browser \
           loads. Anything outside these rows belongs in a sibling \
           project that also speaks cast-watch, not in cast-web.",
}

cast::io::continues_in! {
    target:  "crates/cast-web/web/src/main.ts",
    lang:    typescript,
    anchor:  "main",
    concept: "cast_web",
    why:     "The frontend half. Plain TypeScript with no framework \
              and no bundler — `tsc` compiles `src/*.ts` to `dist/*.js` \
              one-to-one, the browser loads them as native ES modules \
              via `<script type=\"module\">`, and `rust-embed` bakes \
              `web/dist/*` into the cast-web release binary. The \
              entry point is a free `main()` function that opens the \
              WebSocket, fetches the manual, and mounts the views.",
    tags:     ["cast_web"],
}

cast::rule! {
    rule:    "cast-web speaks to cast-watch through a unix-socket \
              JSON-line client only. It must not import `cast = \
              { features = [\"analysis\"] }` and must not parse \
              `.rs` / `.cast` files itself.",
    why:     "The whole point of the shim is to make cast-web small \
              and rebuildable: one process owns the analyzer (cast-\
              watch), every consumer reads it. If cast-web grew an \
              analyzer of its own the two would drift, and a \
              cast-web reader would need rust-analyzer machinery in \
              their head to follow what the binary does. Keep the \
              dependency edge at the unix socket — that is the \
              encapsulation seam.",
    governs: [
        crate::shim::watch_client,
    ],
    tags:     ["cast_web"],
}

cast::concept! {
    name: "watcher_log",
    summary: "On-disk log of the assistant ↔ cast-watch conversation. \
              Every `POST /watcher/query` request and its response \
              are written as numbered JSON files (`NNN-query.json`, \
              `NNN+1-response.json`) in the directory configured by \
              `--watcher-log-dir`. Atomic next-number allocation via \
              O_CREAT|O_EXCL — same shape as the user/assistant \
              mailbox. The log makes assistant queries observable: \
              the human user (or any third process) can tail the \
              directory and follow what the assistant is asking the \
              watcher in real time. The HTTP body still flows back \
              synchronously to the caller; logging is a side-effect, \
              not a delivery channel.",
    anchors: [
        crate::shim::watcher_routes::WatcherLog,
        crate::shim::watcher_routes::query,
        CAST::AS_PRIMITIVE::crate::mailbox::next_number,
    ],
    tags: ["cast_web"],
}

cast::continues_in! {
    target:  cast_stdlib::messaging::side_effect_tee::SideEffectTee,
    concept: "watcher_log",
    why:     "The HTTP request and response are forwarded to the \
              caller synchronously; the on-disk log is a tee that \
              fires only when --watcher-log-dir is configured AND \
              the response parsed as JSON. A log-write failure is \
              warned-and-swallowed — the watcher reply still flows \
              back to the caller. Observability without coupling.",
    tags:    ["cast_web"],
}

cast::concept! {
    name: "watcher_log_auto_trim",
    summary: "The watcher log is bounded: `append_turn` calls \
              `WatcherLog::trim_to(MAX_TURNS=100, TARGET=90)` after \
              each successful write, and any time the directory \
              holds more than 100 turns the oldest are deleted until \
              90 remain. The 10-turn buffer keeps the trim from \
              firing on every single append once we approach the \
              cap — we let the directory grow to 100 then snap to \
              90, instead of one delete per write at exactly 100. \
              Bound is on TURNS (each turn = one query file + one \
              response file = 2 on-disk files), not files, so 90 \
              kept turns means 180 files. Trim failures are logged \
              but never fail the append: the turn is already on \
              disk and observable; pruning is housekeeping. The \
              user/assistant mailbox is not auto-trimmed — that one \
              is human conversation and is cleared explicitly via \
              the chat panel's `clear` button.",
    anchors: [
        crate::shim::watcher_routes::WATCHER_LOG_MAX_TURNS,
        crate::shim::watcher_routes::WATCHER_LOG_TARGET_TURNS,
        CAST::AS_PRIMITIVE::crate::shim::watcher_routes::WatcherLog,
        CAST::AS_PRIMITIVE::crate::mailbox::leading_number,
    ],
    tags: ["cast_web"],
}

cast::continues_in! {
    target:  cast_stdlib::storage::bounded_log::BoundedLog,
    concept: "watcher_log_auto_trim",
    why:     "Bounded append-only log over a filesystem directory: \
              MAX_TURNS is the high-water mark, TARGET_TURNS the low-\
              water mark, and trim_to evicts oldest-first to bring \
              the directory between them. The hysteresis amortises \
              eviction so most appends don't pay a delete cost.",
    tags:    ["cast_web"],
}

cast::concept! {
    name: "mailbox_clear_vs_force_restart",
    summary: "Two scrub-state knobs in the cast-web dev loop, in \
              widening order of blast radius. (1) `Mailbox::clear` — \
              wipes the user/assistant chat thread only; daemons keep \
              running, watcher-log untouched, meta.json kept. Reached \
              from the chat panel's `clear` button (POST \
              /mailbox/clear). Use when the thread got noisy or you \
              want to start a fresh conversation. (2) \
              `dev/ensure-cast-web.sh --force-restart` — kills cast-\
              web and cast-watch, removes the socket, then re-boots \
              both daemons against the existing mailbox + watcher-log \
              directories. Source edits to cast-spec / cast-watch / \
              cast-web are picked up; chat history and watcher-log \
              survive. Pick the smallest knob that solves the \
              immediate problem so prior work stays observable: clear \
              before force-restart.",
    anchors: [
        crate::mailbox::Mailbox,
        crate::shim::mailbox_routes::clear,
    ],
    tags: ["cast_web"],
}

cast::io::continues_in! {
    target:  "crates/cast-web/dev/ensure-cast-web.sh",
    lang:    external,
    concept: "mailbox_clear_vs_force_restart",
    why:     "Implements the wider knob: `--force-restart` kills both \
              daemons, removes the socket, and reboots without \
              touching the mailbox or watcher-log directories.",
    tags:     ["cast_web"],
}

cast::concept! {
    name: "force_restart_kill_scope",
    summary: "Restart entry points in the cast-web dev loop must \
              identify their target process via `pkill -x <comm>`, \
              never via substring matches on argv. The kernel \
              truncates `comm` to 15 bytes, and `-x` matches it \
              exactly — so `pkill -x cast-web` hits only the cast-web \
              binary, never the dev shell, the editor, or the bash \
              invocation that mentions `cast-web` in its arguments. \
              Substring matches against argv (e.g. `pkill -f cast-\
              web`) silently widen the kill scope to anything whose \
              command line contains the string and have killed the \
              calling shell mid-script. The constraint is honoured by \
              the live restart path: `dev/ensure-cast-web.sh \
              --force-restart` runs `pkill -x cast-web` and \
              `pkill -x cast-watch` before scrubbing the socket.",
    tags: ["cast_web"],
}

cast::io::continues_in! {
    target:  "crates/cast-web/dev/ensure-cast-web.sh",
    lang:    external,
    concept: "force_restart_kill_scope",
    why:     "The `--force-restart` branch runs `pkill -x cast-web` \
              and `pkill -x cast-watch` to scrub the daemons before \
              rebooting them. Both are exact-comm matches — substring \
              matching here would have killed the shell that runs \
              the script.",
    tags:     ["cast_web"],
}

cast::continues_in! {
    target:  cast_stdlib::trust::least_privilege::LeastPrivilege,
    concept: "force_restart_kill_scope",
    why:     "`pkill -x <comm>` grants the kill operation only the \
              precision it needs (exact 15-byte comm match) and no \
              more — a `pkill -f` substring match would over-grant \
              the kill scope to anything whose argv mentions the \
              string, including the dev shell that runs the restart. \
              Same shape as least-privilege grants in security: \
              every widening of the match predicate widens the blast \
              radius.",
    tags:     ["cast_web"],
}

cast::compare! {
    user_assistant_mailbox @ crate::mailbox::Mailbox: "Conversation \
        between the human and the assistant Claude. Markdown bodies \
        with role-tagged frontmatter; user asks, assistant answers. \
        Numbered files; atomic O_EXCL append; UI thread renders the \
        sequence. The driver of the chat experience.",
    watcher_log @ crate::shim::watcher_routes::WatcherLog: "Log of \
        the assistant ↔ cast-watch conversation. JSON bodies \
        (request/response pairs); the assistant queries, the watcher \
        answers. Numbered files; atomic O_EXCL append. Same on-disk \
        shape, different semantics: this one is a log of an \
        assistant-driven side channel, not a thread the human writes \
        into. Rendered as a separate UI panel so the human can see \
        what the assistant is asking the watcher.",
    tags: ["cast_web"],
    note: "Two append-only directories that share the same \
           filesystem-as-protocol pattern but model different \
           conversations. Splitting them keeps the user-side chat \
           free of debugging noise (queries can fire many times per \
           user message) and gives the watcher-log its own \
           navigation surface for the human to scan.",
}

cast::concept! {
    name: "viewport_fitted_width",
    parent: "cast_web_frontend",
    summary: "Every rendered element in cast-web's UI fits within its \
              container's width on any viewport. Long anchor paths in \
              the concept detail, long markdown bodies in the chat, \
              long URLs in the watcher log, long unbroken identifiers \
              in concept names — all wrap or break rather than push \
              the layout horizontally. Containment is enforced at two \
              levels: the document body via `max-width: 96rem` + \
              `box-sizing: border-box`, and inside each scrolling \
              region via `overflow-wrap: anywhere` (or `word-break: \
              break-word`) on the dynamic-content selectors — \
              `ul.list li` for the three list panels (concepts, \
              messages, watcher-log), `.detail` for concept detail, \
              `.body` for chat message text, and `.turn .r` for \
              watcher-log entries (the original carrier of the \
              invariant — extend that rule to the others, don't \
              re-derive it). The CSS realising this lives in the \
              inline <style> block of `crates/cast-web/web/index.html`.",
    anchors: [
        CAST::AS_PRIMITIVE::crate::frontend,
    ],
    tags: ["cast_web"],
}

cast::continues_in! {
    target:  cast_stdlib::layout::overflow_containment::OverflowContainment,
    concept: "viewport_fitted_width",
    why:     "Overflow containment is the load-bearing stdlib \
              primitive: each child element's effective width is \
              bounded by its container's content box, never the other \
              way around. The CSS rules that realise it in cast-web \
              (max-width on body, overflow-wrap on lists, word-break \
              on message bodies) are local realisations of this \
              single invariant — one container per scrolling region, \
              every child wrapped. Without the invariant a single \
              long anchor path or URL forces a horizontal scrollbar \
              on the whole document and breaks the multi-column \
              layout's column widths.",
    tags:    ["cast_web"],
}

cast::io::continues_in! {
    target:  "crates/cast-web/web/index.html",
    lang:    external,
    concept: "viewport_fitted_width",
    why:     "The CSS that realises the constraint lives in this \
              file's inline <style> block. Selectors that already \
              carry the invariant: `body` (max-width + box-sizing) \
              and `.turn .r` (word-break on watcher-log entries). \
              Selectors that need to be extended to carry it: \
              `.detail` (concept detail can render long anchor \
              paths), `ul.list li` (the three list panels render \
              dynamic concept names / message ids / watcher-log \
              ids), and `.body` (chat message text is `pre-wrap` but \
              has no word-break; long unbroken strings overflow). \
              Edits that add new dynamic-content selectors must \
              extend the constraint or new content can defeat it.",
    tags:     ["cast_web"],
}

cast::concept! {
    name: "chat_priority_over_watcher_log",
    parent: "cast_web_frontend",
    summary: "The right column is split 85/15 between the user ↔ \
              assistant mailbox (top) and the watcher log (bottom), \
              not 50/50. Two reasons: (1) the mailbox panel's \
              effective message-viewing area is reduced by the meta- \
              line, textarea, and send button stacked under the \
              message list, so an even split leaves the actual \
              conversation cramped; (2) the watcher log is debug- \
              shape side traffic — the human glances at it to see \
              what the assistant is asking cast-watch, but does not \
              read it line-by-line. Compressing it to ~15% keeps it \
              visible and scrollable without starving the chat. The \
              CSS that enforces the ratio lives in `.layout > \
              section.right { grid-template-rows: minmax(0, 17fr) \
              minmax(0, 3fr) }` in `crates/cast-web/web/index.html`.",
    anchors: [
        CAST::AS_PRIMITIVE::crate::frontend,
        CAST::AS_PRIMITIVE::crate::mailbox::Mailbox,
        CAST::AS_PRIMITIVE::crate::shim::watcher_routes::WatcherLog,
    ],
    tags: ["cast_web"],
}

pub mod draft;
pub mod frontend;
pub mod mailbox;
pub mod shim;
