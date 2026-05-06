//! `cast::concept!` — names a concept and lists the Rust paths that
//! participate in it.
//!
//! ```text
//! cast::concept! {
//!     name:    "intent_envelope_consumption",
//!     summary: "An intent created server-side, signed by mobile, ...",
//!     anchors: [
//!         sample::api::mobile::sign_request,
//!         sample::reconciler::apply::dispatch,
//!     ],
//!     tags:    ["trust_foundation"],
//! }
//! ```
//!
//! The concept graph builder pairs `continues_in!`/`io::continues_in!`
//! invocations against `concept!` declarations by `name:` string. A
//! declaration is optional — orphan and undeclared concepts surface as
//! warnings, not errors.

use super::common::CommonFields;
use super::manual::FieldDoc;
use super::take::{require, step_separator, take_path_list, take_str};
use syn::{parse::Parse, parse::ParseStream, Ident, Path};

/// Schema-side mirror of the fields accepted by `Concept::parse`.
pub const FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "name",
        kind: "string",
        required: true,
        doc: "Stable identifier; merge key in the graph.",
    },
    FieldDoc {
        name: "summary",
        kind: "string",
        required: true,
        doc: "Human-readable description.",
    },
    FieldDoc {
        name: "anchors",
        kind: "list<rust_path>",
        required: true,
        doc: "Rust HIR-resolvable paths the concept embodies. \
              Resolved by the rust-analyzer-backed validator. \
              Each entry is by default an EMBODIED anchor — the \
              concept instantiates that code. Prefix any entry \
              with `CAST::AS_PRIMITIVE::` (case-insensitive) to \
              declare it as a PRIMITIVE anchor instead — the \
              concept rests on this code as an atom but does not \
              model its insides. Same path can be embodied under \
              one concept and primitive under another. Reports \
              carry the role per anchor.",
    },
    FieldDoc {
        name: "parent",
        kind: "string",
        required: false,
        doc: "Optional explicit parent in the canonical concept tree, \
              by concept name. Overrides the strict-prefix-on-anchors \
              rule for this concept only. Use when two concepts share \
              an anchor exactly (the strict-prefix rule treats them \
              as peers by design, but one is conceptually a \
              specialization of the other). The named concept must \
              exist in the graph; otherwise an UnresolvedParent \
              warning fires. Sibling concepts that legitimately are \
              peers leave this unset.",
    },
];

cast::concept! {
    name: "anchor_role",
    summary: "Per-anchor classification: `Embodied` means \"this concept \
              instantiates / is realised by this code,\" `Primitive` \
              means \"this concept rests on this code as an atom — I \
              refer to it but I do not model its insides.\" The role \
              is per-anchor and per-concept, so the same Rust path can \
              be `Embodied` under one concept (the frame that builds \
              and owns it) and `Primitive` under another (a frame that \
              only references it). Surfaces in queries via the `role` \
              filter and in `dim: full` rendering as a per-anchor tag.",
    anchors: [
        crate::parser::concept::AnchorRole,
        crate::parser::concept::AnchorEntry,
    ],
    tags: ["cast_grammar"],
}

cast::concept! {
    name: "as_primitive_marker",
    summary: "Syntax for marking a single anchor as `Primitive` inside \
              the existing `anchors:` list. The writer prefixes the \
              anchor path with the two-segment marker \
              `CAST::AS_PRIMITIVE::` (case-insensitive on the marker \
              segments), e.g. \
              `anchors: [sample::funnel::Landing, \
              CAST::AS_PRIMITIVE::sample::game::Game]`. The parser \
              strips the marker and stores the remaining path with \
              role `Primitive`; un-marked entries default to \
              `Embodied`. Picked over a parallel `primitive_anchors:` \
              field so the two roles stay co-located in a single \
              ordered list — the order in source IS the order the \
              reader sees in `dim: full` output.",
    anchors: [
        crate::parser::concept::strip_primitive_marker,
        CAST::AS_PRIMITIVE::crate::parser::concept::AnchorRole,
    ],
    tags: ["cast_grammar"],
}

/// Role an anchor plays in the parent concept.
///
/// `Embodied` (the default) means "this concept embodies / instantiates
/// this code." `Primitive` means "this concept rests on this code as
/// an atom — I refer to it but I do not model its insides." The
/// distinction is per-anchor and per-concept: the same Rust path can be
/// `Embodied` under one concept and `Primitive` under another, and
/// `Primitive` is the right read whenever the parent concept's reader
/// shouldn't go deeper than the name.
///
/// Marker syntax inside the `anchors:` list is a leading two-segment
/// path prefix `CAST::AS_PRIMITIVE::` on the anchor path. The parser
/// strips the marker and stores the remaining path with `Primitive`
/// role; un-marked entries default to `Embodied`. See
/// `strip_primitive_marker`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnchorRole {
    Embodied,
    Primitive,
}

#[derive(Debug, Clone)]
pub struct AnchorEntry {
    pub path: Path,
    pub role: AnchorRole,
}

#[derive(Debug, Clone)]
pub struct Concept {
    pub name: String,
    pub summary: String,
    pub anchors: Vec<AnchorEntry>,
    /// Optional explicit parent in the canonical concept tree. When
    /// present, the placement engine uses this name as a hard parent
    /// override instead of computing a parent from the strict-prefix
    /// rule on anchors. The named concept must exist in the graph;
    /// the validator emits a warning otherwise. Used when the
    /// strict-prefix rule would land the concept at the wrong
    /// position — typically when a concept's anchors are equal to
    /// (not strict-prefixes of) a sibling's, which the placement
    /// rule treats as peers by design.
    pub parent: Option<String>,
    pub common: CommonFields,
}

impl Parse for Concept {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut summary = None;
        let mut anchors = None;
        let mut parent = None;
        let mut common = CommonFields::default();

        while !input.is_empty() {
            let field: Ident = input.parse()?;
            match field.to_string().as_str() {
                "name" => take_str(&field, input, &mut name)?,
                "summary" => take_str(&field, input, &mut summary)?,
                "anchors" => take_path_list(&field, input, &mut anchors)?,
                "parent" => take_str(&field, input, &mut parent)?,
                _ => {
                    if !common.take_field_if_known(&field, input)? {
                        return Err(syn::Error::new(
                            field.span(),
                            format!("unknown field for cast::concept!: {field}"),
                        ));
                    }
                }
            }
            if !step_separator(input)? {
                break;
            }
        }

        let name = require(name, "name")?;
        let summary = require(summary, "summary")?;
        // `anchors` is optional and may be empty. Anchor count says
        // nothing about whether the concept is primitive — that role
        // is per-anchor (see `AnchorRole`) and is set with the
        // `CAST::AS_PRIMITIVE::` marker in the anchors list. A concept
        // with 0 anchors is just declaration-only: cross-cutting roles
        // like `encrypted_connection` or surfaces that live entirely
        // in non-Rust files (anchored via `io::continues_in!`).
        let anchors: Vec<AnchorEntry> = anchors
            .unwrap_or_default()
            .into_iter()
            .map(strip_primitive_marker)
            .collect();

        Ok(Concept {
            name,
            summary,
            anchors,
            parent,
            common,
        })
    }
}

/// Detect a leading `CAST::AS_PRIMITIVE::` marker on an anchor path,
/// strip it, and return an `AnchorEntry` with the appropriate role.
/// Un-marked paths get `AnchorRole::Embodied`.
///
/// The marker is a two-segment prefix matched case-insensitively on
/// each segment so the writer can pick whichever case reads best
/// (`CAST::AS_PRIMITIVE::sample::game::Game` and
/// `cast::as_primitive::sample::game::Game` both work). The remainder
/// must still be a valid Rust path — the resolver sees only the
/// stripped path.
fn strip_primitive_marker(path: Path) -> AnchorEntry {
    let segs = &path.segments;
    if segs.len() >= 3
        && segs[0].ident.to_string().eq_ignore_ascii_case("CAST")
        && segs[1].ident.to_string().eq_ignore_ascii_case("AS_PRIMITIVE")
    {
        let remainder: syn::punctuated::Punctuated<syn::PathSegment, syn::Token![::]> =
            segs.iter().skip(2).cloned().collect();
        AnchorEntry {
            path: Path {
                leading_colon: None,
                segments: remainder,
            },
            role: AnchorRole::Primitive,
        }
    } else {
        AnchorEntry {
            path,
            role: AnchorRole::Embodied,
        }
    }
}

cast::concept! {
    name: "concept_macro_parser",
    summary: "syn::parse::Parse implementation for the body of \
              cast::concept! { ... }. Walks the comma-separated `field: \
              value` entries via the take_* helpers, accumulates them \
              into Option slots, then folds them into a Concept value \
              with `require` finalising required fields. Deterministic — \
              same syn input always yields the same Concept or the same \
              syn::Error — but not pure: it advances the ParseStream \
              cursor and is called for its side effect of consuming \
              tokens.",
    anchors: [
        crate::parser::concept::Concept,
    ],
    tags: ["cast_spec_parser", "macro_body_parser"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::deterministic,
    concept: "concept_macro_parser",
    why: "ParseStream advance is deterministic from the input; outcome \
          is a function of the token stream and nothing else.",
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "as_primitive_marker",
    why: "strip_primitive_marker is pure: takes a syn::Path, returns an \
          AnchorEntry; no mutation of the input, no I/O, no time/RNG \
          dependency.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "anchor_role",
    why: "AnchorRole is a two-variant enum (Embodied, Primitive); \
          exactly one variant is inhabited per anchor.",
}

cast::concept! {
    name: "parsed_annotation_value",
    summary: "Concept value produced by the parser — product_type with \
              required name + summary + anchors and optional parent.",
    anchors: [
        crate::parser::concept::Concept,
        crate::parser::concept::AnchorEntry,
    ],
    tags: ["cast_spec_parser", "parsed_annotation"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "parsed_annotation_value",
    why: "Both Concept and AnchorEntry are structs whose fields are \
          all simultaneously inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::value_type,
    concept: "parsed_annotation_value",
    why: "Cloneable, structurally compared, no resource handle or \
          mutable interior; serialised across the Report wire-format.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "anchor_role",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::typed_handle,
    concept: "parsed_annotation_value",
    why: lazy,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use syn::parse_str;

    #[test]
    fn parses_full_concept() {
        let src = indoc! {r#"
            name:    "intent_envelope_consumption",
            summary: "An intent created server-side, signed by mobile, consumed by the reconciler",
            anchors: [
                sample::api::mobile::sign_request,
                sample::reconciler::apply::dispatch,
                sample::types::IntentHash,
            ],
            tags: ["trust_foundation"]
        "#};
        let parsed: Concept = parse_str(src).expect("parse");
        assert_eq!(parsed.name, "intent_envelope_consumption");
        assert_eq!(parsed.anchors.len(), 3);
        assert_eq!(parsed.common.tags.as_slice(), &[String::from("trust_foundation")][..]);
    }

    #[test]
    fn accepts_missing_anchors() {
        // Primitive shape: a concept with no anchors at the
        // declaration site. Cross-cutting roles like
        // `encrypted_connection` use this form.
        let src = r#"name: "encrypted_connection", summary: "A TLS-secured channel.""#;
        let parsed = parse_str::<Concept>(src).expect("parse");
        assert_eq!(parsed.name, "encrypted_connection");
        assert!(parsed.anchors.is_empty());
    }

    #[test]
    fn accepts_empty_anchors() {
        let src = r#"name: "x", summary: "y", anchors: []"#;
        let parsed = parse_str::<Concept>(src).expect("parse");
        assert!(parsed.anchors.is_empty());
    }

    #[test]
    fn parses_marker_primitive_anchor() {
        // `CAST::AS_PRIMITIVE::<path>` strips to `<path>` with role
        // Primitive; un-marked siblings stay Embodied.
        let src = indoc! {r#"
            name:    "sales_funnel",
            summary: "Mobile-game user acquisition funnel.",
            anchors: [
                sample::funnel::Landing,
                sample::funnel::Checkout,
                CAST::AS_PRIMITIVE::sample::game::Game,
            ]
        "#};
        let parsed = parse_str::<Concept>(src).expect("parse");
        assert_eq!(parsed.anchors.len(), 3);
        assert_eq!(parsed.anchors[0].role, AnchorRole::Embodied);
        assert_eq!(parsed.anchors[1].role, AnchorRole::Embodied);
        assert_eq!(parsed.anchors[2].role, AnchorRole::Primitive);
        // Marker is stripped from the stored path.
        let last = &parsed.anchors[2].path;
        assert_eq!(last.segments.len(), 3);
        assert_eq!(last.segments[0].ident.to_string(), "sample");
        assert_eq!(last.segments[1].ident.to_string(), "game");
        assert_eq!(last.segments[2].ident.to_string(), "Game");
    }

    #[test]
    fn marker_is_case_insensitive() {
        let src = indoc! {r#"
            name:    "x",
            summary: "y",
            anchors: [cast::as_primitive::sample::game::Game]
        "#};
        let parsed = parse_str::<Concept>(src).expect("parse");
        assert_eq!(parsed.anchors[0].role, AnchorRole::Primitive);
    }
}
