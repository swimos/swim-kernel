// Apple C library
// https://opensource.apple.com/source/Libc/
// https://opensource.apple.com/source/xnu/

cfg_if! {
    if #[cfg(target_arch = "x86")] {
        pub type cchar = i8;
        pub type long = i32;
        pub type ulong = u32;
    } else if #[cfg(target_arch = "x86_64")] {
        pub type cchar = i8;
        pub type long = i64;
        pub type ulong = u64;
    }
}

// ISO/IEC 9899
pub mod errno;
pub mod stdint;
pub mod stdio;
pub mod stdlib;
pub mod time;

// POSIX.1
pub mod pthread;
pub mod sys;
pub mod unistd;

#[link(name = "c")]
#[link(name = "m")]
extern "C" {
}
