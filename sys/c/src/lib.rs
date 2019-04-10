//! # C Library Bindings

#![no_std]

#![feature(core_intrinsics)]

extern crate swim_core;
extern crate swim_mem;
extern crate swim_c_sys;

pub use swim_c_sys::*;

#[macro_use]
mod macros;

pub mod stdlib;

pub mod cstr;
pub mod cstring;
