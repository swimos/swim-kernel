#![no_std]

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate swim_c_sys;

pub mod pyconfig;

pub mod object;
pub mod objimpl;
pub mod typeslots;
pub mod pyhash;

pub mod pydebug;

pub mod descrobject;
pub mod methodobject;
pub mod moduleobject;
pub mod structmember;
pub mod tupleobject;

pub mod codecs;
pub mod pyerrors;

pub mod pystate;
pub mod context;

pub mod pyarena;
pub mod modsupport;
#[cfg(not(Py_LIMITED_API))]
pub mod code;
pub mod compile;
pub mod symtable;
pub mod pythonrun;
pub mod pylifecycle;
