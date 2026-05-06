//! `output_encoding` — prevent injection in rendered output.

/// Sentinel for `output_encoding`.
pub struct OutputEncoding;

cast::concept! {
    name: "output_encoding",
    summary: "Prevent injection in rendered output. Data is encoded for \
              the destination context (HTML, SQL, shell, log) at the \
              point of rendering; sanitising at input time guesses wrong \
              for at least one of those.",
    anchors: [cast_stdlib::security::output_encoding::OutputEncoding],
    tags: ["cast_stdlib", "security"],
}
