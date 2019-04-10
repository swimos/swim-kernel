use crate::{cchar, int, uint, long, ulong, longlong, ulonglong, float, double, size_t, void};

// 7.20 General utilities

pub use crate::lib::stdlib::*;

extern "C" {
    // 7.20.1 Numeric conversion functions
    pub fn atof(nptr: *const cchar) -> double;
    pub fn atoi(nptr: *const cchar) -> int;
    pub fn atol(nptr: *const cchar) -> long;
    pub fn atoll(nptr: *const cchar) -> longlong;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "strtod$UNIX2003")]
    pub fn strtod(nptr: *const cchar, endptr: *mut *mut cchar) -> double;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "strtof$UNIX2003")]
    pub fn strtof(nptr: *const cchar, endptr: *mut *mut cchar) -> float;
    pub fn strtol(nptr: *const cchar, endptr: *mut *mut cchar, base: int) -> long;
    pub fn strtoll(nptr: *const cchar, endptr: *mut *mut cchar, base: int) -> longlong;
    pub fn strtoul(nptr: *const cchar, endptr: *mut *mut cchar, base: int) -> ulong;
    pub fn strtoull(nptr: *const cchar, endptr: *mut *mut cchar, base: int) -> ulonglong;

    // 7.20.2 Pseudo-random sequence generation functions
    pub fn rand() -> int;
    pub fn srand(seed: uint);

    // 7.20.3 Memory management functions
    pub fn calloc(nmemb: size_t, size: size_t) -> *mut void;
    pub fn free(ptr: *mut void);
    pub fn malloc(size: size_t) -> *mut void;
    pub fn realloc(ptr: *mut void, size: size_t) -> *mut void;

    // 7.20.4 Communication with the environment
    pub fn abort() -> !;
    pub fn atexit(func: extern fn()) -> int;
    pub fn exit(status: int) -> !;
    pub fn _Exit(status: int) -> !;
    pub fn getenv(name: *const cchar) -> *mut cchar;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "system$UNIX2003")]
    pub fn system(string: *const cchar) -> int;
}
