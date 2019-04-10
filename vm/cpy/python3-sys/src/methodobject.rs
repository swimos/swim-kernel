use core::mem;
use core::ptr;
use swim_c_sys::{cchar, int};
use crate::object::{PyObject, PyTypeObject, Py_TYPE};

extern "C" {
    pub static mut PyCFunction_Type: PyTypeObject;
}

#[inline(always)]
pub unsafe fn PyCFunction_Check(op: *mut PyObject) -> int {
    (Py_TYPE(op) == &mut PyCFunction_Type) as int
}

pub type PyCFunction = unsafe extern "C" fn (slf: *mut PyObject, args: *mut PyObject) -> *mut PyObject;
pub type PyCFunctionWithKeywords = unsafe extern "C" fn (slf: *mut PyObject, args: *mut PyObject, kwds: *mut PyObject) -> *mut PyObject;
pub type PyNoArgsFunction = unsafe extern "C" fn (slf: *mut PyObject) -> *mut PyObject;

extern "C" {
    pub fn PyCFunction_GetFunction(f: *mut PyObject) -> Option<PyCFunction>;
    pub fn PyCFunction_GetSelf(f: *mut PyObject) -> *mut PyObject;
    pub fn PyCFunction_GetFlags(f: *mut PyObject) -> int;
    pub fn PyCFunction_Call(f: *mut PyObject, args: *mut PyObject, kwds: *mut PyObject) -> *mut PyObject;
}

#[derive(Copy)]
#[repr(C)]
pub struct PyMethodDef {
    pub ml_name: *const cchar,
    pub ml_meth: Option<PyCFunction>,
    pub ml_flags: int,
    pub ml_doc: *const cchar,
}
impl Clone for PyMethodDef {
    #[inline(always)]
    fn clone(&self) -> PyMethodDef {
        *self
    }
}
impl Default for PyMethodDef {
    #[inline(always)]
    fn default() -> PyMethodDef {
        unsafe { mem::zeroed() }
    }
}

#[inline(always)]
pub unsafe fn PyCFunction_New(ml: *mut PyMethodDef, slf: *mut PyObject) -> *mut PyObject {
    PyCFunction_NewEx(ml, slf, ptr::null_mut())
}

extern "C" {
    pub fn PyCFunction_NewEx(arg1: *mut PyMethodDef, arg2: *mut PyObject, arg3: *mut PyObject) -> *mut PyObject;
}

// Flag passed to newmethodobject
pub const METH_VARARGS: int = 0x0001;
pub const METH_KEYWORD: int = 0x0002;
// METH_NOARGS and METH_O must not be combined with the flags above.
pub const METH_NOARGS: int = 0x0004;
pub const METH_O: int = 0x0008;

// METH_CLASS and METH_STATIC are a little different; these control
// the construction of methods for a class.  These cannot be used for
// functions in modules.
pub const METH_CLASS: int = 0x0010;
pub const METH_STATIC: int = 0x0020;

// METH_COEXIST allows a method to be entered eventhough a slot has
// already filled the entry.  When defined, the flag allows a separate
// method, "__contains__" for example, to coexist with a defined 
// slot like sq_contains.
pub const METH_COEXIST: int = 0x0040;

extern "C" {
    pub fn PyCFunction_ClearFreeList() -> int;
}
