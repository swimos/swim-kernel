use swim_c_sys::{cchar, int, void};
use crate::methodobject::PyMethodDef;
use crate::object::{PyObject, PyTypeObject};
use crate::structmember::PyMemberDef;

pub type getter = unsafe extern "C" fn (slf: *mut PyObject, closure: *mut void) -> *mut PyObject;
pub type setter = unsafe extern "C" fn (slf: *mut PyObject, value: *mut PyObject, closure: *mut void) -> int;

#[derive(Copy)]
#[repr(C)]
pub struct PyGetSetDef {
    pub name: *mut cchar,
    pub get: Option<getter>,
    pub set: Option<setter>,
    pub doc: *mut cchar,
    pub closure: *mut void,
}
impl Clone for PyGetSetDef {
    #[inline(always)]
    fn clone(&self) -> PyGetSetDef {
        *self
    }
}

extern "C" {
    pub static mut PyClassMethodDescr_Type: PyTypeObject;
    pub static mut PyGetSetDescr_Type: PyTypeObject;
    pub static mut PyMemberDescr_Type: PyTypeObject;
    pub static mut PyMethodDescr_Type: PyTypeObject;
    pub static mut PyWrapperDescr_Type: PyTypeObject;
    pub static mut PyDictProxy_Type: PyTypeObject;

    pub fn PyDescr_NewMethod(arg1: *mut PyTypeObject, arg2: *mut PyMethodDef) -> *mut PyObject;
    pub fn PyDescr_NewClassMethod(arg1: *mut PyTypeObject, arg2: *mut PyMethodDef) -> *mut PyObject;
    pub fn PyDescr_NewMember(arg1: *mut PyTypeObject, arg2: *mut PyMemberDef) -> *mut PyObject;
    pub fn PyDescr_NewGetSet(arg1: *mut PyTypeObject, arg2: *mut PyGetSetDef) -> *mut PyObject;

    pub fn PyDictProxy_New(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyWrapper_New(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;

    pub static mut PyProperty_Type: PyTypeObject;
}
