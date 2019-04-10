use crate::{cchar, int, size_t, void};
use crate::sys::types::{mode_t, off_t};

pub use crate::lib::sys::mman::*;

extern "C" {
    pub fn mlock(addr: *const void, len: size_t) -> int;
    pub fn mlockall(flags: int) -> int;
    pub fn mmap(addr: *mut void, len: size_t, prot: int, flags: int, fd: int, off: off_t) -> *mut void;
    pub fn mprotect(addr: *mut void, len: size_t, prot: int) -> int;
    pub fn msync(addr: *mut void, len: size_t, flags: int) -> int;
    pub fn munlock(addr: *const void, len: size_t) -> int;
    pub fn munlockall() -> int;
    pub fn munmap(addr: *mut void, len: size_t) -> int;
    pub fn shm_open(name: *const cchar, oflags: int, mode: mode_t) -> int;
    pub fn shm_unlink(name: *const cchar) -> int;
}
