//! `cast::io::continues_in!` (defined as `cast::continues_in_io!`) —
//! points at code outside Rust: Kotlin, TypeScript, Vue, an RFC, a
//! vendor doc.
//!
//! ```text
//! cast::io::continues_in! {
//!     target:  "mobile/android/app/src/main/.../SecureKeystoreManager.kt",
//!     lang:    kotlin,
//!     concept: "intent_envelope_consumption",
//!     why:     "Android side stores the envelope and signs it",
//! }
//! ```
//!
//! Validation is graduated by `lang:` (file existence, anchor grep,
//! RFC pattern) — that lives in the validator stage, not here. The
//! parser only enforces the field shape and a closed set of `lang`
//! identifiers.

use super::common::{CommonFields, WhyValue};
use super::manual::FieldDoc;
use super::take::{require, step_separator, take_ident, take_str, take_why};
use syn::{parse::Parse, parse::ParseStream, Ident};

/// Schema-side mirror of the fields accepted by `IoContinuesIn::parse`.
pub const FIELDS: &[FieldDoc] = &[
    FieldDoc {
        name: "target",
        kind: "string (relative path)",
        required: true,
        doc: "Path from repo root. File must exist.",
    },
    FieldDoc {
        name: "lang",
        kind: "ident",
        required: true,
        doc: "Language tag — see `languages[]`. `external` = file existence only.",
    },
    FieldDoc {
        name: "anchor",
        kind: "string",
        required: false,
        doc: "Foreign-language symbol. Required for non-external langs that support anchors; optional for external.",
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
        doc: "Reason for the bridge. Either a string literal or the \
              bare ident `lazy` — a deferred marker meaning the prose \
              justification can be filled in later.",
    },
];

#[derive(Debug, Clone)]
pub struct IoContinuesIn {
    pub target: String,
    pub lang: Lang,
    pub concept: String,
    pub why: Option<WhyValue>,
    pub anchor: Option<String>,
    pub common: CommonFields,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    C,
    Kotlin,
    Swift,
    TypeScript,
    JavaScript,
    Vue,
    Yaml,
    Sql,
    Shell,
    Markdown,
    Html,
    Css,
    Python,
    Rfc,
    External,
}

cast::concept! {
    name: "io_continues_in_lang",
    summary: "Language tag on a cast::io::continues_in! edge — selects \
              the foreign-language backend that resolves the anchor.",
    anchors: [
        crate::parser::io_continues_in::Lang,
    ],
    tags: ["cast_spec_parser"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "io_continues_in_lang",
    why: "Closed enum (C, Kotlin, Swift, TypeScript, JavaScript, Vue, \
          Yaml, Sql, Shell, Markdown, Html, Css, Python, Rfc, \
          External); exactly one inhabited per edge.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "io_continues_in_lang",
    why: lazy,
}

impl Lang {
    fn from_ident(ident: &Ident) -> syn::Result<Self> {
        Ok(match ident.to_string().as_str() {
            "c" => Lang::C,
            "kotlin" => Lang::Kotlin,
            "swift" => Lang::Swift,
            "typescript" => Lang::TypeScript,
            "javascript" => Lang::JavaScript,
            "vue" => Lang::Vue,
            "yaml" => Lang::Yaml,
            "sql" => Lang::Sql,
            "shell" => Lang::Shell,
            "markdown" => Lang::Markdown,
            "html" => Lang::Html,
            "css" => Lang::Css,
            "python" => Lang::Python,
            "rfc" => Lang::Rfc,
            "external" => Lang::External,
            other => {
                return Err(syn::Error::new(
                    ident.span(),
                    format!(
                        "unknown lang `{other}`; expected one of: \
                         c, kotlin, swift, typescript, javascript, vue, \
                         yaml, sql, shell, markdown, html, css, python, \
                         rfc, external"
                    ),
                ));
            }
        })
    }
}

impl Parse for IoContinuesIn {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut target = None;
        let mut lang_ident: Option<Ident> = None;
        let mut concept = None;
        let mut why = None;
        let mut anchor = None;
        let mut common = CommonFields::default();

        while !input.is_empty() {
            let name: Ident = input.parse()?;
            match name.to_string().as_str() {
                "target" => take_str(&name, input, &mut target)?,
                "lang" => take_ident(&name, input, &mut lang_ident)?,
                "concept" => take_str(&name, input, &mut concept)?,
                "why" => take_why(&name, input, &mut why)?,
                "anchor" => take_str(&name, input, &mut anchor)?,
                _ => {
                    if !common.take_field_if_known(&name, input)? {
                        return Err(syn::Error::new(
                            name.span(),
                            format!("unknown field for cast::io::continues_in!: {name}"),
                        ));
                    }
                }
            }
            if !step_separator(input)? {
                break;
            }
        }

        let lang_ident = require(lang_ident, "lang")?;
        let lang = Lang::from_ident(&lang_ident)?;

        Ok(IoContinuesIn {
            target: require(target, "target")?,
            lang,
            concept: require(concept, "concept")?,
            why,
            anchor,
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
    fn parses_io_continues_in() {
        let src = indoc! {r#"
            target:  "mobile/android/.../SecureKeystoreManager.kt",
            lang:    kotlin,
            concept: "intent_envelope_consumption",
            why:     "Android side stores the envelope and signs it"
        "#};
        let parsed: IoContinuesIn = parse_str(src).expect("parse");
        assert_eq!(parsed.lang, Lang::Kotlin);
        assert!(parsed.anchor.is_none());
    }

    #[test]
    fn rejects_unknown_lang() {
        let src = r#"target: "x", lang: pig_latin, concept: "c", why: "w""#;
        let err = parse_str::<IoContinuesIn>(src).unwrap_err();
        assert!(err.to_string().contains("unknown lang `pig_latin`"));
    }

    #[test]
    fn anchor_is_optional() {
        let src = r#"
            target: "rfc-8628", lang: rfc, concept: "device_flow", why: "spec"
        "#;
        let parsed: IoContinuesIn = parse_str(src).expect("parse");
        assert_eq!(parsed.lang, Lang::Rfc);
    }
}
