use swim_c_sys::{cchar, int, void};
use crate::pyconfig::Py_ssize_t;
use crate::methodobject::PyMethodDef;
use crate::object::{PyObject, PyTypeObject, Py_TYPE, PyObject_TypeCheck};

extern "C" {
    pub static mut PyModule_Type: PyTypeObject;
}

#[inline(always)]
pub unsafe fn PyModule_Check(op: *mut PyObject) -> int {
    PyObject_TypeCheck(op, &mut PyModule_Type)
}
#[inline(always)]
pub unsafe fn PyModule_CheckExact(op: *mut PyObject) -> int {
    (Py_TYPE(op) == &mut PyModule_Type) as int
}

extern "C" {
    pub fn PyModule_NewObject(name: *mut PyObject) -> *mut PyObject;
    pub fn PyModule_New(name: *const cchar) -> *mut PyObject;
    pub fn PyModule_GetDict(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyModule_GetNameObject(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyModule_GetName(arg1: *mut PyObject) -> *const cchar;
    pub fn PyModule_GetFilename(arg1: *mut PyObject) -> *const cchar;
    pub fn PyModule_GetFilenameObject(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyModule_GetDef(arg1: *mut PyObject) -> *mut PyModuleDef;
    pub fn PyModule_GetState(arg1: *mut PyObject) -> *mut void;

    #[cfg(any(not(Py_LIMITED_API), Py_3_5))]
    pub fn PyModuleDef_Init(arg1: *mut PyModuleDef) -> *mut PyObject;
    #[cfg(any(not(Py_LIMITED_API), Py_3_5))]
    pub static mut PyModuleDef_Type: PyTypeObject;
}

#[derive(Copy)]
#[repr(C)]
pub struct PyModuleDef_Base {
    pub ob_base: PyObject,
    pub m_init: Option<extern "C" fn () -> *mut PyObject>,
    pub m_index: Py_ssize_t,
    pub m_copy: *mut PyObject,
}
impl Clone for PyModuleDef_Base {
    #[inline(always)]
    fn clone(&self) -> PyModuleDef_Base {
        *self
    }
}

cfg_if! {
    if #[cfg(any(not(Py_LIMITED_API), Py_3_5))] {
        #[derive(Copy)]
        #[repr(C)]
        pub struct PyModuleDef_Slot {
            pub slot: int,
            pub value: *mut void,
        }
        impl Clone for PyModuleDef_Slot {
            #[inline(always)]
            fn clone(&self) -> PyModuleDef_Slot {
                *self
            }
        }

        pub const Py_mod_create: int = 1;
        pub const Py_mod_exec: int = 2;

        #[cfg(not(Py_LIMITED_API))]
        pub const _Py_mod_LAST_SLOT: int = 2;
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct PyModuleDef {
    pub m_base: PyModuleDef_Base,
    pub m_name: *const cchar,
    pub m_doc: *const cchar,
    pub m_size: Py_ssize_t,
    pub m_methods: *mut PyMethodDef,
    #[cfg(not(Py_3_5))]
    pub m_reload: Option<crate::object::inquiry>,
    #[cfg(Py_3_5)]
    pub m_slots: *mut PyModuleDef_Slot,
    pub m_traverse: Option<crate::object::traverseproc>,
    pub m_clear: Option<crate::object::inquiry>,
    pub m_free: Option<crate::object::freefunc>,
}
impl Clone for PyModuleDef {
    #[inline(always)]
    fn clone(&self) -> PyModuleDef {
        *self
    }
}
