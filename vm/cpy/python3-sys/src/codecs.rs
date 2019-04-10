use swim_c_sys::{cchar, int};
use crate::object::PyObject;

extern "C" {
    pub fn PyCodec_Register(search_function: *mut PyObject) -> int;
    pub fn PyCodec_KnownEncoding(encoding: *const cchar) -> int;
    pub fn PyCodec_Encode(object: *mut PyObject, encoding: *const cchar, errors: *const cchar) -> *mut PyObject;
    pub fn PyCodec_Decode(object: *mut PyObject, encoding: *const cchar, errors: *const cchar) -> *mut PyObject;
    pub fn PyCodec_Encoder(encoding: *const cchar) -> *mut PyObject;
    pub fn PyCodec_Decoder(encoding: *const cchar) -> *mut PyObject;
    pub fn PyCodec_IncrementalEncoder(encoding: *const cchar, errors: *const cchar) -> *mut PyObject;
    pub fn PyCodec_IncrementalDecoder(encoding: *const cchar, errors: *const cchar) -> *mut PyObject;
    pub fn PyCodec_StreamReader(encoding: *const cchar, stream: *mut PyObject, errors: *const cchar) -> *mut PyObject;
    pub fn PyCodec_StreamWriter(encoding: *const cchar, stream: *mut PyObject, errors: *const cchar) -> *mut PyObject;
    pub fn PyCodec_RegisterError(name: *const cchar, error: *mut PyObject) -> int;
    pub fn PyCodec_LookupError(name: *const cchar) -> *mut PyObject;
    pub fn PyCodec_StrictErrors(exc: *mut PyObject) -> *mut PyObject;
    pub fn PyCodec_IgnoreErrors(exc: *mut PyObject) -> *mut PyObject;
    pub fn PyCodec_ReplaceErrors(exc: *mut PyObject) -> *mut PyObject;
    pub fn PyCodec_XMLCharRefReplaceErrors(exc: *mut PyObject) -> *mut PyObject;
    pub fn PyCodec_BackslashReplaceErrors(exc: *mut PyObject) -> *mut PyObject;
}
