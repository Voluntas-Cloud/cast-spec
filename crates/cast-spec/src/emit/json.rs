//! JSON rendering of a `Report`. Uses serde_json's pretty printer so
//! the output diff-reads cleanly in PR review and `jq` consumption.

use super::model::Report;

pub fn render(report: &Report) -> String {
    // `to_string_pretty` is infallible on a `#[derive(Serialize)]` value
    // whose fields are all `Serialize` — no custom `Serialize` impls to
    // panic on. The unwrap is therefore total.
    serde_json::to_string_pretty(report).expect("Report is always serializable")
}

