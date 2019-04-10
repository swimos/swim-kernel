#![allow(non_upper_case_globals)]

use crate::{int, uint};
use crate::sys::types::off_t;

// ISO/IEC 9899
// 7.19 Input/output

pub enum FILE {}

pub type fpos_t = off_t;

pub const _IOFBF: int = 0;
pub const _IOLBF: int = 1;
pub const _IONBF: int = 2;

pub const BUFSIZ: uint = 1024;

pub const EOF: int = -1;

pub const FOPEN_MAX: uint = 20;
pub const FILENAME_MAX: uint = 1024;

pub const L_tmpnam: uint = 1024;

pub const SEEK_SET: int = 0;
pub const SEEK_CUR: int = 1;
pub const SEEK_END: int = 2;

pub const TMP_MAX: uint = 308915776;
