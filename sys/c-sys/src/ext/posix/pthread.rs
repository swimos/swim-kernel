use crate::{int, size_t, void};
use crate::time::timespec;

pub use crate::lib::pthread::*;

extern "C" {
    pub fn pthread_attr_destroy(attr: *mut pthread_attr_t) -> int;
    pub fn pthread_attr_getdetachstate(attr: *const pthread_attr_t, detachstate: *mut int) -> int;
    pub fn pthread_attr_getguardsize(attr: *const pthread_attr_t, guardsize: *mut size_t) -> int;
    pub fn pthread_attr_getinheritsched(attr: *const pthread_attr_t, inheritsched: *mut int) -> int;
    pub fn pthread_attr_getschedparam(attr: *const pthread_attr_t, schedparam: *mut sched_param) -> int;
    pub fn pthread_attr_getschedpolicy(attr: *const pthread_attr_t, schedpolicy: *mut int) -> int;
    pub fn pthread_attr_getscope(attr: *const pthread_attr_t, scope: *mut int) -> int;
    pub fn pthread_attr_getstackaddr(attr: *const pthread_attr_t, stackaddr: *mut *mut void) -> int;
    pub fn pthread_attr_getstacksize(attr: *const pthread_attr_t, stacksize: *mut size_t) -> int;
    pub fn pthread_attr_init(attr: *mut pthread_attr_t) -> int;
    pub fn pthread_attr_setdetachstate(attr: *mut pthread_attr_t, detachstate: int) -> int;
    pub fn pthread_attr_setguardsize(attr: *mut pthread_attr_t, guardsize: size_t) -> int;
    pub fn pthread_attr_setinheritsched(attr: *mut pthread_attr_t, inheritsched: int) -> int;
    pub fn pthread_attr_setschedparam(attr: *mut pthread_attr_t, schedparam: *const sched_param) -> int;
    pub fn pthread_attr_setschedpolicy(attr: *mut pthread_attr_t, schedpolicy: int) -> int;
    pub fn pthread_attr_setscope(attr: *mut pthread_attr_t, scope: int) -> int;
    pub fn pthread_attr_setstackaddr(attr: *mut pthread_attr_t, stackaddr: *mut void) -> int;
    pub fn pthread_attr_setstacksize(attr: *mut pthread_attr_t, stacksize: size_t) -> int;

    pub fn pthread_cond_broadcast(cond: *mut pthread_cond_t) -> int;
    pub fn pthread_cond_destroy(cond: *mut pthread_cond_t) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_cond_init$UNIX2003")]
    pub fn pthread_cond_init(cond: *mut pthread_cond_t, attr: *const pthread_condattr_t) -> int;
    pub fn pthread_cond_signal(cond: *mut pthread_cond_t) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_cond_timedwait$UNIX2003")]
    pub fn pthread_cond_timedwait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t,
                                  abstime: *const timespec) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_cond_wait$UNIX2003")]
    pub fn pthread_cond_wait(cond: *mut pthread_cond_t, mutex: *mut pthread_mutex_t) -> int;

    pub fn pthread_condattr_destroy(attr: *mut pthread_condattr_t) -> int;
    pub fn pthread_condattr_getpshared(attr: *const pthread_condattr_t, pshared: *mut int) -> int;
    pub fn pthread_condattr_init(attr: *mut pthread_condattr_t) -> int;
    pub fn pthread_condattr_setpshared(attr: *mut pthread_condattr_t, pshared: int) -> int;

    pub fn pthread_key_create(key: *mut pthread_key_t, dtor: Option<unsafe extern fn(*mut void)>) -> int;
    pub fn pthread_key_delete(key: pthread_key_t) -> int;

    pub fn pthread_mutex_destroy(mutex: *mut pthread_mutex_t) -> int;
    pub fn pthread_mutex_getprioceiling(mutex: *const pthread_mutex_t, prioceiling: *mut int) -> int;
    pub fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const pthread_mutexattr_t) -> int;
    pub fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> int;
    pub fn pthread_mutex_setprioceiling(mutex: *mut pthread_mutex_t, prioceiling: int,
                                        old_prioceiling: *mut int) -> int;
    pub fn pthread_mutex_trylock(mutex: *mut pthread_mutex_t) -> int;
    pub fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_mutexattr_destroy$UNIX2003")]
    pub fn pthread_mutexattr_destroy(attr: *mut pthread_mutexattr_t) -> int;
    pub fn pthread_mutexattr_getprioceiling(attr: *const pthread_mutexattr_t, prioceiling: *mut int) -> int;
    pub fn pthread_mutexattr_getprotocol(attr: *const pthread_mutexattr_t, protocol: *mut int) -> int;
    pub fn pthread_mutexattr_getpshared(attr: *const pthread_mutexattr_t, pshared: *mut int) -> int;
    pub fn pthread_mutexattr_gettype(attr: *const pthread_mutexattr_t, mutextype: *mut int) -> int;
    pub fn pthread_mutexattr_init(attr: *mut pthread_mutexattr_t) -> int;
    pub fn pthread_mutexattr_setprioceiling(attr: *mut pthread_mutexattr_t, prioceiling: int) -> int;
    pub fn pthread_mutexattr_setprotocol(attr: *mut pthread_mutexattr_t, protocol: int) -> int;
    pub fn pthread_mutexattr_setpshared(attr: *mut pthread_mutexattr_t, pshared: int) -> int;
    pub fn pthread_mutexattr_settype(attr: *mut pthread_mutexattr_t, mutextype: int) -> int;

    pub fn pthread_once(once_control: *mut pthread_once_t, init_routine: extern fn()) -> int;

    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_rwlock_destroy$UNIX2003")]
    pub fn pthread_rwlock_destroy(rwlock: *mut pthread_rwlock_t) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_rwlock_init$UNIX2003")]
    pub fn pthread_rwlock_init(rwlock: *mut pthread_rwlock_t, attr: *const pthread_rwlockattr_t) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_rwlock_rdlock$UNIX2003")]
    pub fn pthread_rwlock_rdlock(rwlock: *mut pthread_rwlock_t) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_rwlock_tryrdlock$UNIX2003")]
    pub fn pthread_rwlock_tryrdlock(rwlock: *mut pthread_rwlock_t) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_rwlock_trywrlock$UNIX2003")]
    pub fn pthread_rwlock_trywrlock(rwlock: *mut pthread_rwlock_t) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_rwlock_unlock$UNIX2003")]
    pub fn pthread_rwlock_unlock(rwlock: *mut pthread_rwlock_t) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_rwlock_wrlock$UNIX2003")]
    pub fn pthread_rwlock_wrlock(rwlock: *mut pthread_rwlock_t) -> int;

    pub fn pthread_rwlockattr_destroy(attr: *mut pthread_rwlockattr_t) -> int;
    pub fn pthread_rwlockattr_getpshared(attr: *const pthread_rwlockattr_t, pshared: *mut int) -> int;
    pub fn pthread_rwlockattr_init(attr: *mut pthread_rwlockattr_t) -> int;
    pub fn pthread_rwlockattr_setpshared(attr: *mut pthread_rwlockattr_t, pshared: int) -> int;

    pub fn pthread_create(thread: *mut pthread_t, attr: *const pthread_attr_t,
                          start_routine: extern fn(*mut void) -> *mut void,
                          arg: *mut void) -> int;
    pub fn pthread_detach(thread: pthread_t) -> int;
    pub fn pthread_equal(thread1: pthread_t, thread2: pthread_t) -> int;
    pub fn pthread_exit(value: *mut void);
    pub fn pthread_getconcurrency() -> int;
    pub fn pthread_getschedparam(thread: pthread_t, policy: *mut int, schedparam: *mut sched_param) -> int;
    pub fn pthread_getspecific(key: pthread_key_t) -> *mut void;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_join$UNIX2003")]
    pub fn pthread_join(thread: pthread_t, value: *mut *mut void) -> int;
    pub fn pthread_self() -> pthread_t;
    pub fn pthread_setcancelstate(cancelstate: int, old_cancelstate: *mut int) -> int;
    pub fn pthread_setcanceltype(canceltype: int, old_canceltype: *mut int) -> int;
    pub fn pthread_setconcurrency(concurrency: int) -> int;
    pub fn pthread_setschedparam(thread: pthread_t, policy: int, schedparam: *const sched_param) -> int;
    pub fn pthread_setspecific(key: pthread_key_t, value: *const void) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pthread_testcancel$UNIX2003")]
    pub fn pthread_testcancel();
}
