//! `cast::compare!` — two-or-more named concepts contrasted side-by-side.
//!
//! Grammar (per `crates/cast-spec/GRAMMAR.md`):
//!
//! ```text
//! cast::compare! {
//!     intent_id   @ sample::types::IntentId:
//!         "stable identity",
//!     intent_hash @ sample::types::IntentHash:
//!         "replay-guard key",
//!     tags: ["trust_foundation"],
//! }
//! ```
//!
//! Each entry: `Ident @ Path: LitStr`. Common fields (`tags`, `since`,
//! `note`) may appear in any position.

use super::common::CommonFields;
use super::manual::FieldDoc;
use super::path_anchor::PathAnchor;
use syn::{parse::Parse, parse::ParseStream, Ident, LitStr, Token};

/// Schema-side mirror of the fields accepted by `Compare::parse`.
/// Compare's body is unique: positional `name @ path: "description"`
/// entries interleaved with optional common fields (`tags`, `since`,
/// `note`). The single non-common "field" is therefore `entries`.
pub const FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "entries",
        kind: "list<key @ rust_path: \"description\">",
        required: true,
        doc: "Per-entry anchor: each line is `key @ path: \"...\"`. Two or more entries are required.",
    },
];

#[derive(Debug, Clone)]
pub struct Compare {
    pub entries: Vec<CompareEntry>,
    pub common: CommonFields,
}

#[derive(Debug, Clone)]
pub struct CompareEntry {
    pub anchor: PathAnchor,
    pub description: String,
}

impl Parse for Compare {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut entries = Vec::new();
        let mut common = CommonFields::default();

        while !input.is_empty() {
            // Peek: is the next ident a common field name? If yes,
            // route to CommonFields. Otherwise, parse as an entry.
            let lookahead: Ident = input.fork().parse()?;
            if matches!(lookahead.to_string().as_str(), "tags" | "since" | "note") {
                let name: Ident = input.parse()?;
                common.take_field_if_known(&name, input)?;
            } else {
                let anchor: PathAnchor = input.parse()?;
                input.parse::<Token![:]>()?;
                let desc: LitStr = input.parse()?;
                entries.push(CompareEntry {
                    anchor,
                    description: desc.value(),
                });
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                break;
            }
        }

        if entries.len() < 2 {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "cast::compare! requires at least 2 entries",
            ));
        }

        // Every entry must have an explicit `@ path` anchor — comparisons
        // are between two real things in the codebase.
        for entry in &entries {
            if entry.anchor.path.is_none() {
                return Err(syn::Error::new(
                    entry.anchor.name.span(),
                    format!(
                        "compare! entry `{}` is missing its `@ path` anchor",
                        entry.anchor.name
                    ),
                ));
            }
        }

        Ok(Compare { entries, common })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use syn::parse_str;

    #[test]
    fn parses_minimal_compare() {
        let src = indoc! {r#"
            intent_id   @ sample::types::IntentId:
                "stable identity",
            intent_hash @ sample::types::IntentHash:
                "replay-guard key"
        "#};
        let parsed: Compare = parse_str(src).expect("parse");
        assert_eq!(parsed.entries.len(), 2);
        assert_eq!(parsed.entries[0].description, "stable identity");
        assert!(parsed.entries[0].anchor.path.is_some());
    }

    #[test]
    fn rejects_missing_anchor() {
        // No @ Path — should be rejected per grammar (compare! requires
        // every entry to anchor to code).
        let src = r#"intent_id: "stable identity", intent_hash: "replay-guard key""#;
        let err = parse_str::<Compare>(src).unwrap_err();
        assert!(err.to_string().contains("missing its `@ path` anchor"));
    }

    #[test]
    fn rejects_single_entry() {
        let src = r#"intent_id @ sample::types::IntentId: "lonely""#;
        let err = parse_str::<Compare>(src).unwrap_err();
        assert!(err.to_string().contains("at least 2 entries"));
    }

    #[test]
    fn accepts_common_fields() {
        let src = indoc! {r#"
            intent_id   @ sample::types::IntentId: "a",
            intent_hash @ sample::types::IntentHash: "b",
            tags: ["trust_foundation"],
            note: "added during fi.tt #4 work"
        "#};
        let parsed: Compare = parse_str(src).expect("parse");
        assert_eq!(parsed.common.tags.as_slice(), &[String::from("trust_foundation")][..]);
        assert_eq!(parsed.common.note.as_deref(), Some("added during fi.tt #4 work"));
        assert_eq!(parsed.common.since, None);
    }
}
