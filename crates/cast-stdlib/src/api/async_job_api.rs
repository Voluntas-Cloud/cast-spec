//! `async_job_api` — request starts a job, caller polls or subscribes.

/// Sentinel for `async_job_api`.
pub struct AsyncJobApi;

cast::concept! {
    name: "async_job_api",
    summary: "Request starts a job and returns handle. Caller polls or \
              subscribes to the handle for completion.",
    anchors: [cast_stdlib::api::async_job_api::AsyncJobApi],
    tags: ["cast_stdlib", "api"],
}
