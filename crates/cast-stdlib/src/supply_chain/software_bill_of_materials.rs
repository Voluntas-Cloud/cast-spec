//! `software_bill_of_materials` — list of included components.

/// Sentinel for `software_bill_of_materials`.
pub struct SoftwareBillOfMaterials;

cast::concept! {
    name: "software_bill_of_materials",
    summary: "List of included components. Every dependency at every \
              version that ended up in the artifact; the substrate \
              for vulnerability scans, license audits, and supply-\
              chain reasoning.",
    anchors: [cast_stdlib::supply_chain::software_bill_of_materials::SoftwareBillOfMaterials],
    tags: ["cast_stdlib", "supply_chain"],
}
