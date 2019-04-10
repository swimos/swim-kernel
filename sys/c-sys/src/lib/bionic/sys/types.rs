use crate::{uint, ulong};

// POSIX.1

pub type blkcnt_t = ulong;
pub type blksize_t = ulong;
pub type dev_t = ulong;
pub type fsblkcnt_t = ulong;
pub type fsfilcnt_t = ulong;
pub type gid_t = u32;
pub type id_t = uint;
pub type ino_t = ulong;
pub type key_t = int;
#[cfg(target_arch = "aarch64")] pub type mode_t = u32;
#[cfg(target_arch = "arm")]     pub type mode_t = u16;
#[cfg(target_arch = "x86")]     pub type mode_t = u16;
#[cfg(target_arch = "x86_64")]  pub type mode_t = u32;
pub type nlink_t = u32;
pub type off_t = long;
pub type pid_t = i32;
pub type suseconds_t = long;
pub type uid_t = u32;
pub type useconds_t = u32;
#[cfg(target_arch = "aarch64")] pub type wchar_t = u32;
#[cfg(target_arch = "arm")]     pub type wchar_t = u32;
#[cfg(target_arch = "x86")]     pub type wchar_t = i32;
#[cfg(target_arch = "x86_64")]  pub type wchar_t = i32;
