//! `cast::continues_in!` — points at another Rust path in the same
//! workspace, naming the concept that continues there.
//!
//! ```text
//! cast::continues_in! {
//!     target:  sample::reconciler::apply::dispatch,
//!     concept: "intent_envelope_consumption",
//!     why:     "the envelope created here is consumed by the reconciler",
//! }
//! ```
//!
//! The `concept:` string is the key the cross-file graph builds against:
//! two `continues_in!` invocations sharing a concept form an edge.

use super::common::{CommonFields, WhyValue};
use super::manual::FieldDoc;
use super::take::{require, step_separator, take_path, take_str, take_why};
use syn::{parse::Parse, parse::ParseStream, Ident, Path};

/// Schema-side mirror of the fields accepted by `ContinuesIn::parse`.
pub const FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "target",
        kind: "rust_path",
        required: true,
        doc: "The module/item the concept extends to.",
    },
    FieldDoc {
        name: "concept",
        kind: "string",
        required: true,
        doc: "Name of the concept this edge belongs to.",
    },
    FieldDoc {
        name: "why",
        kind: "string | `lazy`",
        required: false,
        doc: "Reason for the continuation. Either a string literal or \
              the bare ident `lazy`. `lazy` is a deferred marker — \
              meaning the structural connection was the load-bearing \
              information and the prose justification can be filled \
              in later (LLM background pass, on-demand at query \
              time, or never).",
    },
];

#[derive(Debug, Clone)]
pub struct ContinuesIn {
    pub target: Path,
    pub concept: String,
    pub why: Option<WhyValue>,
    pub common: CommonFields,
}

impl Parse for ContinuesIn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut target = None;
        let mut concept = None;
        let mut why = None;
        let mut common = CommonFields::default();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            match name.to_string().as_str() {
                "target" => take_path(&name, input, &mut target)?,
                "concept" => take_str(&name, input, &mut concept)?,
                "why" => take_why(&name, input, &mut why)?,
                _ => {
                    if !common.take_field_if_known(&name, input)? {
                        return Err(syn::Error::new(
                            name.span(),
                            format!("unknown field for cast::continues_in!: {name}"),
                        ));
                    }
                }
            }
            if !step_separator(input)? {
                break;
            }
        }

        Ok(ContinuesIn {
            target: require(target, "target")?,
            concept: require(concept, "concept")?,
            why,
            common,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use syn::parse_str;

    #[test]
    fn parses_continues_in() {
        let src = indoc! {r#"
            target:  sample::reconciler::apply::dispatch,
            concept: "intent_envelope_consumption",
            why:     "the envelope created here is consumed by the reconciler"
        "#};
        let parsed: ContinuesIn = parse_str(src).expect("parse");
        assert_eq!(parsed.concept, "intent_envelope_consumption");
    }

    #[test]
    fn parses_without_why() {
        let src = indoc! {r#"
            target:  sample::reconciler::apply::dispatch,
            concept: "intent_envelope_consumption"
        "#};
        let parsed: ContinuesIn = parse_str(src).expect("parse");
        assert!(parsed.why.is_none());
    }

    #[test]
    fn parses_lazy_why() {
        let src = indoc! {r#"
            target:  sample::reconciler::apply::dispatch,
            concept: "intent_envelope_consumption",
            why:     lazy
        "#};
        let parsed: ContinuesIn = parse_str(src).expect("parse");
        assert!(matches!(parsed.why, Some(WhyValue::Lazy)));
    }

    #[test]
    fn parses_prose_why() {
        let src = indoc! {r#"
            target:  sample::reconciler::apply::dispatch,
            concept: "intent_envelope_consumption",
            why:     "the envelope is consumed"
        "#};
        let parsed: ContinuesIn = parse_str(src).expect("parse");
        match parsed.why {
            Some(WhyValue::Prose { text }) => {
                assert_eq!(text, "the envelope is consumed")
            }
            other => panic!("expected Prose, got {other:?}"),
        }
    }

    #[test]
    fn rejects_unknown_why_ident() {
        let src = r#"target: sample::x, concept: "c", why: maybe"#;
        let err = parse_str::<ContinuesIn>(src).unwrap_err();
        assert!(err.to_string().contains("expected string literal or `lazy`"));
    }

    #[test]
    fn rejects_missing_target() {
        let src = r#"concept: "x", why: "y""#;
        let err = parse_str::<ContinuesIn>(src).unwrap_err();
        assert!(err.to_string().contains("missing required field: target"));
    }
}
