use swim_c_sys::{cchar, int, long};
use crate::pyconfig::Py_ssize_t;
use crate::moduleobject::PyModuleDef;
use crate::object::PyObject;

extern "C" {
    pub fn PyArg_Parse(arg1: *mut PyObject, arg2: *const cchar, ...) -> int;
    pub fn PyArg_ParseTuple(arg1: *mut PyObject, arg2: *const cchar, ...) -> int;
    pub fn PyArg_ParseTupleAndKeywords(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *const cchar, arg4: *mut *mut cchar, ...) -> int;
    pub fn PyArg_ValidateKeywordArguments(arg1: *mut PyObject) -> int;
    pub fn PyArg_UnpackTuple(arg1: *mut PyObject, arg2: *const cchar, arg3: Py_ssize_t, arg4: Py_ssize_t, ...) -> int;
    pub fn Py_BuildValue(arg1: *const cchar, ...) -> *mut PyObject;

    pub fn PyModule_AddObject(arg1: *mut PyObject, arg2: *const cchar, arg3: *mut PyObject) -> int;
    pub fn PyModule_AddIntConstant(arg1: *mut PyObject, arg2: *const cchar, arg3: long) -> int;
    pub fn PyModule_AddStringConstant(arg1: *mut PyObject, arg2: *const cchar, arg3: *const cchar);

    #[cfg(Py_3_5)]
    pub fn PyModule_SetDocString(arg1: *mut PyObject, arg2: *const cchar) -> int;
    #[cfg(Py_3_5)]
    pub fn PyModule_AddFunctions(arg1: *mut PyObject, arg2: *mut crate::methodobject::PyMethodDef) -> int;
    #[cfg(Py_3_5)]
    pub fn PyModule_ExecDef(module: *mut PyObject, def: *mut PyModuleDef) -> int;
}

pub const Py_CLEANUP_SUPPORTED: i32 = 0x20000;

pub const PYTHON_API_VERSION: i32 = 1013;
pub const PYTHON_API_STRING: &'static str = "1013";
pub const PYTHON_ABI_VERSION: i32 = 3;
pub const PYTHON_ABI_STRING: &'static str = "3";

extern "C" {
    #[cfg(not(py_sys_config="Py_TRACE_REFS"))]
    pub fn PyModule_Create2(module: *mut PyModuleDef, apiver: int) -> *mut PyObject;
}

#[inline(always)]
pub unsafe fn PyModule_Create(module: *mut PyModuleDef) -> *mut PyObject {
    PyModule_Create2(module, if cfg!(Py_LIMITED_API) { PYTHON_ABI_VERSION } else { PYTHON_API_VERSION })
}

extern "C" {
    #[cfg(not(py_sys_config="Py_TRACE_REFS"))]
    #[cfg(Py_3_5)]
    pub fn PyModule_FromDefAndSpec2(def: *mut PyModuleDef, spec: *mut PyObject, module_api_version: int) -> *mut PyObject;
}

#[cfg(Py_3_5)]
#[inline(always)]
pub unsafe fn PyModule_FromDefAndSpec(def: *mut PyModuleDef, spec: *mut PyObject) -> *mut PyObject {
    PyModule_FromDefAndSpec2(def, spec, if cfg!(Py_LIMITED_API) { PYTHON_ABI_VERSION } else { PYTHON_API_VERSION })
}
