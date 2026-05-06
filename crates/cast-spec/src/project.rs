//! Loads a Voluntas-style multi-crate project into rust-analyzer's
//! analysis database.
//!
//! Voluntas has no top-level Cargo workspace — each Rust subtree is its
//! own project. `ra_ap_load_cargo::load_workspace_at` accepts a path
//! and walks dependency graphs, so calling it once per discovered
//! Cargo.toml gives us full coverage. The handles are kept alive for
//! the lifetime of the analysis run.
//!
//! The first compile of this file will almost certainly surface API
//! drift against snapshot 0.0.330 — that is expected and fine. We
//! correct against the actual published surface, not against my
//! recollection of it.

use anyhow::Result;
use ra_ap_ide_db::RootDatabase;
use ra_ap_load_cargo::{LoadCargoConfig, ProcMacroServerChoice};
use ra_ap_vfs::Vfs;
use std::path::{Path, PathBuf};
use std::process::Command;

cast::concept! {
    name: "block_walk_up",
    summary: "rust-analyzer's load_workspace_at calls `cargo \
              locate-project --workspace --manifest-path X` to find the \
              enclosing workspace, which always walks up to the parent \
              workspace root. assert_standalone_root detects this case \
              by running `cargo locate-project` twice (with and without \
              `--workspace`) and comparing the manifest paths. CLIs \
              expose this behind `--no-walk-up` for callers who want to \
              isolate analysis to a single crate — pointing cast-watch \
              at a sub-crate of a larger workspace would otherwise \
              quietly load the entire parent.",
    anchors: [
        crate::project::assert_standalone_root,
    ],
    tags: ["cast_grammar"],
}

cast::rule! {
    rule: "When `--no-walk-up` is requested, the loader must refuse to \
           proceed if the target Cargo.toml is a workspace member of an \
           enclosing workspace. The error message must name the parent \
           workspace's manifest path so the user can decide whether to \
           re-target cast at the parent or drop the flag.",
    why:  "Silent walk-up turns `cast-watch path/to/sub-crate` into \
           `cast-watch path/to/whole-workspace` without a visible \
           signal — the user sees CPU spin on a tree they didn't ask \
           for and a snapshot full of unrelated invocations. Refusing \
           with a path-named error makes the surprise impossible.",
    governs: [
        crate::project::assert_standalone_root,
    ],
    tags: ["cast_grammar"],
}

cast::concept! {
    name: "analysis_working_set",
    summary: "What's resident in RAM during a cast-extract / cast-watch \
              cold load. The simultaneous-residency cost is dominated \
              by rust-analyzer's RootDatabase per loaded project: each \
              ProjectHandle in MultiProject carries a RootDatabase + \
              Vfs that together sit in the GB range for medium \
              workspaces. load_projects today loads every handle \
              up-front, and MultiProject keeps them alive across the \
              entire run_multi_root_analysis pass — peak residency is \
              N (handle DBs) + N (per-handle Reports) + 1 (merged \
              Report), all coexisting before any are dropped. The \
              diagnostic insight from the May 2026 voluntas OOM: a \
              single-workspace invocation lands at ~3.7 GB for three \
              standalone Rust leaves, but an N-root invocation against \
              the same tree (one path per Cargo.toml in the voluntas \
              shape) blows past 8 GB by holding N independent RA \
              databases concurrently. Multiplicity is the cost driver, \
              not raw residency.",
    anchors: [
        crate::project::MultiProject,
        crate::project::load_projects,
        crate::project::ProjectHandle,
        CAST::AS_PRIMITIVE::crate::run_multi_root_analysis,
    ],
    tags: ["cast_spec", "residency"],
}

cast::anti_pattern! {
    avoid:      "Holding every loaded ProjectHandle's RootDatabase \
                 alive until the merged Report is built. With N \
                 project roots each at 1-2 GB of RA index, peak \
                 residency scales as N × per-DB cost concurrently — \
                 and on a voluntas-shaped tree (no toplevel workspace, \
                 one path per Cargo.toml subtree) that's the path \
                 that OOMs the host.",
    why:        "rust-analyzer's RootDatabase is the heaviest object \
                 in the analyzer; each one allocates GB-scale name- \
                 resolution caches and syntax trees. Keeping them all \
                 resident through the entire pipeline only matters if \
                 a later pass needs them — but run_multi_root_analysis's \
                 per-handle pass produces a Report and never touches \
                 that handle's DB again. The handle could be dropped \
                 the moment its Report is built; it isn't, today, and \
                 that's the structural reason multi-root invocations \
                 OOM where single-root invocations don't.",
    instead:    "Drain handles one at a time inside \
                 run_multi_root_analysis: load handle N, build its \
                 Report under attach_db, drop the RootDatabase + Vfs, \
                 accumulate just the Report, repeat for N+1. Peak \
                 residency becomes max(per-handle cost) + accumulated \
                 Reports, not N × per-handle cost. Cross-workspace \
                 anchor resolution (validator's job today) needs \
                 reworking — either finalize anchors per-handle \
                 without cross-handle fallback, or defer unresolved \
                 anchors to a second pass that loads only the foreign \
                 handle each one needs.",
    governs: [
        crate::project::load_projects,
        crate::run_multi_root_analysis,
    ],
    tags: ["cast_spec", "residency"],
}

/// Detect whether `root`'s Cargo.toml is a member of an enclosing
/// workspace. Returns `Ok(())` when the manifest at `root` is its own
/// workspace root (or has no enclosing workspace at all). Returns
/// `Err` with the parent workspace's manifest path when there is a
/// mismatch — that's the actionable signal for the CLIs.
///
/// Runs two `cargo locate-project` invocations; cargo is the source of
/// truth for what the workspace root is, so we trust its answer rather
/// than parsing TOML ourselves.
pub fn assert_standalone_root(root: &Path) -> Result<()> {
    let manifest = root.join("Cargo.toml");
    if !manifest.exists() {
        anyhow::bail!(
            "no Cargo.toml at {} — cast-watch / cast-extract need a Rust project root",
            root.display()
        );
    }

    let package_manifest = locate_project(&manifest, false)?;
    let workspace_manifest = locate_project(&manifest, true)?;

    if package_manifest != workspace_manifest {
        anyhow::bail!(
            "{} is a member of the workspace at {}.\n\
             `--no-walk-up` refuses this — drop the flag to analyse the whole workspace, or point cast at {} directly.",
            package_manifest.display(),
            workspace_manifest.display(),
            workspace_manifest.parent().map(|p| p.display().to_string()).unwrap_or_default(),
        );
    }
    Ok(())
}

fn locate_project(manifest: &Path, workspace: bool) -> Result<PathBuf> {
    let mut cmd = Command::new("cargo");
    cmd.arg("locate-project");
    if workspace {
        cmd.arg("--workspace");
    }
    cmd.arg("--message-format").arg("plain");
    cmd.arg("--manifest-path").arg(manifest);

    let output = cmd
        .output()
        .map_err(|e| anyhow::anyhow!("failed to run `cargo locate-project`: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("`cargo locate-project` failed: {}", stderr.trim());
    }
    let stdout = String::from_utf8(output.stdout)?;
    Ok(PathBuf::from(stdout.trim()))
}

/// One loaded project. Owns the analysis DB and the VFS that backs it.
/// Both must stay alive for the duration of any `Semantics` query.
pub struct ProjectHandle {
    pub db: RootDatabase,
    pub vfs: Vfs,
}

pub fn load_project(project_root: &Path) -> Result<ProjectHandle> {
    let config = LoadCargoConfig {
        load_out_dirs_from_check: false,
        with_proc_macro_server: ProcMacroServerChoice::None,
        prefill_caches: false,
        num_worker_threads: 1,
        proc_macro_processes: 1,
    };

    // `all_targets` includes integration tests, benches, examples — i.e.
    // every place a user might write a `cast::*!` invocation. Without it,
    // tests/smoke.rs is invisible.
    //
    // `set_test` puts `cfg(test)` on the test crates so any `#[cfg(test)]`
    // gating in the surrounding code is active during analysis.
    let cargo_config = ra_ap_project_model::CargoConfig {
        all_targets: true,
        set_test: true,
        ..Default::default()
    };
    let progress = &|_msg: String| {};

    let (db, vfs, _proc_macro) = ra_ap_load_cargo::load_workspace_at(
        project_root,
        &cargo_config,
        &config,
        progress,
    )?;

    Ok(ProjectHandle { db, vfs })
}

/// Multi-workspace bundle: N independently loaded `ProjectHandle`s
/// plus a parallel index of their canonicalized roots, so a source
/// file path can be mapped back to the handle that owns it.
///
/// `roots[i]` is the canonical project root for `handles[i]`. We keep
/// the parallel-vector form rather than a `HashMap<PathBuf, usize>`
/// because the lookup operation we actually need is **longest prefix
/// match**, not exact equality — a file path is owned by the deepest
/// project root that contains it.
pub struct MultiProject {
    pub handles: Vec<ProjectHandle>,
    pub roots: Vec<PathBuf>,
}

/// Load every project root in `roots` and return them bundled as a
/// `MultiProject`. Each root is canonicalized at load time so later
/// `handle_for_file` lookups can compare canonicalized paths directly.
pub fn load_projects(roots: &[PathBuf]) -> Result<MultiProject> {
    let mut handles = Vec::with_capacity(roots.len());
    let mut canonical = Vec::with_capacity(roots.len());
    for root in roots {
        handles.push(load_project(root)?);
        canonical.push(root.canonicalize()?);
    }
    Ok(MultiProject {
        handles,
        roots: canonical,
    })
}

/// Return the index of the handle whose project root is the deepest
/// ancestor of `file`. Returns `None` if `file` is outside every
/// loaded project.
///
/// Longest-prefix wins: when project roots nest (e.g. `/repo` and
/// `/repo/sub`), a file under `/repo/sub` belongs to the inner handle.
pub fn handle_for_file(multi: &MultiProject, file: &Path) -> Option<usize> {
    let canonical = file.canonicalize().ok()?;
    multi
        .roots
        .iter()
        .enumerate()
        .filter(|(_, root)| canonical.starts_with(root))
        .max_by_key(|(_, root)| root.components().count())
        .map(|(idx, _)| idx)
}

/// Find the repository root by walking up from `start` until we hit a
/// `.git` directory (or worktree file). Used to anchor `target:` paths
/// in `cast::io::continues_in!` — those are repo-relative, but the
/// project we hand to `load_project` is a sub-tree of the repo.
pub fn find_repo_root(start: &Path) -> Option<PathBuf> {
    let mut cur: PathBuf = start.canonicalize().ok()?;
    if cur.is_file() {
        cur = cur.parent()?.to_path_buf();
    }
    loop {
        if cur.join(".git").exists() {
            return Some(cur);
        }
        cur = cur.parent()?.to_path_buf();
    }
}

cast::concept! {
    name: "cargo_workspace_loader",
    summary: "Loads one or more Cargo workspaces into rust-analyzer \
              ProjectHandles. Each load_project call shells out to \
              cargo (locate-project, metadata) and constructs a \
              RootDatabase + Vfs from the returned manifest set. \
              Non-deterministic: depends on filesystem state, \
              Cargo.lock contents, the current toolchain, and the \
              cargo subprocess outcome.",
    anchors: [
        crate::project::load_project,
        crate::project::load_projects,
        crate::project::locate_project,
    ],
    tags: ["cast_spec_project"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "cargo_workspace_loader",
    why: "Filesystem state and the cargo subprocess both participate; \
          the same (project_root) input can produce different \
          ProjectHandle contents across calls if Cargo.toml or the \
          source tree changed.",
}

cast::concept! {
    name: "repo_path_resolver",
    summary: "Walks the filesystem to map a path into the loaded \
              MultiProject and to find the enclosing repo root. \
              find_repo_root canonicalises and walks up looking for \
              .git; handle_for_file looks the path up across every \
              ProjectHandle's Vfs. Non-deterministic on the \
              filesystem: a `git init` or a moved file flips the \
              outcome.",
    anchors: [
        crate::project::find_repo_root,
        crate::project::handle_for_file,
    ],
    tags: ["cast_spec_project"],
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "repo_path_resolver",
    why: "Filesystem state at call time decides the outcome.",
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "block_walk_up",
    why: "assert_standalone_root shells out to `cargo locate-project` \
          twice and compares manifest paths — both subprocess invocations \
          read filesystem state at call time.",
}

cast::continues_in! {
    target: cast_stdlib::function_properties::non_deterministic,
    concept: "analysis_working_set",
    why: "The working set is the residency of loaded ProjectHandles; \
          each ProjectHandle is constructed from a non-deterministic \
          load_project call. The set itself reflects whatever \
          filesystem + cargo state held at load time.",
}

cast::continues_in! {
    target: cast_stdlib::type_properties::resource_handle,
    concept: "analysis_working_set",
    why: "MultiProject and ProjectHandle wrap a ra_ap_ide RootDatabase \
          + Vfs — heavyweight external resources whose identity is \
          the underlying database. Not cleanly cloneable or \
          serialisable.",
}

cast::continues_in! {
    target: cast_stdlib::integration::adapter_integration,
    concept: "cargo_workspace_loader",
    why: lazy,
}

cast::continues_in! {
    target: cast_stdlib::integration::adapter_integration,
    concept: "repo_path_resolver",
    why: lazy,
}
