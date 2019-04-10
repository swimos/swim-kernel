use crate::{cchar, uint, long, ulong, longlong};

#[cfg(all(target_arch = "aarch64", target_pointer_width = "32"))]
pub const __SIZEOF_PTHREAD_ATTR_T: usize = 32;
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub const __SIZEOF_PTHREAD_ATTR_T: usize = 64;
#[cfg(target_arch = "arm")]
pub const __SIZEOF_PTHREAD_ATTR_T: usize = 36;
#[cfg(target_arch = "x86")]
pub const __SIZEOF_PTHREAD_ATTR_T: usize = 36;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
pub const __SIZEOF_PTHREAD_ATTR_T: usize = 32;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub const __SIZEOF_PTHREAD_ATTR_T: usize = 56;

#[cfg(all(target_arch = "aarch64", target_pointer_width = "32"))]
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 20;
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 32;
#[cfg(target_arch = "arm")]
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 20;
#[cfg(target_arch = "x86")]
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 20;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 20;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub const __SIZEOF_PTHREAD_BARRIER_T: usize = 32;

#[cfg(all(target_arch = "aarch64", target_pointer_width = "32"))]
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4;
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 8;
#[cfg(target_arch = "arm")]
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const __SIZEOF_PTHREAD_BARRIERATTR_T: usize = 4;

#[cfg(target_arch = "aarch64")]
pub const __SIZEOF_PTHREAD_COND_T: usize = 48;
#[cfg(target_arch = "arm")]
pub const __SIZEOF_PTHREAD_COND_T: usize = 48;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const __SIZEOF_PTHREAD_COND_T: usize = 48;

#[cfg(all(target_arch = "aarch64", target_pointer_width = "32"))]
pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 8;
#[cfg(target_arch = "arm")]
pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const __SIZEOF_PTHREAD_CONDATTR_T: usize = 4;

#[cfg(all(target_arch = "aarch64", target_pointer_width = "32"))]
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 32;
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 48;
#[cfg(target_arch = "arm")]
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 24;
#[cfg(target_arch = "x86")]
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 24;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 32;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub const __SIZEOF_PTHREAD_MUTEX_T: usize = 40;

#[cfg(all(target_arch = "aarch64", target_pointer_width = "32"))]
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 8;
#[cfg(target_arch = "arm")]
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const __SIZEOF_PTHREAD_MUTEXATTR_T: usize = 4;

#[cfg(all(target_arch = "aarch64", target_pointer_width = "32"))]
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 48;
#[cfg(all(target_arch = "aarch64", target_pointer_width = "64"))]
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56;
#[cfg(target_arch = "arm")]
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 32;
#[cfg(target_arch = "x86")]
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 32;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 44;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub const __SIZEOF_PTHREAD_RWLOCK_T: usize = 56;

#[cfg(target_arch = "aarch64")]
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8;
#[cfg(target_arch = "arm")]
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub const __SIZEOF_PTHREAD_RWLOCKATTR_T: usize = 8;

#[repr(C)]
pub union pthread_attr_t {
    __size: [cchar; __SIZEOF_PTHREAD_ATTR_T],
    __align: long,
}

#[repr(C)]
pub union pthread_barrier_t {
    __size: [cchar; __SIZEOF_PTHREAD_BARRIER_T],
    __align: long,
}

#[repr(C)]
pub union pthread_barrierattr_t {
    __size: [cchar; __SIZEOF_PTHREAD_BARRIERATTR_T],
    __align: int,
}

#[repr(C)]
pub union pthread_cond_t {
    __size: [cchar; __SIZEOF_PTHREAD_COND_T],
    __align: longlong,
}

#[repr(C)]
pub union pthread_condattr_t {
    __size: [cchar; __SIZEOF_PTHREAD_CONDATTR_T],
    __align: int,
}

pub type pthread_key_t = uint;

#[repr(C)]
pub union pthread_mutex_t {
    __size: [cchar; __SIZEOF_PTHREAD_MUTEX_T],
    __align: long,
}

#[repr(C)]
pub union pthread_mutexattr_t {
    __size: [cchar; __SIZEOF_PTHREAD_MUTEXATTR_T],
    __align: int,
}

pub type pthread_once_t = int;

#[repr(C)]
pub union pthread_rwlock_t {
    __size: [cchar; __SIZEOF_PTHREAD_RWLOCK_T],
    __align: long,
}

#[repr(C)]
pub union pthread_rwlockattr_t {
    __size: [cchar; __SIZEOF_PTHREAD_RWLOCKATTR_T],
    __align: long,
}

pub type pthread_t = ulong;

#[repr(C)]
pub struct sched_param {
    sched_priority: int,
}

pub const PTHREAD_CREATE_JOINABLE: int = 0;
pub const PTHREAD_CREATE_DETACHED: int = 1;

pub const PTHREAD_MUTEX_NORMAL: int = 0;
pub const PTHREAD_MUTEX_RECURSIVE: int = 1;
pub const PTHREAD_MUTEX_ERRORCHECK: int = 2;
pub const PTHREAD_MUTEX_DEFAULT: int = PTHREAD_MUTEX_NORMAL;

pub const PTHREAD_COND_INITIALIZER: pthread_cond_t = pthread_cond_t {
    __size: [0; __SIZEOF_PTHREAD_COND_T],
};

pub const PTHREAD_MUTEX_INITIALIZER: pthread_mutex_t = pthread_mutex_t {
    __size: [0; __SIZEOF_PTHREAD_MUTEX_T],
};

pub const PTHREAD_ONCE_INIT: pthread_once_init = 0;

pub const PTHREAD_RWLOCK_INITIALIZER: pthread_rwlock_t = pthread_rwlock_t {
    __size: [0; __SIZEOF_PTHREAD_RWLOCK_T],
};

#[cfg(target_arch = "aarch64")]
pub const PTHREAD_STACK_MIN: usize = 131072;
#[cfg(target_arch = "arm")]
pub const PTHREAD_STACK_MIN: usize = 16384;
#[cfg(target_arch = "x86")]
pub const PTHREAD_STACK_MIN: usize = 16384;
#[cfg(target_arch = "x86_64")]
pub const PTHREAD_STACK_MIN: usize = 16384;
