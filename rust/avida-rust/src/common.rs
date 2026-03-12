use crate::time_series_recorder::AvidaTimeSeriesHandle;
use crate::{
    AvidaDoubleSumHandle, AvidaHistogramHandle, AvidaOrderedWeightedIndexHandle,
    AvidaRawBitArrayHandle, AvidaRunningAverageHandle, AvidaRunningStatsHandle,
    AvidaWeightedIndexHandle,
};
use std::ffi::{c_char, c_int, CStr, CString};

macro_rules! define_handle_accessors {
    ($ref_name:ident, $mut_name:ident, $handle_ty:ty) => {
        pub(crate) fn $ref_name<T>(
            handle: *const $handle_ty,
            default: T,
            f: impl FnOnce(&$handle_ty) -> T,
        ) -> T {
            if handle.is_null() {
                return default;
            }
            // SAFETY: pointer was checked for null and is only read.
            let h = unsafe { &*handle };
            f(h)
        }

        pub(crate) fn $mut_name(handle: *mut $handle_ty, f: impl FnOnce(&mut $handle_ty)) {
            if handle.is_null() {
                return;
            }
            // SAFETY: pointer was checked for null and is only mutably borrowed for this call.
            let h = unsafe { &mut *handle };
            f(h);
        }
    };
}

define_handle_accessors!(with_ref, with_mut, AvidaRunningStatsHandle);
define_handle_accessors!(with_ra_ref, with_ra_mut, AvidaRunningAverageHandle);
define_handle_accessors!(with_ds_ref, with_ds_mut, AvidaDoubleSumHandle);
define_handle_accessors!(with_wi_ref, with_wi_mut, AvidaWeightedIndexHandle);
define_handle_accessors!(with_owi_ref, with_owi_mut, AvidaOrderedWeightedIndexHandle);
define_handle_accessors!(with_hist_ref, with_hist_mut, AvidaHistogramHandle);
define_handle_accessors!(with_rba_ref, with_rba_mut, AvidaRawBitArrayHandle);
define_handle_accessors!(with_tsr_ref, with_tsr_mut, AvidaTimeSeriesHandle);

pub(crate) fn with_cstr<T>(ptr: *const c_char, default: T, f: impl FnOnce(&CStr) -> T) -> T {
    if ptr.is_null() {
        return default;
    }
    // SAFETY: pointer was checked for null and is only read.
    let cstr = unsafe { CStr::from_ptr(ptr) };
    f(cstr)
}

pub(crate) fn alloc_c_string(value: String) -> *mut c_char {
    let sanitized: Vec<u8> = value.bytes().filter(|b| *b != 0).collect();
    match CString::new(sanitized) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

pub(crate) fn free_c_string(value: *mut c_char) {
    if value.is_null() {
        return;
    }
    // SAFETY: pointer was allocated by CString::into_raw in this crate.
    unsafe {
        drop(CString::from_raw(value));
    }
}

pub(crate) fn set_out<T>(out: *mut T, value: T) -> bool {
    if out.is_null() {
        return false;
    }
    // SAFETY: output pointer checked for null and written exactly once.
    unsafe { *out = value };
    true
}

pub(crate) fn with_slice<T, R>(
    ptr: *const T,
    count: c_int,
    default: R,
    f: impl FnOnce(&[T]) -> R,
) -> R {
    if ptr.is_null() || count <= 0 {
        return default;
    }
    let count_usize = match usize::try_from(count) {
        Ok(v) => v,
        Err(_) => return default,
    };
    // SAFETY: pointer is checked for null and used read-only for count elements.
    let slice = unsafe { std::slice::from_raw_parts(ptr, count_usize) };
    f(slice)
}

pub(crate) fn with_mut_slice<T>(ptr: *mut T, count: c_int, f: impl FnOnce(&mut [T])) -> bool {
    if ptr.is_null() || count <= 0 {
        return false;
    }
    let count_usize = match usize::try_from(count) {
        Ok(v) => v,
        Err(_) => return false,
    };
    // SAFETY: pointer is checked for null and used mutably for count elements.
    let slice = unsafe { std::slice::from_raw_parts_mut(ptr, count_usize) };
    f(slice);
    true
}

pub(crate) fn boxed_new<T>(value: T) -> *mut T {
    Box::into_raw(Box::new(value))
}

#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub(crate) fn boxed_free<T>(handle: *mut T) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer came from Box::into_raw in this crate and is freed exactly once here.
    unsafe {
        drop(Box::from_raw(handle));
    }
}
