// Microsoft C library
// https://github.com/wine-mirror/wine/tree/master/include/msvcrt

pub type cchar = i8;
pub type long = i32;
pub type ulong = u32;

// ISO/IEC 9899
pub mod errno;
pub mod stdint;
pub mod stdio;
pub mod stdlib;
pub mod time;

#[link(name = "msvcrt")]
extern "C" {
}
