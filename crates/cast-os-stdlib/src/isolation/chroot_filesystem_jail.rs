//! `chroot_filesystem_jail` — change perceived filesystem root.

/// Sentinel for `chroot_filesystem_jail`.
pub struct ChrootFilesystemJail;

cast::concept! {
    name: "chroot_filesystem_jail",
    summary: "change perceived filesystem root.",
    anchors: [cast_os_stdlib::isolation::chroot_filesystem_jail::ChrootFilesystemJail],
    tags: ["cast_os_stdlib", "isolation"],
}
