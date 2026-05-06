//! `import_pipeline` — bring external data into system.

/// Sentinel for `import_pipeline`.
pub struct ImportPipeline;

cast::concept! {
    name: "import_pipeline",
    summary: "A defined path for bringing external data into the \
              system: validate, normalise, persist, observe. \
              Imports without a pipeline become \"someone ran a \
              script that one time\".",
    anchors: [cast_stdlib::integration::import_pipeline::ImportPipeline],
    tags: ["cast_stdlib", "integration"],
}
