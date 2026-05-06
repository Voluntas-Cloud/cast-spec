//! `iommu_memory_protection` — restrict device DMA access.

/// Sentinel for `iommu_memory_protection`.
pub struct IommuMemoryProtection;

cast::concept! {
    name: "iommu_memory_protection",
    summary: "restrict device DMA access.",
    anchors: [cast_os_stdlib::memory_management::iommu_memory_protection::IommuMemoryProtection],
    tags: ["cast_os_stdlib", "memory_management"],
}
