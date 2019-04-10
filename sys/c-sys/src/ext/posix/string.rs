use crate::cchar;

pub use crate::std::string::*;

extern "C" {
    pub fn strdup(cs: *const cchar) -> *mut cchar;
}
