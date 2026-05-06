//! `package_manager_model` — install/update software from packages.

/// Sentinel for `package_manager_model`.
pub struct PackageManagerModel;

cast::concept! {
    name: "package_manager_model",
    summary: "install/update software from packages.",
    anchors: [cast_os_stdlib::update_evolution::package_manager_model::PackageManagerModel],
    tags: ["cast_os_stdlib", "update_evolution"],
}
