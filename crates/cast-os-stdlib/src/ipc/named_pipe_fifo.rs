//! `named_pipe_fifo` — filesystem-named pipe.

/// Sentinel for `named_pipe_fifo`.
pub struct NamedPipeFifo;

cast::concept! {
    name: "named_pipe_fifo",
    summary: "filesystem-named pipe.",
    anchors: [cast_os_stdlib::ipc::named_pipe_fifo::NamedPipeFifo],
    tags: ["cast_os_stdlib", "ipc"],
}
