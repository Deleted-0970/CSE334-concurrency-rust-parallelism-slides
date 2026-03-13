//! C-to-Rust FFI demo: C executable calls Rust cdylib.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

/// Shared struct passed across the FFI boundary.
#[repr(C)]
pub struct Point {
    pub x: c_int,
    pub y: c_int,
}

/// Multiply two integers. Called from C.
#[no_mangle]
pub extern "C" fn rust_multiply(a: c_int, b: c_int) -> c_int {
    a * b
}

/// Concatenate two C strings and return a new C string.
/// Caller (C) must free the result with rust_free_string.
#[no_mangle]
pub extern "C" fn rust_concat_strings(a: *const c_char, b: *const c_char) -> *mut c_char {
    if a.is_null() || b.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let a_str = CStr::from_ptr(a).to_string_lossy();
        let b_str = CStr::from_ptr(b).to_string_lossy();
        let result = format!("{}{}", a_str, b_str);
        match CString::new(result) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

/// Free a string allocated by Rust (e.g., rust_concat_strings).
#[no_mangle]
pub extern "C" fn rust_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
        }
    }
}

/// Create a greeting string.
#[no_mangle]
pub extern "C" fn rust_greet(name: *const c_char) -> *mut c_char {
    if name.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        let name_str = CStr::from_ptr(name).to_string_lossy();
        let greeting = format!("Hello from Rust, {}!", name_str);
        match CString::new(greeting) {
            Ok(c_str) => c_str.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    }
}

/// Add a point's coordinates. Demonstrates #[repr(C)] struct.
#[no_mangle]
pub extern "C" fn rust_point_sum(p: *const Point) -> c_int {
    if p.is_null() {
        return 0;
    }
    unsafe { (*p).x + (*p).y }
}
