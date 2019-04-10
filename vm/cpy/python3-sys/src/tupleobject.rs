use swim_c_sys::int;
use crate::pyconfig::Py_ssize_t;
use crate::object::{PyObject, PyVarObject, PyTypeObject, Py_TYPE, Py_SIZE, PyType_FastSubclass};

#[cfg(not(Py_LIMITED_API))]
#[repr(C)]
pub struct PyTupleObject {
    pub ob_base: PyVarObject,
    pub ob_item: [*mut PyObject; 1],
}

extern "C" {
    pub static mut PyTuple_Type: PyTypeObject;
    pub static mut PyTupleIter_Type: PyTypeObject;
}

#[inline(always)]
pub unsafe fn PyTuple_Check(op: *mut PyObject) -> int {
    PyType_FastSubclass(Py_TYPE(op), crate::object::Py_TPFLAGS_TUPLE_SUBCLASS)
}

#[inline(always)]
pub unsafe fn PyTuple_CheckExact(op: *mut PyObject) -> int {
    (Py_TYPE(op) == &mut PyTuple_Type) as int
}

extern "C" {
    pub fn PyTuple_New(size: Py_ssize_t) -> *mut PyObject;
    pub fn PyTuple_Size(arg1: *mut PyObject) -> Py_ssize_t;
    pub fn PyTuple_GetItem(arg1: *mut PyObject, arg2: Py_ssize_t) -> *mut PyObject;
    pub fn PyTuple_SetItem(arg1: *mut PyObject, arg2: Py_ssize_t, arg3: *mut PyObject) -> int;
    pub fn PyTuple_GetSlice(arg1: *mut PyObject, arg2: Py_ssize_t, arg3: Py_ssize_t) -> *mut PyObject;
    pub fn PyTuple_Pack(arg1: Py_ssize_t, ...) -> *mut PyObject;
}

#[cfg(not(Py_LIMITED_API))]
#[inline(always)]
pub unsafe fn PyTuple_GET_ITEM(op: *mut PyObject, i: Py_ssize_t) -> *mut PyObject {
   *(*(op as *mut PyTupleObject)).ob_item.as_ptr().offset(i as isize)
}

#[cfg(not(Py_LIMITED_API))]
#[inline(always)]
pub unsafe fn PyTuple_GET_SIZE(op: *mut PyObject) -> Py_ssize_t {
    assert!(PyTuple_Check(op) != 0);
    Py_SIZE(op as *mut PyVarObject)
}

#[cfg(not(Py_LIMITED_API))]
#[inline(always)]
pub unsafe fn PyTuple_SET_ITEM(op: *mut PyObject, i: Py_ssize_t, v: *mut PyObject) {
   *(*(op as *mut PyTupleObject)).ob_item.as_mut_ptr().offset(i as isize) = v;
}

extern "C" {
    pub fn PyTuple_ClearFreeList() -> int;
}
