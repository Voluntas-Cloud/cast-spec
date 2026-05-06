use serde::{Deserialize, Serialize};

cast::concept! {
    name: "query_protocol",
    summary: "Request/response/broadcast message shapes for the query \
              socket. Requests are LLM-shaped: 'list unresolved anchors \
              in this file', 'what concepts does this symbol participate \
              in', 'what rules govern this function', 'show the concept \
              graph for tag X', 'rebuild now'. Responses always include \
              the snapshot generation number so a client can tell if a \
              broadcast they receive next supersedes a response in flight.",
    anchors: [
        crate::socket::Request,
        crate::socket::Response,
        crate::socket::Broadcast,
        CAST::AS_PRIMITIVE::crate::socket::dispatch_request,
    ],
    tags: ["cast_watch"],
}

cast::continues_in! {
    target:  crate::socket::dispatch_request,
    concept: "query_protocol",
    why:     "query_protocol continues into dispatch at dispatch_request()",
}

cast::continues_in! {
    target:  cast_stdlib::messaging::request_response::RequestResponse,
    concept: "query_protocol",
    why:     "Request/Response is the synchronous spine: the caller \
              sends a `Request` and waits on the same connection for \
              the matching `Response`, with `snapshot_generation` as \
              the correlation key. Broadcast rides the same connection \
              once the caller has subscribed but is logically a sister \
              pattern (publish_subscribe), not part of the same \
              request-reply pair.",
    tags:    ["cast_watch"],
}

cast::rule! {
    rule: "Every Response and Broadcast carries the `snapshot_generation` \
           the daemon was on when the message was produced. Clients \
           use this to dedupe and to detect that a follow-up rebuild \
           has already invalidated their last reply.",
    why:  "Without a generation marker, a client has no way to tell \
           whether the broadcast it just received is new information \
           or a duplicate of the response it just got. LLMs in \
           particular need this signal to avoid reprocessing the same \
           state twice.",
    governs: [
        crate::socket::Response,
        crate::socket::Broadcast,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "The `help` body's per-query field names must equal the \
           request schema's serde-deserialized field names exactly. \
           `Help` is the public protocol contract: if a client copies a \
           name out of help and the server rejects it as 'missing \
           field', the protocol has lied.",
    why:  "An LLM client builds requests from `help` — that is the \
           protocol's whole reason for existing. Drift between a help \
           description (e.g. \"named file\") and the actual schema \
           field (e.g. `path`) breaks first-call attempts and forces \
           the client to read source to recover. Help is a contract, \
           not documentation.",
    governs: [
        crate::socket::Request,
        crate::socket::HelpEntry,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "Detail response bodies serialize the same typed shapes that \
           `cast::emit::model` exports — `PathReport`, `InvocationReport`, \
           `IoReport` — not pre-formatted strings. Same data, same \
           shape, across the socket and the `cast-extract --emit json` \
           CLI.",
    why:  "Returning `Vec<String>` of human-readable lines forces every \
           socket client to write a parser for the daemon's prose, and \
           means the daemon and the CLI describe identical data in two \
           incompatible ways. A client that wants per-anchor outcomes \
           cannot get them from the socket today and must shell out to \
           cast-extract — exactly what the daemon exists to avoid.",
    governs: [
        crate::socket::ResponseBody,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "Heavy queries return their result inline. After `rebuild` \
           commits a new snapshot, the `Rebuilt` body carries the same \
           `Summary` that `Status` and `Snapshot` return — clients \
           never have to round-trip a second query to read what just \
           changed.",
    why:  "The rebuild round-trip is already 30s+ on Voluntas-sized \
           trees; making the client follow up with `status` to see the \
           new counts adds latency for no reason and risks racy reads \
           (a filesystem event between `rebuilt` and `status` would \
           muddy the result). Hand back the result of the work the \
           request actually performed.",
    governs: [
        crate::socket::ResponseBody,
        crate::socket::handle_rebuild,
    ],
    tags: ["cast_watch"],
}

/// Bumped when the wire format changes incompatibly. Surfaced via
/// `help` so a client can tell whether its hardcoded shape still
/// matches the daemon it just connected to.
pub const PROTOCOL_VERSION: u32 = 1;

/// Which stream of records the query draws from.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Stream {
    /// All `cast::*!` invocations (every annotation, regardless of kind).
    Invocations,
    /// Concept records from the concept graph (one per `cast::concept!`
    /// declaration name; declarations sharing a name merge).
    Concepts,
    /// Subset of invocations with kind = Rule. Convenience over
    /// `Invocations` + `where: { kind: rule }`.
    Rules,
    /// Path records — one row per Rust-path field on every invocation,
    /// flattened with the invocation's location and tag.
    Paths,
}

/// Predicate filter applied to selected records. All fields are
/// optional; an unspecified field matches everything. Fields that
/// don't apply to a stream are silently ignored (e.g. `outcome` only
/// affects Paths; ignored on Concepts).
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QueryPredicate {
    /// Exact match on invocation `tag:`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Substring match on invocation file path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_contains: Option<String>,
    /// Annotation kind filter (concept, rule, anti_pattern, …).
    /// Applies to Invocations and Paths streams.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<cast::emit::model::AnnotationKind>,
    /// Path outcome filter — Paths stream only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<cast::emit::model::PathOutcomeRepr>,
    /// Match concepts whose declarations include this Rust path as
    /// an anchor — Concepts stream only.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_anchor: Option<String>,
    /// Anchor-role filter.
    /// - Concepts stream: matches if ANY declaration of the concept
    ///   carries at least one anchor of the requested role.
    /// - Paths stream: matches the path field if its role equals the
    ///   requested role. Non-anchor fields (governs, target, etc.)
    ///   never match — they have no role.
    /// - Invocations stream: ignored (an invocation is one block, not
    ///   one anchor; ask the Paths stream instead).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role: Option<cast::emit::model::AnchorRoleRepr>,
    /// Negative tag filter. Excludes records whose tag matches.
    /// Concepts stream: matches if ANY declaration of the concept
    /// carries the named tag (i.e., the concept is excluded if it is
    /// declared anywhere with that tag). Paths/Invocations streams:
    /// excludes records whose own `tag:` field matches. Useful for
    /// excluding cross-cutting tag groups from a domain query, e.g.
    /// `where: { tag_not: "scaffolding" }`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag_not: Option<String>,
    /// Filter on `cast::continues_in!` / `io::continues_in!` edge
    /// `why:` field. Concepts stream: matches if any of the
    /// concept's outgoing edges has a `why` of the requested kind.
    /// Values: `"lazy"` (deferred-explanation marker), `"prose"`
    /// (writer typed prose), `"absent"` (no `why:` field provided).
    /// Useful for the background fill-in workflow:
    /// `where: { why_kind: "lazy" }` lists concepts with edges
    /// awaiting an LLM to fill in the explanation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub why_kind: Option<WhyKindFilter>,
}

/// Three-way classifier for the `why_kind:` filter on QueryPredicate.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WhyKindFilter {
    /// Edge has `why: lazy` — deferred-explanation marker.
    Lazy,
    /// Edge has `why: "..."` — writer typed prose.
    Prose,
    /// Edge has no `why:` field at all.
    Absent,
}

/// Edge kinds the walk follows. Empty list in a request means "all
/// edge kinds the daemon currently traverses." v1 supports the two
/// kinds that exist as graph edges in `ConceptReport.edges`.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WalkEdge {
    /// `cast::continues_in!` — Rust → Rust. Target resolved via the
    /// anchor index (concepts whose anchors include the edge target
    /// are the next-hop concepts).
    ContinuesIn,
    /// `cast::io::continues_in!` — Rust → foreign file. Recorded in
    /// `io_bridges` and traversed along two indices: by_file (concepts
    /// declared at the target) and by_io_target (sibling concepts
    /// bridging into the same foreign target). `external` lang is the
    /// only short-circuit.
    IoContinuesIn,
}

/// Flashlight brightness — how detailed each hit is in the response
/// independent of how widely walk reached. See the rules pinning the
/// `flashlight` stage at the bottom of this file (`Walk and flashlight
/// are separate primitives` + `Flashlight has two dimensions`). v1
/// implements coarse trimming on `Concepts`-stream responses; flat
/// streams (`Invocations`, `Paths`) currently pass through and gain
/// per-stream `Dim` mapping in a follow-up.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Dim {
    /// Just the identifying key (e.g. concept name). Concept bodies
    /// (declarations, edges) are emptied. Cheapest shape for "what
    /// concepts exist?" lookups.
    NameOnly,
    /// Identifying key plus the first declaration's summary line.
    /// Anchor lists and edges are dropped. Good "give me a glance"
    /// shape — keeps the human-readable axis, drops the structural one.
    Summary,
    /// No trimming — emit the full record. Default. Backward-compat
    /// with callers written before flashlight was exposed.
    Full,
}

impl Default for Dim {
    fn default() -> Self {
        Dim::Full
    }
}

/// Wire-format negotiation for query/walk responses — orthogonal to
/// `Dim`. `Dim` controls *what data* is included (content selection);
/// `OutputFormat` controls *how the data is encoded* (serialization).
/// The two compose: `dim: name_only, format: yaml` is a flat YAML
/// list; `dim: full, format: json` is the legacy structured response.
///
/// The wire envelope stays JSON-lines per the protocol invariant
/// pinned at `socket.rs`. Non-JSON formats land inside a
/// `ResponseBody::Rendered { format, content }` variant: the line
/// itself is JSON, but `content` is the formatted text the caller
/// asked for. This is the same shape cast-extract's `cast::emit::Format`
/// chose for the CLI (one Report → many renderers), adapted for
/// query-shaped responses — see the `cast::compare!` near
/// `output_format` for the explicit precedent.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    /// Default. Emit the typed `ResponseBody` as the body of the
    /// JSON-line — every existing caller keeps working.
    Json,
    /// Render the typed body to a YAML string and wrap it in
    /// `ResponseBody::Rendered`. Cheaper to skim for humans/LLMs;
    /// implementation: serde_yaml over the same Serialize impls.
    Yaml,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Json
    }
}

/// One row in the Paths stream — flattens an invocation's path field
/// up with the invocation's location and tag for direct serialization.
/// Per the `socket.rs:154` rule the embedded `outcome` and `field`
/// names match `cast::emit::model::PathReport`.
#[derive(Debug, Serialize, Clone)]
pub struct PathHit {
    pub file: String,
    pub line: usize,
    pub macro_path: String,
    pub kind: cast::emit::model::AnnotationKind,
    #[serde(default)]
    pub tags: Vec<String>,
    pub field: String,
    pub text: String,
    pub outcome: cast::emit::model::PathOutcomeRepr,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// One bridge encountered during `Walk`. `traversed` is true when
/// walk had at least one neighbour to hop to via this bridge — either
/// concept(s) declared in the target file (Rust by_file index) or
/// sibling concept(s) bridging into the same foreign target
/// (by_io_target hub). `traversed: false` marks true cul-de-sacs:
/// `external` lang (can't anchor) or a target nobody else bridges
/// into. `lang` is set when the source edge was
/// `cast::io::continues_in!` and the parser recovered a language tag
/// (e.g. `Kotlin`, `Vue`).
#[derive(Debug, Serialize, Clone)]
pub struct IoBridgeHit {
    pub from_concept: String,
    pub target: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    pub traversed: bool,
}

pub fn default_hops() -> usize {
    1
}

/// Client → daemon.
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Request {
    /// "Describe the protocol — what queries do you accept?"
    /// Always works, including during cold load.
    Help,
    /// "What's your current phase, uptime, and (if ready) snapshot stats?"
    /// Always works, including during cold load and after a failure.
    Status,
    /// "Describe the cast vocabulary — macros, languages, anchor
    /// conventions, warning kinds, and recommended workflows."
    /// Static reference doc for LLM clients. Always works, including
    /// during cold load — content does not depend on the snapshot.
    Manual,
    /// "What's the current state of the concept graph?"
    Snapshot,
    /// "What anchors in this file are unresolved right now?"
    UnresolvedInFile { path: std::path::PathBuf },
    /// "What concepts mention this Rust path?"
    ConceptsForPath { path: String },
    /// "What rules govern this Rust path?"
    RulesForPath { path: String },
    /// "Show me the concept graph filtered to this tag."
    GraphForTag { tag: String },
    /// "Run a full RA reload now and update the snapshot."
    Rebuild,
    /// "Tell me when the snapshot changes."
    Subscribe,
    /// "Filter the snapshot's typed records by predicate."
    ///
    /// `from:` selects which record stream to draw from; `where:` is
    /// an optional predicate over those records (every field optional;
    /// fields irrelevant to the chosen stream are silently ignored).
    /// Today's bespoke verbs (`unresolved_in_file`, `concepts_for_path`,
    /// `rules_for_path`, `graph_for_tag`) are zero-hop degenerate cases
    /// this verb subsumes — see the `query_language` concept block at
    /// the bottom of this file for the full design.
    Query {
        from: Stream,
        #[serde(default, rename = "where", skip_serializing_if = "Option::is_none")]
        filter: Option<QueryPredicate>,
        /// Flashlight brightness — see the `Dim` enum. Defaults to
        /// `Full` (no trimming) for backward compatibility.
        #[serde(default)]
        dim: Dim,
        /// Flashlight radius — fractional graph distance. `0.0`
        /// (default) is no widening. Integer values include that
        /// many full rings of incident neighbors (`1.0` = ring 1,
        /// `2.0` = rings 1 and 2). Fractional values include a
        /// prefix of the next ring: `0.5` = first half of ring 1
        /// (sorted by concept name); `1.5` = all of ring 1 + first
        /// half of ring 2. The "first half" tiebreaker is
        /// deterministic — sorted by concept name — so a caller can
        /// trust that `radius: 0.3` and `radius: 0.4` are nested
        /// (the smaller is a subset of the larger). Symmetric
        /// widening: outgoing edges from the hit, AND incoming
        /// edges pointing at the hit's anchors/files. See the rule
        /// pinning fractional ring-coherent semantics.
        #[serde(default)]
        radius: f64,
        /// Flashlight edge filter — which edge kinds to widen along
        /// when `radius > 0`. Independent of walk's `follow`.
        /// Empty = all edge kinds (the default). Same enum as walk's
        /// `follow:`.
        #[serde(default)]
        via: Vec<WalkEdge>,
        /// Output format — see the `OutputFormat` enum. Default
        /// `Json` keeps the legacy typed `ResponseBody` body shape;
        /// non-Json formats wrap a rendered string in
        /// `ResponseBody::Rendered`.
        #[serde(default)]
        format: OutputFormat,
    },
    /// "Walk the concept graph from a starting set along typed edges."
    ///
    /// `from:` is a list of concept names to start from. `follow:`
    /// names which edge kinds to traverse (empty = all available);
    /// `hops:` bounds traversal depth (default 1). v1 follows
    /// `continues_in` (Rust → Rust, target resolved via anchor index)
    /// and records `io_continues_in` bridges without traversing
    /// further (cast does not analyze the target language).
    Walk {
        from: Vec<String>,
        #[serde(default)]
        follow: Vec<WalkEdge>,
        #[serde(default = "default_hops")]
        hops: usize,
        /// Flashlight brightness — see the `Dim` enum. Defaults to
        /// `Full` (no trimming).
        #[serde(default)]
        dim: Dim,
        /// Flashlight radius — see notes on `Query::radius`.
        /// Fractional: `0.0` is no widening; `1.0` = full first
        /// ring; `0.5` = first half of ring 1 by concept-name sort.
        #[serde(default)]
        radius: f64,
        /// Flashlight edge filter — see notes on `Query::via`.
        #[serde(default)]
        via: Vec<WalkEdge>,
        /// Output format — see notes on `Query::format`.
        #[serde(default)]
        format: OutputFormat,
    },
    /// "Give me the canonical concept tree."
    ///
    /// One tree per workspace, rooted at the concept declared in
    /// `Cast.cast`. Concepts are placed by longest-prefix match on
    /// embodied anchors (with hoist-to-LCA on multi-anchor); zero-anchor
    /// concepts fall under their crate-umbrella when one is detectable.
    /// Non-concept macros (rules, anti-patterns, comparisons, pipelines,
    /// tiers, matrices, gut_checks) are surfaced as per-concept *counts*
    /// — fetch the full bodies via `tree_expand`.
    Tree,
    /// "Expand one concept's non-concept macros."
    ///
    /// Returns the rules / anti-patterns / comparisons / pipelines /
    /// tiers / matrices / gut_checks attributed to the named concept by
    /// the same longest-prefix-match rule that places concept nodes in
    /// the canonical tree. The renderer fetches this lazily on click.
    TreeExpand {
        concept: String,
    },
    /// "Follow a single foreign-language bridge."
    ///
    /// Single-step alternative to `walk` for `io_continues_in!` edges
    /// whose target is a non-Rust file. The caller names the source
    /// concept and the bridge's target file (plus an optional anchor
    /// for disambiguation when multiple bridges share a target); the
    /// response carries every other concept that lands at the same
    /// target. Composes with walk: walk finds candidate bridges, jump
    /// commits to one. See the `walk-vs-jump` compare in `socket.rs`.
    Jump {
        from: String,
        target: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        anchor: Option<String>,
    },
}

/// Daemon → client (in response to a Request). Write-only: cast-watch
/// emits these but never deserializes them, so we don't carry the
/// `Deserialize` derive — embedded `Summary` / `ConceptReport` types
/// serialize to nested JSON objects, not double-encoded strings.
#[derive(Debug, Serialize)]
pub struct Response {
    pub snapshot_generation: u64,
    pub body: ResponseBody,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ResponseBody {
    Help {
        protocol_version: u32,
        transport: String,
        queries: Vec<HelpEntry>,
    },
    Manual {
        version: String,
        macros: Vec<ManualMacro>,
        languages: Vec<ManualLanguage>,
        spec_sources: Vec<ManualSpecSource>,
        warnings: Vec<ManualWarningKind>,
        workflows: Vec<ManualWorkflow>,
    },
    Status {
        phase: String,
        uptime_seconds: f64,
        roots: Vec<String>,
        repo_root: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        snapshot_generation: Option<u64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        summary: Option<cast::emit::model::Summary>,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<String>,
    },
    Snapshot {
        summary: cast::emit::model::Summary,
        stale_files: Vec<String>,
    },
    UnresolvedInFile {
        entries: Vec<String>,
    },
    ConceptsForPath {
        concepts: Vec<String>,
    },
    RulesForPath {
        rules: Vec<String>,
    },
    GraphForTag {
        graph: std::collections::BTreeMap<String, cast::emit::model::ConceptReport>,
    },
    Rebuilt {
        paths_resolved: u64,
        paths_unresolved: u64,
    },
    Subscribed,
    Error {
        message: String,
    },
    /// Result of `Request::Query`. Per the `socket.rs:154` rule
    /// (detail bodies reuse `cast::emit::model` shapes), `invocations`
    /// and `concepts` carry the same types as the JSON CLI emits.
    /// Exactly one of the optional fields is populated, determined by
    /// the `stream` field — clients can match on `stream` to know which
    /// to read.
    QueryResult {
        stream: Stream,
        count: usize,
        #[serde(skip_serializing_if = "Option::is_none")]
        invocations: Option<Vec<cast::emit::model::InvocationReport>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        concepts: Option<std::collections::BTreeMap<String, cast::emit::model::ConceptReport>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        paths: Option<Vec<PathHit>>,
    },
    /// Result of `Request::Walk`. `visited` contains every concept
    /// reached within `hops` (including the starting set). `io_bridges`
    /// records every `io_continues_in` edge encountered, with
    /// `traversed: bool` indicating whether walk had a neighbour to
    /// hop to. Cul-de-sacs (`external` lang or a target with no other
    /// bridgers and no in-file concept declarations) come back as
    /// `traversed: false`.
    WalkResult {
        starting: Vec<String>,
        follow: Vec<WalkEdge>,
        hops_requested: usize,
        hops_used: usize,
        visited: std::collections::BTreeMap<String, cast::emit::model::ConceptReport>,
        io_bridges: Vec<IoBridgeHit>,
    },
    /// Result of `Request::Tree`. Carries the canonical concept tree
    /// rooted at the workspace umbrella plus the top-level edge list.
    /// See `cast::tree::CanonicalTree` for shape.
    Tree {
        tree: cast::tree::CanonicalTree,
    },
    /// Result of `Request::TreeExpand`. Returns a stable list of the
    /// non-concept macros attributed to the requested concept. v1 ships
    /// the existing `InvocationReport` shape so the renderer has
    /// kind/file/line/tags/paths to work with; richer prose fields
    /// (rule/why/title) will land here once the report model carries
    /// them.
    TreeExpandResult {
        concept: String,
        children: Vec<cast::emit::model::InvocationReport>,
    },
    /// Result of `Request::Jump`. `landing` is every other concept whose
    /// `io_continues_in!` targets the same file as the source bridge.
    /// `lang` reflects the source concept's bridge declaration; `anchor`
    /// echoes the request's anchor for trace-ability.
    JumpResult {
        from: String,
        target: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        lang: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        anchor: Option<String>,
        landing: Vec<String>,
    },
    /// Wrapper carrying a non-JSON serialization of a `QueryResult` /
    /// `WalkResult` body. Emitted only when the request specified a
    /// non-default `format:` (see `OutputFormat`); JSON callers never
    /// see this variant. The wire envelope stays JSON-line per the
    /// protocol invariant — only the body's content is rendered.
    Rendered {
        format: OutputFormat,
        content: String,
    },
}

/// One entry in the `help` response — describes a single request kind.
#[derive(Debug, Serialize)]
pub struct HelpEntry {
    pub kind: String,
    pub summary: String,
    /// `"request_response"` — exactly one response, then the daemon
    /// stops writing on this request. To learn about future state, send
    /// another query or `subscribe`. Polling does not stream.
    /// `"stream"` — the response opens a continuing channel; the daemon
    /// keeps emitting until the client closes the connection.
    pub delivery: String,
    pub request_fields: Vec<HelpField>,
    pub response: String,
}

#[derive(Debug, Serialize)]
pub struct HelpField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub required: bool,
}

// ─── Manual reference shapes ────────────────────────────────────────────

#[derive(Debug, Serialize)]
pub struct ManualMacro {
    pub path: String,
    pub purpose: String,
    pub anchor_required: String,
    pub fields: Vec<ManualField>,
}

#[derive(Debug, Serialize)]
pub struct ManualField {
    pub name: String,
    #[serde(rename = "type")]
    pub field_type: String,
    pub required: bool,
    pub summary: String,
}

#[derive(Debug, Serialize)]
pub struct ManualLanguage {
    pub value: String,
    pub backend: String,
    pub anchor_supported: bool,
    pub anchorable_kinds: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ManualSpecSource {
    pub kind: String,
    pub files: String,
    pub anchor_form: String,
    pub notes: String,
}

#[derive(Debug, Serialize)]
pub struct ManualWarningKind {
    pub kind: String,
    pub trigger: String,
    pub fix: String,
}

#[derive(Debug, Serialize)]
pub struct ManualWorkflow {
    pub name: String,
    pub summary: String,
    pub steps: Vec<String>,
}

/// Daemon → subscriber (unprompted). Write-only — see `Response`.
#[derive(Debug, Clone, Serialize)]
pub struct Broadcast {
    pub snapshot_generation: u64,
    pub kind: BroadcastKind,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum BroadcastKind {
    /// Snapshot was replaced; subscribers should refetch what they care about.
    SnapshotChanged,
    /// An RA reload is starting; queries against the prior snapshot remain valid.
    RebuildStarted,
    /// An RA reload finished and the new snapshot is in effect.
    RebuildCompleted,
}

cast::concept! {
    name: "wire_request_dispatch",
    summary: "Sum-typed envelopes on the JSON-lines protocol. Each \
              variant names one request kind, response body, stream \
              type, walk edge, dimension, output format, or broadcast \
              kind; the dispatcher matches and routes.",
    anchors: [
        crate::socket::protocol::Request,
        crate::socket::protocol::ResponseBody,
        crate::socket::protocol::Stream,
        crate::socket::protocol::WalkEdge,
        crate::socket::protocol::Dim,
        crate::socket::protocol::OutputFormat,
        crate::socket::protocol::BroadcastKind,
    ],
    tags: ["cast_watch_protocol"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "wire_request_dispatch",
    why: "Each is an enum with closed variants; exactly one inhabited \
          per value.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::serializable,
    concept: "wire_request_dispatch",
    why: "All derive Serialize/Deserialize; serde tag conventions \
          define the JSON wire shape.",
}

cast::concept! {
    name: "wire_request_payload",
    summary: "Product-typed payload structures inside the protocol: \
              Response envelope, predicates, helper hits, manual and \
              help entries.",
    anchors: [
        crate::socket::protocol::QueryPredicate,
        crate::socket::protocol::PathHit,
        crate::socket::protocol::IoBridgeHit,
        crate::socket::protocol::Response,
        crate::socket::protocol::HelpEntry,
        crate::socket::protocol::HelpField,
        crate::socket::protocol::ManualMacro,
        crate::socket::protocol::ManualField,
        crate::socket::protocol::ManualLanguage,
        crate::socket::protocol::ManualSpecSource,
        crate::socket::protocol::ManualWarningKind,
        crate::socket::protocol::ManualWorkflow,
    ],
    tags: ["cast_watch_protocol"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "wire_request_payload",
    why: "Each is a struct with all fields simultaneously inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::serializable,
    concept: "wire_request_payload",
    why: "All derive Serialize/Deserialize; the wire-format JSON IS \
          the value.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "wire_request_dispatch",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::typed_handle,
    concept: "wire_request_payload",
    why: lazy,
}
