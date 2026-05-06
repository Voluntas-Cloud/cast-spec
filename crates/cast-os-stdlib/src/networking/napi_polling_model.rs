//! `napi_polling_model` — Linux hybrid interrupt/poll network processing.

/// Sentinel for `napi_polling_model`.
pub struct NapiPollingModel;

cast::concept! {
    name: "napi_polling_model",
    summary: "Linux hybrid interrupt/poll network processing.",
    anchors: [cast_os_stdlib::networking::napi_polling_model::NapiPollingModel],
    tags: ["cast_os_stdlib", "networking"],
}
