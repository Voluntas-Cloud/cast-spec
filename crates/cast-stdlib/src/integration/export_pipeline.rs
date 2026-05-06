//! `export_pipeline` — send internal data outward.

/// Sentinel for `export_pipeline`.
pub struct ExportPipeline;

cast::concept! {
    name: "export_pipeline",
    summary: "A defined path for sending internal data outward: \
              filter, redact, format, deliver. The export pipeline \
              is where data classification rules earn their keep — \
              or quietly fail to.",
    anchors: [cast_stdlib::integration::export_pipeline::ExportPipeline],
    tags: ["cast_stdlib", "integration"],
}
