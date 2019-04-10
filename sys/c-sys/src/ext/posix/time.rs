use crate::{cchar, int};

pub use crate::std::time::*;

extern "C" {
    pub fn asctime_r(timeptr: *const tm, result: *mut cchar) -> *mut cchar;
    pub fn ctime_r(timer: *const time_t, result: *mut cchar) -> *mut cchar;
    pub fn gmtime_r(timer: *const time_t, result: *mut tm) -> *mut tm;
    pub fn localtime_r(timer: *const time_t, result: *mut tm) -> *mut tm;

    #[cfg_attr(target_arch = "x86", link_name = "nanosleep$UNIX2003")]
    pub fn nanosleep(rqtp: *const timespec, rmtp: *mut timespec) -> int;
}
