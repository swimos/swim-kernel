use swim_c_sys::{cchar, int};
use swim_c_sys::sys::types::wchar_t;
use crate::pystate::PyThreadState;

extern "C" {
    pub fn Py_SetProgramName(arg1: *mut wchar_t);
    pub fn Py_GetProgramName() -> *mut wchar_t;
    pub fn Py_SetPythonHome(arg1: *mut wchar_t);
    pub fn Py_GetPythonHome() -> *mut wchar_t;
    pub fn Py_Initialize() -> ();
    pub fn Py_InitializeEx(arg1: int);
    pub fn Py_Finalize();
    pub fn Py_IsInitialized();
    pub fn Py_NewInterpreter() -> *mut PyThreadState;
    pub fn Py_EndInterpreter(arg1: *mut PyThreadState);

    pub fn Py_AtExit(func: Option<extern "C" fn ()>) -> int;

    pub fn Py_Exit(arg1: int);

    pub fn Py_Main(argc: int, argv: *mut *mut wchar_t) -> int;

    pub fn Py_GetProgramFullPath() -> *mut wchar_t;
    pub fn Py_GetPrefix() -> *mut wchar_t;
    pub fn Py_GetExecPrefix() -> *mut wchar_t;
    pub fn Py_GetPath() -> *mut wchar_t;
    pub fn Py_SetPath(arg1: *const wchar_t);

    pub fn Py_GetVersion() -> *const cchar;
    pub fn Py_GetPlatform() -> *const cchar;
    pub fn Py_GetCopyright() -> *const cchar;
    pub fn Py_GetCompiler() -> *const cchar;
    pub fn Py_GetBuildInfo() -> *const cchar;
}
