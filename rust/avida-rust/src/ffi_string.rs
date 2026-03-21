//! Unified string FFI helpers for Rust↔C++ string lifecycle.
//!
//! Provides a canonical `avd_string_free` and helpers for safe string
//! passing across the FFI boundary. Consolidates scattered string
//! lifecycle functions.

use std::ffi::{c_char, c_int, CStr, CString};

use crate::common::{alloc_c_string, free_c_string};

/// Free a Rust-allocated C string. This is the canonical free function
/// for any `char*` returned by Rust FFI functions.
///
/// Safe to call with null (no-op).
#[no_mangle]
pub extern "C" fn avd_string_free(ptr: *mut c_char) {
    free_c_string(ptr);
}

/// Deep-copy a C string into Rust-owned memory.
/// Returns a new allocation that must be freed with `avd_string_free`.
/// Returns null if `src` is null.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_string_from_cstr(src: *const c_char) -> *mut c_char {
    if src.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: src checked for null, read as NUL-terminated C string.
    let cstr = unsafe { CStr::from_ptr(src) };
    let s = cstr.to_string_lossy().into_owned();
    alloc_c_string(s)
}

/// Return the length of a Rust-owned C string (not counting NUL terminator).
/// Returns 0 if ptr is null.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_string_len(ptr: *const c_char) -> c_int {
    if ptr.is_null() {
        return 0;
    }
    // SAFETY: ptr checked for null, read as NUL-terminated C string.
    let cstr = unsafe { CStr::from_ptr(ptr) };
    cstr.to_bytes().len() as c_int
}

/// Allocate a Rust-owned C string from a Rust String.
/// This is the public version of common::alloc_c_string for use by other modules.
pub fn rust_string_to_c(s: String) -> *mut c_char {
    alloc_c_string(s)
}

/// Create a CString from a &str, filtering NUL bytes.
pub fn str_to_cstring(s: &str) -> Option<CString> {
    let sanitized: Vec<u8> = s.bytes().filter(|b| *b != 0).collect();
    CString::new(sanitized).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_roundtrip() {
        let src = CString::new("hello world").unwrap();
        let copy = avd_string_from_cstr(src.as_ptr());
        assert!(!copy.is_null());
        assert_eq!(avd_string_len(copy), 11);
        // SAFETY: copy is a valid Rust-allocated C string.
        let result = unsafe { CStr::from_ptr(copy) };
        assert_eq!(result.to_str().unwrap(), "hello world");
        avd_string_free(copy);
    }

    #[test]
    fn test_null_safety() {
        assert!(avd_string_from_cstr(std::ptr::null()).is_null());
        assert_eq!(avd_string_len(std::ptr::null()), 0);
        avd_string_free(std::ptr::null_mut()); // should not crash
    }

    #[test]
    fn test_empty_string() {
        let src = CString::new("").unwrap();
        let copy = avd_string_from_cstr(src.as_ptr());
        assert!(!copy.is_null());
        assert_eq!(avd_string_len(copy), 0);
        avd_string_free(copy);
    }
}
