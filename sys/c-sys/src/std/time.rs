use crate::{cchar, double, size_t};

// 7.23 Date and time

pub use crate::lib::time::*;

extern "C" {
    // 7.23.2 Time manipulation functions
    #[cfg_attr(target_arch = "x86", link_name = "clock$UNIX2003")]
    pub fn clock() -> clock_t;
    pub fn difftime(time1: time_t, time0: time_t) -> double;
    #[cfg_attr(target_arch = "x86", link_name = "mktime$UNIX2003")]
    pub fn mktime(timeptr: *mut tm) -> time_t;
    pub fn time(timer: *mut time_t) -> time_t;

    // 7.23.3 Time conversion functions
    pub fn asctime(timeptr: *const tm) -> *mut cchar;
    pub fn ctime(timer: *const time_t) -> *mut cchar;
    pub fn gmtime(timer: *const time_t) -> *mut tm;
    pub fn localtime(timer: *const time_t) -> *mut tm;
    #[cfg_attr(target_arch = "x86", link_name = "strftime$UNIX2003")]
    pub fn strftime(s: *mut cchar, maxsize: size_t, format: *const cchar, timeptr: *const tm) -> size_t;
}
