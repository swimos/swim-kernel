use swim_c_sys::int;

#[cfg(not(Py_LIMITED_API))]
extern "C" {
    pub static mut Py_DebugFlag: int;
    pub static mut Py_VerboseFlag: int;
    pub static mut Py_QuietFlag: int;
    pub static mut Py_InteractiveFlag: int;
    pub static mut Py_InspectFlag: int;
    pub static mut Py_OptimizeFlag: int;
    pub static mut Py_NoSiteFlag: int;
    pub static mut Py_BytesWarningFlag: int;
    pub static mut Py_UseClassExceptionsFlag: int;
    pub static mut Py_FrozenFlag: int;
    pub static mut Py_IgnoreEnvironmentFlag: int;
    pub static mut Py_DontWriteBytecodeFlag: int;
    pub static mut Py_NoUserSiteDirectory: int;
    pub static mut Py_UnbufferedStdioFlag: int;
    pub static mut Py_HashRandomizationFlag: int;
    #[cfg(Py_3_4)]
    pub static mut Py_IsolatedFlag: int;
    #[cfg(all(Py_3_6, windows))]
    pub static mut Py_LegacyWindowsStdioFlag: int;
}
