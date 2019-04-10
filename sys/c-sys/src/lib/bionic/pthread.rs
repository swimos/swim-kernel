use crate::{cchar, long, size_t, void};
use crate::sys::types::{int32_t, uint32_t, int64_t};

#[repr(C)]
pub struct pthread_attr_t {
    flags: uint32_t,
    stack_base: *mut void,
    stack_size: size_t,
    guard_size: size_t,
    sched_policy: int32_t,
    sched_priority: int32_t,
    #[cfg(target_pointer_width = "64")]
    __reserved: [cchar; 16],
}

#[repr(C)]
pub struct pthread_barrier_t {
    #[cfg(target_pointer_width = "32")]
    __private: [int32_t; 8],
    #[cfg(target_pointer_width = "64")]
    __private: [int64_t; 4],
}

pub type pthread_barrierattr_t = int;

#[repr(C)]
pub struct pthread_cond_t {
    #[cfg(target_pointer_width = "32")]
    __private: [int32_t; 1],
    #[cfg(target_pointer_width = "64")]
    __private: [int32_t; 12],
}

pub type pthread_condattr_t = long;

pub type pthread_key_t = int;

#[repr(C)]
pub struct pthread_mutex_t {
    #[cfg(target_pointer_width = "32")]
    __private: [int32_t; 1],
    #[cfg(target_pointer_width = "64")]
    __private: [int32_t; 10],
}

pub type pthread_mutexattr_t = long;

pub type pthread_once_t = int;

#[repr(C)]
pub struct pthread_rwlock_t {
    #[cfg(target_pointer_width = "32")]
    __private: [int32_2; 10],
    #[cfg(target_pointer_width = "64")]
    __private: [int32_2; 14],
}

pub type pthread_rwlockattr_t = long;

pub type pthread_t = long;

#[repr(C)]
pub struct sched_param {
    sched_priority: int,
}

pub const PTHREAD_CREATE_JOINABLE: int = 0;
pub const PTHREAD_CREATE_DETACHED: int = 1;

pub const PTHREAD_EXPLICIT_SCHED: int = 0;
pub const PTHREAD_INHERIT_SCHED: int = 1;

pub const PTHREAD_SCOPE_SYSTEM: int = 0;
pub const PTHREAD_SCOPE_PROCESS: int = 1;

pub const PTHREAD_PROCESS_PRIVATE: int = 0;
pub const PTHREAD_PROCESS_SHARED: int = 1;

pub const PTHREAD_PRIO_NONE: int = 0;
pub const PTHREAD_PRIO_INHERIT: int = 1;

pub const PTHREAD_MUTEX_NORMAL: int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: int = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: int = 2;
pub const PTHREAD_MUTEX_DEFAULT: int = PTHREAD_MUTEX_NORMAL;

pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    #[cfg(target_pointer_width = "32")]
    __private: [0; 1],
    #[cfg(target_pointer_width = "64")]
    __private: [0; 12],
};

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    #[cfg(target_pointer_width = "32")]
    __private: [(PTHREAD_MUTEX_NORMAL & 3) << 14],
    #[cfg(target_pointer_width = "64")]
    __private: [(PTHREAD_MUTEX_NORMAL & 3) << 14, 0, 0, 0, 0, 0, 0, 0, 0, 0],
};

pub const PTHREAD_ONCE_INIT: pthread_once_init = 0;

pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    #[cfg(target_pointer_width = "32")]
    __private: [0; 10],
    #[cfg(target_pointer_width = "64")]
    __private: [0; 14],
};

#[cfg(target_pointer_width = "32")]
pub const PTHREAD_STACK_MIN: usize = 2 * 4096;
#[cfg(target_pointer_width = "64")]
pub const PTHREAD_STACK_MIN: usize = 4 * 4096;
