//! `TreeSitterBackend` — AST-based foreign-language anchor
//! resolution, sketched.
//!
//! Today's [`super::foreign_grep::ForeignGrepBackend`] matches
//! anchors with declaration-prefix grep + word boundary. Conservative
//! by design but inherently lossy: a declaration that crosses a line,
//! sits behind macro substitution, or appears in an unusual format
//! is invisible. False positives in strings or comments are
//! avoided only by the trailing-boundary check, not by understanding
//! syntactic context.
//!
//! `TreeSitterBackend` upgrades every supported [`crate::parser::io_continues_in::Lang`]
//! variant from grep-resolved to AST-resolved by going through the
//! `tree-sitter` Rust ecosystem. One backend handles all grammars;
//! the per-language work is choosing a `tree-sitter-<lang>` grammar
//! crate and writing a `.scm` query naming the declaration nodes
//! that count as anchorable.
//!
//! Sketch shape, anchors deliberately unresolved (impl TODO):
//!
//! ```text
//! pub struct TreeSitterBackend {
//!     grammars: Vec<Grammar>,   // one per supported Lang
//! }
//! pub struct Grammar {
//!     lang:    Lang,
//!     ts_lang: tree_sitter::Language,
//!     query:   tree_sitter::Query,   // compiled from a .scm string
//! }
//! pub fn find_anchor(
//!     backend: &TreeSitterBackend,
//!     file_path: &Path,
//!     lang: Lang,
//!     anchor: &str,
//! ) -> bool { /* parse → run query → match captures by name */ }
//! ```
//!
//! Integration: `crate::validator::io_target::anchor_found` grows a
//! preference for an injected `TreeSitterBackend` when the
//! requested `Lang` has a grammar registered, falling back to the
//! existing grep paths otherwise. No `Lang` variant changes; no
//! call-site of `validate_io_annotation` changes — that is the
//! whole point of the encapsulation seam.

cast::concept! {
    name: "tree_sitter_backend",
    summary: "Tree-sitter-based foreign-language anchor resolution. \
              One backend handles all grammars: each supported Lang \
              registers a `tree-sitter-<lang>` grammar crate plus a \
              .scm query naming the declaration nodes that count as \
              anchorable. Replaces the conservative grep approach in \
              ForeignGrepBackend with AST-level accuracy — no false \
              positives in comments or strings, no false negatives \
              from declarations that cross lines or sit behind \
              macro substitution. Adding a new language becomes \
              `cargo add tree-sitter-<lang>` plus a query file.",
    anchors: [
        crate::language_backend::tree_sitter::TreeSitterBackend,
        crate::language_backend::tree_sitter::Grammar,
        crate::language_backend::tree_sitter::find_anchor,
    ],
    tags: ["language_backend"],
}

cast::compare! {
    grep      @ crate::language_backend::foreign_grep::ForeignGrepBackend:
        "Lossy by design. Per-language declaration-prefix list plus \
         a trailing-boundary check. Fast, no build dep, conservative \
         (false negatives prompt grammar refinement). Cannot \
         distinguish a declaration from a same-shape mention in a \
         comment or string — only the prefix gate keeps it honest.",
    tree_sitter @ crate::language_backend::tree_sitter::TreeSitterBackend:
        "AST-faithful. tree-sitter parses the foreign source into a \
         concrete syntax tree; a per-language .scm query selects \
         declaration nodes by kind and extracts their names. \
         Error-tolerant by design (incremental parsing was its \
         original use case), so partial files still surface \
         declarations. Cost: a C-built grammar crate per language.",
    tags: ["language_backend"],
    note: "Both impl ForeignBackend (today: still a sketch — the \
           sub-trait that extends LanguageBackend with a typed \
           anchor-finding method has not been added yet, on \
           purpose, until the second backend forces the unification \
           to be real and not speculative).",
}

cast::rule! {
    rule: "A Lang variant is upgraded from grep-resolved to \
           AST-resolved by registering a Grammar in TreeSitterBackend \
           — a (Lang, tree_sitter::Language, .scm query) triple. \
           No edit to anchor_found's match arms, no edit to the \
           Lang enum, no edit to validate_io_annotation. Adding a \
           grammar without a .scm query is a no-op: the query is \
           what names declarations as anchorable.",
    why:  "The point of the encapsulation seam is that growing \
           foreign-language coverage is local. If upgrading C from \
           grep to tree-sitter required touching anchor_found, \
           validate_io_annotation, or the Lang enum, the seam \
           wouldn't have done its job — we'd be back at the bolt-on \
           shape ForeignGrepBackend started as. Forcing the \
           Grammar registration to be the only edit-site keeps the \
           seam load-bearing: each new grammar is one entry in one \
           list, with one .scm string.",
    governs: [
        crate::language_backend::tree_sitter::TreeSitterBackend,
        crate::language_backend::tree_sitter::Grammar,
    ],
    tags: ["language_backend"],
}

cast::rule! {
    rule: "Lang variants supported by tree-sitter-<lang> grammars \
           must use TreeSitterBackend; only Lang::External and \
           Lang::Rfc skip backend dispatch (file-existence and \
           regex-shape are not anchor problems). For a grammar \
           that does not exist or is not registered, the validator \
           falls through to ForeignGrepBackend automatically — \
           grep is the floor, never the ceiling.",
    why:  "Two backends with overlapping responsibility invite \
           silent disagreement. If a Lang has both a grammar and a \
           grep matcher, calls must dispatch to the AST path \
           deterministically — otherwise the same anchor can \
           resolve in one report and not the next, depending on \
           which path the validator happened to take. Pinning the \
           dispatch order avoids that whole class of bug. The \
           grep path stays as a fallback (and as the implementation \
           for variants without grammars) so unsupported languages \
           are never worse off than today.",
    governs: [
        crate::language_backend::tree_sitter::TreeSitterBackend,
        crate::validator::io_target,
    ],
    tags: ["language_backend"],
}

cast::concept! {
    name: "markdown_language_backend",
    summary: "Tree-sitter-md grammar for anchoring io_continues_in! \
              targets at markdown heading paths. Unlike the other \
              registered grammars (c, kotlin, typescript, swift, \
              bash, yaml — all flat-identifier matches) markdown \
              anchors are *heading-path strings*: '#' nesting maps \
              to '::' segments, so 'Architecture::Mailbox' picks the \
              H2 'Mailbox' under the H1 'Architecture'. The \
              TreeSitterBackend dispatch seam is reused unchanged; \
              the grammar's query captures atx_heading / setext_heading \
              nodes and the anchor-match step is a path-walk through \
              the heading tree — structurally different from the \
              equality check that works for flat-identifier languages.",
    anchors: [
        crate::parser::io_continues_in::Lang::Markdown,
        crate::language_backend::tree_sitter::markdown_grammar,
        crate::language_backend::tree_sitter::MARKDOWN_QUERY,
        crate::language_backend::tree_sitter::find_markdown_anchor,
    ],
    tags: ["language_backend"],
}

cast::rule! {
    rule: "Markdown anchor matching is a heading-path walk, not an \
           identifier equality test. The TreeSitterBackend grammar \
           list dispatches Lang::Markdown to find_markdown_anchor \
           rather than the shared find_anchor() used by every other \
           registered grammar. Adding a 'markdown_query' that lists \
           heading captures is necessary but not sufficient — the \
           path-walk is what makes 'Architecture::Mailbox' a stable \
           addressing scheme.",
    why:  "Markdown's whole reason for being a language backend is \
           citing a *position in the doc*, not a name. A flat \
           identifier match would hit any heading whose text equals \
           the anchor, regardless of where in the document it sits — \
           two unrelated 'Mailbox' headings under different sections \
           would alias. The heading-path walk forces the citation to \
           be unambiguous: each segment narrows the subtree, the \
           final segment is the leaf heading. This is also how a \
           human reader cites docs.",
    governs: [
        crate::language_backend::tree_sitter::TreeSitterBackend,
        crate::language_backend::tree_sitter::find_markdown_anchor,
    ],
    tags: ["language_backend"],
}

cast::rule! {
    rule: "Heading anchor text is the *inline-rendered* string, not \
           the raw markdown bytes between the `#` marker and \
           end-of-line. find_markdown_anchor parses each heading's \
           inline node with tree-sitter-md's INLINE_LANGUAGE, walks \
           the resulting subtree, and concatenates leaf text — \
           dropping emphasis markers (`*`, `_`), code-span backticks, \
           and link syntax (`[text](url)` becomes `text`). A heading \
           written as `## *Mailbox*` is addressable as `Mailbox`, \
           not `*Mailbox*`.",
    why:  "Humans cite documents by what they read, not by what \
           they typed. Nobody writes `Architecture::*Mailbox*` in a \
           continues_in! anchor to mean the heading `## *Mailbox*` — \
           they write `Architecture::Mailbox`. Treating raw markdown \
           bytes as the anchor surface would force authors to escape \
           formatting in their citations, drift the citation when \
           the heading's emphasis is added or removed, and cite \
           link-form headings (`## [Spec](spec.md)`) by their \
           parenthesized URL. Inline-render is what makes the \
           citation surface stable under cosmetic edits.",
    governs: [
        crate::language_backend::tree_sitter::find_markdown_anchor,
    ],
    tags: ["language_backend"],
}

use super::LanguageBackend;
use crate::parser::io_continues_in::Lang;
use std::sync::OnceLock;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

/// A registered tree-sitter grammar for one [`Lang`] variant.
/// Bundles the language object with a precompiled query whose
/// captures are the names of declarations considered anchorable.
pub struct Grammar {
    pub lang: Lang,
    pub ts_lang: tree_sitter::Language,
    pub query: Query,
}

/// AST-based foreign anchor resolver. Holds one [`Grammar`] per
/// supported [`Lang`]; [`Self::find_anchor_in_source`] parses the
/// source, runs the grammar's query against the syntax tree, and
/// checks whether any captured identifier equals the anchor.
pub struct TreeSitterBackend {
    grammars: Vec<Grammar>,
}

impl TreeSitterBackend {
    pub fn new() -> Self {
        Self {
            grammars: vec![
                c_grammar(),
                kotlin_grammar(),
                typescript_grammar(),
                javascript_grammar(),
                swift_grammar(),
                bash_grammar(),
                yaml_grammar(),
                markdown_grammar(),
                html_grammar(),
                css_grammar(),
                python_grammar(),
            ],
        }
    }

    /// True if a grammar is registered for `lang`. Validator uses
    /// this to decide whether to dispatch here or fall through to
    /// the grep matcher.
    pub fn supports(&self, lang: Lang) -> bool {
        self.grammars.iter().any(|g| g.lang == lang)
    }

    /// Match `anchor` against declarations in `source` for the
    /// given `lang`. Returns `None` if the lang has no registered
    /// grammar (caller falls through); `Some(true)` / `Some(false)`
    /// when the AST query was actually run.
    pub fn find_anchor_in_source(
        &self,
        source: &[u8],
        lang: Lang,
        anchor: &str,
    ) -> Option<bool> {
        let grammar = self.grammars.iter().find(|g| g.lang == lang)?;
        Some(match lang {
            Lang::Markdown => find_markdown_anchor(grammar, source, anchor),
            _ => find_anchor(grammar, source, anchor),
        })
    }
}

impl Default for TreeSitterBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageBackend for TreeSitterBackend {
    fn name(&self) -> &'static str {
        "tree-sitter"
    }
}

/// Free-function entry point for the AST query — used by
/// [`TreeSitterBackend::find_anchor_in_source`] and exposed for
/// per-grammar unit tests that don't want to construct the full
/// backend.
pub fn find_anchor(grammar: &Grammar, source: &[u8], anchor: &str) -> bool {
    let mut parser = Parser::new();
    if parser.set_language(&grammar.ts_lang).is_err() {
        return false;
    }
    let Some(tree) = parser.parse(source, None) else {
        return false;
    };
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&grammar.query, tree.root_node(), source);
    while let Some(m) = matches.next() {
        for cap in m.captures {
            if let Ok(text) = cap.node.utf8_text(source) {
                if text == anchor {
                    return true;
                }
            }
        }
    }
    false
}

/// Heading-path resolver for markdown. Unlike [`find_anchor`] (a flat
/// identifier-equality check shared by every other registered grammar)
/// markdown anchors address a *position* in the heading tree:
/// `Architecture::Mailbox` matches the H2 `Mailbox` nested under the
/// H1 `Architecture`, and only that one — a `Mailbox` H2 under a
/// different H1 does not collide.
///
/// Strategy:
/// 1. Parse the source with the block grammar; collect every heading
///    capture as `(level, inline_node)` and sort by document order.
/// 2. Walk the headings with a level-keyed stack: a heading at level
///    `n` pops every entry at depth `>= n`, then pushes itself. The
///    stack at any moment is the current heading path.
/// 3. After each push, join the stack's rendered texts with `::` and
///    compare against the anchor.
///
/// The stack invariant means deeper headings extend the path and same
/// or shallower headings reset it — exactly how a reader follows the
/// heading hierarchy down a document.
pub fn find_markdown_anchor(grammar: &Grammar, source: &[u8], anchor: &str) -> bool {
    let mut parser = Parser::new();
    if parser.set_language(&grammar.ts_lang).is_err() {
        return false;
    }
    let Some(tree) = parser.parse(source, None) else {
        return false;
    };

    let capture_names = grammar.query.capture_names();
    let mut headings: Vec<(usize, tree_sitter::Node)> = Vec::new();
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&grammar.query, tree.root_node(), source);
    while let Some(m) = matches.next() {
        for cap in m.captures {
            let level = match capture_names[cap.index as usize] {
                "h1" | "sh1" => 1,
                "h2" | "sh2" => 2,
                "h3" => 3,
                "h4" => 4,
                "h5" => 5,
                "h6" => 6,
                _ => continue,
            };
            headings.push((level, cap.node));
        }
    }
    headings.sort_by_key(|(_, n)| n.start_byte());

    let mut stack: Vec<(usize, String)> = Vec::new();
    for (level, node) in headings {
        while stack.last().is_some_and(|(top, _)| *top >= level) {
            stack.pop();
        }
        stack.push((level, inline_render(source, node)));
        let path: String = stack
            .iter()
            .map(|(_, t)| t.as_str())
            .collect::<Vec<_>>()
            .join("::");
        if path == anchor {
            return true;
        }
    }
    false
}

/// Render a heading's `(inline)` node to plain text by re-parsing
/// its bytes with tree-sitter-md's INLINE_LANGUAGE and concatenating
/// the leaf text — dropping markup leaves like `*`, `` ` ``, and
/// `link_destination`. The block grammar treats inline content as
/// opaque; only the inline grammar exposes the structure that lets
/// us discriminate "what the reader sees" from "the markup that
/// produced it".
fn inline_render(source: &[u8], inline_node: tree_sitter::Node) -> String {
    let bytes = &source[inline_node.start_byte()..inline_node.end_byte()];
    let mut parser = Parser::new();
    if parser.set_language(markdown_inline_lang()).is_err() {
        return String::from_utf8_lossy(bytes).trim().to_string();
    }
    let Some(tree) = parser.parse(bytes, None) else {
        return String::from_utf8_lossy(bytes).trim().to_string();
    };
    let mut out = String::new();
    walk_inline(tree.root_node(), bytes, &mut out);
    out.trim().to_string()
}

/// Recursive walk over the inline grammar's parse tree, emitting
/// rendered text by *byte gap*: any byte range inside a node that no
/// child covers is emitted as text. Marker children (delimiters,
/// destinations, titles, labels) are recursed into but emit nothing,
/// so their byte ranges are skipped. The gap-driven approach is
/// forced by the inline grammar's emphasis output: `*italic*` parses
/// as `emphasis(emphasis_delimiter, emphasis_delimiter)` with the
/// text "italic" living in the parent-local byte gap between
/// delimiters rather than as a separate child node.
///
/// Special case: `image` recurses only into its `image_description`
/// child so the URL doesn't leak into the rendered text.
fn walk_inline(node: tree_sitter::Node, source: &[u8], out: &mut String) {
    match node.kind() {
        "emphasis_delimiter"
        | "code_span_delimiter"
        | "link_destination"
        | "link_title"
        | "link_label" => return,
        "image" => {
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "image_description" {
                    walk_inline(child, source, out);
                }
            }
            return;
        }
        _ => {}
    }
    let mut cursor = node.walk();
    let children: Vec<_> = node.children(&mut cursor).collect();
    if children.is_empty() {
        if let Ok(text) = node.utf8_text(source) {
            out.push_str(text);
        }
        return;
    }
    let mut pos = node.start_byte();
    for child in children {
        if child.start_byte() > pos {
            if let Ok(text) = std::str::from_utf8(&source[pos..child.start_byte()]) {
                out.push_str(text);
            }
        }
        walk_inline(child, source, out);
        pos = child.end_byte();
    }
    if pos < node.end_byte() {
        if let Ok(text) = std::str::from_utf8(&source[pos..node.end_byte()]) {
            out.push_str(text);
        }
    }
}

/// Process-shared backend used by the validator. Built once on
/// first use; cheap thereafter — no per-call query compilation.
pub fn shared() -> &'static TreeSitterBackend {
    static B: OnceLock<TreeSitterBackend> = OnceLock::new();
    B.get_or_init(TreeSitterBackend::new)
}

fn c_grammar() -> Grammar {
    let ts_lang: tree_sitter::Language = tree_sitter_c::LANGUAGE.into();
    let query = Query::new(&ts_lang, C_QUERY).expect("c query parses");
    Grammar {
        lang: Lang::C,
        ts_lang,
        query,
    }
}

fn kotlin_grammar() -> Grammar {
    let ts_lang: tree_sitter::Language = tree_sitter_kotlin_ng::LANGUAGE.into();
    let query = Query::new(&ts_lang, KOTLIN_QUERY).expect("kotlin query parses");
    Grammar {
        lang: Lang::Kotlin,
        ts_lang,
        query,
    }
}

fn typescript_grammar() -> Grammar {
    let ts_lang: tree_sitter::Language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
    let query = Query::new(&ts_lang, TYPESCRIPT_QUERY).expect("typescript query parses");
    Grammar {
        lang: Lang::TypeScript,
        ts_lang,
        query,
    }
}

fn swift_grammar() -> Grammar {
    let ts_lang: tree_sitter::Language = tree_sitter_swift::LANGUAGE.into();
    let query = Query::new(&ts_lang, SWIFT_QUERY).expect("swift query parses");
    Grammar {
        lang: Lang::Swift,
        ts_lang,
        query,
    }
}

fn bash_grammar() -> Grammar {
    let ts_lang: tree_sitter::Language = tree_sitter_bash::LANGUAGE.into();
    let query = Query::new(&ts_lang, BASH_QUERY).expect("bash query parses");
    Grammar {
        lang: Lang::Shell,
        ts_lang,
        query,
    }
}

fn yaml_grammar() -> Grammar {
    let ts_lang: tree_sitter::Language = tree_sitter_yaml::LANGUAGE.into();
    let query = Query::new(&ts_lang, YAML_QUERY).expect("yaml query parses");
    Grammar {
        lang: Lang::Yaml,
        ts_lang,
        query,
    }
}

fn markdown_grammar() -> Grammar {
    let ts_lang: tree_sitter::Language = tree_sitter_md_025::LANGUAGE.into();
    let query = Query::new(&ts_lang, MARKDOWN_QUERY).expect("markdown query parses");
    Grammar {
        lang: Lang::Markdown,
        ts_lang,
        query,
    }
}

fn markdown_inline_lang() -> &'static tree_sitter::Language {
    static L: OnceLock<tree_sitter::Language> = OnceLock::new();
    L.get_or_init(|| tree_sitter_md_025::INLINE_LANGUAGE.into())
}

fn javascript_grammar() -> Grammar {
    let ts_lang: tree_sitter::Language = tree_sitter_javascript::LANGUAGE.into();
    let query = Query::new(&ts_lang, JAVASCRIPT_QUERY).expect("javascript query parses");
    Grammar {
        lang: Lang::JavaScript,
        ts_lang,
        query,
    }
}

fn html_grammar() -> Grammar {
    let ts_lang: tree_sitter::Language = tree_sitter_html::LANGUAGE.into();
    let query = Query::new(&ts_lang, HTML_QUERY).expect("html query parses");
    Grammar {
        lang: Lang::Html,
        ts_lang,
        query,
    }
}

fn css_grammar() -> Grammar {
    let ts_lang: tree_sitter::Language = tree_sitter_css::LANGUAGE.into();
    let query = Query::new(&ts_lang, CSS_QUERY).expect("css query parses");
    Grammar {
        lang: Lang::Css,
        ts_lang,
        query,
    }
}

fn python_grammar() -> Grammar {
    let ts_lang: tree_sitter::Language = tree_sitter_python::LANGUAGE.into();
    let query = Query::new(&ts_lang, PYTHON_QUERY).expect("python query parses");
    Grammar {
        lang: Lang::Python,
        ts_lang,
        query,
    }
}

/// Tree-sitter query naming every C declaration shape considered
/// anchorable. Two `function_definition` patterns cover regular
/// (`int main(...)`) and pointer-return (`char *make_buffer(...)`)
/// signatures — the AST nests a `pointer_declarator` between the
/// `function_declarator` and the inner `identifier` for the
/// pointer-return case.
const C_QUERY: &str = r#"
(function_definition
  declarator: (function_declarator
    declarator: (identifier) @name))

(function_definition
  declarator: (pointer_declarator
    declarator: (function_declarator
      declarator: (identifier) @name)))

(struct_specifier
  name: (type_identifier) @name)

(union_specifier
  name: (type_identifier) @name)

(enum_specifier
  name: (type_identifier) @name)

(preproc_def
  name: (identifier) @name)

(preproc_function_def
  name: (identifier) @name)
"#;

const KOTLIN_QUERY: &str = r#"
(function_declaration
  (identifier) @name)

(class_declaration
  (identifier) @name)

(object_declaration
  (identifier) @name)
"#;

const TYPESCRIPT_QUERY: &str = r#"
(function_declaration
  name: (identifier) @name)

(class_declaration
  name: (type_identifier) @name)

(interface_declaration
  name: (type_identifier) @name)

(type_alias_declaration
  name: (type_identifier) @name)

(enum_declaration
  name: (identifier) @name)

(lexical_declaration
  (variable_declarator
    name: (identifier) @name))
"#;

const SWIFT_QUERY: &str = r#"
(function_declaration
  name: (simple_identifier) @name)

(class_declaration
  name: (type_identifier) @name)

(protocol_declaration
  name: (type_identifier) @name)
"#;

const BASH_QUERY: &str = r#"
(function_definition
  name: (word) @name)
"#;

const YAML_QUERY: &str = r#"
(block_mapping_pair
  key: (flow_node) @name)
"#;

/// Heading captures for the markdown block grammar. Capture names
/// encode the heading level (`@h1`–`@h6` for atx, `@sh1`/`@sh2` for
/// setext) so the path-walker can reconstruct the document's
/// hierarchy from a flat capture list. Atx and setext are distinct
/// node kinds in tree-sitter-md but address the same tier-1/tier-2
/// space, hence the shared sh1/h1, sh2/h2 collapse at use sites.
const MARKDOWN_QUERY: &str = r#"
(atx_heading (atx_h1_marker) (inline) @h1)
(atx_heading (atx_h2_marker) (inline) @h2)
(atx_heading (atx_h3_marker) (inline) @h3)
(atx_heading (atx_h4_marker) (inline) @h4)
(atx_heading (atx_h5_marker) (inline) @h5)
(atx_heading (atx_h6_marker) (inline) @h6)
(setext_heading (paragraph (inline) @sh1) (setext_h1_underline))
(setext_heading (paragraph (inline) @sh2) (setext_h2_underline))
"#;

/// Tree-sitter query for JavaScript declaration shapes considered
/// anchorable. Mirrors TYPESCRIPT_QUERY without TS-only kinds
/// (interface_declaration, type_alias_declaration). Anchors on
/// function/class/lexical-const declarations.
const JAVASCRIPT_QUERY: &str = r#"
(function_declaration
  name: (identifier) @name)

(class_declaration
  name: (identifier) @name)

(lexical_declaration
  (variable_declarator
    name: (identifier) @name))
"#;

/// Tree-sitter query for HTML. Anchors on element id values —
/// `<div id="foo">` is addressable as `foo`. The `#eq?` predicate
/// constrains the captured attribute_name to "id" so other attributes
/// don't pollute the anchor space.
const HTML_QUERY: &str = r#"
(attribute
  (attribute_name) @_attr
  (quoted_attribute_value (attribute_value) @name)
  (#eq? @_attr "id"))
"#;

/// Tree-sitter query for CSS. Anchors on selector names — class
/// names (`.foo` → `foo`), id names (`#foo` → `foo`), and bare
/// `tag_name` (element selectors like `body`, `header`). Captured
/// without the leading punctuation so `anchor: "primary"` matches
/// both `.primary` and `#primary` if present.
const CSS_QUERY: &str = r#"
(class_selector
  (class_name) @name)

(id_selector
  (id_name) @name)

(tag_name) @name
"#;

/// Tree-sitter query for Python. Anchors on function definitions,
/// class definitions, and module-level identifier assignments
/// (constants like `MAX_RETRIES = 3`).
const PYTHON_QUERY: &str = r#"
(function_definition
  name: (identifier) @name)

(class_definition
  name: (identifier) @name)

(assignment
  left: (identifier) @name)
"#;

#[cfg(test)]
mod tests {
    use super::*;

    fn matches(src: &str, anchor: &str) -> bool {
        TreeSitterBackend::new()
            .find_anchor_in_source(src.as_bytes(), Lang::C, anchor)
            .unwrap()
    }

    #[test]
    fn matches_simple_function() {
        assert!(matches("int main(void) { return 0; }\n", "main"));
    }

    #[test]
    fn matches_pointer_return_function() {
        assert!(matches(
            "char *make_buffer(size_t n) { return 0; }\n",
            "make_buffer"
        ));
    }

    #[test]
    fn matches_struct() {
        assert!(matches("struct point { int x; int y; };\n", "point"));
    }

    #[test]
    fn matches_define() {
        assert!(matches("#define MAX_NODES 16\n", "MAX_NODES"));
    }

    #[test]
    fn rejects_substring_in_comment() {
        assert!(!matches("// main is documented elsewhere\n", "main"));
    }

    #[test]
    fn rejects_longer_function_name() {
        assert!(!matches("int not_main(void) { return 0; }\n", "main"));
    }

    fn matches_in(src: &str, lang: Lang, anchor: &str) -> bool {
        TreeSitterBackend::new()
            .find_anchor_in_source(src.as_bytes(), lang, anchor)
            .unwrap()
    }

    #[test]
    fn kotlin_matches_function() {
        assert!(matches_in(
            "fun greet(name: String) { println(name) }\n",
            Lang::Kotlin,
            "greet",
        ));
    }

    #[test]
    fn kotlin_rejects_longer_name() {
        assert!(!matches_in(
            "fun greet_user(name: String) {}\n",
            Lang::Kotlin,
            "greet",
        ));
    }

    #[test]
    fn typescript_matches_function() {
        assert!(matches_in(
            "function greet(name: string): void {}\n",
            Lang::TypeScript,
            "greet",
        ));
    }

    #[test]
    fn typescript_matches_const() {
        assert!(matches_in(
            "export const apiClient = createClient();\n",
            Lang::TypeScript,
            "apiClient",
        ));
    }

    #[test]
    fn swift_matches_function() {
        assert!(matches_in(
            "func greet(name: String) { print(name) }\n",
            Lang::Swift,
            "greet",
        ));
    }

    #[test]
    fn bash_matches_function() {
        assert!(matches_in(
            "greet() { echo \"Hello, $1!\"; }\n",
            Lang::Shell,
            "greet",
        ));
    }

    #[test]
    fn yaml_matches_top_level_key() {
        assert!(matches_in(
            "metadata:\n  name: example\n",
            Lang::Yaml,
            "metadata",
        ));
    }

    #[test]
    fn markdown_atx_h1() {
        assert!(matches_in("# Architecture\n", Lang::Markdown, "Architecture"));
    }

    #[test]
    fn markdown_nested_h2_full_path() {
        let src = "# Architecture\n\n## Mailbox\n";
        assert!(matches_in(src, Lang::Markdown, "Architecture::Mailbox"));
    }

    #[test]
    fn markdown_three_level_path() {
        let src = "# Top\n\n## Mid\n\n### Leaf\n";
        assert!(matches_in(src, Lang::Markdown, "Top::Mid::Leaf"));
    }

    #[test]
    fn markdown_partial_path_misses() {
        // `Mailbox` alone does not address the nested H2 — the path
        // walk requires every segment from the root.
        let src = "# Architecture\n\n## Mailbox\n";
        assert!(!matches_in(src, Lang::Markdown, "Mailbox"));
    }

    #[test]
    fn markdown_unknown_anchor_misses() {
        assert!(!matches_in("# Architecture\n", Lang::Markdown, "Mailbox"));
    }

    #[test]
    fn markdown_setext_h1() {
        let src = "Architecture\n============\n";
        assert!(matches_in(src, Lang::Markdown, "Architecture"));
    }

    #[test]
    fn markdown_inline_emphasis_strips_markers() {
        // `# *italic*` is addressable as `italic`, not `*italic*` —
        // the inline-render rule promises the *rendered* string.
        assert!(matches_in("# *italic*\n", Lang::Markdown, "italic"));
    }

    #[test]
    fn markdown_sibling_h2s_under_same_h1() {
        let src = "# Top\n\n## A\n\n## B\n";
        assert!(matches_in(src, Lang::Markdown, "Top::A"));
        assert!(matches_in(src, Lang::Markdown, "Top::B"));
    }
}
