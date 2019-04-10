use crate::{cchar, int, size_t, void};

// 7.21 String handling

extern "C" {
    // 7.21.2 Copying functions
    pub fn memcpy(dest: *mut void, src: *const void, n: size_t) -> *mut void;
    pub fn memmove(dest: *mut void, src: *const void, n: size_t) -> *mut void;
    pub fn strcpy(dst: *mut cchar, src: *const cchar) -> *mut cchar;
    pub fn strncpy(dst: *mut cchar, src: *const cchar, n: size_t) -> *mut cchar;

    // 7.21.3 Concatenation functions
    pub fn strcat(s: *mut cchar, ct: *const cchar) -> *mut cchar;
    pub fn strncat(s: *mut cchar, ct: *const cchar, n: size_t) -> *mut cchar;

    // 7.21.4 Comparison functions
    pub fn memcmp(cx: *const void, ct: *const void, n: size_t) -> int;
    pub fn strcmp(cs: *const cchar, ct: *const cchar) -> int;
    pub fn strcoll(cs: *const cchar, ct: *const cchar) -> int;
    pub fn strncmp(cs: *const cchar, ct: *const cchar, n: size_t) -> int;
    pub fn strxfrm(s: *mut cchar, ct: *const cchar, n: size_t) -> size_t;

    // 7.21.5 Search functions
    pub fn memchr(cx: *const void, c: int, n: size_t) -> *mut void;
    pub fn strchr(cs: *const cchar, c: int) -> *mut cchar;
    pub fn strcspn(cs: *const cchar, ct: *const cchar) -> size_t;
    pub fn strpbrk(cs: *const cchar, ct: *const cchar) -> *mut cchar;
    pub fn strrchr(cs: *const cchar, c: int) -> *mut cchar;
    pub fn strspn(cs: *const cchar, ct: *const cchar) -> size_t;
    pub fn strstr(cs: *const cchar, ct: *const cchar) -> *mut cchar;
    pub fn strtok(s: *mut cchar, t: *const cchar) -> *mut cchar;

    // 7.21.6 Miscellaneous functions
    pub fn memset(dest: *mut void, c: int, n: size_t) -> *mut void;
    #[cfg_attr(all(target_os = "macos", target_arch = "x86"), link_name = "strerror$UNIX2003")]
    pub fn strerror(n: int) -> *mut cchar;
    pub fn strlen(cs: *const cchar) -> size_t;
}

// Non-standard
cfg_if! {
    if #[cfg(target_os = "linux")] {
        extern "C" {
            #[cfg(target_os = "linux")]
            pub fn memrchr(cx: *const void, c: int, n: size_t) -> *mut void;
        }
    } else {
        pub unsafe fn memrchr(cx: *const void, c: int, n: size_t) -> *mut void {
            let cx = cx as *const u8;
            let c = c as u8;
            let mut p = cx.offset(n as isize);
            while p > cx {
                p = p.offset(-1);
                if *p == c {
                    return p as *mut void;
                }
            }
            return core::ptr::null_mut();
        }
    }
}
