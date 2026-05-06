# cast-web

> **Highly experimental.** APIs, HTTP routes, and crate boundaries can change at any time. Pin exact versions if you depend on this. Feedback and breakage reports welcome at <https://github.com/Voluntas-Cloud/cast-spec/issues>.

Small Rust HTTP/WebSocket shim plus a plain-TypeScript frontend that lets a human (and optionally a Claude session) browse and refine the cast concept graph served by [`cast-watch`](https://crates.io/crates/cast-watch).

To cast what `gitweb` / `cgit` are to git: a single-purpose, readable, useful-out-of-the-box web UI over one data plane (cast-watch's Unix socket, in our case; a `.git` directory in theirs). If you want the equivalent of Forgejo or GitHub for cast — multi-tenant, plugin-rich, configurable — start a sibling project that also speaks cast-watch. Don't extend cast-web in place.

## Three responsibilities

That's the whole surface; resist anything that doesn't fit:

1. **Data plane** — relay JSON-line traffic between the browser and a single cast-watch Unix socket.
2. **Mailbox** — serve a directory of numbered markdown messages over SSE, append the next-numbered file on POST.
3. **Static** — serve the embedded compiled-from-TypeScript frontend bundle.

The Rust shim never imports the `cast` analyzer. It depends on `cast` for vocabulary only and reaches the analyzer through cast-watch's socket — the encapsulation seam. (See `cast::rule!` in `src/lib.rs`.)

## Install

```bash
cargo install cast-web
```

## Usage

```bash
# Minimum: point at a running cast-watch socket
cast-web --socket /tmp/cast-watch.sock

# Custom HTTP bind (default 127.0.0.1:8765)
cast-web --socket /tmp/cast-watch.sock --http 0.0.0.0:9000

# Enable the mailbox plane for LLM collaboration
cast-web --socket /tmp/cast-watch.sock --mailbox-dir ~/.cast-mailbox/<session>

# Enable the watcher-log plane (assistant queries tee'd to disk)
cast-web --socket /tmp/cast-watch.sock --watcher-log-dir /tmp/cast-web-log
```

Open `http://127.0.0.1:8765` to browse the concept graph.

## HTTP surface

Always mounted:

| Route | Method | What it does |
|---|---|---|
| `/` | GET | Embedded `index.html` (the frontend entry) |
| `/static/*path` | GET | Embedded TypeScript bundle |
| `/ws` | GET (upgrade) | WebSocket bridge — one cast-watch Unix socket per client; JSON lines pass through in both directions |
| `/watcher/query` | POST | One-shot synchronous query: open short-lived socket → forward one line → return one line. Tee'd to `--watcher-log-dir` if set |

Mounted when `--mailbox-dir` is set:

| Route | Method | What it does |
|---|---|---|
| `/mailbox/send` | POST | Append next-numbered message file (atomic O_EXCL) |
| `/mailbox/messages` | GET | List messages parsed from the directory |
| `/mailbox/meta` | GET | Session liveness — `last_heartbeat_at` from `meta.json` |
| `/mailbox/stream` | GET (SSE) | Push notifications when files arrive in the directory |

Mounted when `--watcher-log-dir` is set:

| Route | Method | What it does |
|---|---|---|
| `/watcher/log/entries` | GET | List query/response turns in the log directory |
| `/watcher/log/stream` | GET (SSE) | Push notifications when new log files appear |

## CLI client

`/watcher/query` accepts any JSON-line request. Smoke test with `curl`:

```bash
curl -fsS -X POST -H 'content-type: application/json' \
  -d '{"kind":"manual"}' http://127.0.0.1:8765/watcher/query
curl -fsS -X POST -H 'content-type: application/json' \
  -d '{"kind":"query","from":"paths","where":{"outcome":"unresolved"}}' \
  http://127.0.0.1:8765/watcher/query
```

The repo's `crates/cast-web/dev/` directory has thin shell wrappers (`cwq.sh`, `cwqpy.sh`) for ad-hoc development use; they are unreviewed helpers and not bundled with the published crate.

## Mailbox shape

A directory of numbered markdown files. Each file is one message; append = create the next-numbered file. No delimiter parsing, no partial-write corruption.

```
~/.cast-mailbox/<session>/
├── meta.json           # { session_id, started_at, last_heartbeat_at }
├── 001-prompt.md       # role: user
├── 002-reply.md        # role: assistant
├── 003-draft.md        # role: user, attaches a structured draft
└── ...
```

See `examples/mailbox/` for the wire shape — frontmatter + markdown body + optional fenced JSON blocks tagged `json draft` / `json applied` / `json question` / `json alternatives`.

If `last_heartbeat_at` in `meta.json` goes stale (>90s), the UI grays the "send" button and shows "no Claude session attached." cast-web works without any Claude session — the AI features just gray out.

## Watcher-log shape

When `--watcher-log-dir` is set, every `/watcher/query` round trip is tee'd to disk as a numbered JSON pair:

```
/tmp/cast-web-log/
├── 001-query.json
├── 002-response.json
├── 003-query.json
├── 004-response.json
└── ...
```

Same atomic O_EXCL append shape as the mailbox. The UI tails the directory and renders the assistant ↔ watcher conversation as a separate panel, so a human watching the cast-web tab can see what the assistant is asking the watcher in real time.

## Frontend

Plain TypeScript, **no framework, no bundler**. `web/src/main.ts` is the entry; `tsc` compiles it to `web/dist/` 1:1; the browser loads via native ESM (`<script type="module">`); `rust-embed` bakes the compiled bundle into the release binary.

Three-pane UI:
- **Left**: concept list (over `/ws`).
- **Middle**: concept detail (over `/watcher/query` on click).
- **Right**: two stacked mailboxes — user/assistant chat + watcher log.

## Companion crates

- [`cast-spec`](https://crates.io/crates/cast-spec) — vocabulary (always) + analyzer (feature-gated). cast-web uses vocabulary only.
- [`cast-watch`](https://crates.io/crates/cast-watch) — the daemon cast-web fronts. Required at runtime.
- [`cast-extract`](https://crates.io/crates/cast-extract) — peer one-shot CLI over the analyzer.
- [`cast-lsp`](https://crates.io/crates/cast-lsp) — peer LSP server over the analyzer.

## License

MIT. See [`LICENSE`](LICENSE).
