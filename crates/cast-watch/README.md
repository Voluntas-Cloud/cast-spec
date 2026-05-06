# cast-watch

> **Highly experimental.** APIs, query-socket protocol, and crate boundaries can change at any time. Pin exact versions if you depend on this. Feedback and breakage reports welcome at <https://github.com/Voluntas-Cloud/cast-spec/issues>.

Live, file-watching daemon for [`cast-spec`](https://crates.io/crates/cast-spec)'s analyzer. Peer to [`cast-extract`](https://crates.io/crates/cast-extract) and [`cast-lsp`](https://crates.io/crates/cast-lsp): all three depend on `cast` with `features = ["analysis"]` and share the analyzer library, so the live daemon, the one-shot CLI, and the LSP server can never disagree about what a `cast::*!` invocation means. A resident daemon that maintains an incremental concept graph and exposes a JSON-lines query socket so LLMs and editor tooling can consult the live model while code is being written.

## What it does

Cast-extract is one-shot: load workspace, analyse, exit. That's right for CI. It's wrong for the inner loop, where you're editing annotations and code continuously and want sub-second feedback.

`cast-watch` keeps the analysis warm:

- Subscribes to filesystem events with `notify`.
- Classifies each change as **macro-only** (cheap, syntactic — milliseconds) or **implementation** (heavy, RA-backed — seconds-to-minutes).
- Macro-only edits commit a new snapshot in-process via `syn`.
- Implementation edits, by default, mark anchors stale; the heavy reload happens only when a client issues `rebuild`.
- A query socket serves the protocol described by `{"kind":"help"}` (request shapes) and `{"kind":"manual"}` (the cast vocabulary as the daemon understands it: macros, fields, languages, warning kinds, recommended workflows). Today's request kinds include bespoke verbs (`snapshot`, `unresolved_in_file`, `concepts_for_path`, `rules_for_path`, `graph_for_tag`), the generic `query` and `walk` graph verbs, plus `subscribe` and `rebuild`. Don't hardcode this list — query `help` / `manual` against the running daemon for the current shape.

Subscribers receive broadcasts when the snapshot changes, each carrying a monotonic `snapshot_generation` number for deduping.

## Install

```bash
cargo install cast-watch
```

## Usage

```bash
# Default: lazy mode — implementation changes mark anchors stale, no auto-rebuild
cast-watch path/to/workspace

# Eager: implementation changes trigger an immediate RA reload
cast-watch --eager path/to/workspace

# Custom socket path
cast-watch --socket /tmp/cast-watch.sock path/to/workspace
```

## Talking to the socket

Every line is a JSON object. One request per line, one response per line. Two ways to talk to the socket:

**Through `cast-web` (recommended).** Run [`cast-web`](https://crates.io/crates/cast-web) in front of cast-watch and POST your queries to its `/watcher/query` endpoint. `cast-web` opens one short-lived socket connection per request, forwards the line, returns the response, and tees the request/response pair to a watcher-log directory the human can tail in the cast-web UI:

```bash
crates/cast-web/dev/cwq.sh '{"kind":"manual"}'
crates/cast-web/dev/cwq.sh '{"kind":"snapshot"}'
crates/cast-web/dev/cwq.sh '{"kind":"concepts_for_path","path":"crate::watcher::handle_macro_only"}'
```

This is the prescribed front door for both humans and LLMs — the watcher-log makes assistant queries observable.

**Direct to the socket.** When no `cast-web` is running, talk to the socket directly. Subscribers (which want broadcasts on the same connection) need a persistent client; one-shots can use any JSON-line transport:

```bash
# One-shot
echo '{"kind":"snapshot"}' | nc -U /tmp/cast-watch.sock

# Subscribe — broadcasts arrive on the same connection
nc -U /tmp/cast-watch.sock <<<'{"kind":"subscribe"}'
```

Responses include the `snapshot_generation` they were produced under, so a client can dedupe a broadcast received right after a response.

Always send `{"kind":"help"}` and `{"kind":"manual"}` once on first connect — they describe the protocol and the cast vocabulary respectively, and they're the source of truth when this README drifts.

## When you'd use it

- **LLM-shaped workflows**: an agent drafts cast annotations, asks "what's still unresolved in this file?" between edits.
- **Editor integration**: surface stale anchors in the gutter without running the full RA pipeline on every save.
- **Concept-first development**: write the spec as `cast::*!` macros first, watch the unresolved list shrink as the implementation fills in.

## Companion crates

- [`cast-spec`](https://crates.io/crates/cast-spec) — vocabulary (always) + analyzer (feature-gated). Published as `cast-spec`, imports as `cast`.
- [`cast-extract`](https://crates.io/crates/cast-extract) — peer one-shot CLI over the same analyzer.
- [`cast-lsp`](https://crates.io/crates/cast-lsp) — peer LSP server. Same analyzer, output as inline editor diagnostics.
- [`cast-web`](https://crates.io/crates/cast-web) — small browser UI + LLM-mailbox shim that fronts a single cast-watch socket.

## License

MIT. See [`LICENSE`](LICENSE).
