//! `cast::gut_check!` — a single binary question with optional code
//! anchors for each branch.
//!
//! ```text
//! cast::gut_check! {
//!     question: "Can the code be regenerated later?",
//!     yes:      "it is a key (TOTP/HOTP)",
//!     yes_at:   sample::auth::vault::derived,
//!     no:       "it is a stored artifact",
//!     no_at:    sample::auth::vault::stored,
//! }
//! ```
//!
//! The grammar enforces only the question / yes / no triple. The two
//! `*_at:` paths are optional — the value of a gut-check is the same
//! whether or not both branches land in code.

use super::common::CommonFields;
use super::manual::FieldDoc;
use super::take::{require, step_separator, take_path, take_str};
use syn::{parse::Parse, parse::ParseStream, Ident, Path};

/// Schema-side mirror of the fields accepted by `GutCheck::parse`.
pub const FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "question",
        kind: "string",
        required: true,
        doc: "The binary question itself.",
    },
    FieldDoc {
        name: "yes",
        kind: "string",
        required: true,
        doc: "Description of what's true / what to do when the answer is yes.",
    },
    FieldDoc {
        name: "yes_at",
        kind: "rust_path",
        required: false,
        doc: "Optional code anchor for the yes branch.",
    },
    FieldDoc {
        name: "no",
        kind: "string",
        required: true,
        doc: "Description of what's true / what to do when the answer is no.",
    },
    FieldDoc {
        name: "no_at",
        kind: "rust_path",
        required: false,
        doc: "Optional code anchor for the no branch.",
    },
];

#[derive(Debug, Clone)]
pub struct GutCheck {
    pub question: String,
    pub yes: String,
    pub yes_at: Option<Path>,
    pub no: String,
    pub no_at: Option<Path>,
    pub common: CommonFields,
}

impl Parse for GutCheck {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut question = None;
        let mut yes = None;
        let mut yes_at = None;
        let mut no = None;
        let mut no_at = None;
        let mut common = CommonFields::default();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            match name.to_string().as_str() {
                "question" => take_str(&name, input, &mut question)?,
                "yes" => take_str(&name, input, &mut yes)?,
                "yes_at" => take_path(&name, input, &mut yes_at)?,
                "no" => take_str(&name, input, &mut no)?,
                "no_at" => take_path(&name, input, &mut no_at)?,
                _ => {
                    if !common.take_field_if_known(&name, input)? {
                        return Err(syn::Error::new(
                            name.span(),
                            format!("unknown field for cast::gut_check!: {name}"),
                        ));
                    }
                }
            }
            if !step_separator(input)? {
                break;
            }
        }

        Ok(GutCheck {
            question: require(question, "question")?,
            yes: require(yes, "yes")?,
            yes_at,
            no: require(no, "no")?,
            no_at,
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
    fn parses_full_gut_check() {
        let src = indoc! {r#"
            question: "Can the code be regenerated later?",
            yes:      "it is a key (TOTP/HOTP)",
            yes_at:   sample::auth::vault::derived,
            no:       "it is a stored artifact",
            no_at:    sample::auth::vault::stored
        "#};
        let parsed: GutCheck = parse_str(src).expect("parse");
        assert!(parsed.yes_at.is_some());
        assert!(parsed.no_at.is_some());
    }

    #[test]
    fn at_paths_are_optional() {
        let src = r#"question: "q", yes: "y", no: "n""#;
        let parsed: GutCheck = parse_str(src).expect("parse");
        assert!(parsed.yes_at.is_none());
        assert!(parsed.no_at.is_none());
    }

    #[test]
    fn rejects_missing_no() {
        let src = r#"question: "q", yes: "y""#;
        let err = parse_str::<GutCheck>(src).unwrap_err();
        assert!(err.to_string().contains("missing required field: no"));
    }
}
