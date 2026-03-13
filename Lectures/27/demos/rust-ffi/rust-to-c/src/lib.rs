//! Rust-to-C FFI demo: Rust calls C functions.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

extern "C" {
    fn c_add(a: c_int, b: c_int) -> c_int;
    fn c_greet(name: *const c_char) -> *mut c_char;
}

/// Call C's add function from Rust.
pub fn call_c_add(a: i32, b: i32) -> i32 {
    unsafe { c_add(a, b) }
}

/// Call C's greet function. C returns a malloc'd string; we copy and free it.
pub fn call_c_greet(name: &str) -> Option<String> {
    let c_name = CString::new(name).ok()?;
    let ptr = unsafe { c_greet(c_name.as_ptr()) };
    if ptr.is_null() {
        return None;
    }
    let result = unsafe {
        let s = CStr::from_ptr(ptr).to_string_lossy().into_owned();
        libc::free(ptr as *mut libc::c_void);
        s
    };
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_calls_c() {
        assert_eq!(call_c_add(2, 3), 5);
        assert_eq!(call_c_greet("Rust").unwrap(), "Hello from C, Rust!");
    }
}
