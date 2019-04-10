use core::mem;
use crate::swim_c_sys::{cchar, int, void};
use crate::pyconfig::{Py_ssize_t, Py_hash_t};

cfg_if! {
    if #[cfg(not(Py_LIMITED_API))] {
        #[derive(Copy)]
        #[repr(C)]
        pub struct PyHash_FuncDef {
            pub hash: Option<extern "C" fn (arg1: *const void, arg2: Py_ssize_t) -> Py_hash_t>,
            pub name: *const cchar,
            pub hash_bits: int,
            pub seed_bits: int,
        }
        impl Clone for PyHash_FuncDef {
            #[inline(always)]
            fn clone(&self) -> PyHash_FuncDef {
                *self
            }
        }
        impl Default for PyHash_FuncDef {
            #[inline(always)]
            fn default() -> PyHash_FuncDef {
                unsafe { mem::zeroed() }
            }
        }

        extern "C" {
            pub fn PyHash_GetFuncDef() -> *mut PyHash_FuncDef;
        }
    }
}
