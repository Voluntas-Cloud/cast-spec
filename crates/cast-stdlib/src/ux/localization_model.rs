//! `localization_model` — support language/region differences.

/// Sentinel for `localization_model`.
pub struct LocalizationModel;

cast::concept! {
    name: "localization_model",
    summary: "Support language and region differences as a model, not \
              as a string-replace pass on the way out. Pluralisation, \
              date formats, and right-to-left text are not a `tr()` \
              function's job to figure out.",
    anchors: [cast_stdlib::ux::localization_model::LocalizationModel],
    tags: ["cast_stdlib", "ux"],
}
