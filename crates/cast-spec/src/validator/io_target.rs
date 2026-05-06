//! Cross-language anchor validation for `cast::io::continues_in!`.
//!
//! Per `GRAMMAR.md`, the validation rule is graduated by `lang:`:
//!
//! ```text
//! kotlin|swift|typescript|vue|yaml|sql|shell
//!     → file must exist; if anchor: present, language-aware grep
//! rfc      → target must match `RFC <num>(#section-...)?`
//! external → file or URL existence only; URL fetch is off by default
//! ```
//!
//! Anchor matching uses a small per-language list of declaration
//! prefixes (e.g. `fun X` for Kotlin, `function X` for TS) plus a
//! trailing word-boundary check. This is conservative on purpose:
//! false negatives (legit anchor not found) prompt a small grammar
//! refinement, while false positives (substring match in a comment)
//! defeat the whole point of the validation.

use crate::parser::io_continues_in::{IoContinuesIn, Lang};
use std::path::{Path, PathBuf};

// ─── lang: external is intentionally a stub today ────────────────────
//
// `validate_external_target` checks file existence on disk and stops
// there. No anchor resolution, no symbol lookup. That is correct for
// today's single-handle architecture: the target Rust file lives in
// another Cargo workspace whose `crate::` root is not loaded into
// our analysis DB, so any anchor inside it cannot be resolved.
//
// The right way to close this gap is NOT to make `lang: external`
// guess at the target's symbols. It is to load multiple project
// handles, build a path→handle index, and dispatch the anchor lookup
// to whichever handle owns the target file. Then `lang: external`
// upgrades from "file exists?" to "symbol exists in the right
// workspace's DB?" — without changing the macro grammar.

cast::anti_pattern! {
    avoid:      "Treat cross-workspace io anchors as forever \
                 file-existence-only by special-casing `lang: external` \
                 in the validator.",
    why:        "File-existence is a useful smoke test but it does \
                 not catch renames, deletions, or signature changes — \
                 the very things an architectural annotation is \
                 supposed to catch. A multi-handle resolver gives \
                 `lang: external` the same fidelity as workspace- \
                 local anchors. The cost is bookkeeping (N handles, \
                 one path index); the cost of NOT doing it is silent \
                 drift across workspace boundaries.",
    instead:    "Hold N ProjectHandles, build a path→handle index \
                 from their VFSes, dispatch anchor resolution to \
                 whichever handle owns the target file. \
                 `lang: external` becomes a fallback for files in \
                 workspaces that were not loaded, not the default.",
    instead_at: crate::validator::io_target::validate_io_annotation,
    governs: [
        crate::validator::io_target::validate_io_annotation,
        crate::validator::io_target::validate_external_target,
    ],
    tags: ["cast_extract_pipeline"],
}

#[derive(Debug, Clone)]
pub struct IoDiagnostic {
    pub target: String,
    pub lang: Lang,
    pub anchor: Option<String>,
    pub outcome: IoOutcome,
}

#[derive(Debug, Clone)]
pub enum IoOutcome {
    Ok,
    /// Local file (kotlin/swift/.../external) didn't exist on disk.
    FileMissing { resolved_path: PathBuf },
    /// `lang: rfc` target didn't match the documented shape.
    RfcSyntaxInvalid,
    /// File existed, anchor was set, language-aware grep failed.
    AnchorMissing { resolved_path: PathBuf },
}

impl IoDiagnostic {
    pub fn is_error(&self) -> bool {
        !matches!(self.outcome, IoOutcome::Ok)
    }
}

impl std::fmt::Display for IoOutcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoOutcome::Ok => write!(f, "ok"),
            IoOutcome::FileMissing { resolved_path } => {
                write!(f, "file does not exist: {}", resolved_path.display())
            }
            IoOutcome::RfcSyntaxInvalid => write!(
                f,
                "rfc target must match `RFC <num>` or `RFC <num>#section-<id>`"
            ),
            IoOutcome::AnchorMissing { resolved_path } => {
                write!(f, "anchor not found in {}", resolved_path.display())
            }
        }
    }
}

/// Validate the target (and optional anchor) of a single
/// `cast::io::continues_in!` annotation.
pub fn validate_io_annotation(repo_root: &Path, ann: &IoContinuesIn) -> IoDiagnostic {
    let outcome = match ann.lang {
        Lang::Rfc => validate_rfc_target(&ann.target),
        Lang::External => validate_external_target(repo_root, &ann.target),
        _ => validate_source_target(repo_root, &ann.target, ann.lang, ann.anchor.as_deref()),
    };
    IoDiagnostic {
        target: ann.target.clone(),
        lang: ann.lang,
        anchor: ann.anchor.clone(),
        outcome,
    }
}

fn validate_rfc_target(target: &str) -> IoOutcome {
    let rest = match target.strip_prefix("RFC ") {
        Some(r) => r,
        None => return IoOutcome::RfcSyntaxInvalid,
    };
    let (num, frag) = match rest.split_once('#') {
        Some((n, f)) => (n, Some(f)),
        None => (rest, None),
    };
    if num.is_empty() || !num.chars().all(|c| c.is_ascii_digit()) {
        return IoOutcome::RfcSyntaxInvalid;
    }
    if let Some(frag) = frag {
        let section = match frag.strip_prefix("section-") {
            Some(s) => s,
            None => return IoOutcome::RfcSyntaxInvalid,
        };
        let ok_section_char = |c: char| c.is_ascii_alphanumeric() || c == '.' || c == '-';
        if section.is_empty() || !section.chars().all(ok_section_char) {
            return IoOutcome::RfcSyntaxInvalid;
        }
    }
    IoOutcome::Ok
}

fn validate_external_target(repo_root: &Path, target: &str) -> IoOutcome {
    if target.starts_with("http://") || target.starts_with("https://") {
        // URL fetch is optional, off by default. Accept syntactically.
        return IoOutcome::Ok;
    }
    let path = repo_root.join(target);
    if path.exists() {
        IoOutcome::Ok
    } else {
        IoOutcome::FileMissing { resolved_path: path }
    }
}

fn validate_source_target(
    repo_root: &Path,
    target: &str,
    lang: Lang,
    anchor: Option<&str>,
) -> IoOutcome {
    let path = repo_root.join(target);
    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => return IoOutcome::FileMissing { resolved_path: path },
    };
    if let Some(anchor) = anchor {
        if !anchor_found(&contents, lang, anchor) {
            return IoOutcome::AnchorMissing { resolved_path: path };
        }
    }
    IoOutcome::Ok
}

/// Per-language anchor search. Tree-sitter first when a grammar is
/// registered for the language; grep-with-declaration-prefix is the
/// fallback for variants without a grammar — per the
/// `tree-sitter is the floor, never the ceiling` rule pinned in
/// `language_backend::tree_sitter`.
fn anchor_found(contents: &str, lang: Lang, anchor: &str) -> bool {
    let backend = crate::language_backend::tree_sitter::shared();
    if let Some(found) = backend.find_anchor_in_source(contents.as_bytes(), lang, anchor) {
        return found;
    }
    match lang {
        Lang::C => c_anchor_found(contents, anchor),
        Lang::Kotlin => match_with_prefixes(
            contents,
            anchor,
            &["class ", "object ", "interface ", "fun ", "val ", "var "],
        ),
        Lang::Swift => match_with_prefixes(
            contents,
            anchor,
            &[
                "class ",
                "func ",
                "struct ",
                "enum ",
                "protocol ",
                "extension ",
                "let ",
                "var ",
            ],
        ),
        Lang::TypeScript => match_with_prefixes(
            contents,
            anchor,
            &[
                "function ",
                "class ",
                "interface ",
                "type ",
                "enum ",
                "const ",
                "let ",
                "var ",
            ],
        ),
        Lang::JavaScript => match_with_prefixes(
            contents,
            anchor,
            &["function ", "class ", "const ", "let ", "var "],
        ),
        Lang::Python => match_with_prefixes(
            contents,
            anchor,
            &["def ", "async def ", "class "],
        ),
        Lang::Sql => match_with_prefixes(
            contents,
            anchor,
            &["TABLE ", "VIEW ", "FUNCTION ", "PROCEDURE ", "INDEX ", "TYPE "],
        ),
        Lang::Yaml => yaml_key_present(contents, anchor),
        Lang::Shell => shell_function_present(contents, anchor),
        // Vue files mix template, script, and style with no single
        // declaration form. Fall back to word-boundary substring; this
        // catches `ref="x"`, `function x`, `<x>` tags, etc.
        Lang::Vue => word_match(contents, anchor),
        // Markdown anchors only resolve via the tree-sitter backend
        // (heading-path walk, not a flat-identifier match). If the
        // grammar isn't registered for some reason, refuse to match
        // — falling back to substring grep would silently re-introduce
        // the kind of cross-section aliasing the path walk exists to
        // prevent.
        Lang::Markdown => false,
        // HTML / CSS resolve only via the tree-sitter backend — id
        // attributes for HTML, class/id/type selectors for CSS.
        // The substring grep fallback would over-match (e.g. an
        // attribute *value* in HTML or a comment in CSS) so refuse
        // when the grammar is missing.
        Lang::Html | Lang::Css => false,
        Lang::Rfc | Lang::External => true, // unreachable — caller dispatches
    }
}

fn match_with_prefixes(contents: &str, anchor: &str, prefixes: &[&str]) -> bool {
    prefixes.iter().any(|prefix| {
        let needle = format!("{prefix}{anchor}");
        contents
            .match_indices(&needle)
            .any(|(idx, _)| trailing_boundary(contents, idx + needle.len()))
    })
}

fn yaml_key_present(contents: &str, anchor: &str) -> bool {
    // Match `<anchor>:` at any indentation. Trailing colon disambiguates
    // a key from an inline value mention.
    contents.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed
            .strip_prefix(anchor)
            .is_some_and(|rest| rest.starts_with(':'))
    })
}

/// C anchor matching. C has no single declaration keyword like
/// `fun` or `function` — function definitions are
/// `<return_type> <name>(...)`. We combine:
///
/// - Prefix matching for keyword-led declarations: `struct X`,
///   `enum X`, `union X`, `#define X`, `typedef ... X` (the
///   typedef name is at the *end*, but a leading `typedef` plus a
///   later `X` on the same logical declaration is the closest we
///   get without parsing).
/// - A function-definition heuristic: a line containing
///   `<anchor>(` where `<anchor>` is preceded by whitespace (so it
///   isn't part of a longer identifier) and followed by `(` (so
///   it isn't a stray reference). Conservative — won't match
///   forward declarations split across lines, but catches the
///   `int main(void) { … }` shape.
fn c_anchor_found(contents: &str, anchor: &str) -> bool {
    if match_with_prefixes(
        contents,
        anchor,
        &["struct ", "enum ", "union ", "#define "],
    ) {
        return true;
    }
    let needle_open = format!("{anchor}(");
    contents.lines().any(|line| {
        let mut start = 0;
        while let Some(off) = line[start..].find(&needle_open) {
            let idx = start + off;
            let leading_ok = idx == 0
                || matches!(
                    line.as_bytes()[idx - 1],
                    b' ' | b'\t' | b'*' | b'(' | b','
                ) && (idx == 0 || !is_ident_byte(line.as_bytes()[idx - 1]));
            // Reject `<anchor>(` when preceded by an ident byte —
            // catches `not_main(` matching anchor `main`.
            let preceded_by_ident =
                idx > 0 && is_ident_byte(line.as_bytes()[idx - 1]);
            if leading_ok && !preceded_by_ident {
                return true;
            }
            start = idx + needle_open.len();
        }
        false
    })
}

fn shell_function_present(contents: &str, anchor: &str) -> bool {
    contents.lines().any(|line| {
        let trimmed = line.trim_start();
        // POSIX form: `name() { ...`
        if let Some(rest) = trimmed.strip_prefix(anchor) {
            if rest.starts_with("()") || rest.trim_start().starts_with("()") {
                return true;
            }
        }
        // bash form: `function name`
        if let Some(rest) = trimmed.strip_prefix("function ") {
            if let Some(after) = rest.strip_prefix(anchor) {
                return after.is_empty()
                    || after.starts_with(' ')
                    || after.starts_with('(')
                    || after.starts_with('{');
            }
        }
        false
    })
}

fn word_match(contents: &str, anchor: &str) -> bool {
    contents.match_indices(anchor).any(|(idx, _)| {
        let leading_ok = idx == 0
            || !is_ident_byte(contents.as_bytes()[idx - 1]);
        leading_ok && trailing_boundary(contents, idx + anchor.len())
    })
}

fn trailing_boundary(contents: &str, pos: usize) -> bool {
    contents
        .as_bytes()
        .get(pos)
        .copied()
        .map(|b| !is_ident_byte(b))
        .unwrap_or(true)
}

fn is_ident_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc_canonical_form_passes() {
        assert!(matches!(validate_rfc_target("RFC 8628"), IoOutcome::Ok));
    }

    #[test]
    fn rfc_with_section_passes() {
        assert!(matches!(
            validate_rfc_target("RFC 8628#section-3.4"),
            IoOutcome::Ok
        ));
    }

    #[test]
    fn rfc_lowercase_rejected() {
        assert!(matches!(
            validate_rfc_target("rfc-8628"),
            IoOutcome::RfcSyntaxInvalid
        ));
    }

    #[test]
    fn rfc_missing_number_rejected() {
        assert!(matches!(
            validate_rfc_target("RFC "),
            IoOutcome::RfcSyntaxInvalid
        ));
    }

    #[test]
    fn rfc_garbage_section_rejected() {
        assert!(matches!(
            validate_rfc_target("RFC 8628#intro"),
            IoOutcome::RfcSyntaxInvalid
        ));
    }

    #[test]
    fn external_url_accepted_without_fetch() {
        let out = validate_external_target(
            Path::new("/nonexistent"),
            "https://example.com/spec.html",
        );
        assert!(matches!(out, IoOutcome::Ok));
    }

    #[test]
    fn kotlin_anchor_matches_fun_decl() {
        let src = "package x\n\nfun signEnvelope() { TODO() }\n";
        assert!(anchor_found(src, Lang::Kotlin, "signEnvelope"));
    }

    #[test]
    fn kotlin_anchor_rejects_substring_in_comment() {
        // Bare mention in a comment must NOT count — the prefix gate is
        // why the validation is worth running.
        let src = "// signEnvelope is documented elsewhere\n";
        assert!(!anchor_found(src, Lang::Kotlin, "signEnvelope"));
    }

    #[test]
    fn c_anchor_matches_function_definition() {
        let src = "#include <stdio.h>\n\nint main(void) {\n    return 0;\n}\n";
        assert!(anchor_found(src, Lang::C, "main"));
    }

    #[test]
    fn c_anchor_matches_struct() {
        let src = "struct point { int x; int y; };\n";
        assert!(anchor_found(src, Lang::C, "point"));
    }

    #[test]
    fn c_anchor_matches_define() {
        let src = "#define MAX_NODES 16\n";
        assert!(anchor_found(src, Lang::C, "MAX_NODES"));
    }

    #[test]
    fn c_anchor_matches_pointer_return() {
        // Function definition with pointer return type — the `*` sits
        // adjacent to the anchor: `char *make_buffer(...)` should still
        // match anchor `make_buffer`.
        let src = "char *make_buffer(size_t n) {\n    return NULL;\n}\n";
        assert!(anchor_found(src, Lang::C, "make_buffer"));
    }

    #[test]
    fn c_anchor_word_boundary_rejects_longer_name() {
        // `not_main(` must not match anchor `main`.
        let src = "int not_main(void) { return 0; }\n";
        assert!(!anchor_found(src, Lang::C, "main"));
    }

    #[test]
    fn c_anchor_rejects_substring_in_comment() {
        // Bare mention in a comment without `(` must not count.
        let src = "// main is documented elsewhere\n";
        assert!(!anchor_found(src, Lang::C, "main"));
    }

    #[test]
    fn typescript_const_anchor_matches() {
        let src = "export const apiClient = createClient();\n";
        assert!(anchor_found(src, Lang::TypeScript, "apiClient"));
    }

    #[test]
    fn typescript_anchor_word_boundary() {
        // `apiClientFactory` must not match anchor `apiClient`.
        let src = "export const apiClientFactory = () => {};\n";
        assert!(!anchor_found(src, Lang::TypeScript, "apiClient"));
    }

    #[test]
    fn yaml_top_level_key_matches() {
        let src = "metadata:\n  name: sample\n";
        assert!(anchor_found(src, Lang::Yaml, "metadata"));
        assert!(anchor_found(src, Lang::Yaml, "name"));
    }

    #[test]
    fn yaml_value_mention_does_not_match() {
        let src = "image: sample:latest\n";
        // `sample` appears as a value, not as a key.
        assert!(!anchor_found(src, Lang::Yaml, "sample"));
    }

    #[test]
    fn shell_function_posix_form() {
        let src = "rotate_keys() {\n  echo hi\n}\n";
        assert!(anchor_found(src, Lang::Shell, "rotate_keys"));
    }

    #[test]
    fn shell_function_bash_form() {
        let src = "function rotate_keys {\n  :\n}\n";
        assert!(anchor_found(src, Lang::Shell, "rotate_keys"));
    }

    #[test]
    fn sql_create_table_matches() {
        let src = "CREATE TABLE intents (id UUID PRIMARY KEY);\n";
        assert!(anchor_found(src, Lang::Sql, "intents"));
    }
}
