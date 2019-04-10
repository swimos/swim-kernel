use crate::{int, void};

// POSIX.1

pub const PROT_NONE: int = 0x0;
pub const PROT_READ: int = 0x1;
pub const PROT_WRITE: int = 0x2;
pub const PROT_EXEC: int = 0x4;

pub const MAP_FILE: int = 0;
pub const MAP_SHARED: int = 0x01;
pub const MAP_PRIVATE: int = 0x02;
pub const MAP_FIXED: int = 0x10;
pub const MAP_ANONYMOUS: int = 0x20;

pub const MCL_CURRENT: int = 1;
pub const MCL_FUTURE: int = 2;
pub const MCL_ONFAULT: int = 4;

pub const MAP_FAILED: *mut void = !0 as *mut void;

pub const MS_ASYNC: int = 1;
pub const MS_INVALIDATE: int = 2;
pub const MS_SYNC: int = 4;

pub const MADV_NORMAL: int = 0;
pub const MADV_RANDOM: int = 1;
pub const MADV_SEQUENTIAL: int = 2;
pub const MADV_WILLNEED: int = 3;
pub const MADV_DONTNEED: int = 4;
pub const MADV_FREE: int = 8;
pub const MADV_REMOVE: int = 9;
pub const MADV_DONTFORK: int = 10;
pub const MADV_DOFORK: int = 11;
pub const MADV_HWPOISON: int = 100;
pub const MADV_SOFT_OFFLINE: int = 101;
pub const MADV_MERGEABLE: int = 12;
pub const MADV_UNMERGEABLE: int = 13;
pub const MADV_HUGEPAGE: int = 14;
pub const MADV_NOHUGEPAGE: int = 15;
pub const MADV_DONTDUMP: int = 16;
pub const MADV_DODUMP: int = 17;
pub const MADV_WIPEONFORK: int = 18;
pub const MADV_KEEPONFORK: int = 19;
