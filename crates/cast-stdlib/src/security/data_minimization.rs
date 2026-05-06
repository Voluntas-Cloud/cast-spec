//! `data_minimization` — store only what is needed.

/// Sentinel for `data_minimization`.
pub struct DataMinimization;

cast::concept! {
    name: "data_minimization",
    summary: "Store only what is needed. Fields collected \"just in case\" \
              become tomorrow's breach disclosure; the cheapest data to \
              protect is the data the system never had.",
    anchors: [cast_stdlib::security::data_minimization::DataMinimization],
    tags: ["cast_stdlib", "security"],
}
