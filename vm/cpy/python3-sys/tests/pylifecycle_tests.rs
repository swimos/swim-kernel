#![allow(non_snake_case)]

extern crate swim_c_sys;
extern crate swim_python3_sys;

use swim_python3_sys::pylifecycle::*;
use std::ffi::CStr;

#[test]
fn test_Py_GetVersion() {
    let version = unsafe { CStr::from_ptr(Py_GetVersion()) }.to_str().unwrap();
    println!("Py_GetVersion: {}", version);
}
