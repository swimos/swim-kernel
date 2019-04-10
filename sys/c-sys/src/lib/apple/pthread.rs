#![allow(non_upper_case_globals)]

use crate::{cchar, int, long, ulong, void};

// POSIX.1

#[cfg(target_pointer_width = "32")] const __PTHREAD_ATTR_SIZE__: usize = 36;
#[cfg(target_pointer_width = "64")] const __PTHREAD_ATTR_SIZE__: usize = 56;
#[cfg(target_pointer_width = "32")] const __PTHREAD_COND_SIZE__: usize = 24;
#[cfg(target_pointer_width = "64")] const __PTHREAD_COND_SIZE__: usize = 40;
#[cfg(target_pointer_width = "32")] const __PTHREAD_CONDATTR_SIZE__: usize = 4;
#[cfg(target_pointer_width = "64")] const __PTHREAD_CONDATTR_SIZE__: usize = 8;
#[cfg(target_pointer_width = "32")] const __PTHREAD_MUTEX_SIZE__: usize = 40;
#[cfg(target_pointer_width = "64")] const __PTHREAD_MUTEX_SIZE__: usize = 56;
#[cfg(target_pointer_width = "32")] const __PTHREAD_MUTEXATTR_SIZE__: usize = 8;
#[cfg(target_pointer_width = "64")] const __PTHREAD_MUTEXATTR_SIZE__: usize = 8;
#[cfg(target_pointer_width = "32")] const __PTHREAD_ONCE_SIZE__: usize = 4;
#[cfg(target_pointer_width = "64")] const __PTHREAD_ONCE_SIZE__: usize = 8;
#[cfg(target_pointer_width = "32")] const __PTHREAD_RWLOCK_SIZE__: usize = 124;
#[cfg(target_pointer_width = "64")] const __PTHREAD_RWLOCK_SIZE__: usize = 192;
#[cfg(target_pointer_width = "32")] const __PTHREAD_RWLOCKATTR_SIZE__: usize = 12;
#[cfg(target_pointer_width = "64")] const __PTHREAD_RWLOCKATTR_SIZE__: usize = 16;
#[cfg(target_pointer_width = "32")] const __PTHREAD_SIZE__: usize = 4088;
#[cfg(target_pointer_width = "64")] const __PTHREAD_SIZE__: usize = 8176;

const _PTHREAD_MUTEX_SIG_init: long = 0x32AAABA7;

const _PTHREAD_ERRORCHECK_MUTEX_SIG_init: long = 0x32AAABA1;
const _PTHREAD_RECURSIVE_MUTEX_SIG_init: long = 0x32AAABA2;
const _PTHREAD_FIRSTFIT_MUTEX_SIG_init: long = 0x32AAABA3;

const _PTHREAD_COND_SIG_init: long = 0x3CB0B1BB;
const _PTHREAD_ONCE_SIG_init: long = 0x30B1BCBA;
const _PTHREAD_RWLOCK_SIG_init: long = 0x2DA8B3B4;

pub const SCHED_OTHER: int = 1;
pub const SCHED_FIFO: int = 4;
pub const SCHED_RR: int = 2;

const __SCHED_PARAM_SIZE__: usize = 4;

#[repr(C)]
pub struct pthread_attr_t {
    __sig: long,
    __opaque: [cchar; __PTHREAD_ATTR_SIZE__],
}

#[repr(C)]
pub struct pthread_cond_t {
    __sig: long,
    __opaque: [cchar; __PTHREAD_COND_SIZE__],
}

#[repr(C)]
pub struct pthread_condattr_t {
    __sig: long,
    __opaque: [cchar; __PTHREAD_CONDATTR_SIZE__],
}

pub type pthread_key_t = ulong;

#[repr(C)]
pub struct pthread_mutex_t {
    __sig: long,
    __opaque: [cchar; __PTHREAD_MUTEX_SIZE__],
}

#[repr(C)]
pub struct pthread_mutexattr_t {
    __sig: long,
    __opaque: [cchar; __PTHREAD_MUTEXATTR_SIZE__],
}

#[repr(C)]
pub struct pthread_once_t {
    __sig: long,
    __opaque: [cchar; __PTHREAD_ONCE_SIZE__],
}

#[repr(C)]
pub struct pthread_rwlock_t {
    __sig: long,
    __opaque: [cchar; __PTHREAD_RWLOCK_SIZE__],
}

#[repr(C)]
pub struct pthread_rwlockattr_t {
    __sig: long,
    __opaque: [cchar; __PTHREAD_RWLOCKATTR_SIZE__],
}

pub type pthread_t = *mut void;

#[repr(C)]
pub struct sched_param {
    sched_priority: int,
    __opaque: [cchar; __SCHED_PARAM_SIZE__],
}

pub const PTHREAD_CREATE_JOINABLE: int = 1;
pub const PTHREAD_CREATE_DETACHED: int = 2;

pub const PTHREAD_INHERIT_SCHED: int = 1;
pub const PTHREAD_EXPLICIT_SCHED: int = 2;

pub const PTHREAD_CANCEL_ENABLE: int = 0x01;
pub const PTHREAD_CANCEL_DISABLE: int = 0x00;
pub const PTHREAD_CANCEL_DEFERRED: int = 0x02;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: int = 0x00;

pub const PTHREAD_CANCELED: *mut void = 1 as *mut void;

pub const PTHREAD_SCOPE_SYSTEM: int = 1;
pub const PTHREAD_SCOPE_PROCESS: int = 2;

pub const PTHREAD_PROCESS_SHARED: int = 1;
pub const PTHREAD_PROCESS_PRIVATE: int = 2;

pub const PTHREAD_PRIO_NONE: int = 0;
pub const PTHREAD_PRIO_INHERIT: int = 1;
pub const PTHREAD_PRIO_PROTECT: int = 2;

pub const PTHREAD_MUTEX_NORMAL: int = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: int = 1;
pub const PTHREAD_MUTEX_RECURSIVE: int = 2;
pub const PTHREAD_MUTEX_DEFAULT: int = PTHREAD_MUTEX_NORMAL;

pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __sig: _PTHREAD_COND_SIG_init,
    __opaque: [0; __PTHREAD_COND_SIZE__],
};

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __sig: _PTHREAD_MUTEX_SIG_init,
    __opaque: [0; __PTHREAD_MUTEX_SIZE__],
};

pub const PTHREAD_ONCE_INIT: pthread_once_t = pthread_once_t {
    __sig: _PTHREAD_ONCE_SIG_init,
    __opaque: [0; __PTHREAD_ONCE_SIZE__],
};

pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __sig: _PTHREAD_RWLOCK_SIG_init,
    __opaque: [0; __PTHREAD_RWLOCK_SIZE__],
};

#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
pub const PTHREAD_STACK_MIN: usize = 16384;
#[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
pub const PTHREAD_STACK_MIN: usize = 8192;
