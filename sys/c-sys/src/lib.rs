//! # C Library Bridge

#![no_std]

#![allow(non_camel_case_types)]

extern crate swim_core;

#[macro_use]
mod macros;

// C language
mod lang;

// C library
cfg_if! {
    if #[cfg(target_env = "uclibc")] {
        #[path = "lib/uclibc/mod.rs"]
        mod lib;
    } else if #[cfg(any(target_os = "macos", target_os = "ios"))] {
        #[path = "lib/apple/mod.rs"]
        mod lib;
    } else if #[cfg(target_os = "android")] {
        #[path = "lib/bionic/mod.rs"]
        mod lib;
    } else if #[cfg(target_os = "linux")] {
        #[path = "lib/glibc/mod.rs"]
        mod lib;
    } else if #[cfg(target_os = "windows")] {
        #[path = "lib/msvcrt/mod.rs"]
        mod lib;
    }
}

// C standard
mod std;

// C extension
cfg_if! {
    if #[cfg(unix)] {
        #[path = "ext/posix/mod.rs"]
        mod posix;
    } else if #[cfg(windows)] {
        #[path = "ext/windows/mod.rs"]
        mod windows;
    }
}

// C platform
pub use crate::lang::*;
cfg_if! {
    if #[cfg(unix)] {
        pub use crate::posix::*;
    } else if #[cfg(windows)] {
        pub use crate::windows::*;
    } else {
        pub use crate::std::*;
    }
}
