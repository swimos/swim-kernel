#[macro_export]
macro_rules! cstr {
    ($s:expr) => (
        #[allow(unused_unsafe)]
        unsafe { $crate::cstr::CStr::from_ptr(concat!($s, "\0").as_ptr()) }
    );
}
