cfg_if! {
    if #[cfg(not(Py_LIMITED_API))] {
        use swim_c_sys::{cchar, int};
        use crate::object::{PyObject, PyTypeObject, Py_TYPE};

        pub enum PyContext {
            // Opaque.
        }
        extern "C" {
            pub static mut PyContext_Type: PyTypeObject;
        }

        pub enum PyContextVar {
            // Opaque;
        }
        extern "C" {
            pub static mut PyContextVar_Type: PyTypeObject;
        }

        pub enum PyContextToken {
            // Opaque;
        }
        extern "C" {
            pub static mut PyContextToken_Type: PyTypeObject;
        }

        #[inline(always)]
        pub unsafe fn PyContext_CheckExact(o: *mut PyObject) -> int {
            (Py_TYPE(o) == &mut PyContext_Type) as int
        }
        #[inline(always)]
        pub unsafe fn PyContextVar_CheckExact(o: *mut PyObject) -> int {
            (Py_TYPE(o) == &mut PyContextVar_Type) as int
        }
        #[inline(always)]
        pub unsafe fn PyContextToken_CheckExact(o: *mut PyObject) -> int {
            (Py_TYPE(o) == &mut PyContextToken_Type) as int
        }

        extern "C" {
            pub fn PyContext_New() -> *mut PyContext;
            pub fn PyContext_Copy(context: *mut PyContext) -> *mut PyContext;
            pub fn PyContext_CopyCurrent() -> *mut PyContext;

            pub fn PyContext_Enter(context: *mut PyContext) -> int;
            pub fn PyContext_Exit(context: *mut PyContext) -> int;

            pub fn PyContextVar_New(name: *const cchar, default_value: *mut PyObject) -> *mut PyContextVar;

            pub fn PyContextVar_Get(var: *mut PyContextVar, default_value: *mut PyObject, value: *mut *mut PyObject) -> int;

            pub fn PyContextVar_Set(var: *mut PyContextVar, value: *mut PyObject) -> *mut PyContextToken;

            pub fn PyContextVar_Reset(var: *mut PyContextVar, token: *mut PyContextToken) -> int;

            pub fn _PyContext_NewHamtForTests() -> *mut PyObject;

            pub fn PyContext_ClearFreeList() -> int;
        }
    }
}
