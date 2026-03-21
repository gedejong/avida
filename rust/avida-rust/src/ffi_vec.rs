//! Rust-owned Vec<T> exposed to C++ via FFI.
//!
//! C++ code accesses elements through data_ptr/len or get/set functions.
//! Rust retains ownership and manages the allocation.

use std::ffi::{c_double, c_int};

use crate::common::{boxed_free, boxed_new};

/// Rust-owned dynamic array exposed to C++ via FFI handle.
pub struct FfiVec<T> {
    data: Vec<T>,
}

impl<T: Clone> FfiVec<T> {
    pub fn new(capacity: usize) -> Self {
        FfiVec {
            data: Vec::with_capacity(capacity),
        }
    }

    pub fn from_vec(v: Vec<T>) -> Self {
        FfiVec { data: v }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn data_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    pub fn data_ptr_mut(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index < self.data.len() {
            self.data[index] = value;
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
    }

    pub fn resize(&mut self, new_len: usize, fill: T) {
        self.data.resize(new_len, fill);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

// ---------------------------------------------------------------------------
// Macro to generate typed FFI exports for FfiVec<T>
// ---------------------------------------------------------------------------

macro_rules! ffi_vec_exports {
    ($prefix:ident, $T:ty, $default:expr) => {
        paste::paste! {
            #[no_mangle]
            pub extern "C" fn [<avd_ $prefix _vec_new>](capacity: c_int) -> *mut FfiVec<$T> {
                let cap = if capacity > 0 { capacity as usize } else { 0 };
                boxed_new(FfiVec::new(cap))
            }

            #[no_mangle]
            pub extern "C" fn [<avd_ $prefix _vec_free>](handle: *mut FfiVec<$T>) {
                boxed_free(handle);
            }

            #[no_mangle]
            pub extern "C" fn [<avd_ $prefix _vec_clone>](handle: *const FfiVec<$T>) -> *mut FfiVec<$T> {
                if handle.is_null() { return std::ptr::null_mut(); }
                // SAFETY: handle checked for null, read-only borrow for clone.
                let h = unsafe { &*handle };
                boxed_new(FfiVec::from_vec(h.data.clone()))
            }

            #[no_mangle]
            pub extern "C" fn [<avd_ $prefix _vec_len>](handle: *const FfiVec<$T>) -> c_int {
                if handle.is_null() { return 0; }
                // SAFETY: handle checked for null, read-only borrow.
                let h = unsafe { &*handle };
                h.len() as c_int
            }

            #[no_mangle]
            pub extern "C" fn [<avd_ $prefix _vec_data>](handle: *const FfiVec<$T>) -> *const $T {
                if handle.is_null() { return std::ptr::null(); }
                // SAFETY: handle checked for null, read-only borrow.
                let h = unsafe { &*handle };
                h.data_ptr()
            }

            #[no_mangle]
            pub extern "C" fn [<avd_ $prefix _vec_data_mut>](handle: *mut FfiVec<$T>) -> *mut $T {
                if handle.is_null() { return std::ptr::null_mut(); }
                // SAFETY: handle checked for null, exclusive borrow.
                let h = unsafe { &mut *handle };
                h.data_ptr_mut()
            }

            #[no_mangle]
            pub extern "C" fn [<avd_ $prefix _vec_get>](handle: *const FfiVec<$T>, index: c_int) -> $T {
                if handle.is_null() || index < 0 { return $default; }
                // SAFETY: handle checked for null, read-only borrow.
                let h = unsafe { &*handle };
                h.get(index as usize).copied().unwrap_or($default)
            }

            #[no_mangle]
            pub extern "C" fn [<avd_ $prefix _vec_set>](handle: *mut FfiVec<$T>, index: c_int, value: $T) {
                if handle.is_null() || index < 0 { return; }
                // SAFETY: handle checked for null, exclusive borrow.
                let h = unsafe { &mut *handle };
                h.set(index as usize, value);
            }

            #[no_mangle]
            pub extern "C" fn [<avd_ $prefix _vec_push>](handle: *mut FfiVec<$T>, value: $T) {
                if handle.is_null() { return; }
                // SAFETY: handle checked for null, exclusive borrow.
                let h = unsafe { &mut *handle };
                h.push(value);
            }

            #[no_mangle]
            pub extern "C" fn [<avd_ $prefix _vec_resize>](handle: *mut FfiVec<$T>, new_len: c_int, fill: $T) {
                if handle.is_null() || new_len < 0 { return; }
                // SAFETY: handle checked for null, exclusive borrow.
                let h = unsafe { &mut *handle };
                h.resize(new_len as usize, fill);
            }

            #[no_mangle]
            pub extern "C" fn [<avd_ $prefix _vec_clear>](handle: *mut FfiVec<$T>) {
                if handle.is_null() { return; }
                // SAFETY: handle checked for null, exclusive borrow.
                let h = unsafe { &mut *handle };
                h.clear();
            }
        }
    };
}

// Generate FFI exports for the two most common element types.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
mod int_vec_ffi {
    use super::*;
    ffi_vec_exports!(int, c_int, 0);
}
#[allow(clippy::not_unsafe_ptr_arg_deref)]
mod double_vec_ffi {
    use super::*;
    ffi_vec_exports!(double, c_double, 0.0);
}
pub use double_vec_ffi::*;
pub use int_vec_ffi::*;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_vec_lifecycle() {
        let h = avd_int_vec_new(4);
        assert!(!h.is_null());
        assert_eq!(avd_int_vec_len(h), 0);

        avd_int_vec_push(h, 10);
        avd_int_vec_push(h, 20);
        avd_int_vec_push(h, 30);
        assert_eq!(avd_int_vec_len(h), 3);
        assert_eq!(avd_int_vec_get(h, 0), 10);
        assert_eq!(avd_int_vec_get(h, 1), 20);
        assert_eq!(avd_int_vec_get(h, 2), 30);
        assert_eq!(avd_int_vec_get(h, 3), 0); // out of bounds → default

        avd_int_vec_set(h, 1, 99);
        assert_eq!(avd_int_vec_get(h, 1), 99);

        // data pointer access
        let ptr = avd_int_vec_data(h);
        assert!(!ptr.is_null());
        // SAFETY: ptr is valid for 3 elements, just verified above.
        unsafe {
            assert_eq!(*ptr.add(0), 10);
            assert_eq!(*ptr.add(1), 99);
            assert_eq!(*ptr.add(2), 30);
        }

        // clone
        let h2 = avd_int_vec_clone(h);
        assert!(!h2.is_null());
        assert_eq!(avd_int_vec_len(h2), 3);
        assert_eq!(avd_int_vec_get(h2, 0), 10);

        // resize
        avd_int_vec_resize(h, 5, 42);
        assert_eq!(avd_int_vec_len(h), 5);
        assert_eq!(avd_int_vec_get(h, 3), 42);
        assert_eq!(avd_int_vec_get(h, 4), 42);

        // clear
        avd_int_vec_clear(h);
        assert_eq!(avd_int_vec_len(h), 0);

        avd_int_vec_free(h);
        avd_int_vec_free(h2);
    }

    #[test]
    fn test_double_vec_lifecycle() {
        let h = avd_double_vec_new(0);
        assert!(!h.is_null());

        avd_double_vec_push(h, 1.5);
        avd_double_vec_push(h, 2.7);
        assert_eq!(avd_double_vec_len(h), 2);
        assert!((avd_double_vec_get(h, 0) - 1.5).abs() < f64::EPSILON);
        assert!((avd_double_vec_get(h, 1) - 2.7).abs() < f64::EPSILON);

        avd_double_vec_free(h);
    }

    #[test]
    fn test_null_safety() {
        assert_eq!(avd_int_vec_len(std::ptr::null()), 0);
        assert_eq!(avd_int_vec_get(std::ptr::null(), 0), 0);
        assert!(avd_int_vec_data(std::ptr::null()).is_null());
        assert!(avd_int_vec_data_mut(std::ptr::null_mut()).is_null());
        assert!(avd_int_vec_clone(std::ptr::null()).is_null());
        avd_int_vec_set(std::ptr::null_mut(), 0, 1);
        avd_int_vec_push(std::ptr::null_mut(), 1);
        avd_int_vec_resize(std::ptr::null_mut(), 5, 0);
        avd_int_vec_clear(std::ptr::null_mut());
        avd_int_vec_free(std::ptr::null_mut());
    }

    #[test]
    fn test_negative_index_safety() {
        let h = avd_int_vec_new(0);
        avd_int_vec_push(h, 42);

        assert_eq!(avd_int_vec_get(h, -1), 0);
        avd_int_vec_set(h, -1, 99);
        assert_eq!(avd_int_vec_get(h, 0), 42);

        avd_int_vec_resize(h, -5, 0);
        assert_eq!(avd_int_vec_len(h), 1);

        avd_int_vec_free(h);
    }
}
