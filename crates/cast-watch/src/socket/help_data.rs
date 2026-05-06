use super::protocol::{
    HelpEntry, HelpField, ManualField, ManualLanguage, ManualMacro, ManualSpecSource,
    ManualWarningKind, ManualWorkflow, ResponseBody, PROTOCOL_VERSION,
};
use cast::parser::manual::{FieldDoc, COMMON_FIELDS};
use cast::parser::{
    anti_pattern, compare, concept, continues_in, gut_check, io_continues_in, matrix,
    pipeline, policy, rule, tier,
};

/// Convert a parser-side `FieldDoc` into the manual's wire-shape
/// `ManualField`. Pure projection — keeps the manual response stable
/// even if the parser-side struct grows fields the wire doesn't carry.
fn from_field_doc(d: &FieldDoc) -> ManualField {
    ManualField {
        name: d.name.into(),
        field_type: d.kind.into(),
        required: d.required,
        summary: d.doc.into(),
    }
}

/// Build a `ManualMacro` for the given path. `parser_fields` is the
/// macro-specific FIELDS const exported by the parser module; common
/// fields (tags / since / note) are appended automatically.
fn make_macro(
    path: &str,
    purpose: &str,
    anchor_required: &str,
    parser_fields: &[FieldDoc],
) -> ManualMacro {
    let mut fields: Vec<ManualField> =
        parser_fields.iter().map(from_field_doc).collect();
    fields.extend(COMMON_FIELDS.iter().map(from_field_doc));
    ManualMacro {
        path: path.into(),
        purpose: purpose.into(),
        anchor_required: anchor_required.into(),
        fields,
    }
}

pub fn help_body() -> ResponseBody {
    fn field(name: &str, ty: &str) -> HelpField {
        HelpField { name: name.into(), field_type: ty.into(), required: true }
    }
    let entry = |kind: &str,
                 summary: &str,
                 delivery: &str,
                 fields: Vec<HelpField>,
                 response: &str|
     -> HelpEntry {
        HelpEntry {
            kind: kind.into(),
            summary: summary.into(),
            delivery: delivery.into(),
            request_fields: fields,
            response: response.into(),
        }
    };
    let req = "request_response";
    let stream = "stream";
    let queries = vec![
        entry(
            "help",
            "Return this protocol description. Always works.",
            req,
            vec![],
            "help { protocol_version, transport, queries[] }",
        ),
        entry(
            "status",
            "Daemon phase (loading|ready|failed), uptime, roots, and \
             — when ready — snapshot generation and summary stats. \
             Always works.",
            req,
            vec![],
            "status { phase, uptime_seconds, roots[], repo_root, snapshot_generation?, summary?, error? }",
        ),
        entry(
            "manual",
            "Cast vocabulary reference — macro grammar, supported \
             foreign-language anchors, spec-source rules (.rs vs \
             .cast files), warning kinds, and recommended workflows. \
             Static; does not depend on the snapshot. New LLM \
             clients should fetch this once after `help` to learn \
             how to write annotations correctly without grepping \
             cast source. Always works.",
            req,
            vec![],
            "manual { version, macros[], languages[], spec_sources[], warnings[], workflows[] }",
        ),
        entry(
            "snapshot",
            "Current concept-graph summary plus the stale-files list.",
            req,
            vec![],
            "snapshot { summary, stale_files[] }",
        ),
        entry(
            "unresolved_in_file",
            "Anchor entries that don't resolve in the named file. The TODO list \
             for finishing an annotation pass.",
            req,
            vec![field("path", "path")],
            "unresolved_in_file { entries[] }",
        ),
        entry(
            "concepts_for_path",
            "Concepts whose anchors include this Rust path.",
            req,
            vec![field("path", "string")],
            "concepts_for_path { concepts[] }",
        ),
        entry(
            "rules_for_path",
            "Rules whose body mentions this Rust path.",
            req,
            vec![field("path", "string")],
            "rules_for_path { rules[] }",
        ),
        entry(
            "graph_for_tag",
            "Concept graph filtered to a single cast tag.",
            req,
            vec![field("tag", "string")],
            "graph_for_tag { graph{name -> concept} }",
        ),
        entry(
            "rebuild",
            "Force a full rust-analyzer reload now. Heavy — this is the \
             only query that pays the cold-load cost. File-system edits \
             already trigger automatic rebuilds; send this only to \
             override the lazy classifier.",
            req,
            vec![],
            "rebuilt { paths_resolved, paths_unresolved }",
        ),
        entry(
            "subscribe",
            "Stream broadcasts on this connection when the snapshot \
             changes. Subsequent broadcasts arrive interleaved with \
             responses on the same socket. THIS is how a client tracks \
             state changes — repeated `status` queries do not deliver \
             updates, they only sample the current generation.",
            stream,
            vec![],
            "subscribed (then snapshot_changed / rebuild_started / rebuild_completed broadcasts on the same connection)",
        ),
        entry(
            "query",
            "Filter the snapshot's typed records by predicate. `from` \
             picks the record stream (invocations | concepts | rules | \
             paths). `where` is an optional predicate; every field is \
             optional and fields irrelevant to the chosen stream are \
             ignored. v1 predicate fields: tag, file_contains, kind, \
             outcome (paths only), has_anchor (concepts only). \
             Subsumes today's bespoke verbs — e.g. \
             `unresolved_in_file` ≈ from:paths, where:{outcome:unresolved, file_contains:<path>}. \
             `dim`, `radius`, and `via` come from the flashlight \
             stage — see the `walk` entry below for their semantics. \
             `format` picks the wire encoding of the body — `json` \
             (default) keeps the typed `query_result` shape; `yaml` \
             wraps a yaml-rendered string in a `rendered` body.",
            req,
            vec![
                HelpField { name: "from".into(), field_type: "stream".into(), required: true },
                HelpField { name: "where".into(), field_type: "predicate?".into(), required: false },
                HelpField { name: "dim".into(), field_type: "name_only|summary|full?".into(), required: false },
                HelpField { name: "radius".into(), field_type: "float?".into(), required: false },
                HelpField { name: "via".into(), field_type: "edge[]?".into(), required: false },
                HelpField { name: "format".into(), field_type: "json|yaml?".into(), required: false },
            ],
            "query_result { stream, count, invocations? | concepts? | paths? } | rendered { format, content }",
        ),
        entry(
            "walk",
            "Walk the concept graph from a starting set along typed \
             edges. `from` is a list of concept names. `follow` lists \
             which edge kinds to traverse (continues_in, \
             io_continues_in); empty = all available. `hops` bounds \
             traversal depth (default 1). v1 traverses continues_in \
             via the anchor index and records io_continues_in as \
             bridges without traversing further (cast does not \
             analyze the target language). \
             FLASHLIGHT: `dim` controls per-hit brightness — \
             `name_only` (just the concept name), `summary` (name + \
             first declaration's summary line; anchors and edges \
             dropped), `full` (default, no trimming). `radius` is a \
             FRACTIONAL graph distance: integer values include that \
             many full rings of symmetric incident neighbors (`1.0` \
             = ring 1, `2.0` = rings 1+2); fractional values include \
             a prefix of the next ring sorted by concept name (`0.5` \
             = first half of ring 1; `1.5` = ring 1 + first half of \
             ring 2). `0.0` (default) is no widening. The sort \
             tiebreaker makes nested-radius requests strict subsets — \
             `radius: 0.3` is always a subset of `radius: 0.4`. \
             `via` filters the edge kinds flashlight follows when \
             widening; empty (default) = all edge kinds, independent \
             of walk's `follow:`. `format` picks the wire encoding — \
             see `query` above.",
            req,
            vec![
                HelpField { name: "from".into(), field_type: "string[]".into(), required: true },
                HelpField { name: "follow".into(), field_type: "edge[]".into(), required: false },
                HelpField { name: "hops".into(), field_type: "uint".into(), required: false },
                HelpField { name: "dim".into(), field_type: "name_only|summary|full?".into(), required: false },
                HelpField { name: "radius".into(), field_type: "float?".into(), required: false },
                HelpField { name: "via".into(), field_type: "edge[]?".into(), required: false },
                HelpField { name: "format".into(), field_type: "json|yaml?".into(), required: false },
            ],
            "walk_result { starting, follow, hops_requested, hops_used, visited{name -> concept}, io_bridges[] } | rendered { format, content }",
        ),
    ];
    let transport = "JSON-lines over Unix socket. One request per line, one \
        response per line. \
        \
        FIRST CONNECT: always send `help` first to discover the protocol \
        surface and delivery model, then `status` to confirm the daemon \
        is ready. New LLM clients should also fetch `manual` once after \
        `help` — it returns the cast vocabulary reference (macros, \
        languages, spec-source rules, warning kinds, workflows) so you \
        can write annotations correctly without grepping cast source. \
        Don't assume any other query works before status reports \
        phase=ready. \
        \
        DELIVERY: every query is request_response except `subscribe`, \
        which is the only stream — after `subscribed`, the daemon keeps \
        writing broadcasts on the same connection until the client \
        closes it. For request_response queries the daemon emits \
        exactly one line and then waits silently. Re-sending the same \
        query later just samples the new state; no event is pushed at \
        you. To learn about state changes (file edits, rebuild \
        completions), use `subscribe` — polling does not stream. \
        \
        RECOMMENDED WORKFLOW (REPL pattern): keep ONE long-lived \
        connection per session. After `help` and `status`, send \
        `subscribe` on that connection. Then send every subsequent \
        request_response query on the SAME connection — broadcasts \
        and responses interleave on it, and the `snapshot_generation` \
        field on each line lets you correlate. The natural edit loop \
        is: edit a file → wait for the `rebuild_completed` broadcast → \
        send a targeted query (typically `unresolved_in_file` or \
        `query`) → read the response. Do NOT open a fresh connection \
        per query; one-shot connections defeat the design, lose the \
        rebuild signal, and force the daemon to re-validate session \
        state on each connect.";
    ResponseBody::Help {
        protocol_version: PROTOCOL_VERSION,
        transport: transport.into(),
        queries,
    }
}

/// The cast vocabulary reference. Static content — does not depend on
/// the snapshot. LLM clients should fetch this once on first connect
/// (after `help` and `status`) to learn how to write annotations
/// correctly without grepping cast source.
///
/// Per-macro field schemas come from each parser module's
/// `pub const FIELDS: &[FieldDoc]` (in `cast::parser::*`), so the
/// manual cannot drift from the parser without a same-file diff in
/// the parser source. Per-macro `purpose` and `anchor_required`
/// strings stay here as documentation about the macro's role rather
/// than its grammar.
pub fn manual_body() -> ResponseBody {
    let macros = vec![
        make_macro(
            "cast::concept",
            "Name a domain concept and point at the code that embodies it. \
             Multiple concept! blocks with the same `name:` merge in the \
             graph (declarations.extend); this is how you split a concept's \
             declaration across files.",
            "yes",
            concept::FIELDS,
        ),
        make_macro(
            "cast::rule",
            "A constraint the code must obey, anchored at the items that \
             enforce or are governed by it.",
            "yes",
            rule::FIELDS,
        ),
        make_macro(
            "cast::anti_pattern",
            "A mistake to avoid, anchored at code that should NOT do it. \
             Acts as a tripwire if someone refactors toward the wrong shape.",
            "yes",
            anti_pattern::FIELDS,
        ),
        make_macro(
            "cast::compare",
            "Two-or-more named items contrasted side-by-side.",
            "per_entry",
            compare::FIELDS,
        ),
        make_macro(
            "cast::pipeline",
            "A multi-stage flow with named steps and a directed `flow:` graph.",
            "per_stage",
            pipeline::FIELDS,
        ),
        make_macro(
            "cast::tier",
            "A layered hierarchy (e.g. capability tiers, lifecycle states).",
            "optional",
            tier::FIELDS,
        ),
        make_macro(
            "cast::matrix",
            "A 2-D classification grid.",
            "optional",
            matrix::FIELDS,
        ),
        make_macro(
            "cast::gut_check",
            "A heuristic / smell test, prose-shaped.",
            "optional",
            gut_check::FIELDS,
        ),
        make_macro(
            "cast::continues_in",
            "Rust → Rust cross-module pointer: `this concept continues in \
             that other module`. Validates that `target` is in declared \
             anchors for the concept; otherwise raises TargetNotInAnchors.",
            "yes",
            continues_in::FIELDS,
        ),
        make_macro(
            "cast::io::continues_in",
            "Cross-language pointer: `this concept continues in a foreign \
             file`. Resolved by file existence + (when lang supports it) \
             foreign-grep or tree-sitter anchor matching.",
            "yes (file path; anchor optional)",
            io_continues_in::FIELDS,
        ),
        make_macro(
            "cast::policy",
            "Repo-level convention declaration. Sits in `Cast.cast` (or \
             any other `.cast` file) at the workspace root and tells the \
             analyzer which spec layouts the project uses. Carries no \
             anchors — it's metadata about the repo, not a concept that \
             points at code. Enforcement (warnings on violations) is \
             scheduled for a follow-up commit; today the parser stores \
             the value and the manual lists it.",
            "none",
            policy::FIELDS,
        ),
    ];

    let languages = vec![
        ManualLanguage {
            value: "c".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["function".into(), "struct".into(), "#define constant".into()],
        },
        ManualLanguage {
            value: "kotlin".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["function_declaration".into(), "class_declaration".into(), "object_declaration".into()],
        },
        ManualLanguage {
            value: "swift".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["function".into(), "protocol".into()],
        },
        ManualLanguage {
            value: "typescript".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["function".into(), "interface".into(), "lexical const (variable_declarator)".into()],
        },
        ManualLanguage {
            value: "javascript".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["function_declaration".into(), "class_declaration".into(), "lexical const (variable_declarator)".into()],
        },
        ManualLanguage {
            value: "vue".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["template-script symbols (function/const)".into()],
        },
        ManualLanguage {
            value: "yaml".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["top-level mapping key (block_mapping_pair)".into()],
        },
        ManualLanguage {
            value: "shell".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["function_definition".into()],
        },
        ManualLanguage {
            value: "markdown".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec![
                "heading path (atx_heading h1..h6 / setext, segments joined \
                 by `::`; anchor text is the inline-rendered string — \
                 `Architecture::Mailbox` matches the H2 `Mailbox` nested \
                 under the H1 `Architecture`, and `# *italic*` is \
                 addressable as `italic`)".into(),
            ],
        },
        ManualLanguage {
            value: "html".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["element id attribute (`<div id=\"foo\">` → anchor `foo`)".into()],
        },
        ManualLanguage {
            value: "css".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec![
                "selector names (class `.foo` → `foo`, id `#foo` → \
                 `foo`, type selector `body` → `body`)".into(),
            ],
        },
        ManualLanguage {
            value: "python".into(),
            backend: "tree_sitter".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["function_definition".into(), "class_definition".into(), "module-level assignment".into()],
        },
        ManualLanguage {
            value: "sql".into(),
            backend: "foreign_grep".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["TABLE/VIEW/FUNCTION declarations (heuristic)".into()],
        },
        ManualLanguage {
            value: "rfc".into(),
            backend: "foreign_grep".into(),
            anchor_supported: true,
            anchorable_kinds: vec!["section headers (heuristic)".into()],
        },
        ManualLanguage {
            value: "external".into(),
            backend: "file_existence".into(),
            anchor_supported: false,
            anchorable_kinds: vec![
                "USE ONLY when pointing at something whose source code we do \
                 not have access to (e.g. an RFC URL, a third-party API \
                 spec, an external service's docs, an upstream binary). For \
                 ANY file in this repo — even Rust files in sibling \
                 workspaces outside the watcher's analysis root — pick a \
                 real language: cross-workspace Rust should be expressed \
                 via `cast::continues_in!` (with that workspace added as a \
                 multi-root) rather than reaching for `external`.".into(),
            ],
        },
    ];

    let spec_sources = vec![
        ManualSpecSource {
            kind: "rust_inline".into(),
            files: "*.rs (cast::*! macros embedded in Rust source)".into(),
            anchor_form: "relative or absolute (`crate::*`, `self::*`, or `<crate>::<...>`)".into(),
            notes: "Discovered via the rust-analyzer HIR walker. Anchors resolve \
                    using `resolve_syn_path` against the calling module. The \
                    declaring concept and its inline edges live in the same .rs \
                    file by convention (but they can also span files — graph \
                    merges by `name`).".into(),
        },
        ManualSpecSource {
            kind: "rs_cast_sidecar".into(),
            files: "<name>.rs.cast files placed next to <name>.rs (e.g. \
                    apps/voluntas-core/src/store/mod.rs.cast next to \
                    apps/voluntas-core/src/store/mod.rs)".into(),
            anchor_form: "relative or absolute — calling module is the sibling \
                          .rs file's module, looked up via the loaded \
                          rust-analyzer state. Same anchor semantics as \
                          inline cast::*! blocks.".into(),
            notes: "Use to give .cast files inline-grade anchor ergonomics at \
                    per-file granularity, without putting cast vocabulary in \
                    the .rs source itself. Discovery walks for *.rs.cast files \
                    where the sibling .rs exists AND is part of one of the \
                    loaded project handles — orphans (sidecar with no sibling, \
                    or sibling outside any analysed crate) are silently \
                    skipped today; a future commit may surface them as \
                    `orphaned_sidecar` warnings. The shape `cast::policy! { \
                    layout: sidecar_only, ... }` opts a repo into using this \
                    notation exclusively, and emits `inline_macro_forbidden` \
                    warnings for any cast::*! block that's still inline.".into(),
        },
        ManualSpecSource {
            kind: "cast_file_root".into(),
            files: "Cast.cast at the repo root (one per repository, conventional \
                    filename — the walker accepts any *.cast file but Cast.cast \
                    is the established home for workspace-level concepts)".into(),
            anchor_form: "absolute only (`<crate>::<module>::<item>`); relative \
                          paths fail with NoCallingModule".into(),
            notes: "Use for workspace-level UMBRELLA concepts that span \
                    multiple subtrees / languages — concepts whose Rust HIR \
                    home is in the analysis root but whose IO edges reach \
                    into Vue, Kotlin, YAML, Swift, etc. The umbrella \
                    declares the concept once and fans out via \
                    `cast::io::continues_in!` to the major components. \
                    Per-subtree detail belongs in spec.cast files (see \
                    cast_file_subtree). For sibling-workspace Rust files, \
                    use multi-root analysis + `cast::continues_in!` rather \
                    than `lang: external` — see the `external` entry in \
                    `languages[]`.".into(),
        },
        ManualSpecSource {
            kind: "cast_file_subtree".into(),
            files: "spec.cast under any subtree (e.g. mobile/android/spec.cast, \
                    infra/voluntas-units/spec.cast, apps/voluntas-home/spec.cast). \
                    Conventional filename — again, the walker accepts any \
                    *.cast file, but spec.cast is the established home for \
                    per-subtree detail".into(),
            anchor_form: "absolute only (`<crate>::<module>::<item>`); relative \
                          paths fail with NoCallingModule".into(),
            notes: "Use for per-subtree SUB-CONCEPTS that catalog the specific \
                    foreign anchors in that subtree — e.g. Kotlin classes in \
                    mobile/android, YAML descriptors in infra/voluntas-units, \
                    Vue components in apps/voluntas-home. Each spec.cast \
                    typically declares one or more sub-concepts (anchored at \
                    the Rust kernel module the subtree consumes) plus many \
                    `cast::io::continues_in!` edges — one per anchorable item \
                    in the foreign files. Tag sub-concepts with the same tag \
                    as the umbrella so `graph_for_tag` walks the full \
                    neighborhood. Sub-concepts can also extend pre-existing \
                    inline-Rust concepts: just emit `cast::io::continues_in! \
                    { concept: \"<existing-name>\", ... }` blocks — they merge \
                    by name in the graph.".into(),
        },
    ];

    let warnings = vec![
        ManualWarningKind {
            kind: "orphan".into(),
            trigger: "concept has total_refs ≤ 1 (declarations.len() + edges.len()). \
                      Either an unanchored declaration or a single floating edge.".into(),
            fix: "Give the concept either more declarations (split summary across \
                  files) OR cast::io::continues_in!/cast::continues_in! edges \
                  to where the concept surfaces in other files/languages.".into(),
        },
        ManualWarningKind {
            kind: "undeclared".into(),
            trigger: "Edges exist for this concept but no `concept!` block declares \
                      it anywhere in the merged graph.".into(),
            fix: "Add a cast::concept! { name: \"<that name>\", ... } somewhere — \
                  in the .rs file the concept is most naturally about, OR in a \
                  Cast.cast / spec.cast file in the relevant subtree.".into(),
        },
        ManualWarningKind {
            kind: "target_not_in_anchors".into(),
            trigger: "A cast::continues_in! Rust target is not listed among the \
                      concept's declared anchors. (io_continues_in is exempt — \
                      foreign targets are not Rust paths.)".into(),
            fix: "Either add the target to the concept's `anchors:` list, or change \
                  the continues_in target to one of the existing anchors.".into(),
        },
        ManualWarningKind {
            kind: "unresolved_parent".into(),
            trigger: "A `cast::concept!` block sets `parent: \"<name>\"` but no \
                      concept with that name exists in the merged graph. Fires per \
                      declaration that sets the bad parent, so the file/line is \
                      precise even when the concept is split across files.".into(),
            fix: "Either fix the typo in `parent:`, or add a `cast::concept!` block \
                  that declares the named parent somewhere in the analyzed roots. \
                  `parent:` is an explicit override of the strict-prefix tree-placement \
                  rule — only use it when two concepts share an anchor exactly and \
                  one is conceptually a specialization of the other.".into(),
        },
        ManualWarningKind {
            kind: "inline_macro_forbidden".into(),
            trigger: "A `cast::*!` block lives in a `.rs` file while the active \
                      `cast::policy!` declaration sets `inline_in_rust: forbid` \
                      (or `warn`, or derives `forbid` from `layout: sidecar_only`). \
                      The cast::policy! block itself is exempt — it's allowed to \
                      live inline.".into(),
            fix: "Move the offending `cast::*!` block into a sibling `<name>.rs.cast` \
                  file (once the Sidecar SpecSource lands; today, into a `.cast` \
                  file at any level — graph merges by `name:`). Or relax the policy \
                  to `inline_in_rust: allow` if inline annotations are intentional \
                  for this repo. The warning is soft — it does not fail the build.".into(),
        },
        ManualWarningKind {
            kind: "missing_stdlib_grounding".into(),
            trigger: "A non-umbrella concept has no `cast::continues_in!` edge whose \
                      target starts with `cast_stdlib::`. Umbrella-shaped concepts \
                      (>=2 embodied anchors at depth <=2 — typically per-crate roots) \
                      are exempt; orphans and undeclared concepts are also skipped \
                      because their primary warning already fires.".into(),
            fix: "Add a `cast::continues_in! { target: cast_stdlib::<area>::<primitive>, \
                  concept: \"<this-concept>\", why: \"...\" }` edge pointing at the \
                  stdlib primitive the concept rests on. By established practice \
                  (see crates/cast-watch/src/state.rs, crates/cast-web/src/lib.rs) \
                  the stdlib target is NOT also added to the concept's `anchors:` \
                  list — that would create a TargetNotInAnchors warning today. \
                  Grounding ties workspace-specific concepts to the shared \
                  cast_stdlib vocabulary so cross-project queries can walk from \
                  a primitive to every concept that realises it.".into(),
        },
    ];

    let workflows = vec![
        ManualWorkflow {
            name: "repl_iteration".into(),
            summary: "How to iterate on annotations against a live watcher".into(),
            steps: vec![
                "Open ONE persistent connection (Python socket, socat, or any \
                 JSON-lines-over-Unix-socket client).".into(),
                "Send `help`, then `status` until phase=ready, then `manual` once \
                 to learn the vocabulary.".into(),
                "Send `subscribe` on the same connection to receive \
                 rebuild_completed broadcasts.".into(),
                "Edit a .rs or .cast file. The watcher autodetects via inotify \
                 and rebuilds incrementally.".into(),
                "On each `rebuild_completed`, send a targeted query \
                 (`unresolved_in_file {path}` or `query from:paths \
                 where:{outcome:unresolved}`) on the SAME connection to see \
                 whether the edit cleared the issue.".into(),
                "Never open a fresh connection per query — request and broadcast \
                 lines interleave on the subscribed socket; the \
                 `snapshot_generation` field on each line is the correlation \
                 key.".into(),
            ],
        },
        ManualWorkflow {
            name: "umbrella_subconcept".into(),
            summary: "Express a workspace-level concept that spans multiple \
                      languages / subtrees, using Cast.cast + per-subtree \
                      spec.cast files".into(),
            steps: vec![
                "Declare the UMBRELLA concept in `Cast.cast` at the repo root. \
                 Anchor it at the kernel-side Rust modules that own the concept. \
                 Add a small set of representative `cast::io::continues_in!` \
                 edges that point at top-level files in each major subtree \
                 (e.g. apps/voluntas-home/src/App.vue, agents/voluntlet/.../main.rs, \
                 mobile/android/.../MainActivity.kt, \
                 infra/voluntas-units/.../bootstrap.target/descriptor.yaml).".into(),
                "For each non-Rust subtree, create a `spec.cast` file at a \
                 sensible root within that subtree (e.g. mobile/android/spec.cast, \
                 infra/voluntas-units/spec.cast, apps/voluntas-home/spec.cast). \
                 Declare a SUB-CONCEPT there, anchored at the kernel module \
                 the subtree consumes, and list multiple \
                 `cast::io::continues_in!` edges — one per anchorable item in \
                 the foreign files (e.g. one per Kotlin class, one per YAML \
                 top-level key, one per TS exported const).".into(),
                "Tag the umbrella AND the sub-concepts with the same `tag:`. \
                 `graph_for_tag {tag:...}` then surfaces the whole \
                 neighborhood, and `walk` from the umbrella name traverses \
                 along continues_in / io_continues_in edges.".into(),
                "To extend a pre-existing inline-Rust concept (e.g. one \
                 declared in apps/voluntas-core/src/store/mod.rs) into a \
                 foreign subtree, you do NOT redeclare it in spec.cast — just \
                 emit `cast::io::continues_in! { concept: \"<existing-name>\", \
                 ... }` blocks in the relevant spec.cast file. The graph \
                 builder merges by `name:`, so the inline declaration and the \
                 cross-file edges fold into one node.".into(),
                "Filename note: Cast.cast and spec.cast are CONVENTIONS — the \
                 walker accepts any *.cast file under the repo root. Use the \
                 conventional names so other clients (and humans) know where \
                 to look.".into(),
            ],
        },
        ManualWorkflow {
            name: "concept_first".into(),
            summary: "Write specs before implementation; let unresolveds drive \
                      the work".into(),
            steps: vec![
                "Write `cast::concept!` / `cast::rule!` blocks BEFORE the code \
                 they describe. The unresolved anchors (`paths_unresolved` in \
                 `status` summary) are the impl TODO list.".into(),
                "After each piece of implementation lands, the corresponding \
                 anchor resolves and the unresolved count drops — visible \
                 immediately in the next `rebuild_completed` broadcast.".into(),
                "When unresolved hits zero for the concept, the spec is fully \
                 backed by code.".into(),
            ],
        },
    ];

    ResponseBody::Manual {
        version: "1.0".into(),
        macros,
        languages,
        spec_sources,
        warnings,
        workflows,
    }
}
