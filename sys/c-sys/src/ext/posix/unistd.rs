use crate::{cchar, int, uint, long, size_t, ssize_t, void};
use crate::sys::types::{gid_t, off_t, pid_t, uid_t};

pub use crate::lib::unistd::*;

pub const STDIN_FILENO: int = 0;
pub const STDOUT_FILENO: int = 1;
pub const STDERR_FILENO: int = 2;

extern "C" {
    pub fn access(path: *const cchar, amode: int) -> int;
    pub fn alarm(seconds: uint) -> uint;
    pub fn chdir(dir: *const cchar) -> int;
    pub fn chroot(name: *const cchar) -> int;
    pub fn chown(path: *const cchar, uid: uid_t, gid: gid_t) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "close$UNIX2003")]
    pub fn close(fd: int) -> int;
    pub fn dup(fd: int) -> int;
    pub fn dup2(src: int, dst: int) -> int;
    pub fn execl(path: *const cchar, arg0: *const cchar, ...) -> int;
    pub fn execle(path: *const cchar, arg0: *const cchar, ...) -> int;
    pub fn execlp(file: *const cchar, arg0: *const cchar, ...) -> int;
    pub fn execv(prog: *const cchar, argv: *const *const cchar) -> int;
    pub fn execve(prog: *const cchar, argv: *const *const cchar, envp: *const *const cchar) -> int;
    pub fn execvp(c: *const cchar, argv: *const *const cchar) -> int;
    pub fn fchown(fd: int, owner: uid_t, group: gid_t) -> int;
    pub fn fchdir(dirfd: int) -> int;
    pub fn fork() -> pid_t;
    pub fn fpathconf(filedes: int, name: int) -> long;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "fsync$UNIX2003")]
    pub fn fsync(fd: int) -> int;
    pub fn ftruncate(fd: int, length: off_t) -> int;
    pub fn getcwd(buf: *mut cchar, size: size_t) -> *mut cchar;
    pub fn getdtablesize() -> int;
    pub fn getegid() -> gid_t;
    pub fn geteuid() -> uid_t;
    pub fn getgid() -> gid_t;
    pub fn getgroups(ngroups_max: int, groups: *mut gid_t) -> int;
    pub fn getlogin() -> *mut cchar;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "getopt$UNIX2003")]
    pub fn getopt(argc: int, argv: *const *mut cchar, optstr: *const cchar) -> int;
    pub fn getpgid(pid: pid_t) -> pid_t;
    pub fn getpgrp() -> pid_t;
    pub fn getpid() -> pid_t;
    pub fn getppid() -> pid_t;
    pub fn getsid(pid: pid_t) -> pid_t;
    pub fn getuid() -> uid_t;
    pub fn isatty(fd: int) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "lchown$UNIX2003")]
    pub fn lchown(path: *const cchar, uid: uid_t, gid: gid_t) -> int;
    pub fn link(src: *const cchar, dst: *const cchar) -> int;
    pub fn lseek(fd: int, offset: off_t, whence: int) -> off_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "nice$UNIX2003")]
    pub fn nice(incr: int) -> int;
    pub fn pathconf(path: *const cchar, name: int) -> long;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pause$UNIX2003")]
    pub fn pause() -> int;
    pub fn pipe(fds: *mut int) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pread$UNIX2003")]
    pub fn pread(fd: int, buf: *mut void, count: size_t, offset: off_t) -> ssize_t;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "pwrite$UNIX2003")]
    pub fn pwrite(fd: int, buf: *const void, count: size_t, offset: off_t) -> ssize_t;
    pub fn read(fd: int, buf: *mut void, count: size_t) -> ssize_t;
    pub fn readlink(path: *const cchar, buf: *mut cchar, bufsz: size_t) -> ssize_t;
    pub fn rmdir(path: *const cchar) -> int;
    pub fn setgid(gid: gid_t) -> int;
    pub fn setpgid(pid: pid_t, pgid: pid_t) -> int;
    pub fn setsid() -> pid_t;
    pub fn setuid(uid: uid_t) -> int;
    pub fn sleep(secs: uint) -> uint;
    pub fn symlink(path1: *const cchar, path2: *const cchar) -> int;
    pub fn sysconf(name: int) -> long;
    pub fn tcgetpgrp(fd: int) -> pid_t;
    pub fn tcsetpgrp(fd: int, pgrp: pid_t) -> int;
    pub fn ttyname(fd: int) -> *mut cchar;
    pub fn unlink(c: *const cchar) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "usleep$UNIX2003")]
    pub fn usleep(secs: uint) -> int;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "write$UNIX2003")]
    pub fn write(fd: int, buf: *const void, count: size_t) -> ssize_t;
}
