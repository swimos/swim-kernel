use crate::{cchar, int, long, ulong};

// ISO/IEC 9899
// 7.23 Date and time

pub type clock_t = long;
pub type clockid_t = i32;
pub type time_t = long;

pub struct tm {
    pub tm_sec: int,
    pub tm_min: int,
    pub tm_hour: int,
    pub tm_mday: int,
    pub tm_mon: int,
    pub tm_year: int,
    pub tm_wday: int,
    pub tm_yday: int,
    pub tm_isdst: int,
    pub tm_gmtoff: long,
    pub tm_zone: *const cchar,
}

#[repr(C)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: long,
}

pub const CLOCK_REALTIME: clockid_t = 0;
pub const CLOCK_MONOTONIC: clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: clockid_t = 2;
pub const CLOCK_THREAD_CPUTIME_ID: clockid_t = 3;
