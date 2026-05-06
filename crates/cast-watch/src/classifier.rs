//! Change classifier — decides whether a filesystem event needs the
//! cheap macro-only path or the heavy RA-backed full path.
//!
//! The classifier is the gatekeeper of `tiered_analysis`: every
//! decision to spend RA time goes through it.

use std::path::Path;

cast::tier! {
    axis: analysis_cost,
    direction: increasing,
    of: crate::classifier::ChangeKind,
    stages: {
        irrelevant     @ crate::classifier::ChangeKind::Irrelevant:
            "File is outside the analyzed tree, or is a non-Rust file \
             with no anchor pointing at it. Cost: zero.",
        macro_only     @ crate::classifier::ChangeKind::MacroOnly:
            "Edit confined to bytes inside one or more cast::*! \
             invocations. Reparse with syn, swap snapshot, broadcast. \
             Cost: milliseconds.",
        implementation @ crate::classifier::ChangeKind::Implementation:
            "Rust file changed outside cast::*! invocations, or a non-\
             Rust file referenced by an anchor changed (e.g. a Vue or \
             YAML target of cast::io::continues_in!). Marks anchor \
             diagnostics stale. Cost: free in `--lazy`, full RA reload \
             on `rebuild`.",
    },
    tags: ["cast_watch"],
    note: "Direction is `increasing` cost. Classification is a strict \
           ordering: the highest-cost matching kind wins. A change to a \
           file that contains both cast::*! invocations and ordinary \
           Rust outside them is classified `implementation`, not \
           `macro_only` — anchor resolution may have drifted.",
}

cast::rule! {
    rule: "Classification compares the pre-edit and post-edit byte \
           ranges of every cast::*! invocation in the file. If the \
           bytes outside those ranges are identical *modulo \
           whitespace* between before and after, the change is \
           MacroOnly. Otherwise it is Implementation.",
    why:  "Whitespace-tolerant equality is the right strictness: \
           inserting or deleting blank lines around a macro, indenting, \
           or running rustfmt across a file doesn't change anchor \
           resolution and shouldn't escalate to RA. Stricter byte-\
           equality misclassifies 'add a new cast::*! invocation' \
           (which inserts a newline outside the macro) as \
           Implementation, defeating the use case the daemon exists \
           for. Weaker would let real impl edits slip through — but \
           Rust's lexical structure means inserting/removing \
           whitespace can never change semantics between tokens, so \
           filtering whitespace before comparison is exactly tight \
           enough.",
    governs: [
        crate::classifier::classify,
    ],
    tags: ["cast_watch"],
}

cast::rule! {
    rule: "If syn cannot parse either the before or after content \
           (mid-edit syntax errors), the classifier returns \
           Implementation — never MacroOnly. The cheap path requires \
           proof; absence of proof escalates.",
    why:  "syn::parse_file is the source of truth for macro byte \
           ranges. Without it we cannot guarantee a change was confined \
           to macro bodies, and a wrong MacroOnly classification means \
           cast-watch silently skips an RA reload that would have \
           caught a real bug.",
    governs: [
        crate::classifier::classify,
    ],
    tags: ["cast_watch"],
}

cast::anti_pattern! {
    avoid:      "Pre-warming the file-bytes cache only for files that \
                 already appear in the analyzer's report (i.e. files \
                 that already contain cast::*! invocations).",
    why:        "When an LLM or developer adds the *first* cast::*! \
                 invocation to a previously-bare .rs file, the watcher \
                 reads `before = None` from the cache and falls through \
                 the new-file branch in `classify`. With surrounding \
                 Rust code present, that branch returns Implementation \
                 — escalating an annotation-only edit to a stale-marker \
                 (or in --eager, a full RA reload). The fast path is \
                 silently bypassed for the very flow the daemon was \
                 built for: an LLM adding annotations.",
    instead:    "Seed every watched .rs file (recursively under the \
                 project roots, filtered through `should_watch`) into \
                 the cache at startup, regardless of whether the initial \
                 report referenced it. The cost is a one-time directory \
                 walk; the win is correct classification on first \
                 annotation.",
    instead_at: crate::pre_warm_file_bytes,
    governs:    [crate::classifier::classify],
    tags:        ["cast_watch"],
    note:       "Surfaced 2026-04-29 during a dogfood pass: adding the \
                 first cast::*! invocation to a previously-bare module \
                 left the file in stale_files instead of entering the \
                 macro-only path.",
}

/// One change-classification result. `tiered_analysis` lives or dies
/// by this enum: every escalation to RA passes through `Implementation`,
/// every fast-path edit through `MacroOnly`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    Irrelevant,
    MacroOnly,
    Implementation,
}

/// Classify a filesystem change against the prior file contents.
///
/// `before` is the bytes of the file before the change (or `None` for
/// a newly-created file); `after` is the bytes now on disk. Returns
/// `Irrelevant` for non-Rust files (refinement to check anchor targets
/// is future work — see `tier_dispatch` concept).
pub fn classify(path: &Path, before: Option<&[u8]>, after: &[u8]) -> ChangeKind {
    if path.extension().and_then(|s| s.to_str()) != Some("rs") {
        return ChangeKind::Irrelevant;
    }
    let after_str = match std::str::from_utf8(after) {
        Ok(s) => s,
        Err(_) => return ChangeKind::Implementation,
    };
    let after_non_macro = match non_macro_bytes(after_str) {
        Some(b) => b,
        None => return ChangeKind::Implementation,
    };
    let before_str = match before {
        Some(b) => match std::str::from_utf8(b) {
            Ok(s) => s,
            Err(_) => return ChangeKind::Implementation,
        },
        None => {
            // New file: macro-only iff every byte is inside a cast::*!
            // invocation (file is wholly annotation, no surrounding code).
            return if after_non_macro.iter().all(|b| b.is_ascii_whitespace()) {
                ChangeKind::MacroOnly
            } else {
                ChangeKind::Implementation
            };
        }
    };
    let before_non_macro = match non_macro_bytes(before_str) {
        Some(b) => b,
        None => return ChangeKind::Implementation,
    };
    if strip_whitespace(&before_non_macro) == strip_whitespace(&after_non_macro) {
        ChangeKind::MacroOnly
    } else {
        ChangeKind::Implementation
    }
}

/// Drop ASCII whitespace bytes. Whitespace between Rust tokens is
/// semantically inert, so two non-macro byte slices that differ only
/// in whitespace describe the same surrounding code — see the
/// classification rule for full justification.
fn strip_whitespace(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().copied().filter(|b| !b.is_ascii_whitespace()).collect()
}

/// Return the bytes of `src` with every `cast::*!` invocation excised.
/// `None` if syn cannot parse the file.
fn non_macro_bytes(src: &str) -> Option<Vec<u8>> {
    let file = syn::parse_file(src).ok()?;
    let mut ranges = Vec::new();
    collect_cast_macro_ranges(&file, &mut ranges);
    ranges.sort_by_key(|r: &(usize, usize)| r.0);
    let bytes = src.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut cursor = 0;
    for (start, end) in ranges {
        if start >= cursor {
            out.extend_from_slice(&bytes[cursor..start.min(bytes.len())]);
        }
        cursor = end.max(cursor);
    }
    if cursor < bytes.len() {
        out.extend_from_slice(&bytes[cursor..]);
    }
    Some(out)
}

/// Walk the parsed file collecting (start_byte, end_byte) ranges of
/// every macro invocation whose path's first segment is `cast`.
fn collect_cast_macro_ranges(file: &syn::File, out: &mut Vec<(usize, usize)>) {
    use syn::visit::Visit;
    struct V<'a> {
        out: &'a mut Vec<(usize, usize)>,
    }
    impl<'a, 'ast> Visit<'ast> for V<'a> {
        fn visit_macro(&mut self, m: &'ast syn::Macro) {
            if is_cast_macro(&m.path) {
                let span = syn::spanned::Spanned::span(m);
                let r = span.byte_range();
                self.out.push((r.start, r.end));
            }
            syn::visit::visit_macro(self, m);
        }
    }
    let mut v = V { out };
    v.visit_file(file);
}

fn is_cast_macro(path: &syn::Path) -> bool {
    path.segments
        .first()
        .map(|s| s.ident == "cast")
        .unwrap_or(false)
}

cast::concept! {
    name: "change_classifier_helpers",
    summary: "Pure helpers under classify: strip_whitespace, \
              non_macro_bytes, collect_cast_macro_ranges, is_cast_macro. \
              All operate on the input byte slice / syn::File and return \
              new values without I/O or mutation.",
    anchors: [
        crate::classifier::classify,
        crate::classifier::strip_whitespace,
        crate::classifier::non_macro_bytes,
        crate::classifier::collect_cast_macro_ranges,
        crate::classifier::is_cast_macro,
    ],
    tags: ["cast_watch_classifier"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::pure_function,
    concept: "change_classifier_helpers",
    why: "All read-only on inputs; output decided solely by the bytes.",
}

cast::concept! {
    name: "change_kind_category",
    summary: "Three-way classification of a filesystem change: \
              Irrelevant, MacroOnly, Implementation.",
    anchors: [
        crate::classifier::ChangeKind,
    ],
    tags: ["cast_watch_classifier"],
}

cast::continues_in! {
    target: cast_stdlib::type_properties::sum_type,
    concept: "change_kind_category",
    why: "Three-variant enum; exactly one inhabited per classify call.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "change_classifier_helpers",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::architecture::classifier_dispatch,
    concept: "change_kind_category",
    why: lazy,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn rs(name: &str) -> PathBuf {
        PathBuf::from(format!("/tmp/{name}.rs"))
    }

    #[test]
    fn non_rust_is_irrelevant() {
        assert_eq!(
            classify(&PathBuf::from("/tmp/x.yaml"), Some(b"a"), b"b"),
            ChangeKind::Irrelevant
        );
    }

    #[test]
    fn pure_macro_body_edit_is_macro_only() {
        let before = r#"
fn keep() {}
cast::concept! {
    name: "x",
    summary: "before",
    anchors: [],
    tags: ["t"],
}
"#;
        let after = r#"
fn keep() {}
cast::concept! {
    name: "x",
    summary: "AFTER",
    anchors: [],
    tags: ["t"],
}
"#;
        assert_eq!(
            classify(&rs("x"), Some(before.as_bytes()), after.as_bytes()),
            ChangeKind::MacroOnly
        );
    }

    #[test]
    fn code_edit_outside_macro_is_implementation() {
        let before = "fn keep() {}\ncast::concept! { name: \"x\", summary: \"\", anchors: [], tag: \"t\" }\n";
        let after = "fn renamed() {}\ncast::concept! { name: \"x\", summary: \"\", anchors: [], tag: \"t\" }\n";
        assert_eq!(
            classify(&rs("x"), Some(before.as_bytes()), after.as_bytes()),
            ChangeKind::Implementation
        );
    }

    #[test]
    fn adding_new_macro_is_macro_only() {
        let before = "fn keep() {}\n";
        let after = "fn keep() {}\ncast::concept! { name: \"x\", summary: \"\", anchors: [], tag: \"t\" }\n";
        assert_eq!(
            classify(&rs("x"), Some(before.as_bytes()), after.as_bytes()),
            ChangeKind::MacroOnly
        );
    }

    #[test]
    fn broken_syntax_escalates() {
        let before = "fn ok() {}\n";
        let after = "fn ok( {}\n"; // unclosed paren
        assert_eq!(
            classify(&rs("x"), Some(before.as_bytes()), after.as_bytes()),
            ChangeKind::Implementation
        );
    }

    #[test]
    fn new_file_pure_macro_is_macro_only() {
        let after = "cast::concept! { name: \"x\", summary: \"\", anchors: [], tag: \"t\" }\n";
        assert_eq!(
            classify(&rs("x"), None, after.as_bytes()),
            ChangeKind::MacroOnly
        );
    }

    #[test]
    fn new_file_with_code_is_implementation() {
        let after = "fn new() {}\n";
        assert_eq!(
            classify(&rs("x"), None, after.as_bytes()),
            ChangeKind::Implementation
        );
    }

    #[test]
    fn non_cast_macro_treated_as_code() {
        let before = "println!(\"a\");\n";
        let after = "println!(\"b\");\n";
        assert_eq!(
            classify(&rs("x"), Some(before.as_bytes()), after.as_bytes()),
            ChangeKind::Implementation
        );
    }
}
