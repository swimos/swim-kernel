use swim_c_sys::{cchar, int};
use swim_c_sys::stdio::FILE;
use crate::object::PyObject;
use crate::compile::{_node, PyCompilerFlags};
use crate::symtable::symtable;

cfg_if! {
    if #[cfg(not(Py_LIMITED_API))] {
        use crate::compile::_mod;
        use crate::pyarena::PyArena;

        extern "C" {
            pub fn PyRun_SimpleStringFlags(arg1: *const cchar, arg2: *mut PyCompilerFlags) -> int;
            pub fn PyRun_AnyFileFlags(arg1: *mut FILE, arg2: *const cchar, arg3: *mut PyCompilerFlags) -> int;
            pub fn PyRun_AnyFileExFlags(fp: *mut FILE, filename: *const cchar, closeit: int, flags: *mut PyCompilerFlags) -> int;
            pub fn PyRun_SimpleFileExFlags(fp: *mut FILE, filename: *const cchar, closeit: int, flags: *mut PyCompilerFlags) -> int;
            pub fn PyRun_InteractiveOneFlags(fp: *mut FILE, filename: *const cchar, flags: *mut PyCompilerFlags) -> int;
            #[cfg(Py_3_4)]
            pub fn PyRun_InteractiveOneObject(fp: *mut FILE, filename: *mut PyObject, flags: *mut PyCompilerFlags) -> int;
            pub fn PyRun_InteractiveLoopFlags(fp: *mut FILE, filename: *const cchar, flags: *mut PyCompilerFlags) -> int;
            pub fn PyParser_ASTFromString(s: *const cchar, filename: *const cchar, start: int, flags: *mut PyCompilerFlags, arena: *mut PyArena) -> *mut _mod;
            #[cfg(Py_3_4)]
            pub fn PyParser_ASTFromStringObject(s: *const cchar, filename: *mut PyObject, start: int, flags: *mut PyCompilerFlags, arena: *mut PyArena) -> *mut _mod;
            pub fn PyParser_ASTFromFile(fp: *mut FILE, filename: *const cchar, enc: *const cchar, start: int, ps1: *const cchar, ps2: *const cchar, flags: *mut PyCompilerFlags, errcode: *mut int, arena: *mut PyArena) -> *mut _mod;
            #[cfg(Py_3_4)]
            pub fn PyParser_ASTFromFileObject(fp: *mut FILE, filename: *mut PyObject, enc: *const cchar, start: int, ps1: *const cchar, ps2: *const cchar, flags: *mut PyCompilerFlags, errcode: *mut int, arena: *mut PyArena) -> *mut _mod;
        }
    }
}

extern "C" {
    pub fn PyParser_SimpleParseStringFlags(arg1: *const cchar, arg2: int, arg3: int) -> *mut _node;

    #[cfg(any(Py_3_3, not(Py_LIMITED_API)))]
    pub fn PyParser_SimpleParseStringFlagsFilename(arg1: *const cchar, arg2: *const cchar, arg3: int, arg4: int) -> *mut _node;

    #[cfg(not(Py_LIMITED_API))]
    pub fn PyParser_SimpleParseFileFlags(arg1: *mut FILE, arg2: *const cchar, arg3: int, arg4: int) -> *mut _node;

    #[cfg(not(Py_LIMITED_API))]
    pub fn PyRun_StringFlags(arg1: *const cchar, arg2: int, arg3: *mut PyObject, arg4: *mut PyObject, arg5: *mut PyCompilerFlags) -> *mut PyObject;

    #[cfg(not(Py_LIMITED_API))]
    pub fn PyRun_FileExFlags(fp: *mut FILE, filename: *const cchar, start: int, globals: *mut PyObject, locals: *mut PyObject, closeit: int, flags: *mut PyCompilerFlags) -> *mut PyObject;
}

cfg_if! {
    if #[cfg(Py_LIMITED_API)] {
        extern "C" {
            pub fn Py_CompileString(string: *const cchar, p: *const cchar, s: int) -> *mut PyObject;
        }
    } else {
        #[inline(always)]
        pub unsafe fn Py_CompileString(string: *const cchar, p: *const cchar, s: int) -> *mut PyObject {
            Py_CompileStringExFlags(string, p, s, core::ptr::null_mut(), -1)
        }
        #[inline(always)]
        pub unsafe fn Py_CompileStringFlags(string: *const cchar, p: *const cchar, s: int, f: *mut PyCompilerFlags) -> *mut PyObject {
            Py_CompileStringExFlags(string, p, s, f, -1)
        }
        extern "C" {
            pub fn Py_CompileStringExFlags(str: *const cchar, filename: *const cchar, start: int, flags: *mut PyCompilerFlags, optimize: int) -> *mut PyObject;
            #[cfg(Py_3_4)]
            pub fn Py_CompileStringObject(str: *const cchar, filename: *mut PyObject, start: int, flags: *mut PyCompilerFlags, optimize: int) -> *mut PyObject;
        }
    }
}
extern "C" {
    pub fn Py_SymtableString(str: *const cchar, filename: *const cchar, start: int) -> *mut symtable;
    #[cfg(all(Py_3_4, not(Py_LIMITED_API)))]
    pub fn Py_SymtableStringObject(str: *const cchar, filename: *mut PyObject, start: int) -> *mut symtable;

    pub fn PyErr_Print();
    pub fn PyErr_PrintEx(arg1: int);
    pub fn PyErr_Display(arg1: *mut PyObject, arg2: *mut PyObject, arg3: *mut PyObject);
}
