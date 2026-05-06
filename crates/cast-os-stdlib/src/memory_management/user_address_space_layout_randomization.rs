//! `user_address_space_layout_randomization` — randomize user memory layout.

/// Sentinel for `user_address_space_layout_randomization`.
pub struct UserAddressSpaceLayoutRandomization;

cast::concept! {
    name: "user_address_space_layout_randomization",
    summary: "randomize user memory layout.",
    anchors: [cast_os_stdlib::memory_management::user_address_space_layout_randomization::UserAddressSpaceLayoutRandomization],
    tags: ["cast_os_stdlib", "memory_management"],
}
