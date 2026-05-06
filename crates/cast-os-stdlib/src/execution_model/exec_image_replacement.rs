//! `exec_image_replacement` — replace current process image.

/// Sentinel for `exec_image_replacement`.
pub struct ExecImageReplacement;

cast::concept! {
    name: "exec_image_replacement",
    summary: "replace current process image.",
    anchors: [cast_os_stdlib::execution_model::exec_image_replacement::ExecImageReplacement],
    tags: ["cast_os_stdlib", "execution_model"],
}
