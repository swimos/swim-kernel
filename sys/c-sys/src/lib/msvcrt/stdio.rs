#![allow(non_upper_case_globals)]

use crate::{int, uint, longlong};

// ISO/IEC 9899
// 7.19 Input/output

pub enum FILE {}

pub type fpos_t = longlong;

pub const _IOFBF: int = 0x0000;
pub const _IOLBF: int = 0x0040;
pub const _IONBF: int = 0x0004;

pub const BUFSIZ: uint = 512;

pub const EOF: int = -1;

pub const FOPEN_MAX: uint = 20;
pub const FILENAME_MAX: uint = 260;

pub const L_tmpnam: uint = 16;

pub const SEEK_SET: int = 0;
pub const SEEK_CUR: int = 1;
pub const SEEK_END: int = 2;

pub const TMP_MAX: uint = 32767;
