use swim_c_sys::{int, size_t, void};
use crate::pyconfig::Py_ssize_t;
use crate::object::{PyObject, PyVarObject, PyTypeObject, Py_TYPE, PyType_HasFeature};

extern "C" {
    #[cfg_attr(not(all(py_sys_config="Py_DEBUG", not(Py_3_4))), link_name = "_PyObject_DebugMalloc")]
    pub fn _PyObject_DebugMalloc(arg1: size_t) -> *mut void;
    #[cfg(any(Py_LIMITED_API, Py_3_5))]
    pub fn PyObject_Calloc(nelem: size_t, elsize: size_t) -> *mut void;
    #[cfg_attr(not(all(py_sys_config="Py_DEBUG", not(Py_3_4))), link_name = "_PyObject_DebugRealloc")]
    pub fn _PyObject_DebugRealloc(arg1: *mut void, arg2: size_t) -> *mut void;
    #[cfg_attr(not(all(py_sys_config="Py_DEBUG", not(Py_3_4))), link_name = "_PyObject_DebugFree")]
    pub fn _PyObject_DebugFree(arg1: *mut void);

    #[cfg(any(Py_LIMITED_API, Py_3_4))]
    pub fn _Py_GetAllocatedBlocks() -> Py_ssize_t;

    pub fn PyObject_Init(arg1: *mut PyObject, arg2: *mut PyTypeObject) -> *mut PyObject;
    pub fn PyObject_InitVar(arg1: *mut PyVarObject, arg2: *mut PyTypeObject, arg3: Py_ssize_t) -> *mut PyVarObject;
    pub fn _PyObject_New(arg1: *mut PyTypeObject) -> *mut PyObject;
    pub fn _PyObject_NewVar(arg1: *mut PyTypeObject, arg2: Py_ssize_t) -> *mut PyVarObject;
}

cfg_if! {
    if #[cfg(all(not(Py_LIMITED_API), Py_3_4))] {
        #[derive(Copy)]
        #[repr(C)]
        pub struct PyObjectArenaAllocator {
            pub ctx: *mut void,
            pub alloc: Option<extern "C" fn (ctx: *mut void, size: size_t) -> *mut void>,
            pub free: Option<extern "C" fn (ctx: *mut void, ptr: *mut void, size: size_t)>,
        }
        impl Clone for PyObjectArenaAllocator {
            #[inline(always)]
            fn clone(&self) -> PyObjectArenaAllocator {
                *self
            }
        }
        impl Default for PyObjectArenaAllocator {
            #[inline(always)]
            fn default() -> PyObjectArenaAllocator {
                unsafe { core::mem::zeroed() }
            }
        }

        extern "C" {
            pub fn PyObject_GetArenaAllocator(allocator: *mut PyObjectArenaAllocator);
            pub fn PyObject_SetArenaAllocator(allocator: *mut PyObjectArenaAllocator);
        }
    }
}

extern "C" {
    pub fn PyGC_Collect() -> Py_ssize_t;

    #[cfg(not(Py_LIMITED_API))]
    pub fn _PyGC_CollectNoFail() -> Py_ssize_t;
    #[cfg(not(Py_LIMITED_API))]
    pub fn _PyGC_CollectIfEnabled() -> Py_ssize_t;
}

#[inline(always)]
pub unsafe fn PyType_IS_GC(t: *mut PyTypeObject) -> int {
    PyType_HasFeature(t, crate::object::Py_TPFLAGS_HAVE_GC)
}

#[inline(always)]
pub unsafe fn PyObject_IS_GC(o: *mut PyObject) -> int {
    (PyType_IS_GC(Py_TYPE(o)) != 0 && match (*Py_TYPE(o)).tp_is_gc {
        Some(tp_is_gc) => tp_is_gc(o) != 0,
        None => true,
    }) as int
}

extern "C" {
    pub fn _PyObject_GC_Resize(arg1: *mut PyVarObject, arg2: Py_ssize_t) -> *mut PyVarObject;

    #[cfg(not(Py_LIMITED_API))]
    pub fn _PyObject_GC_Malloc(size: size_t) -> *mut PyObject;
    #[cfg(all(not(Py_LIMITED_API), Py_3_5))]
    pub fn _PyObject_GC_Calloc(size: size_t) -> *mut PyObject;
    pub fn _PyObject_GC_New(arg1: *mut PyTypeObject) -> *mut PyObject;
    pub fn _PyObject_GC_NewVar(arg1: *mut PyTypeObject, arg2: Py_ssize_t) -> *mut PyVarObject;
    pub fn PyObject_GC_Track(arg1: *mut void);
    pub fn PyObject_GC_UnTrack(arg1: *mut void);
    pub fn PyObject_GC_Del(arg1: *mut void);
 }

#[cfg(not(Py_LIMITED_API))]
#[inline(always)]
pub unsafe fn PyType_SUPPORTS_WEAKREFS(t: *mut PyTypeObject) -> int {
    ((*t).tp_weaklistoffset > 0) as int
}

#[cfg(not(Py_LIMITED_API))]
#[inline(always)]
pub unsafe fn PyObject_GET_WEAKREFS_LISTPTR(o: *mut PyObject) -> *mut *mut PyObject {
    let weaklistoffset = (*Py_TYPE(o)).tp_weaklistoffset as isize;
    (o as *mut u8).offset(weaklistoffset) as *mut *mut PyObject
}
