//! Per-macro parsers. One file per macro grammar; this module is the
//! dispatcher.
//!
//! Each macro's parser implements `syn::parse::Parse` and produces a
//! typed value (e.g. `Compare`, `Rule`). The top-level `CastAnnotation`
//! enum unifies them so downstream stages (validator, emit, graph) can
//! treat all annotations uniformly.
//!
//! Dispatch keys are the *defining* macro paths as reported by
//! ra_ap_hir — so `cast::io::continues_in!` (a re-export) arrives here
//! as `cast::continues_in_io`. The dispatcher accepts both spellings to
//! keep call-site ergonomics independent of the underlying name.

cast::concept! {
    name: "fixture_vocabulary",
    summary: "Every example in cast's docstrings, integration tests, \
              and grammar reference draws from a single fictional \
              fixture vocabulary: Rust paths use `sample::*` (or \
              `sample_agent::*` when a cross-crate reference is \
              needed); io anchors point at `samples/external/*`. No \
              real-world proper noun belongs in a cast example. The \
              public surface (docs.rs, the tokeniser smoke test, and \
              every parser docstring) must read coherent to any \
              outside reader — not as in-jokes from whatever parent \
              project incubated cast. New examples copy from existing \
              ones so the namespace stays narrow and recognisable.",
    anchors: [
        crate::parser::compare,
        crate::parser::concept,
        crate::parser::rule,
        crate::parser::anti_pattern,
        crate::parser::pipeline,
        crate::parser::tier,
        crate::parser::gut_check,
        crate::parser::continues_in,
        crate::parser::io_continues_in,
        crate::parser::matrix,
        crate::parser::path_anchor,
    ],
    tags: ["cast_grammar"],
}

cast::rule! {
    rule: "Examples in parser docstrings, integration tests, and the \
           grammar reference must use the `sample::*` / \
           `sample_agent::*` / `samples/external/*` fixture \
           vocabulary — never a real crate name from a sister \
           project, never a real file path outside `samples/`. \
           Adding a new example means copying an existing one's \
           namespace, not inventing a new one.",
    why:  "Cast ships to crates.io as a standalone vocabulary crate; \
           docs.rs is its public surface. Real-world proper nouns \
           leaking from incubation parents read as in-jokes to \
           outside users and quietly couple cast's documentation to \
           projects most readers will never see. A narrow fixture \
           domain keeps future contributors copying from existing \
           examples rather than reinventing illustrative names \
           per-file, and makes drift detectable: any new path that \
           doesn't start with `sample` is a candidate leak.",
    governs: [
        crate::parser::compare,
        crate::parser::concept,
        crate::parser::rule,
        crate::parser::anti_pattern,
        crate::parser::pipeline,
        crate::parser::tier,
        crate::parser::gut_check,
        crate::parser::continues_in,
        crate::parser::io_continues_in,
        crate::parser::matrix,
    ],
    tags: ["cast_grammar"],
    note: "Pinned 2026-04-29 after a full-workspace decoupling pass \
           rewrote 84 references to a previous incubation parent \
           across 17 files. The rule is load-bearing on next \
           contribution, not retroactively — existing examples \
           already match.",
}

cast::concept! {
    name: "macro_body_parsers",
    summary: "syn::parse::Parse implementations for every cast::*! macro \
              body. Each consumes the comma-separated `field: value` \
              entries via the take_* helpers, accumulates Option slots, \
              then folds them into a typed value (Concept, Rule, \
              AntiPattern, Compare, Pipeline, Tier, Matrix, GutCheck, \
              ContinuesIn, IoContinuesIn, Policy, PathAnchor). \
              Deterministic on input — same syn token stream always \
              produces the same value or syn::Error — but not pure: \
              ParseStream advance is an observable side effect. The \
              field set per macro is grammar-defined; unknown fields \
              fail loud per the C3 dispatch contract.",
    anchors: [
        crate::parser::anti_pattern::AntiPattern,
        crate::parser::compare::Compare,
        crate::parser::compare::CompareEntry,
        crate::parser::continues_in::ContinuesIn,
        crate::parser::gut_check::GutCheck,
        crate::parser::io_continues_in::IoContinuesIn,
        crate::parser::matrix::Matrix,
        crate::parser::matrix::MatrixRow,
        crate::parser::path_anchor::PathAnchor,
        crate::parser::pipeline::Pipeline,
        crate::parser::pipeline::FlowEdge,
        crate::parser::policy::Policy,
        crate::parser::rule::Rule,
        crate::parser::tier::Tier,
        crate::parser::tier::TierStage,
    ],
    tags: ["cast_spec_parser", "macro_body_parser"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::deterministic,
    concept: "macro_body_parsers",
    why: "ParseStream advance and the produced typed value are functions \
          of the input token stream alone; no time / RNG / external \
          state participate.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "macro_body_parsers",
    why: "Each parsed value is a struct (or in compare/pipeline/tier/ \
          matrix's case, a struct holding a Vec of entry structs) — \
          all fields simultaneously inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::value_type,
    concept: "macro_body_parsers",
    why: "Cloneable, structurally compared, serialised across the \
          Report wire-format; no resource handles in any field.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "macro_body_parsers",
    why: lazy,
}

pub mod anti_pattern;
pub mod common;
pub mod compare;
pub mod concept;
pub mod continues_in;
pub mod gut_check;
pub mod io_continues_in;
pub mod manual;
pub mod matrix;
pub mod path_anchor;
pub mod pipeline;
pub mod policy;
pub mod rule;
pub mod take;
pub mod tier;

pub use anti_pattern::AntiPattern;
pub use common::CommonFields;
pub use compare::Compare;
pub use concept::Concept;
pub use continues_in::ContinuesIn;
pub use gut_check::GutCheck;
pub use io_continues_in::IoContinuesIn;
pub use matrix::Matrix;
pub use path_anchor::PathAnchor;
pub use pipeline::Pipeline;
pub use policy::Policy;
pub use rule::Rule;
pub use tier::Tier;

use proc_macro2::TokenStream;
use syn::parse2;

cast::concept! {
    name: "parsed_annotation_dispatch",
    summary: "Sum type unifying every parsed cast::*! macro shape — \
              one variant per macro family. Downstream stages branch \
              on this once and treat each variant uniformly.",
    anchors: [
        crate::parser::CastAnnotation,
    ],
    tags: ["cast_spec_parser"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "parsed_annotation_dispatch",
    why: "Eleven-variant enum, exactly one inhabited per parsed \
          invocation.",
}

#[derive(Debug, Clone)]
pub enum CastAnnotation {
    Compare(Compare),
    Pipeline(Pipeline),
    Tier(Tier),
    Matrix(Matrix),
    Rule(Rule),
    AntiPattern(AntiPattern),
    GutCheck(GutCheck),
    ContinuesIn(ContinuesIn),
    IoContinuesIn(IoContinuesIn),
    Concept(Concept),
    Policy(Policy),
}

/// Parse a macro body for the named macro path.
///
/// `macro_path` is the resolved path of the macro invocation, e.g.
/// `cast::compare` or `cast::continues_in_io`. ra_ap_hir resolves
/// aliasing upstream, so the value here is the macro's *defining* name.
pub fn parse_macro_body(macro_path: &str, tokens: TokenStream) -> syn::Result<CastAnnotation> {
    match macro_path {
        "cast::compare" => parse2::<Compare>(tokens).map(CastAnnotation::Compare),
        "cast::pipeline" => parse2::<Pipeline>(tokens).map(CastAnnotation::Pipeline),
        "cast::tier" => parse2::<Tier>(tokens).map(CastAnnotation::Tier),
        "cast::matrix" => parse2::<Matrix>(tokens).map(CastAnnotation::Matrix),
        "cast::rule" => parse2::<Rule>(tokens).map(CastAnnotation::Rule),
        "cast::anti_pattern" => parse2::<AntiPattern>(tokens).map(CastAnnotation::AntiPattern),
        "cast::gut_check" => parse2::<GutCheck>(tokens).map(CastAnnotation::GutCheck),
        "cast::continues_in" => parse2::<ContinuesIn>(tokens).map(CastAnnotation::ContinuesIn),
        // Defining-name and surface-name both route here.
        "cast::continues_in_io" | "cast::io::continues_in" => {
            parse2::<IoContinuesIn>(tokens).map(CastAnnotation::IoContinuesIn)
        }
        "cast::concept" => parse2::<Concept>(tokens).map(CastAnnotation::Concept),
        "cast::policy" => parse2::<Policy>(tokens).map(CastAnnotation::Policy),
        other => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("unknown cast macro path: {other}"),
        )),
    }
}
