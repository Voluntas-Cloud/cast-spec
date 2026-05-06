# Cast — Macro Grammar

Cast macros are token-level no-ops at compile time. The analyzer parses
the token streams via `syn` and enforces this grammar. Anything not
described here is rejected. Four peer binaries front the analyzer —
`cast-extract` (one-shot CLI), `cast-watch` (resident daemon),
`cast-lsp` (LSP server), and `cast-web` (HTTP/WebSocket shim over a
cast-watch socket). All four parse macros against this grammar.

`docs/ARCHITECTURE_PROSE_STYLE.md` is the conceptual ancestor of this
vocabulary — the writing-style note that spored the idea to cast.
Its block types (Compare, Pipeline, Tier, Matrix, Rule, Anti-pattern,
Gut-check) became the seven `cast::*!` macros named below. Read it for
intent; this file is the surface syntax.

---

## Cross-cutting decisions

| # | Decision | Choice | Reason |
|---|----------|--------|--------|
| C1 | Common optional fields | `tag:`, `since:`, `note:` accepted on every macro | `tag` groups annotations into a concept-graph subset; `since` records the commit/date the assumption was added; `note` is escape valve |
| C2 | Field syntax | `field: value` (colon, not equals) | Reads as a map. `=` reads as Rust assignment and invites confusion with `const`/let bindings |
| C3 | Strictness on required fields | Extractor fails CI on missing required field, unknown field, or duplicate field | Silent drops are how `fi.tt` happened. The grammar must be load-bearing or it is decorative |

Common optional fields — semantics:

```
tag:   "<short_kebab_or_snake>"   group label for graph filtering
since: "<YYYY-MM-DD or commit>"   provenance, for staleness checks
note:  "<free-form prose>"         human aside; not parsed beyond capture
```

Order of fields inside a macro is free; the extractor sorts on output.

---

## Path anchors

Every concept Cast captures must, where possible, anchor to actual code.
This is what stops Cast from drifting into decorative comments.

Two attachment forms:

```
block-level:    at: [path, path, ...]       attaches the whole block
entry-level:    key @ path: value           attaches one entry
```

The `@ path` form tokenises as a Rust attribute-style binding inside the
macro. The extractor parses it via `syn::Path` and verifies the path
resolves in the workspace.

**Path fields are required on macros where the link to code is the whole
point** (`rule!`, `anti_pattern!`, `compare!`, `concept!`,
`continues_in!`). They are optional on macros that exist primarily to
shape *prose* (`tier!`, `matrix!`, `gut_check!`, `pipeline!`'s `flow`),
but the extractor warns when they are missing — an unanchored block is a
note, not an annotation.

Validation rules:

```
path must parse as syn::Path
path must resolve to an item in the workspace (struct, fn, mod, trait,
  enum, type alias, const, static)
unresolved path -> CI failure (C3)
```

External (non-Rust) anchors use `cast::io::continues_in!` instead;
plain `at:` and `@` are Rust-only.

---

## Common value forms

```
ident       = bare Rust identifier              fragile, intent_id
path        = Rust path                         sample::reconciler::apply
str         = double-quoted string              "single point of failure"
list[T]     = bracketed comma-separated         [a, b, c]
map<K,V>    = brace-block of `K: V,` entries    { fragile: "...", recoverable: "..." }
```

Trailing commas are always allowed and encouraged.

---

## Block macros

### `cast::compare!`

Two-or-more named concepts contrasted side-by-side. Each entry **must**
anchor to a Rust path — comparisons are between two real things.

```rust
cast::compare! {
    intent_id   @ sample::types::IntentId:
        "stable identity, used for traceability across logs",
    intent_hash @ sample::types::IntentHash:
        "replay-guard key, bound to exact server-signed bytes",
}
```

| field | type | required | notes |
|-------|------|----------|-------|
| (entries) | `map<ident @ path, str>` | yes, ≥2 | Each key carries a `@ path` anchor |

Extractor enforces:
- ≥2 entries (a "comparison" of one is a `note:`)
- every entry has a `@ path` anchor that resolves
- paths must be **distinct** — comparing a thing to itself is a typo

---

### `cast::pipeline!`

A directed chain of named stages. **Named, not positional**, so other macros
(`rule!`, `anti_pattern!`) can reference a specific stage by name.

Stages are declared *separately* from the flow so that anchor paths are
written once, not every time the stage appears in a `->` edge.

```rust
cast::pipeline! {
    stages: {
        raw_facts               @ sample_agent::heartbeat::report,
        normalized_capabilities @ sample::stability::normalize,
        cluster_capabilities    @ sample::stability::aggregate,
        stability_assessment    @ sample::stability::assess,
        upgrade_opportunities   @ sample::stability::propose,
    },
    flow: [
        raw_facts               -> normalized_capabilities,
        normalized_capabilities -> cluster_capabilities,
        cluster_capabilities    -> stability_assessment,
        stability_assessment    -> upgrade_opportunities,
    ],
}
```

| field | type | required | notes |
|-------|------|----------|-------|
| `stages` | `map<ident @ path, _>` (path optional per stage) | yes, ≥2 | Path anchors are *recommended*; warn-only when missing |
| `flow` | `list[ident -> ident]` | yes, ≥1 | Edges. Every ident referenced must appear in `stages` |
| `cyclic` | `bool` | no, default `false` | Allow cycles (e.g. retry loops) |
| `entry` | `ident` | no | Pin the start node when ambiguous |

The extractor:
1. Verifies every `flow` ident is declared in `stages` (no implicit nodes).
2. Builds an edge list and topologically sorts.
3. Errors on cycles unless `cyclic: true`.
4. Errors on disconnected sub-chains (one pipeline is one DAG).
5. Warns on stages missing `@ path` (a stage with no code anchor is
   suspicious — it usually means the stage is aspirational).

---

### `cast::tier!`

Ordered named buckets along one axis (strictness, risk, maturity).

```rust
cast::tier! {
    axis:      stability,
    direction: increasing,
    of:        sample::stability::StabilityLevel,
    stages: {
        fragile           @ sample::stability::StabilityLevel::Fragile:
            "single point of failure exists",
        recoverable       @ sample::stability::StabilityLevel::Recoverable:
            "failure causes downtime, data survives",
        resilient         @ sample::stability::StabilityLevel::Resilient:
            "failure tolerated with degraded service",
        highly_available  @ sample::stability::StabilityLevel::HighlyAvailable:
            "failure tolerated transparently",
    },
}
```

| field | type | required | notes |
|-------|------|----------|-------|
| `axis` | `ident` | yes | What is being tiered (e.g. `stability`, `risk`, `trust`) |
| `direction` | `ident` (`increasing` \| `decreasing`) | no, default `increasing` | Whether later stages are "more" or "less" |
| `of` | `path` | no | The Rust enum/type that materialises the tier; if set, every stage's `@ path` must be a variant of it |
| `stages` | `map<ident @ path?, str>`, ordered | yes, ≥2 | Order matters; per-stage anchor is optional but warned when missing |

Direction is explicit because `fragile → highly_available` is increasing
*safety* but a `risk` tier reads in the opposite direction.

---

### `cast::matrix!`

Several rows sharing a column structure.

```rust
cast::matrix! {
    columns: [derived, stored, scope, example],
    rows: {
        hotp_tan        @ sample::auth::tan::HotpTan:
            ["yes", "seed only",   "any login",    "RFC 4226"],
        static_tan_list @ sample::auth::tan::StaticTanList:
            ["no",  "finite bag",  "any login",    "recovery codes"],
        transaction_tan @ sample::auth::tan::TransactionTan:
            ["yes", "server-side", "bound to txn", "photoTAN"],
    },
}
```

| field | type | required | notes |
|-------|------|----------|-------|
| `columns` | `list[ident]` | yes, ≥1 | Header row |
| `rows` | `map<ident @ path?, list[str]>` | yes, ≥1 | Each list length must equal `columns.len()`. Per-row anchor optional but warned when missing |

Extractor errors on row-arity mismatch — silent column drift is the failure
mode this macro exists to prevent.

---

### `cast::rule!`

A short imperative rule + reason. The single most-used block in the work
files.

```rust
cast::rule! {
    rule: "Unify at the task level, separate at the secret/lifecycle level",
    why:  "Mixing storage models for derived vs stored secrets produces \
           either silent replay or breakage",
    governs: [
        sample::auth::credential_router,
        sample::auth::vault::derived,
        sample::auth::vault::stored,
    ],
}
```

| field | type | required | notes |
|-------|------|----------|-------|
| `rule` | `str` | yes | Imperative; should fit on one screen line |
| `why` | `str` | yes | One-line reason. Multi-line via `\` continuation |
| `governs` | `list[path]` | yes, ≥1 | Rust paths the rule governs. Extractor verifies each resolves |

`governs` is what makes `rule!` load-bearing rather than decorative. A rule
the extractor cannot attach to code is just a comment with structure — and
we already have those.

---

### `cast::anti_pattern!`

"Don't do it this way, because…" with the better path named.

```rust
cast::anti_pattern! {
    avoid:      "if (x && y && z) then do_thing",
    why:        "spaghetti decisions are unmaintainable and unauditable",
    instead:    "inputs -> evaluation -> ranked options -> chosen action",
    instead_at: sample::decision::evaluate,
    governs:    [sample::decision],
}
```

| field | type | required | notes |
|-------|------|----------|-------|
| `avoid` | `str` | yes | The tempting wrong path |
| `why` | `str` | yes | What goes wrong |
| `instead` | `str` | yes | The right shape — required, because an anti-pattern without an alternative is a complaint |
| `instead_at` | `path` | no | The path that *implements* the right shape. Required when `instead:` describes a shape that exists in this codebase |
| `governs` | `list[path]` | yes, ≥1 | Code regions where this anti-pattern is tempting. Extractor verifies each resolves |

---

### `cast::gut_check!`

A single binary question that resolves a recurring ambiguity at the
*writing* layer.

```rust
cast::gut_check! {
    question: "Can the code be regenerated later?",
    yes:      "it is a key (TOTP/HOTP)",
    yes_at:   sample::auth::vault::derived,
    no:       "it is a stored artifact (recovery code, TAN)",
    no_at:    sample::auth::vault::stored,
}
```

| field | type | required | notes |
|-------|------|----------|-------|
| `question` | `str` | yes | Yes/no shape |
| `yes` | `str` | yes | What follows from "yes" |
| `yes_at` | `path` | no | Where "yes" lands in code |
| `no` | `str` | yes | What follows from "no" |
| `no_at` | `path` | no | Where "no" lands in code |

`yes_at` / `no_at` are the difference between a writing-time gut-check and
a code-time one. When both are set, the extractor links the gut-check to
both branches.

Use sparingly. The style doc warns against over-using gut-checks; the
grammar does not enforce that, but the extractor emits a warning if more
than one `gut_check!` exists per module.

---

## Reference macros

### `cast::continues_in!`

A concept that continues into another part of *the same Rust workspace*.

```rust
cast::continues_in! {
    target:  sample::reconciler::apply::dispatch,
    concept: "intent_envelope_consumption",
    why:     "the envelope created here is consumed by the reconciler",
}
```

| field | type | required | notes |
|-------|------|----------|-------|
| `target` | `path` | yes | Rust path. Extractor parses with `syn::Path` and verifies it resolves in the workspace |
| `concept` | `str` | yes | The named idea travelling between sites; used to build the cross-file concept graph |
| `why` | `str` | yes | Why the reader should follow the link |

Validation: **strict**. Path must resolve; concept name must match a
counterpart on the target side (either another `continues_in!` pointing
back, or the target site declares `concept_anchor!` — TBD if needed).

---

### `cast::io::continues_in!`

A concept that continues into code *outside Rust* — Kotlin, TypeScript,
Vue, an RFC, a vendor doc. The "io" namespace flags that the target cannot
participate in the annotation graph itself.

```rust
cast::io::continues_in! {
    target:  "samples/external/keystore_manager.kt",
    lang:    kotlin,
    concept: "intent_envelope_consumption",
    why:     "external side stores the envelope and signs it",
}
```

| field | type | required | notes |
|-------|------|----------|-------|
| `target` | `str` | yes | Repo-relative path, URL, or RFC identifier |
| `lang` | `ident` | yes | One of: `c`, `kotlin`, `swift`, `typescript`, `vue`, `yaml`, `sql`, `shell`, `markdown`, `rfc`, `external` |
| `concept` | `str` | yes | Same role as in `continues_in!` |
| `why` | `str` | yes | |
| `anchor` | `str` | no | A symbol/section to grep for inside `target` (function name, header) |

Validation is **graduated** by `lang`:

```
lang = c | kotlin | swift | typescript | shell | yaml
  → file must exist; tree-sitter backend (when registered) parses the
    target and finds `anchor:` as a declaration node, falling back to
    a language-aware grep otherwise. See docs/tree-sitter.md
lang = vue | sql | markdown
  → file must exist; if `anchor:` present, grep with language-aware
    declaration prefixes must find it (no tree-sitter grammar yet)
lang = rfc
  → target must match `RFC \d+(#section-...)?`; URL fetch is optional, off by default
lang = external
  → file/URL existence only; no grep. RESERVED for things whose source
    we genuinely do not have access to (third-party APIs, vendor docs).
    Code in sibling workspaces gets a real lang, not `external`
```

`lang` is **required even when the extension is unambiguous**, because the
extractor's validation rule is keyed on `lang`, not on the path. Making it
explicit prevents the rule from drifting if someone renames a `.kt` to
`.kts` later.

---

## Module-level macro

### `cast::concept!`

Declares a concept name that other annotations reference. Optional —
`continues_in!` can reference a concept that was never formally declared,
and the extractor will still build the graph. Use `concept!` when the
concept needs prose that does not belong inside any single block.

```rust
cast::concept! {
    name: "intent_envelope_consumption",
    summary: "An intent created server-side, signed by mobile, consumed once \
              by the reconciler. Replay-guarded by intent_hash.",
    anchors: [
        sample::api::mobile::sign_request,
        sample::reconciler::apply::dispatch,
        sample::types::IntentHash,
    ],
    tag: "trust_foundation",
}
```

| field | type | required | notes |
|-------|------|----------|-------|
| `name` | `str` | yes | The string `continues_in!` invocations match against |
| `summary` | `str` | yes | Prose that does not belong in any single block |
| `anchors` | `list[path]` | no, may be empty | Rust paths that *participate in* this concept. Extractor verifies each resolves. A concept with 0 or 1 anchor is the **primitive** shape — a vocabulary atom that domain concepts reference via `continues_in!` / `io::continues_in!` merge-by-name. Forcing ≥1 anchor would block primitives like `encrypted_connection` that have no single canonical home |

---

## Common-field reminder

Every macro above also accepts:

```rust
tag:   "trust_foundation",
since: "2026-04-12",
note:  "introduced when fi.tt #4 landed; revisit after second-node milestone",
```

Extractor uses these for:
- `tag` — graph slicing. The analyzer carries the tag through to every
  emitted record so downstream tools (e.g. cast-watch's
  `graph_for_tag` query) can filter on it
- `since` — provenance and staleness; passed through, not interpreted
  by any current CLI flag
- `note` — passed through verbatim, not interpreted

---

## What the extractor will do (forward reference)

Not implemented yet. Listed here so the grammar is judged against the use
case it serves:

1. Walk the workspace via `syn`, find every `cast::*!` and `cast::io::*!`.
2. Parse each invocation per this grammar; **fail CI** on any deviation.
3. Build a concept graph keyed on `concept:` strings.
4. Emit:
   - JSON dump of all annotations (for tooling)
   - Markdown report grouping by `tag:` (for humans)
   - Cross-language link report — every `io::continues_in!` validated per its `lang`
5. Report orphaned concepts (referenced by `continues_in!` but no anchor) and
   one-sided links (A points at B, B does not point back) — **warning, not
   error**, because asymmetry is sometimes correct (Rust → RFC is one-way).

---

## Open questions deferred

- Whether `continues_in!` should have a counterpart `concept_anchor!` for
  the target side, or whether matching `concept:` strings on both ends is
  enough. Lean: matching strings; `concept!` declarations already serve as
  anchors when one is needed.
- Whether `pipeline!` should permit forks (one stage feeding two
  successors). Current grammar permits it (DAG, not list); the extractor
  will need to render forks coherently.
- Whether `tag:` should be a list (multi-tag) or single ident. Lean: single
  ident, because multi-tagging tempts ad-hoc taxonomies; if a concept truly
  belongs to two tag-spaces, declare it twice via `concept!`.
- Whether `@ path` anchors should accept *expressions* (e.g. trait method
  calls, `<T as Trait>::method`) or just `syn::Path`. Lean: `syn::Path`
  only — anchors are identifiers for items, not call-sites.
- Whether `governs:` paths must be modules/items, or whether they may point
  at specific lines (file path + line range). Lean: items only; line
  ranges drift on every refactor, items survive.
