use crate::{uint, ulong};

// POSIX.1

#[cfg(target_pointer_width = "32")] pub type blkcnt_t = i32;
#[cfg(target_pointer_width = "64")] pub type blkcnt_t = i64;
#[cfg(target_arch = "aarch64")] pub type blksize_t = i32;
#[cfg(target_arch = "arm")]     pub type blksize_t = i32;
#[cfg(target_arch = "x86")]     pub type blksize_t = i32;
#[cfg(target_arch = "x86_64")]  pub type blksize_t = i64;
pub type dev_t = u64;
#[cfg(target_pointer_width = "32")] pub type fsblkcnt_t = ulong;
#[cfg(target_pointer_width = "64")] pub type fsblkcnt_t = u64;
#[cfg(target_pointer_width = "32")] pub type fsfilcnt_t = ulong;
#[cfg(target_pointer_width = "64")] pub type fsfilcnt_t = u64;
pub type gid_t = u32;
pub type id_t = uint;
#[cfg(target_pointer_width = "32")] pub type ino_t = u32;
#[cfg(target_pointer_width = "64")] pub type ino_t = u64;
pub type key_t = int;
pub type mode_t = u32;
#[cfg(target_arch = "aarch64")] pub type nlink_t = u32;
#[cfg(target_arch = "arm")]     pub type nlink_t = u32;
#[cfg(target_arch = "x86")]     pub type nlink_t = u32;
#[cfg(target_arch = "x86_64")]  pub type nlink_t = u64;
#[cfg(target_pointer_width = "32")] pub type off_t = i32;
#[cfg(target_pointer_width = "64")] pub type off_t = i64;
pub type pid_t = i32;
#[cfg(target_pointer_width = "32")] pub type suseconds_t = i32;
#[cfg(target_pointer_width = "64")] pub type suseconds_t = i64;
pub type uid_t = u32;
pub type useconds_t = u32;
pub type wchar_t = i32;
