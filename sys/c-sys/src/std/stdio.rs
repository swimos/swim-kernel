use crate::{cchar, int, long, size_t, void, va_list};

// 7.19 Input/output

pub use crate::lib::stdio::*;

extern {
    pub static stdin: *mut FILE;
    pub static stdout: *mut FILE;
    pub static stderr: *mut FILE;
}

extern "C" {
    // 7.19.4 Operations on files
    pub fn remove(filename: *const cchar) -> int;
    pub fn rename(old: *const cchar, new: *const cchar) -> int;
    pub fn tmpfile() -> *mut FILE;
    pub fn tmpnam(s: *mut cchar) -> *mut cchar;
    pub fn fclose(stream: *mut FILE) -> int;
    pub fn fflush(stream: *mut FILE) -> int;
    pub fn fopen(filename: *const cchar, mode: *const cchar) -> *mut FILE;
    pub fn freopen(filename: *const cchar, mode: *const cchar, stream: *mut FILE) -> *mut FILE;
    pub fn setbuf(stream: *mut FILE, buf: *mut cchar);
    pub fn setvbuf(stream: *mut FILE, buf: *mut cchar, mode: int, size: size_t) -> int;

    // 7.19.6 Formatted input/output functions
    pub fn fprintf(stream: *mut FILE, format: *const cchar, ...) -> int;
    pub fn fscanf(stream: *mut FILE, format: *const cchar, ...) -> int;
    pub fn printf(format: *const cchar, ...) -> int;
    pub fn scanf(format: *const cchar, ...) -> int;
    pub fn snprintf(s: *mut cchar, n: size_t, format: *const cchar, ...) -> int;
    pub fn sprintf(s: *mut cchar, format: *const cchar, ...) -> int;
    pub fn sscanf(s: *mut cchar, format: *const cchar, ...) -> int;
    pub fn vfprintf(stream: *mut FILE, format: *const cchar, arg: va_list) -> int;
    pub fn vfscanf(stream: *mut FILE, format: *const cchar, arg: va_list) -> int;
    pub fn vprintf(format: *const cchar, arg: va_list) -> int;
    pub fn vscanf(format: *const cchar, arg: va_list) -> int;
    pub fn vsnprintf(s: *mut cchar, n: size_t, format: *const cchar, arg: va_list) -> int;
    pub fn vsprintf(s: *mut cchar, format: *const cchar, arg: va_list) -> int;
    pub fn vsscanf(s: *const cchar, format: *const cchar, arg: va_list) -> int;

    // 7.19.7 Character input/output functions
    pub fn fgetc(stream: *mut FILE) -> int;
    pub fn fgets(s: *mut cchar, n: int, stream: *mut FILE) -> *mut cchar;
    pub fn fputc(c: int, stream: *mut FILE) -> int;
    pub fn fputs(s: *const cchar, stream: *mut FILE) -> int;
    pub fn getc(stream: *mut FILE) -> int;
    pub fn getchar() -> int;
    pub fn gets(s: *mut cchar) -> *mut cchar;
    pub fn putc(c: int, stream: *mut FILE) -> int;
    pub fn putchar(c: int) -> int;
    pub fn puts(s: *const cchar) -> int;
    pub fn ungetc(c: int, stream: *mut FILE) -> int;

    // 7.19.8 Direct input/output functions
    pub fn fread(ptr: *mut void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;
    pub fn fwrite(ptr: *const void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;

    // 7.19.9 File positioning functions
    pub fn fgetpos(stream: *mut FILE, pos: *mut fpos_t) -> int;
    pub fn fseek(stream: *mut FILE, offset: long, whence: int) -> int;
    pub fn fsetpos(stream: *mut FILE, pos: *const fpos_t) -> int;
    pub fn ftell(stream: *mut FILE) -> long;
    pub fn rewind(stream: *mut FILE);

    // 7.19.10 Error-handling functions
    pub fn clearerr(stream: *mut FILE);
    pub fn feof(stream: *mut FILE) -> int;
    pub fn ferror(stream: *mut FILE) -> int;
    pub fn perror(s: *const cchar);
}
