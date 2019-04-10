use swim_core::f16;

pub use crate::lib::cchar;
pub type schar = i8;
pub type uchar = u8;
pub type short = i16;
pub type ushort = u16;
pub type int = i32;
pub type uint = u32;
pub use crate::lib::long;
pub use crate::lib::ulong;
pub type half = f16;
pub type float = f32;
pub type double = f64;
pub type longlong = i64;
pub type ulonglong = u64;

pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type ssize_t = isize;

pub type void = core::ffi::c_void;

pub type va_list = *mut void;
