use swim_c_sys::{int, size_t, void};
use crate::object::PyObject;

pub enum PyArena {
    // Opaque.
}

extern "C" {
  pub fn PyArena_New() -> *mut PyArena;
  pub fn PyArena_Free(arena: *mut PyArena);

  pub fn PyArena_Malloc(arena: *mut PyArena, size: size_t) -> *mut void;

  pub fn PyArena_AddPyObject(arena: *mut PyArena, op: *mut PyObject) -> int;
}
