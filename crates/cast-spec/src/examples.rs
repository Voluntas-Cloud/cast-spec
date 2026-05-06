//! The `examples/` corpus — one subdirectory per language under
//! `examples/`, each holding small illustrative programs that cast
//! names via [`cast::io::continues_in!`]. The corpus is dogfood for
//! the foreign-language pointer machinery — every new language cast
//! learns to resolve gets a directory here and a per-language
//! sub-concept rooted at this module.
//!
//! The structure is deliberately a tree of concepts:
//!
//! - `examples` — top-level umbrella concept.
//! - `c_examples` — C-language branch. One concept per language,
//!   anchored at this module, listing the foreign anchors via
//!   `cast::io::continues_in!` edges.
//!
//! Walking from `examples` (via cast-watch's `walk` query, with
//! enough hops) surfaces every per-language sub-concept and the
//! foreign bridges it declares. That is what "walking the foreign
//! language" means in cast: traversing from the umbrella concept
//! out into each language's concrete source files.
//!
//! Future languages (kotlin, typescript, swift, …) join the tree by
//! adding a sibling concept here (`kotlin_examples`,
//! `typescript_examples`, …) plus a sibling subdirectory under
//! `examples/`. No machinery in cast-watch / cast-extract / the
//! validator needs to change — the per-language `Lang` variant and
//! its anchor matcher are the only growable surface, and the
//! encapsulation seam (`SpecSource` + `LanguageBackend`) ensures
//! that growth is local.

cast::concept! {
    name: "examples",
    summary: "Top-level umbrella for the multi-language examples \
              corpus. Branches into one sub-concept per language \
              (c_examples today; kotlin_examples / \
              typescript_examples / swift_examples to follow). \
              Each branch is anchored at this same module, so the \
              graph builder collects them as siblings of the \
              umbrella when walking — visit `examples` and you \
              reach every language's bridges in one hop.",
    anchors: [
        crate::spec_source::cast_file::CastFileSource,
        crate::examples,
    ],
    tags: ["examples"],
}

cast::continues_in! {
    target:  crate::examples,
    concept: "examples",
    why:     "Branches into per-language sub-concepts. Each branch \
              (c_examples today, kotlin_examples / typescript_examples \
              tomorrow) anchors at this same module, so following \
              this continues_in edge through the anchor index lands \
              on every language sub-concept that has been added. \
              That is what 'cast-watch walk from examples' does: \
              one hop and you have every language's bridges.",
}

cast::continues_in! {
    target:  cast_stdlib::docs::example_driven_documentation::ExampleDrivenDocumentation,
    concept: "examples",
    why:     "The corpus IS example-driven documentation of the \
              foreign-language pointer machinery. Each tiny program \
              under examples/<lang>/ teaches what cast can name in \
              that language by demonstrating it; the abstract \
              capability list (\"we support C function definitions, \
              YAML mapping keys, …\") gets read because the worked \
              examples are pinned next to it.",
    tags:    ["examples"],
}

cast::concept! {
    name: "c_examples",
    summary: "C branch of the examples corpus. examples/c/hello.c \
              is a deliberately tiny program that exercises every \
              anchor shape the C handler recognises: three function \
              definitions (`main`, `format_greeting`, \
              `print_greeting`), one `#define` constant \
              (`MAX_GREETING`), and one `struct` declaration \
              (`greeter`). Walking this concept surfaces all five \
              as io_bridges — proof that cast can name a foreign \
              file's structure end-to-end without the foreign code \
              hosting any cast::*! syntax.",
    anchors: [
        crate::examples,
    ],
    tags: ["examples"],
}

cast::io::continues_in! {
    target:  "examples/c/hello.c",
    lang:    c,
    anchor:  "main",
    concept: "c_examples",
    why:     "Entry point. Parses argv[1] (or defaults to 'World') \
              and chains into format_greeting + print_greeting.",
}

cast::io::continues_in! {
    target:  "examples/c/hello.c",
    lang:    c,
    anchor:  "format_greeting",
    concept: "c_examples",
    why:     "Renders 'Hello, <name>!' into a caller-provided \
              buffer using snprintf. Pure formatting; no I/O.",
}

cast::io::continues_in! {
    target:  "examples/c/hello.c",
    lang:    c,
    anchor:  "print_greeting",
    concept: "c_examples",
    why:     "Writes the formatted string to stdout. The only \
              function in this example that touches the outside \
              world.",
}

cast::io::continues_in! {
    target:  "examples/c/hello.c",
    lang:    c,
    anchor:  "MAX_GREETING",
    concept: "c_examples",
    why:     "Buffer-size constant. Demonstrates that the C \
              handler recognises `#define` declarations as \
              anchorable, not just function definitions.",
}

cast::io::continues_in! {
    target:  "examples/c/hello.c",
    lang:    c,
    anchor:  "greeter",
    concept: "c_examples",
    why:     "Locale-and-salutation pair the formatter reads from. \
              Demonstrates that the C handler recognises `struct` \
              declarations as anchorable.",
}

cast::continues_in! {
    target:  cast_stdlib::testing::contract_test::ContractTest,
    concept: "c_examples",
    why:     "examples/c/hello.c is the C handler's contract test: \
              every node kind the C `Lang` variant claims to anchor \
              (function, #define constant, struct) appears in this \
              one tiny program. If the C handler stops recognising \
              one of them, the io_continues_in edges below fail to \
              resolve and the report surfaces it.",
    tags:    ["examples"],
}

cast::concept! {
    name: "kotlin_examples",
    summary: "Kotlin branch of the examples corpus. \
              examples/kotlin/Hello.kt exercises the Kotlin grammar's \
              function_declaration, class_declaration, and \
              object_declaration node patterns.",
    anchors: [
        crate::examples,
    ],
    tags: ["examples"],
}

cast::io::continues_in! {
    target:  "examples/kotlin/Hello.kt",
    lang:    kotlin,
    anchor:  "greet",
    concept: "kotlin_examples",
    why:     "Top-level function definition in Kotlin.",
}

cast::io::continues_in! {
    target:  "examples/kotlin/Hello.kt",
    lang:    kotlin,
    anchor:  "Greeter",
    concept: "kotlin_examples",
    why:     "Class declaration. Demonstrates the type_identifier \
              capture in the Kotlin .scm query.",
}

cast::continues_in! {
    target:  cast_stdlib::testing::contract_test::ContractTest,
    concept: "kotlin_examples",
    why:     "examples/kotlin/Hello.kt is the Kotlin handler's \
              contract test: function_declaration and class_declaration \
              are the two node kinds the Kotlin `Lang` variant claims \
              to anchor, and both appear in this file. Object \
              declaration is reserved for a follow-up addition.",
    tags:    ["examples"],
}

cast::concept! {
    name: "typescript_examples",
    summary: "TypeScript branch of the examples corpus. \
              examples/typescript/hello.ts exercises function, \
              interface, and lexical const declarations.",
    anchors: [
        crate::examples,
    ],
    tags: ["examples"],
}

cast::io::continues_in! {
    target:  "examples/typescript/hello.ts",
    lang:    typescript,
    anchor:  "greet",
    concept: "typescript_examples",
    why:     "Top-level function declaration.",
}

cast::io::continues_in! {
    target:  "examples/typescript/hello.ts",
    lang:    typescript,
    anchor:  "Greeter",
    concept: "typescript_examples",
    why:     "Interface declaration — type-level construct.",
}

cast::io::continues_in! {
    target:  "examples/typescript/hello.ts",
    lang:    typescript,
    anchor:  "enGreeter",
    concept: "typescript_examples",
    why:     "Lexical const declaration — value-level construct \
              captured via the variable_declarator pattern.",
}

cast::continues_in! {
    target:  cast_stdlib::testing::contract_test::ContractTest,
    concept: "typescript_examples",
    why:     "examples/typescript/hello.ts is the TypeScript handler's \
              contract test: function, interface, and lexical const \
              are the three node kinds the TypeScript `Lang` variant \
              claims to anchor, and all three appear here. Loss of \
              any one breaks an io_continues_in edge below.",
    tags:    ["examples"],
}

cast::concept! {
    name: "swift_examples",
    summary: "Swift branch of the examples corpus. \
              examples/swift/Hello.swift exercises function and \
              protocol declarations.",
    anchors: [
        crate::examples,
    ],
    tags: ["examples"],
}

cast::io::continues_in! {
    target:  "examples/swift/Hello.swift",
    lang:    swift,
    anchor:  "greet",
    concept: "swift_examples",
    why:     "Top-level Swift function declaration.",
}

cast::io::continues_in! {
    target:  "examples/swift/Hello.swift",
    lang:    swift,
    anchor:  "Greeter",
    concept: "swift_examples",
    why:     "Protocol declaration — exercises the protocol_declaration \
              capture in the Swift .scm query.",
}

cast::continues_in! {
    target:  cast_stdlib::testing::contract_test::ContractTest,
    concept: "swift_examples",
    why:     "examples/swift/Hello.swift is the Swift handler's \
              contract test: function and protocol are the two node \
              kinds the Swift `Lang` variant claims to anchor, both \
              present here.",
    tags:    ["examples"],
}

cast::concept! {
    name: "bash_examples",
    summary: "Bash/Shell branch of the examples corpus. \
              examples/bash/hello.sh exercises the function_definition \
              capture in the bash .scm query.",
    anchors: [
        crate::examples,
    ],
    tags: ["examples"],
}

cast::io::continues_in! {
    target:  "examples/bash/hello.sh",
    lang:    shell,
    anchor:  "greet",
    concept: "bash_examples",
    why:     "Bash function definition.",
}

cast::continues_in! {
    target:  cast_stdlib::testing::contract_test::ContractTest,
    concept: "bash_examples",
    why:     "examples/bash/hello.sh is the bash handler's contract \
              test: function_definition is the only node kind the \
              shell `Lang` variant claims to anchor, and it appears \
              here.",
    tags:    ["examples"],
}

cast::concept! {
    name: "yaml_examples",
    summary: "YAML branch of the examples corpus. \
              examples/yaml/hello.yaml exercises the \
              block_mapping_pair key capture for top-level keys.",
    anchors: [
        crate::examples,
    ],
    tags: ["examples"],
}

cast::io::continues_in! {
    target:  "examples/yaml/hello.yaml",
    lang:    yaml,
    anchor:  "greeting",
    concept: "yaml_examples",
    why:     "Top-level YAML mapping key.",
}

cast::io::continues_in! {
    target:  "examples/yaml/hello.yaml",
    lang:    yaml,
    anchor:  "metadata",
    concept: "yaml_examples",
    why:     "Second top-level key — verifies the query catches \
              every block_mapping_pair, not just the first.",
}

cast::continues_in! {
    target:  cast_stdlib::testing::contract_test::ContractTest,
    concept: "yaml_examples",
    why:     "examples/yaml/hello.yaml is the YAML handler's \
              contract test: top-level mapping keys are the only \
              node kind the YAML `Lang` variant claims to anchor, \
              and the file pins two of them so the test also \
              catches a regression where only the first match is \
              returned.",
    tags:    ["examples"],
}

cast::concept! {
    name: "markdown_examples",
    summary: "Markdown branch of the examples corpus. \
              examples/markdown/hello.md exercises the \
              tree-sitter-md heading-path walker and the \
              inline-rendered anchor surface — `Hello::Architecture` \
              picks the H2 nested under the H1, and `Hello::Emphasized` \
              addresses `## *Emphasized*` after the inline grammar \
              strips the `*` delimiters.",
    anchors: [
        crate::examples,
    ],
    tags: ["examples"],
}

cast::io::continues_in! {
    target:  "examples/markdown/hello.md",
    lang:    markdown,
    anchor:  "Hello",
    concept: "markdown_examples",
    why:     "Top-level H1 — exercises the simplest heading-path walk.",
}

cast::io::continues_in! {
    target:  "examples/markdown/hello.md",
    lang:    markdown,
    anchor:  "Hello::Architecture",
    concept: "markdown_examples",
    why:     "Nested H2 — verifies path segments join with `::`.",
}

cast::io::continues_in! {
    target:  "examples/markdown/hello.md",
    lang:    markdown,
    anchor:  "Hello::Architecture::Mailbox",
    concept: "markdown_examples",
    why:     "Three-level depth — verifies the level-keyed stack \
              extends past two segments.",
}

cast::io::continues_in! {
    target:  "examples/markdown/hello.md",
    lang:    markdown,
    anchor:  "Hello::Emphasized",
    concept: "markdown_examples",
    why:     "Inline-rendered anchor surface — `## *Emphasized*` is \
              addressable as `Emphasized`, not `*Emphasized*`.",
}

cast::continues_in! {
    target:  cast_stdlib::testing::contract_test::ContractTest,
    concept: "markdown_examples",
    why:     "examples/markdown/hello.md is the markdown handler's \
              contract test: heading-path joining (`::` separators), \
              multi-level depth, and inline-rendered anchor text are \
              the three behaviours the markdown handler must honour, \
              and each appears here pinned by an io_continues_in \
              edge that breaks if the behaviour regresses.",
    tags:    ["examples"],
}
