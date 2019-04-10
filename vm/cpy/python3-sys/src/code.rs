use core::mem;
use swim_c_sys::{cchar, uchar, int, void};
use crate::pyconfig::Py_ssize_t;
use crate::object::{PyObject, PyTypeObject, Py_TYPE};
use crate::tupleobject::PyTuple_GET_SIZE;

#[derive(Copy)]
#[repr(C)]
pub struct PyCodeObject {
    pub ob_base: PyObject,
    pub co_argcount: int,
    pub co_kwonlyargcount: int,
    pub co_nlocals: int,
    pub co_stacksize: int,
    pub co_flags: int,
    #[cfg(Py_3_6)]
    pub co_firstlineno: int,
    pub co_code: *mut PyObject,
    pub co_consts: *mut PyObject,
    pub co_names: *mut PyObject,
    pub co_varnames: *mut PyObject,
    pub co_freevars: *mut PyObject,
    pub co_cellvars: *mut PyObject,
    pub co_cell2arg: *mut uchar,
    pub co_filename: *mut PyObject,
    pub co_name: *mut PyObject,
    #[cfg(not(Py_3_6))]
    pub co_firstlineno: int,
    pub co_lnotab: *mut PyObject,
    pub co_zombieframe: *mut void,
    pub co_weakreflist: *mut PyObject,
    #[cfg(Py_3_6)]
    pub co_extra: *mut void,
}
impl Clone for PyCodeObject {
    #[inline(always)]
    fn clone(&self) -> PyCodeObject {
        *self
    }
}
impl Default for PyCodeObject {
    #[inline(always)]
    fn default() -> PyCodeObject {
        unsafe { mem::zeroed() }
    }
}

// Masks for co_flags
pub const CO_OPTIMIZED: int = 0x0001;
pub const CO_NEWLOCALS: int = 0x0002;
pub const CO_VARARGS: int = 0x0004;
pub const CO_VARKEYWORDS: int = 0x0008;
pub const CO_NESTED: int = 0x0010;
pub const CO_GENERATOR: int = 0x0020;
// The CO_NOFREE flag is set if there are no free or cell variables.
// This information is redundant, but it allows a single flag test
// to determine whether there is any extra work to be done when the
// call frame it setup.
pub const CO_NOFREE: int = 0x0040;

// The CO_COROUTINE flag is set for coroutine functions (defined with
// `async def` keywords)
#[cfg(Py_3_5)]
pub const CO_COROUTINE: int = 0x0080;
#[cfg(Py_3_5)]
pub const CO_ITERABLE_COROUTINE: int = 0x0100;
#[cfg(Py_3_6)]
pub const CO_ASYNC_GENERATOR: int = 0x0200;

pub const CO_FUTURE_DIVISION: int = 0x2000;
pub const CO_FUTURE_ABSOLUTE_IMPORT: int = 0x4000; // do absolute imports by default
pub const CO_FUTURE_WITH_STATEMENT: int = 0x8000;
pub const CO_FUTURE_PRINT_FUNCTION: int = 0x10000;
pub const CO_FUTURE_UNICODE_LITERALS: int = 0x20000;

pub const CO_FUTURE_BARRY_AS_BDFL: int = 0x40000;
#[cfg(Py_3_5)]
pub const CO_FUTURE_GENERATOR_STOP: int = 0x80000;

pub const CO_MAXBLOCKS: usize = 20; // Max static block nesting within a function

extern "C" {
    pub static mut PyCode_Type: PyTypeObject;
}

#[inline(always)]
pub unsafe fn PyCode_Check(op: *mut PyObject) -> int {
    (Py_TYPE(op) == &mut PyCode_Type) as int
}

#[inline(always)]
pub unsafe fn PyCode_GetNumFree(op: *mut PyCodeObject) -> Py_ssize_t {
    PyTuple_GET_SIZE((*op).co_freevars)
}

extern "C" {
    pub fn PyCode_New(arg1: int, arg2: int, arg3: int, arg4: int, arg5: int,
                      arg6: *mut PyObject, arg7: *mut PyObject,
                      arg8: *mut PyObject, arg9: *mut PyObject,
                      arg10: *mut PyObject, arg11: *mut PyObject,
                      arg12: *mut PyObject, arg13: *mut PyObject,
                      arg14: int, arg15: *mut PyObject) -> *mut PyCodeObject;

    pub fn PyCode_NewEmpty(filename: *const cchar, funcname: *const cchar, firstlineno: int) -> *mut PyCodeObject;

    pub fn PyCode_Addr2Line(arg1: *mut PyCodeObject, arg2: int) -> int;

    pub fn PyCode_Optimize(code: *mut PyObject, consts: *mut PyObject, names: *mut PyObject, lnotab: *mut PyObject) -> *mut PyObject;
}
