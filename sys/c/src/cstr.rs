use core::fmt;
use core::isize;
use core::marker::PhantomData;
use core::mem;
use core::slice;
use core::str;
use swim_c_sys::{void, cchar};
use crate::string;

/// A nul-terminated sequence of C `char`s.
pub struct CStr {
    lifetime: PhantomData<[cchar]>,
}

impl CStr {
    /// Converts a pointer to a nul-terminated byte array to a `CStr`.
    pub fn from_ptr<'a>(ptr: *const u8) -> &'a Self {
        unsafe { mem::transmute(ptr) }
    }

    /// Converts a pointer to a nul-terminated C `char` array to a `CStr`.
    pub fn from_cptr<'a>(ptr: *const cchar) -> &'a Self {
        unsafe { mem::transmute(ptr) }
    }

    /// Converts a byte slice to a `CStr`, if the slice ends with a nul byte.
    pub fn from_bytes(bytes: &[u8]) -> Result<&Self, ()> {
        let len = bytes.len();
        let ptr = bytes.as_ptr() as *mut void;
        unsafe {
            if len > 0 && string::memchr(ptr, 0, len) == ptr.offset((len - 1) as isize) {
                Ok(mem::transmute(ptr))
            } else {
                Err(())
            }
        }
    }

    /// Converts a byte slice to a `CStr`, without verifying that the slice
    /// ends with a nul byte.
    pub unsafe fn from_bytes_unchecked(bytes: &[u8]) -> &Self {
        mem::transmute(bytes.as_ptr())
    }

    /// Converts this string to a pointer to a nul-terminated byte array.
    pub fn as_ptr(&self) -> *const u8 {
        unsafe { mem::transmute(self) }
    }

    /// Converts this string to a pointer to a nul-terminated C `char` array.
    pub fn as_cptr(&self) -> *const cchar {
        unsafe { mem::transmute(self) }
    }

    /// Returns a byte slice containing the string and trailing nul byte.
    pub unsafe fn to_bytes(&self) -> &[u8] {
        let len = string::strlen(self.as_cptr()) as usize;
        slice::from_raw_parts(self.as_ptr(), len.wrapping_add(1))
    }

    /// Returns a string slice containing the string with no trailing nul byte.
    pub unsafe fn to_str(&self) -> Result<&str, str::Utf8Error> {
        let len = string::strlen(self.as_cptr()) as usize;
        str::from_utf8(slice::from_raw_parts(self.as_ptr(), len))
    }

    /// Returns a string slice containing the string with no trailing nul byte,
    /// without verifying that the string contains valid UTF-8.
    pub unsafe fn to_str_unchecked(&self) -> &str {
        let len = string::strlen(self.as_cptr()) as usize;
        str::from_utf8_unchecked(slice::from_raw_parts(self.as_ptr(), len))
    }
}

impl AsRef<CStr> for CStr {
    #[inline]
    fn as_ref(&self) -> &CStr {
        self
    }
}

impl fmt::Debug for CStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        unsafe { self.to_str() }.fmt(f)
    }
}

impl Default for &'static CStr {
    #[inline]
    fn default() -> &'static CStr {
        CStr::from_ptr(b"\0".as_ptr())
    }
}
