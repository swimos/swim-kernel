use core::isize;
use swim_c_sys::{size_t, ssize_t};
use swim_c_sys::stdint::{intptr_t, uintptr_t};

pub type Py_uintptr_t = uintptr_t;
pub type Py_intptr_t = intptr_t;
pub type Py_ssize_t = ssize_t;

pub type Py_hash_t = Py_ssize_t;
pub type Py_uhash_t = size_t;

pub const PY_SSIZE_T_MIN: Py_ssize_t = isize::MIN;
pub const PY_SSIZE_T_MAX: Py_ssize_t = isize::MAX;
