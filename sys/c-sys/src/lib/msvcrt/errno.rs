use crate::int;

// ISO/IEC 9899
// 7.5 Errors

pub const EPERM: int = 1;
pub const ENOENT: int = 2;
pub const ESRCH: int = 3;
pub const EINTR: int = 4;
pub const EIO: int = 5;
pub const ENXIO: int = 6;
pub const E2BIG: int = 7;
pub const ENOEXEC: int = 8;
pub const EBADF: int = 9;
pub const ECHILD: int = 10;
pub const EAGAIN: int = 11;
pub const ENOMEM: int = 12;
pub const EACCES: int = 13;
pub const EFAULT: int = 14;
pub const EBUSY: int = 16;
pub const EEXIST: int = 17;
pub const EXDEV: int = 18;
pub const ENODEV: int = 19;
pub const ENOTDIR: int = 20;
pub const EISDIR: int = 21;
pub const EINVAL: int = 22;
pub const ENFILE: int = 23;
pub const EMFILE: int = 24;
pub const ENOTTY: int = 25;
pub const EFBIG: int = 27;
pub const ENOSPC: int = 28;
pub const ESPIPE: int = 29;
pub const EROFS: int = 30;
pub const EMLINK: int = 31;
pub const EPIPE: int = 32;
pub const EDOM: int = 33;
pub const ERANGE: int = 34;
pub const EDEADLK: int = 36;
pub const EDEADLOCK: int = 36;
pub const ENAMETOOLONG: int = 38;
pub const ENOLCK: int = 39;
pub const ENOSYS: int = 40;
pub const ENOTEMPTY: int = 41;
pub const EILSEQ: int = 42;
