//! Resolve a `syn::Path` against the loaded ra_ap_hir crate graph.
//!
//! We can't hand a `syn::Path` to `Semantics::resolve_path` — that
//! takes an `ast::Path` from a tracked syntax tree. Round-tripping
//! through synthetic source would work but is fiddly. Instead we walk
//! the crate graph segment-by-segment using `Module::scope`, which
//! already includes re-exports via the def-map.
//!
//! Walking rules:
//!
//! - **Absolute paths** (`some_crate::foo::bar`): first segment names a
//!   loaded crate; resolution starts at that crate's root module.
//! - **Relative paths** (`crate::foo`, `self::foo`, `super::foo`,
//!   `super::super::foo`): first segments are keywords; resolution
//!   starts at the calling module (or its ancestor / its crate root).
//!   Requires `calling_module` to be supplied.
//! - **Bare-ident paths** (`Foo`, `Foo::Bar`): currently treated as
//!   absolute — first segment must be a crate name. To anchor at the
//!   calling module, prefix `self::`. This avoids ambiguity between
//!   "loaded crate `Foo`" and "local item `Foo` in current scope."
//! - Middle segments must resolve to a `Module` in the current scope,
//!   except: hitting an `Enum` mid-path with one segment remaining
//!   means the next segment is a variant name.
//! - Final segment can be any `ModuleDef`.
//!
//! Failures are surfaced via `PathError` so the CLI can show *why* a
//! path didn't resolve, not just that it didn't.

cast::concept! {
    name: "resolver_ambiguity_as_choice",
    summary: "When path resolution has more than one legitimate \
              candidate — e.g. an assoc-item method exposed by both an \
              inherent impl and a trait, a re-export that shadows a \
              local item, or a future `Type::method` walk that finds \
              multiple trait candidates — the resolver does not pick. \
              It returns a third outcome (an `Ambiguous` arm of \
              `Resolved`) carrying every candidate with a fully-\
              qualifying disambiguator (inherent vs. trait, plus the \
              trait path), and the LLM or developer client rewrites the \
              anchor with the disambiguator they meant. Single-\
              candidate resolution stays a `Resolved::Item` (or \
              `Module` / `Variant`) as today; ambiguity is the only \
              case that escalates.",
    anchors: [
        crate::validator::resolver::Resolved,
        crate::validator::resolver::PathError,
        crate::validator::resolver::resolve_syn_path,
        crate::validator::resolver::step,
    ],
    tags: ["cast_resolver"],
}

cast::rule! {
    rule: "When `step` (or a future assoc-item arm) finds more than one \
           candidate for a segment, it must surface every candidate via \
           an `Ambiguous` outcome — never silently pick the first, \
           never collapse to `DescendedIntoLeaf` or \
           `UnknownInModule`. The candidate list is the contract that \
           lets the LLM caller rewrite the anchor.",
    why:  "Cast's primary consumer is an LLM (cast-watch over a JSON \
           socket, or cast-extract feeding a model). Picking one \
           silently is the worst outcome: the LLM has no signal that \
           its anchor is under-specified and will keep writing it that \
           way. A typed `Ambiguous { candidates }` turns a dead end \
           into a structured prompt back to the writer, exactly \
           parallel to how `unresolved_in_file` returns typed \
           `PathReport`s instead of strings.",
    governs: [
        crate::validator::resolver::Resolved,
        crate::validator::resolver::step,
    ],
    tags: ["cast_resolver"],
    note: "Surfaced 2026-04-29 during the SSOT dogfood pass. The \
           immediate forcing function is `Type::method` paths — see \
           sibling memory `project_cast_resolver_method_limitation` — \
           but the rule covers the broader class: re-export shadowing, \
           trait-method coherence, blanket vs. inherent collisions.",
}

use ra_ap_hir::{Adt, Crate, EnumVariant, Module, ModuleDef, ScopeDef};
use ra_ap_ide_db::RootDatabase;

#[derive(Debug, Clone)]
pub enum Resolved {
    Module(Module),
    Item(ModuleDef),
    Variant(EnumVariant),
}

#[derive(Debug, Clone)]
pub enum PathError {
    /// Path was empty or had no segments.
    Empty,
    /// First segment didn't match any loaded crate by canonical name.
    UnknownCrate(String),
    /// Mid-path segment didn't exist in the parent module's scope.
    UnknownInModule {
        module_path: String,
        segment: String,
    },
    /// Mid-path segment hit a leaf item (struct/fn/const/...) but the
    /// path has more segments after it.
    DescendedIntoLeaf {
        at_segment: String,
        kind: &'static str,
    },
    /// Reached an enum and the next segment doesn't name one of its variants.
    UnknownEnumVariant { enum_path: String, segment: String },
    /// Path used a relative keyword (`crate`, `self`, `super`) but the
    /// invocation site couldn't be mapped to a hir module.
    NoCallingModule,
    /// `super::` chain walked past the crate root.
    NoSuperParent { depth: usize },
}

impl std::fmt::Display for PathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathError::Empty => write!(f, "path has no segments"),
            PathError::UnknownCrate(name) => {
                write!(f, "unknown crate `{name}` (not loaded in this analysis)")
            }
            PathError::UnknownInModule {
                module_path,
                segment,
            } => write!(f, "no item `{segment}` in module `{module_path}`"),
            PathError::DescendedIntoLeaf { at_segment, kind } => write!(
                f,
                "segment `{at_segment}` is a {kind}; path has more segments after it"
            ),
            PathError::UnknownEnumVariant { enum_path, segment } => {
                write!(f, "enum `{enum_path}` has no variant `{segment}`")
            }
            PathError::NoCallingModule => write!(
                f,
                "relative path keyword (`crate`/`self`/`super`) used but invocation has no calling module"
            ),
            PathError::NoSuperParent { depth } => {
                write!(f, "`super::` chain of depth {depth} walked past the crate root")
            }
        }
    }
}

pub fn resolve_syn_path(
    db: &RootDatabase,
    calling_module: Option<Module>,
    path: &syn::Path,
) -> Result<Resolved, PathError> {
    let segments: Vec<String> = path
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect();
    if segments.is_empty() {
        return Err(PathError::Empty);
    }

    // Dispatch on the first segment to pick the starting module.
    // `crate` / `self` / `super` consume the first N segments and
    // anchor at a position derived from the calling module; everything
    // else is treated as absolute (first segment names a loaded crate).
    let (mut current, start_idx) = match segments[0].as_str() {
        "crate" => {
            let m = calling_module.ok_or(PathError::NoCallingModule)?;
            let krate = m.krate(db);
            let name = krate_name(db, krate);
            (
                Cursor::Module {
                    module: krate.root_module(db),
                    path_so_far: name,
                },
                1,
            )
        }
        "self" => {
            let m = calling_module.ok_or(PathError::NoCallingModule)?;
            (
                Cursor::Module {
                    module: m,
                    path_so_far: module_display_path(db, m),
                },
                1,
            )
        }
        "super" => {
            let m = calling_module.ok_or(PathError::NoCallingModule)?;
            // Count the leading `super::super::...` chain. A path like
            // `super::super::foo` walks two parents, then resolves
            // `foo` in that ancestor's scope.
            let mut depth = 0;
            for seg in &segments {
                if seg == "super" {
                    depth += 1;
                } else {
                    break;
                }
            }
            let mut cur = m;
            for _ in 0..depth {
                cur = cur.parent(db).ok_or(PathError::NoSuperParent { depth })?;
            }
            (
                Cursor::Module {
                    module: cur,
                    path_so_far: module_display_path(db, cur),
                },
                depth,
            )
        }
        other => {
            let krate =
                find_crate(db, other).ok_or_else(|| PathError::UnknownCrate(other.to_string()))?;
            (
                Cursor::Module {
                    module: krate.root_module(db),
                    path_so_far: other.to_string(),
                },
                1,
            )
        }
    };

    // If a relative keyword consumed the entire path (e.g. just `self`
    // or `crate`), the cursor already holds the result.
    if start_idx >= segments.len() {
        return Ok(match current {
            Cursor::Module { module, .. } => Resolved::Module(module),
            Cursor::Item(def) => Resolved::Item(def),
            Cursor::Variant(v) => Resolved::Variant(v),
        });
    }

    for (idx, seg) in segments.iter().enumerate().skip(start_idx) {
        let is_last = idx == segments.len() - 1;
        current = step(db, current, seg, is_last)?;
    }

    Ok(match current {
        Cursor::Module { module, .. } => Resolved::Module(module),
        Cursor::Item(def) => Resolved::Item(def),
        Cursor::Variant(v) => Resolved::Variant(v),
    })
}

enum Cursor {
    Module {
        module: Module,
        path_so_far: String,
    },
    Item(ModuleDef),
    Variant(EnumVariant),
}

fn step(db: &RootDatabase, cursor: Cursor, segment: &str, is_last: bool) -> Result<Cursor, PathError> {
    match cursor {
        Cursor::Module { module, path_so_far } => {
            let def = lookup_in_module(db, module, segment).ok_or_else(|| {
                PathError::UnknownInModule {
                    module_path: path_so_far.clone(),
                    segment: segment.to_string(),
                }
            })?;
            Ok(match def {
                ModuleDef::Module(m) => Cursor::Module {
                    module: m,
                    path_so_far: format!("{path_so_far}::{segment}"),
                },
                ModuleDef::Adt(Adt::Enum(e)) if !is_last => {
                    // The next segment must be a variant of this enum;
                    // we encode the enum-cursor inline by building an
                    // Item now and special-casing one extra step. But
                    // simpler: stash the enum on a Variant-search cursor.
                    Cursor::Item(ModuleDef::Adt(Adt::Enum(e)))
                }
                other => {
                    if is_last {
                        Cursor::Item(other)
                    } else {
                        // Stepping into a leaf item; the next iteration
                        // will hit the leaf-with-trailing-segments arm
                        // and complain.
                        Cursor::Item(other)
                    }
                }
            })
        }
        Cursor::Item(ModuleDef::Adt(Adt::Enum(e))) => {
            // Previous step landed us on an Enum mid-path; this segment
            // is the variant.
            let variant = e
                .variants(db)
                .into_iter()
                .find(|v| v.name(db).as_str() == segment);
            match variant {
                Some(v) => {
                    if is_last {
                        Ok(Cursor::Variant(v))
                    } else {
                        // path::Enum::Variant::More — we don't descend
                        // into variants for our use case.
                        Err(PathError::DescendedIntoLeaf {
                            at_segment: segment.to_string(),
                            kind: "enum variant",
                        })
                    }
                }
                None => Err(PathError::UnknownEnumVariant {
                    enum_path: enum_full_path(db, e),
                    segment: segment.to_string(),
                }),
            }
        }
        Cursor::Item(other) => Err(PathError::DescendedIntoLeaf {
            at_segment: segment.to_string(),
            kind: module_def_kind(&other),
        }),
        Cursor::Variant(_) => Err(PathError::DescendedIntoLeaf {
            at_segment: segment.to_string(),
            kind: "enum variant",
        }),
    }
}

fn krate_name(db: &RootDatabase, krate: Crate) -> String {
    krate
        .display_name(db)
        .map(|d| d.canonical_name().to_string())
        .unwrap_or_else(|| "?".to_string())
}

/// Build a `crate_name::a::b` display path for `module`. Walks parents
/// up to the crate root. Used to seed `path_so_far` after a relative-
/// keyword resolves so error messages still point at where we are.
fn module_display_path(db: &RootDatabase, module: Module) -> String {
    let mut chain: Vec<String> = Vec::new();
    let mut cur = module;
    while let Some(parent) = cur.parent(db) {
        let name = cur
            .name(db)
            .map(|n| n.as_str().to_string())
            .unwrap_or_else(|| "?".to_string());
        chain.push(name);
        cur = parent;
    }
    let krate = module.krate(db);
    let mut parts: Vec<String> = vec![krate_name(db, krate)];
    parts.extend(chain.into_iter().rev());
    parts.join("::")
}

fn find_crate(db: &RootDatabase, name: &str) -> Option<Crate> {
    Crate::all(db).into_iter().find(|k| {
        k.display_name(db)
            .map(|d| d.canonical_name().to_string() == name)
            .unwrap_or(false)
    })
}

fn lookup_in_module(db: &RootDatabase, module: Module, name: &str) -> Option<ModuleDef> {
    for (n, def) in module.scope(db, None) {
        if n.as_str() == name {
            if let ScopeDef::ModuleDef(md) = def {
                return Some(md);
            }
        }
    }
    None
}

fn enum_full_path(db: &RootDatabase, e: ra_ap_hir::Enum) -> String {
    let krate = e.module(db).krate(db);
    let krate_name = krate
        .display_name(db)
        .map(|d| d.canonical_name().to_string())
        .unwrap_or_else(|| "?".to_string());
    format!("{krate_name}::...::{}", e.name(db).as_str())
}

fn module_def_kind(def: &ModuleDef) -> &'static str {
    match def {
        ModuleDef::Module(_) => "module",
        ModuleDef::Function(_) => "function",
        ModuleDef::Adt(Adt::Struct(_)) => "struct",
        ModuleDef::Adt(Adt::Union(_)) => "union",
        ModuleDef::Adt(Adt::Enum(_)) => "enum",
        ModuleDef::EnumVariant(_) => "enum variant",
        ModuleDef::Const(_) => "const",
        ModuleDef::Static(_) => "static",
        ModuleDef::Trait(_) => "trait",
        ModuleDef::TypeAlias(_) => "type alias",
        ModuleDef::BuiltinType(_) => "builtin type",
        ModuleDef::Macro(_) => "macro",
    }
}
