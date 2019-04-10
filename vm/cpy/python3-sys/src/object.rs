use core::ptr;
use swim_c_sys::{cchar, int, uint, ulong, void};
use crate::pyconfig::{Py_ssize_t, Py_hash_t};
use crate::descrobject::PyGetSetDef;
use crate::methodobject::PyMethodDef;
use crate::structmember::PyMemberDef;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PyObject {
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    _ob_next: *mut PyObject,
    #[cfg(py_sys_config="Py_TRACE_REFS")]
    _ob_prev: *mut PyObject,
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut PyTypeObject,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PyVarObject {
    pub ob_base: PyObject,
    pub ob_size: Py_ssize_t,
}

#[inline(always)]
pub unsafe fn Py_REFCNT(ob: *mut PyObject) -> Py_ssize_t {
    (*ob).ob_refcnt
}

#[inline(always)]
pub unsafe fn Py_TYPE(ob: *mut PyObject) -> *mut PyTypeObject {
    (*ob).ob_type
}

#[inline(always)]
pub unsafe fn Py_SIZE(ob: *mut PyVarObject) -> Py_ssize_t {
    (*ob).ob_size
}

pub type unaryfunc = unsafe extern "C" fn (arg1: *mut PyObject) -> *mut PyObject;
pub type binaryfunc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut PyObject)-> *mut PyObject;
pub type ternaryfunc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> *mut PyObject;
pub type inquiry = unsafe extern "C" fn(arg1: *mut PyObject) -> int;
pub type lenfunc = unsafe extern "C" fn(arg1: *mut PyObject) -> Py_ssize_t;
pub type ssizeargfunc = unsafe extern "C" fn(arg1: *mut PyObject, arg2: Py_ssize_t) -> *mut PyObject;
pub type ssizessizeargfunc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: Py_ssize_t, arg3: Py_ssize_t) -> *mut PyObject;
pub type ssizeobjargproc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: Py_ssize_t, arg3: *mut PyObject) -> int;
pub type ssizessizeobjargproc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: Py_ssize_t, arg3: Py_ssize_t, arg4: *mut PyObject) -> int;
pub type objobjargproc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> int;

cfg_if! {
    if #[cfg(not(Py_LIMITED_API))] {
        #[derive(Copy)]
        #[repr(C)]
        pub struct Py_buffer {
            pub buf: *mut void,
            pub obj: *mut PyObject,
            pub len: Py_ssize_t,
            pub itemsize: Py_ssize_t,
            pub readonly: int,
            pub ndim: int,
            pub format: *mut cchar,
            pub shape: *mut Py_ssize_t,
            pub strides: *mut Py_ssize_t,
            pub suboffsets: *mut Py_ssize_t,
            pub internal: *mut void,
        }
        impl Clone for Py_buffer {
            #[inline(always)]
            fn clone(&self) -> Py_buffer {
                *self
            }
        }
        impl Default for Py_buffer {
            #[inline(always)]
            fn default() -> Py_buffer {
                unsafe { core::mem::zeroed() }
            }
        }

        pub type getbufferproc = extern "C" fn (arg1: *mut PyObject, arg2: *mut Py_buffer, arg3: int) -> int;
        pub type releasebufferproc = extern "C" fn (arg1: *mut PyObject, arg2: *mut Py_buffer);

        /// Maximum number of dimensions
        pub const PyBUF_MAX_NDIM: int = 64;

        // Flags for getting buffers
        pub const PyBUF_SIMPLE: int = 0;
        pub const PyBUF_WRITABLE: int = 0x0001;
        //  we used to include an E, backwards compatible alias
        pub const PyBUF_WRITEABLE: int = PyBUF_WRITABLE;
        pub const PyBUF_FORMAT: int = 0x0004;
        pub const PyBUF_ND: int = 0x0008;
        pub const PyBUF_STRIDES: int = 0x0010 | PyBUF_ND;
        pub const PyBUF_C_CONTIGUOUS: int = 0x0020 | PyBUF_STRIDES;
        pub const PyBUF_F_CONTIGUOUS: int = 0x0040 | PyBUF_STRIDES;
        pub const PyBUF_ANY_CONTIGUOUS: int = 0x0080 | PyBUF_STRIDES;
        pub const PyBUF_INDIRECT: int = 0x0100 | PyBUF_STRIDES;

        pub const PyBUF_CONTIG: int = PyBUF_ND | PyBUF_WRITABLE;
        pub const PyBUF_CONTIG_RO: int = PyBUF_ND;

        pub const PyBUF_STRIDED: int = PyBUF_STRIDES | PyBUF_WRITABLE;
        pub const PyBUF_STRIDED_RO: int = PyBUF_STRIDES;

        pub const PyBUF_RECORDS: int = PyBUF_STRIDES | PyBUF_WRITABLE | PyBUF_FORMAT;
        pub const PyBUF_RECORDS_RO: int = PyBUF_STRIDES | PyBUF_FORMAT;

        pub const PyBUF_FULL: int = PyBUF_INDIRECT | PyBUF_WRITABLE | PyBUF_FORMAT;
        pub const PyBUF_FULL_RO: int = PyBUF_INDIRECT | PyBUF_FORMAT;

        pub const PyBUF_READ: int = 0x100;
        pub const PyBUF_WRIT: int = 0x200;
    }
}

pub type objobjproc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut PyObject) -> int;
pub type visitproc = unsafe extern "C" fn (object: *mut PyObject, arg: *mut void) -> int;
pub type traverseproc = unsafe extern "C" fn (slf: *mut PyObject, visit: visitproc, arg: *mut void) -> int;

cfg_if! {
    if #[cfg(not(Py_LIMITED_API))] {
        #[derive(Copy)]
        #[repr(C)]
        pub struct PyNumberMethods {
            pub nb_add: Option<binaryfunc>,
            pub nb_subtract: Option<binaryfunc>,
            pub nb_multiply: Option<binaryfunc>,
            pub nb_remainder: Option<binaryfunc>,
            pub nb_divmod: Option<binaryfunc>,
            pub nb_power: Option<ternaryfunc>,
            pub nb_negative: Option<unaryfunc>,
            pub nb_positive: Option<unaryfunc>,
            pub nb_absolute: Option<unaryfunc>,
            pub nb_bool: Option<inquiry>,
            pub nb_invert: Option<unaryfunc>,
            pub nb_lshift: Option<binaryfunc>,
            pub nb_rshift: Option<binaryfunc>,
            pub nb_and: Option<binaryfunc>,
            pub nb_xor: Option<binaryfunc>,
            pub nb_or: Option<binaryfunc>,
            pub nb_int: Option<unaryfunc>,
            pub nb_reserved: *mut void,
            pub nb_float: Option<unaryfunc>,
            pub nb_inplace_add: Option<binaryfunc>,
            pub nb_inplace_subtract: Option<binaryfunc>,
            pub nb_inplace_multiply: Option<binaryfunc>,
            pub nb_inplace_remainder: Option<binaryfunc>,
            pub nb_inplace_power: Option<ternaryfunc>,
            pub nb_inplace_lshift: Option<binaryfunc>,
            pub nb_inplace_rshift: Option<binaryfunc>,
            pub nb_inplace_and: Option<binaryfunc>,
            pub nb_inplace_xor: Option<binaryfunc>,
            pub nb_inplace_or: Option<binaryfunc>,
            pub nb_floor_divide: Option<binaryfunc>,
            pub nb_true_divide: Option<binaryfunc>,
            pub nb_inplace_floor_divide: Option<binaryfunc>,
            pub nb_inplace_true_divide: Option<binaryfunc>,
            pub nb_index: Option<unaryfunc>,
            #[cfg(Py_3_5)]
            pub nb_matrix_multiply: Option<binaryfunc>,
            #[cfg(Py_3_5)]
            pub nb_inplace_matrix_multiply: Option<binaryfunc>,
        }
        impl Clone for PyNumberMethods {
            #[inline(always)]
            fn clone(&self) -> PyNumberMethods {
                *self
            }
        }
        impl Default for PyNumberMethods {
            #[inline(always)]
            fn default() -> PyNumberMethods {
                unsafe { core::mem::zeroed() }
            }
        }

        #[derive(Copy)]
        #[repr(C)]
        pub struct PySequenceMethods {
            pub sq_length: Option<lenfunc>,
            pub sq_concat: Option<binaryfunc>,
            pub sq_repeat: Option<ssizeargfunc>,
            pub sq_item: Option<ssizeargfunc>,
            pub was_sq_slice: *mut void,
            pub sq_ass_item: Option<ssizeobjargproc>,
            pub was_sq_ass_slice: *mut void,
            pub sq_contains: Option<objobjproc>,
            pub sq_inplace_concat: Option<binaryfunc>,
            pub sq_inplace_repeat: Option<ssizeargfunc>,
        }
        impl Clone for PySequenceMethods {
            #[inline(always)]
            fn clone(&self) -> PySequenceMethods {
                *self
            }
        }
        impl Default for PySequenceMethods {
            #[inline(always)]
            fn default() -> PySequenceMethods {
                unsafe { core::mem::zeroed() }
            }
        }

        #[derive(Copy)]
        #[repr(C)]
        pub struct PyMappingMethods {
            pub mp_length: Option<lenfunc>,
            pub mp_subscript: Option<binaryfunc>,
            pub mp_ass_subscript: Option<objobjargproc>,
        }
        impl Clone for PyMappingMethods {
            #[inline(always)]
            fn clone(&self) -> PyMappingMethods {
                *self
            }
        }
        impl Default for PyMappingMethods {
            #[inline(always)]
            fn default() -> PyMappingMethods {
                unsafe { core::mem::zeroed() }
            }
        }

        cfg_if! {
            if #[cfg(Py_3_5)] {
                #[derive(Copy)]
                #[repr(C)]
                pub struct PyAsyncMethods {
                    pub am_await: Option<unaryfunc>,
                    pub am_aiter: Option<unaryfunc>,
                    pub am_anext: Option<unaryfunc>,
                }
                impl Clone for PyAsyncMethods {
                    #[inline(always)]
                    fn clone(&self) -> PyAsyncMethods {
                        *self
                    }
                }
                impl Default for PyAsyncMethods {
                    #[inline(always)]
                    fn default() -> PyAsyncMethods {
                        unsafe { core::mem::zeroed() }
                    }
                }
            }
        }

        #[derive(Copy)]
        #[repr(C)]
        pub struct PyBufferProcs {
            pub bf_getbuffer: Option<getbufferproc>,
            pub bf_releasebuffer: Option<releasebufferproc>,
        }
        impl Clone for PyBufferProcs {
            #[inline(always)]
            fn clone(&self) -> PyBufferProcs {
                *self
            }
        }
        impl Default for PyBufferProcs {
            #[inline(always)]
            fn default() -> PyBufferProcs {
                unsafe { core::mem::zeroed() }
            }
        }
    }
}

pub type freefunc = unsafe extern "C" fn (arg1: *mut void);
pub type destructor = unsafe extern "C" fn (arg1: *mut PyObject);
#[cfg(not(Py_LIMITED_API))]
pub type printfunc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut crate::swim_c_sys::stdio::FILE, arg3: int) -> int;

pub type getattrfunc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut cchar) -> *mut PyObject;
pub type getattrofunc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;
pub type setattrfunc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut cchar, arg3: *mut PyObject) -> int;
pub type setattrofunc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> int;
pub type reprfunc = unsafe extern "C" fn (arg1: *mut PyObject) -> *mut PyObject;
pub type hashfunc = unsafe extern "C" fn (arg1: *mut PyObject) -> Py_hash_t;
pub type richcmpfunc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut PyObject, arg3: int) -> *mut PyObject;
pub type getiterfunc = unsafe extern "C" fn (arg1: *mut PyObject) -> *mut PyObject;
pub type iternextfunc = unsafe extern "C" fn (arg1: *mut PyObject) -> *mut PyObject;
pub type descrgetfunc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> *mut PyObject;
pub type descrsetfunc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> int;
pub type initproc = unsafe extern "C" fn (arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> int;
pub type newfunc = unsafe extern "C" fn (arg1: *mut PyTypeObject, arg2: *mut PyObject, arg3: *mut PyObject) -> *mut PyObject;
pub type allocfunc = unsafe extern "C" fn (arg1: *mut PyTypeObject, arg2: Py_ssize_t) -> *mut PyObject;

cfg_if! {
    if #[cfg(Py_LIMITED_API)] {
        pub enum PyTypeObject {
            // Opaque.
        }
    } else {
        #[derive(Copy)]
        #[repr(C)]
        pub struct PyTypeObject {
            pub ob_base: PyVarObject,
            pub tp_name: *const cchar,
            pub tp_basicsize: Py_ssize_t,
            pub tp_itemsize: Py_ssize_t,
            pub tp_dealloc: Option<destructor>,
            pub tp_print: Option<printfunc>,
            pub tp_getattr: Option<getattrfunc>,
            pub tp_setattr: Option<setattrfunc>,
            #[cfg(Py_3_5)]
            pub tp_as_async: *mut PyAsyncMethods,
            #[cfg(not(Py_3_5))]
            pub tp_reserved: *mut void,
            pub tp_repr: Option<reprfunc>,
            pub tp_as_number: *mut PyNumberMethods,
            pub tp_as_sequence: *mut PySequenceMethods,
            pub tp_as_mapping: *mut PyMappingMethods,
            pub tp_hash: Option<hashfunc>,
            pub tp_call: Option<ternaryfunc>,
            pub tp_str: Option<reprfunc>,
            pub tp_getattro: Option<getattrofunc>,
            pub tp_setattro: Option<setattrofunc>,
            pub tp_as_buffer: *mut PyBufferProcs,
            pub tp_flags: ulong,
            pub tp_doc: *const cchar,
            pub tp_traverse: Option<traverseproc>,
            pub tp_clear: Option<inquiry>,
            pub tp_richcompare: Option<richcmpfunc>,
            pub tp_weaklistoffset: Py_ssize_t,
            pub tp_iter: Option<getiterfunc>,
            pub tp_iternext: Option<iternextfunc>,
            pub tp_methods: *mut PyMethodDef,
            pub tp_members: *mut PyMemberDef,
            pub tp_getset: *mut PyGetSetDef,
            pub tp_base: *mut PyTypeObject,
            pub tp_dict: *mut PyObject,
            pub tp_descr_get: Option<descrgetfunc>,
            pub tp_descr_set: Option<descrsetfunc>,
            pub tp_dictoffset: Py_ssize_t,
            pub tp_init: Option<initproc>,
            pub tp_alloc: Option<allocfunc>,
            pub tp_new: Option<newfunc>,
            pub tp_free: Option<freefunc>,
            pub tp_is_gc: Option<inquiry>,
            pub tp_bases: *mut PyObject,
            pub tp_mro: *mut PyObject,
            pub tp_cache: *mut PyObject,
            pub tp_subclasses: *mut PyObject,
            pub tp_weaklist: *mut PyObject,
            pub tp_del: Option<destructor>,
            pub tp_version_tag: uint,
            #[cfg(Py_3_4)]
            pub tp_finalize: Option<destructor>,
            #[cfg(py_sys_config="COUNT_ALLOCS")]
            pub tp_allocs: Py_ssize_t,
            #[cfg(py_sys_config="COUNT_ALLOCS")]
            pub tp_frees: Py_ssize_t,
            #[cfg(py_sys_config="COUNT_ALLOCS")]
            pub tp_maxalloc: Py_ssize_t,
            #[cfg(py_sys_config="COUNT_ALLOCS")]
            pub tp_prev: *mut PyTypeObject,
            #[cfg(py_sys_config="COUNT_ALLOCS")]
            pub tp_next: *mut PyTypeObject,
        }
        impl Clone for PyTypeObject {
            #[inline(always)]
            fn clone(&self) -> PyTypeObject {
                *self
            }
        }
        impl Default for PyTypeObject {
            #[inline(always)]
            fn default() -> PyTypeObject {
                unsafe { core::mem::zeroed() }
            }
        }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct PyType_Slot {
    pub slot: int,
    pub pfunc: *mut void,
}
impl Clone for PyType_Slot {
    #[inline(always)]
    fn clone(&self) -> PyType_Slot {
        *self
    }
}
impl Default for PyType_Slot {
    #[inline(always)]
    fn default() -> PyType_Slot {
        unsafe { core::mem::zeroed() }
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct PyType_Spec {
    pub name: *const cchar,
    pub basicsize: int,
    pub itemsize: int,
    pub flags: uint,
    pub slots: *mut PyType_Slot,
}
impl Clone for PyType_Spec {
    #[inline(always)]
    fn clone(&self) -> PyType_Spec {
        *self
    }
}
impl Default for PyType_Spec {
    #[inline(always)]
    fn default() -> PyType_Spec {
        unsafe { core::mem::zeroed() }
    }
}

extern "C" {
    pub fn PyType_FromSpec(arg1: *mut PyType_Spec) -> *mut PyObject;
    #[cfg(any(not(Py_LIMITED_API), Py_3_3))]
    pub fn PyType_FromSpecWithBases(arg1: *mut PyType_Spec, arg2: *mut PyObject) -> *mut PyObject;
    #[cfg(any(not(Py_LIMITED_API), Py_3_4))]
    pub fn PyType_GetSlot(arg1: *mut PyTypeObject, arg2: int) -> *mut void;
}

cfg_if! {
    if #[cfg(not(Py_LIMITED_API))] {
        #[derive(Copy)]
        #[repr(C)]
        pub struct PyHeapTypeObject {
            pub ht_type: PyTypeObject,
            #[cfg(Py_3_5)]
            pub as_async: PyAsyncMethods,
            pub as_number: PyNumberMethods,
            pub as_mapping: PyMappingMethods,
            pub as_sequence: PySequenceMethods,
            pub as_buffer: PyBufferProcs,
            pub ht_name: *mut PyObject,
            pub ht_slots: *mut PyObject,
            pub ht_qualname: *mut PyObject,
            pub ht_cached_keys: *mut void,
        }
        impl Clone for PyHeapTypeObject {
            #[inline(always)]
            fn clone(&self) -> PyHeapTypeObject {
                *self
            }
        }
        impl Default for PyHeapTypeObject {
            #[inline(always)]
            fn default() -> PyHeapTypeObject {
                unsafe { core::mem::zeroed() }
            }
        }

        //#[inline(always)]
        //pub unsafe fn PyHeapType_GET_MEMBERS(etype: *mut PyHeapTypeObject) -> *mut PyMemberDef {
        //    (etype as *mut cchar).offset((*::object::Py_TYPE(etype as *mut crate::object::PyObject)).tp_basicsize as isize) as *mut crate::structmember::PyMemberDef
        //}
    }
}

extern "C" {
    pub fn PyType_IsSubtype(a: *mut PyTypeObject, b: *mut PyTypeObject) -> int;
}
#[inline(always)]
pub unsafe fn PyObject_TypeCheck(ob: *mut PyObject, tp: *mut PyTypeObject) -> int {
    (Py_TYPE(ob) == tp || PyType_IsSubtype(Py_TYPE(ob), tp) != 0) as int
}

extern "C" {
    /// built-in 'type'
    pub static mut PyType_Type: PyTypeObject;
    /// built-in 'object'
    pub static mut PyBaseObject_Type: PyTypeObject;
    /// built-in 'super'
    pub static mut PySuper_Type: PyTypeObject;
    
    pub fn PyType_GetFlags(arg1: *mut PyTypeObject) -> ulong;
}

#[inline(always)]
pub unsafe fn PyType_Check(op: *mut PyObject) -> int {
    PyType_FastSubclass(Py_TYPE(op), Py_TPFLAGS_TYPE_SUBCLASS)
}
#[inline(always)]
pub unsafe fn PyType_CheckExact(op: *mut PyObject) -> int {
    (Py_TYPE(op) == &mut PyType_Type) as int
}

extern "C" {
    pub fn PyType_Ready(t: *mut PyTypeObject) -> int;
    pub fn PyType_GenericAlloc(t: *mut PyTypeObject, nitems: Py_ssize_t) -> *mut PyObject;
    pub fn PyType_GenericNew(t: *mut PyTypeObject, args: *mut PyObject, kwds: *mut PyObject) -> *mut PyObject;

    pub fn PyType_ClearCache() -> uint;
    pub fn PyType_Modified(t: *mut PyTypeObject);

    #[cfg(not(Py_LIMITED_API))]
    pub fn PyObject_Print(o: *mut PyObject, fp: *mut crate::swim_c_sys::stdio::FILE, flags: int) -> int;

    pub fn PyObject_Repr(o: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_Str(o: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_ASCII(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_Bytes(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_RichCompare(arg1: *mut PyObject, arg2: *mut PyObject, arg3: int) -> *mut PyObject;
    pub fn PyObject_RichCompareBool(arg1: *mut PyObject, arg2: *mut PyObject, arg3: int) -> int;
    pub fn PyObject_GetAttrString(arg1: *mut PyObject, arg2: *const cchar) -> *mut PyObject;
    pub fn PyObject_SetAttrString(arg1: *mut PyObject, arg2: *const cchar, arg3: *mut PyObject) -> int;
    pub fn PyObject_HasAttrString(arg1: *mut PyObject, arg2: *const cchar) -> int;
    pub fn PyObject_GetAttr(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_SetAttr(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> int;
    pub fn PyObject_HasAttr(arg1: *mut PyObject, arg2: *mut PyObject) -> int;
    pub fn PyObject_SelfIter(arg1: *mut PyObject) -> *mut PyObject;
    #[cfg(not(Py_LIMITED_API))]
    pub fn _PyObject_NextNotImplemented(arg1: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_GenericGetAttr(arg1: *mut PyObject, arg2: *mut PyObject) -> *mut PyObject;
    pub fn PyObject_GenericSetAttr(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject) -> int;
    #[cfg(any(not(Py_LIMITED_API), Py_3_3))]
    pub fn PyObject_GenericSetDict(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut void) -> int;
    pub fn PyObject_Hash(arg1: *mut PyObject) -> Py_hash_t;
    pub fn PyObject_HashNotImplemented(arg1: *mut PyObject) -> Py_hash_t;
    pub fn PyObject_IsTrue(arg1: *mut PyObject) -> int;
    pub fn PyObject_Not(arg1: *mut PyObject) -> int;
    pub fn PyCallable_Check(arg1: *mut PyObject) -> int;

    pub fn PyObject_ClearWeakRefs(arg1: *mut PyObject);
    #[cfg(all(not(Py_LIMITED_API), Py_3_4))]
    pub fn PyObject_CallFinalizer(arg1: *mut PyObject);
    #[cfg(all(not(Py_LIMITED_API), Py_3_4))]
    pub fn PyObject_CallFinalizerFromDealloc(arg1: *mut PyObject) -> int;

    pub fn PyObject_Dir(arg1: *mut PyObject) -> *mut PyObject;

    pub fn Py_ReprEnter(arg1: *mut PyObject) -> int;
    pub fn Py_ReprLeave(arg1: *mut PyObject);
}

// Flag bits for printing:
pub const Py_PRINT_RAW: int = 1; // No string quotes etc.

// Set if the type object is dynamically allocated
pub const Py_TPFLAGS_HEAPTYPE: ulong = 1<<9;

// Set if the type allows subclassing
pub const Py_TPFLAGS_BASETYPE: ulong = 1<<10;

// Set if the type is 'ready' -- fully initialized
pub const Py_TPFLAGS_READY: ulong = 1<<12;

// Set while the type is being 'readied', to prevent recursive ready calls
pub const Py_TPFLAGS_READYING: ulong = 1<<13;

// Objects support garbage collection (see objimp.h)
pub const Py_TPFLAGS_HAVE_GC: ulong = 1<<14;

const Py_TPFLAGS_HAVE_STACKLESS_EXTENSION: ulong = 0;

// Objects support type attribute cache
pub const Py_TPFLAGS_HAVE_VERSION_TAG: ulong = 1<<18;
pub const Py_TPFLAGS_VALID_VERSION_TAG: ulong = 1<<19;

// Type is abstract and cannot be instantiated
pub const Py_TPFLAGS_IS_ABSTRACT: ulong = 1<<20;

// These flags are used to determine if a type is a subclass.
pub const Py_TPFLAGS_LONG_SUBCLASS: ulong = 1<<24;
pub const Py_TPFLAGS_LIST_SUBCLASS: ulong = 1<<25;
pub const Py_TPFLAGS_TUPLE_SUBCLASS: ulong = 1<<26;
pub const Py_TPFLAGS_BYTES_SUBCLASS: ulong = 1<<27;
pub const Py_TPFLAGS_UNICODE_SUBCLASS: ulong = 1<<28;
pub const Py_TPFLAGS_DICT_SUBCLASS: ulong = 1<<29;
pub const Py_TPFLAGS_BASE_EXC_SUBCLASS: ulong = 1<<30;
pub const Py_TPFLAGS_TYPE_SUBCLASS : ulong = 1<<31;

pub const Py_TPFLAGS_DEFAULT: ulong = Py_TPFLAGS_HAVE_STACKLESS_EXTENSION | Py_TPFLAGS_HAVE_VERSION_TAG;

// Type structure has tp_finalize member (3.4)
pub const Py_TPFLAGS_HAVE_FINALIZE: ulong = 1<<0;

cfg_if! {
    if #[cfg(Py_LIMITED_API)] {
        #[inline(always)]
        pub unsafe fn PyType_HasFeature(t: *mut PyTypeObject, f: ulong) -> int {
            (PyType_GetFlags(t) & f != 0) as int
        }
    } else {
        #[inline(always)]
        pub unsafe fn PyType_HasFeature(t: *mut PyTypeObject, f: ulong) -> int {
            ((*t).tp_flags & f != 0) as int
        }
    }
}
#[inline(always)]
pub unsafe fn PyType_FastSubclass(t: *mut PyTypeObject, f: ulong) -> int {
    PyType_HasFeature(t, f)
}

cfg_if! {
    if #[cfg(py_sys_config="Py_REF_DEBUG")] {
        extern "C" {
            pub static mut _Py_RefTotal: Py_ssize_t;
            pub fn _Py_NegativeRefcount(fname: *const cchar, lineno: int, op: *mut PyObject);
            pub fn _Py_GetRefTotal() -> Py_ssize_t;
        }
        #[inline(always)]
        pub unsafe fn _Py_INC_REFTOTAL() {
            _Py_RefTotal += 1;
        }
        #[inline(always)]
        pub unsafe fn _Py_DEC_REFTOTAL() {
            _Py_RefTotal -= 1;
        }
        extern "C" {
            pub fn _PyDebug_PrintTotalRefs();
        }
    } else {
        #[inline(always)]
        pub unsafe fn _Py_INC_REFTOTAL() {
            // nop
        }
        #[inline(always)]
        pub unsafe fn _Py_DEC_REFTOTAL() {
            // nop
        }
    }
}

cfg_if! {
    if #[cfg(py_sys_config="COUNT_ALLOCS")] {
        extern "C" {
            pub fn inc_count(t: *mut PyTypeObject);
            pub fn dec_count(t: *mut PyTypeObject);
        }
        #[inline(always)]
        pub unsafe fn _Py_INC_TPALLOCS(op: *mut PyObject) {
            inc_count(Py_TYPE(op));
        }
        #[inline(always)]
        pub unsafe fn _Py_INC_TPFREES(op: *mut PyObject) {
            dec_count(Py_TYPE(op));
        }
        #[inline(always)]
        pub unsafe fn _Py_DEC_TPFREES(op: *mut PyObject) {
            (*Py_TYPE(OP)).tp_frees -= 1;
        }
    } else {
        #[inline(always)]
        pub unsafe fn inc_count(_t: *mut PyTypeObject) {
            // nop
        }
        #[inline(always)]
        pub unsafe fn dec_count(_t: *mut PyTypeObject) {
            // nop
        }
        #[inline(always)]
        pub unsafe fn _Py_INC_TPALLOCS(_op: *mut PyObject) {
            // nop
        }
        #[inline(always)]
        pub unsafe fn _Py_INC_TPFREES(_op: *mut PyObject) {
            // nop
        }
        #[inline(always)]
        pub unsafe fn _Py_DEC_TPFREES(_op: *mut PyObject) {
            // nop
        }
    }
}

cfg_if! {
    if #[cfg(py_sys_config="Py_TRACE_REFS")] {
        extern "C" {
            pub fn _Py_NewReference(op: *mut PyObject);
            pub fn _Py_ForgetReference(op: *mut PyObject);
            pub fn _Py_Dealloc(op: *mut PyObject);
            pub fn _Py_PrintReferences(file: *mut crate::swim_c_sys::stdio::FILE);
            pub fn _Py_PrintReferenceAddresses(file: *mut crate::swim_c_sys::stdio::FILE);
            pub fn _Py_AddToAllObjects(op: *mut PyObject, force: int);
        }
    } else {
        #[inline(always)]
        pub unsafe fn _Py_NewReference(op: *mut PyObject) {
            _Py_INC_TPALLOCS(op);
            _Py_INC_REFTOTAL();
            (*op).ob_refcnt = 1;
        }

        #[inline(always)]
        pub unsafe fn _Py_ForgetReference(op: *mut PyObject) {
            _Py_INC_TPFREES(op);
        }

        cfg_if! {
            if #[cfg(Py_LIMITED_API)] {
                extern "C" {
                    pub fn _Py_Dealloc(op: *mut PyObject);
                }
            } else {
                #[inline(always)]
                pub unsafe fn _Py_Dealloc(op: *mut PyObject) {
                    _Py_INC_TPFREES(op);
                    (*Py_TYPE(op)).tp_dealloc.unwrap()(op);
                }
            }
        }
    }
}


#[inline(always)]
pub unsafe fn Py_INCREF(op: *mut PyObject) {
    if cfg!(py_sys_config="Py_REF_DEBUG") {
        Py_IncRef(op)
    } else {
        (*op).ob_refcnt += 1
    }
}

#[inline(always)]
pub unsafe fn Py_DECREF(op: *mut PyObject) {
    if cfg!(py_sys_config="Py_REF_DEBUG") {
        Py_DecRef(op)
    } else {
        (*op).ob_refcnt -= 1;
        if (*op).ob_refcnt == 0 {
            _Py_Dealloc(op)
        }
    }
}

#[inline(always)]
pub unsafe fn Py_CLEAR(op: &mut *mut PyObject) {
    let tmp = *op;
    if !tmp.is_null() {
        *op = ptr::null_mut();
        Py_DECREF(tmp);
    }
}

#[inline(always)]
pub unsafe fn Py_XINCREF(op: *mut PyObject) {
    if !op.is_null() {
        Py_INCREF(op)
    }
}

#[inline(always)]
pub unsafe fn Py_XDECREF(op: *mut PyObject) {
    if !op.is_null() {
        Py_DECREF(op)
    }
}

extern "C" {
    pub fn Py_IncRef(o: *mut PyObject);
    pub fn Py_DecRef(o: *mut PyObject);

    #[cfg(not(Py_LIMITED_API))]
    static mut _PyNone_Type: PyObject;
    #[cfg(not(Py_LIMITED_API))]
    static mut _PyNotImplemented_Type: PyObject;
}

extern "C" {
    static mut _Py_NoneStruct: PyObject;
}
#[inline(always)]
pub unsafe fn Py_None() -> *mut PyObject {
    &mut _Py_NoneStruct
}

extern "C" {
    static mut _Py_NotImplementedStruct: PyObject;
}
#[inline(always)]
pub unsafe fn Py_NotImplemented() -> *mut PyObject {
    &mut _Py_NotImplementedStruct
}

// Rich comparison opcodes
pub const Py_LT: int = 0;
pub const Py_LE: int = 1;
pub const Py_EQ: int = 2;
pub const Py_NE: int = 3;
pub const Py_GT: int = 4;
pub const Py_GE: int = 5;
