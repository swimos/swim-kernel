use swim_c_sys::{cchar, int};
use crate::pyconfig::Py_ssize_t;
use crate::object::PyObject;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct PyMemberDef {
    pub name: *mut cchar,
    pub type_code: int,
    pub offset: Py_ssize_t,
    pub flags: int,
    pub doc: *mut cchar
}

// Types
pub const T_SHORT: int = 0;
pub const T_INT: int = 1;
pub const T_LONG: int = 2;
pub const T_FLOAT: int = 3;
pub const T_DOUBLE: int = 4;
pub const T_STRING: int = 5;
pub const T_OBJECT: int = 6;
// XXX the ordering here is weird for binary compatibility
pub const T_CHAR: int = 7; // 1-character string
pub const T_BYTE: int = 8; // 8-bit signed int
// unsigned variants:
pub const T_UBYTE: int = 9;
pub const T_USHORT: int = 10;
pub const T_UINT: int = 11;
pub const T_ULONG: int = 12;

// Added by Jack: strings contained in the structure
pub const T_STRING_INPLACE: int = 13;

// Added by Lillo: bools contained in the structure (assumed char)
pub const T_BOOL: int = 14;

// Like T_OBJECT, but raises AttributeError
// when the value is NULL, instead of
// converting to None.
pub const T_OBJECT_EX: int = 16; 

pub const T_LONGLONG: int = 17;
pub const T_ULONGLONG: int = 18;

pub const T_PYSSIZET: int = 19; // Py_ssize_t
pub const T_NONE: int = 20; // Value is always None

// Flags
pub const READONLY: int = 1;
pub const READ_RESTRICTED: int = 2;
pub const PY_WRITE_RESTRICTED: int = 4;
pub const RESTRICTED: int = READ_RESTRICTED | PY_WRITE_RESTRICTED;

extern "C" {
    pub fn PyMember_GetOne(addr: *const cchar, l: *mut PyMemberDef) -> *mut PyObject;
    pub fn PyMember_SetOne(addr: *mut cchar, l: *mut PyMemberDef, value: *mut PyObject) -> int;
}
