//! `connector_pattern` — encapsulated integration component.

/// Sentinel for `connector_pattern`.
pub struct ConnectorPattern;

cast::concept! {
    name: "connector_pattern",
    summary: "An integration packaged as a self-contained \
              component: config, credentials, schema, error handling \
              all together. New integrations slot in without \
              rewiring the core; failing ones can be unplugged.",
    anchors: [cast_stdlib::integration::connector_pattern::ConnectorPattern],
    tags: ["cast_stdlib", "integration"],
}
