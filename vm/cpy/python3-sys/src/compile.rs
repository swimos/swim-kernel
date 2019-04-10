use swim_c_sys::int;

cfg_if! {
    if #[cfg(not(Py_LIMITED_API))] {
        use swim_c_sys::cchar;
        use crate::code::PyCodeObject;
        use crate::pyarena::PyArena;

        pub enum _node {
            // Opaque.
        }

        extern "C" {
            pub fn PyNode_Compile(arg1: *mut _node, arg2: *const cchar) -> *mut PyCodeObject;
        }

        #[derive(Copy, Clone)]
        #[repr(C)]
        pub struct PyCompilerFlags {
            pub cf_flags: int, // bitmask of CO_xxx flags relevant to future
        }

        #[derive(Copy, Clone)]
        #[repr(C)]
        pub struct PyFutureFeatures {
            pub ff_features: int,
            pub ff_lineno: int,
        }

        pub const FUTURE_NESTED_SCOPES: &'static str = "nested_scopes";
        pub const FUTURE_GENERATORS: &'static str = "generators";
        pub const FUTURE_DIVISION: &'static str = "division";
        pub const FUTURE_ABSOLUTE_IMPORT: &'static str = "absolute_import";
        pub const FUTURE_WITH_STATEMENT: &'static str = "with_statement";
        pub const FUTURE_PRINT_FUNCTION: &'static str = "print_function";
        pub const FUTURE_UNICODE_LITERALS: &'static str = "unicode_literals";
        pub const FUTURE_BARRY_AS_BDFL: &'static str = "barry_as_FLUFL";
        #[cfg(Py_3_5)]
        pub const FUTURE_GENERATOR_STOP: &'static str = "generator_stop";

        pub enum _mod {
            // Opaque.
        }

        extern "C" {
            pub fn PyAST_CompileEx(_mod: *mut _mod, filename: *const cchar, flags: *mut PyCompilerFlags, optimize: int, arena: *mut PyArena) -> *mut PyCodeObject;
            #[cfg(Py_3_4)]
            pub fn PyAST_CompileObject(_mod: *mut _mod, filename: *mut crate::object::PyObject, flags: *mut PyCompilerFlags, optimize: int, arena: *mut PyArena) -> *mut PyCodeObject;
            pub fn PyFuture_FromAST(_mod: *mut _mod, filename: *const cchar) -> *mut PyFutureFeatures;
            #[cfg(Py_3_4)]
            pub fn PyFuture_FromASTObject(_mod: *mut _mod, filename: *mut crate::object::PyObject) -> *mut PyFutureFeatures;
            #[cfg(Py_3_4)]
            pub fn PyCompile_OpcodeStackEffect(opcode: int, oparg: int) -> int;
        }
    }
}

pub const Py_single_input: int = 256;
pub const Py_file_input: int = 257;
pub const Py_eval_input: int = 258;
