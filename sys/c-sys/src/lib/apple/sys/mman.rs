use crate::{int, void};

// POSIX.1

pub const PROT_NONE: int = 0x00;
pub const PROT_READ: int = 0x01;
pub const PROT_WRITE: int = 0x02;
pub const PROT_EXEC: int = 0x04;

pub const MAP_FILE: int = 0x0000;
pub const MAP_SHARED: int = 0x0001;
pub const MAP_PRIVATE: int = 0x0002;
pub const MAP_FIXED: int = 0x0010;
pub const MAP_RENAME: int = 0x0020;
pub const MAP_NORESERVE: int = 0x0040;
pub const MAP_RESERVED0080: int = 0x0080;
pub const MAP_NOEXTEND: int = 0x0100;
pub const MAP_HASSEMAPHORE: int = 0x0200;
pub const MAP_NOCACHE: int = 0x0400;
pub const MAP_JIT: int = 0x0800;
pub const MAP_ANON: int = 0x1000;
pub const MAP_ANONYMOUS: int = MAP_ANON;
pub const MAP_RESILIENT_CODESIGN: int = 0x2000;
pub const MAP_RESILIENT_MEDIA: int = 0x4000;

pub const MCL_CURRENT: int = 0x0001;
pub const MCL_FUTURE: int = 0x0002;

pub const MAP_FAILED: *mut void = !0 as *mut void;

pub const MS_ASYNC: int = 0x0001;
pub const MS_INVALIDATE: int = 0x0002;
pub const MS_SYNC: int = 0x0010;
pub const MS_KILLPAGES: int = 0x0004;
pub const MS_DEACTIVATE: int = 0x0008;

pub const MADV_NORMAL: int = 0;
pub const MADV_RANDOM: int = 1;
pub const MADV_SEQUENTIAL: int = 2;
pub const MADV_WILLNEED: int = 3;
pub const MADV_DONTNEED: int = 4;
pub const MADV_FREE: int = 5;
pub const MADV_ZERO_WIRED_PAGES: int = 6;
pub const MADV_FREE_REUSABLE: int = 7;
pub const MADV_FREE_REUSE: int = 8;
pub const MADV_CAN_REUSE: int = 9;
pub const MADV_PAGEOUT: int = 10;
