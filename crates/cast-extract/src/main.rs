//! CLI: load N project roots, run the multi-root analysis pipeline,
//! emit a `Report` in the requested format (text / json / markdown).
//!
//! All analysis lives in `cast` behind the `analysis` feature; main is
//! just argument parsing, repo-root resolution, and rendering.

use anyhow::Result;
use cast::emit::{json as emit_json, markdown as emit_md, text as emit_text, Format};
use cast::{find_repo_root, load_projects, run_multi_root_analysis};
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "cast-extract", about = "Extract cast::*! annotations from one or more Rust workspaces.")]
struct Cli {
    /// One or more project roots. Each is loaded as its own
    /// rust-analyzer DB; cross-workspace anchor resolution dispatches
    /// across them.
    #[arg(required = true, num_args = 1..)]
    projects: Vec<PathBuf>,
    /// Output format. `text` is the human-readable default; `json` is
    /// the structured machine format; `markdown` is PR-comment shaped.
    #[arg(long, default_value = "text")]
    emit: String,
    /// Refuse to silently walk up into an enclosing Cargo workspace.
    /// When set, cast-extract errors out if any project root is a
    /// member of a larger workspace.
    #[arg(long, default_value_t = false)]
    no_walk_up: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let format: Format = cli.emit.parse().map_err(anyhow::Error::msg)?;

    if cli.no_walk_up {
        for p in &cli.projects {
            cast::assert_standalone_root(p)?;
        }
    }

    for p in &cli.projects {
        eprintln!("loading {} ...", p.display());
    }
    let multi = load_projects(&cli.projects)?;

    // Repo root is anchored at the first project root: io targets are
    // repo-relative and the first arg is treated as the primary
    // workspace. Multi-repo io anchoring is a future stage.
    let primary = &cli.projects[0];
    let repo_root = find_repo_root(primary).unwrap_or_else(|| {
        eprintln!(
            "warning: no .git found above {}; resolving io target paths against project root",
            primary.display()
        );
        primary.clone()
    });

    let report = run_multi_root_analysis(&multi, &repo_root);

    let rendered = match format {
        Format::Text => emit_text::render(&report),
        Format::Json => emit_json::render(&report),
        Format::Markdown => emit_md::render(&report),
    };
    println!("{rendered}");

    // CI-fail (non-zero exit) on any failure across all workspaces.
    let s = &report.summary;
    if s.parse_errors > 0
        || s.unimplemented > 0
        || s.paths_unresolved > 0
        || s.io_unresolved > 0
        || s.pipeline_errors > 0
    {
        std::process::exit(1);
    }
    Ok(())
}

cast::concept! {
    name: "cast_extract_cli",
    summary: "One-shot CLI: parse args, load projects, run \
              multi-root analysis, render the chosen format, exit \
              with a non-zero code on any unresolved/error counts. \
              Thin wrapper over cast::run_multi_root_analysis.",
    anchors: [
        crate::main,
        crate::Cli,
    ],
    tags: ["cast_extract"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "cast_extract_cli",
    why: "Filesystem reads, cargo subprocess, stdout/stderr writes, \
          process exit code; all observable external state.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::product_type,
    concept: "cast_extract_cli",
    why: "Cli is a struct with all clap-parsed fields simultaneously \
          inhabited.",
}

cast::continues_in! {
    target: cast_stdlib::architecture::adapter_pattern,
    concept: "cast_extract_cli",
    why: lazy,
}
