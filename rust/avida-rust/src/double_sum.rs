use std::ffi::c_double;

use crate::{
    common::{with_ds_mut, with_ds_ref},
    AvidaDoubleSumHandle,
};

impl AvidaDoubleSumHandle {
    fn new() -> Self {
        Self {
            s1: 0.0,
            s2: 0.0,
            n: 0.0,
            max: f64::MIN_POSITIVE,
        }
    }

    fn clear(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
        self.n = 0.0;
        self.max = f64::MIN_POSITIVE;
    }

    fn add(&mut self, value: f64, weight: f64) {
        let w_val = value * weight;
        self.n += weight;
        self.s1 += w_val;
        self.s2 += w_val * w_val;
        if value > self.max {
            self.max = value;
        }
    }

    fn subtract(&mut self, value: f64, weight: f64) {
        let w_val = value * weight;
        self.n -= weight;
        self.s1 -= w_val;
        self.s2 -= w_val * w_val;
    }

    fn average(&self) -> f64 {
        if self.n > 0.0 {
            self.s1 / self.n
        } else {
            0.0
        }
    }

    fn variance(&self) -> f64 {
        if self.n > 1.0 {
            (self.s2 - self.s1 * self.s1 / self.n) / (self.n - 1.0)
        } else {
            0.0
        }
    }

    fn std_deviation(&self) -> f64 {
        self.variance().sqrt()
    }

    fn std_error(&self) -> f64 {
        if self.n > 1.0 {
            (self.variance() / self.n).sqrt()
        } else {
            0.0
        }
    }
}

#[no_mangle]
pub extern "C" fn avd_ds_new() -> *mut AvidaDoubleSumHandle {
    Box::into_raw(Box::new(AvidaDoubleSumHandle::new()))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_ds_clone(other: *const AvidaDoubleSumHandle) -> *mut AvidaDoubleSumHandle {
    if other.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: pointer was checked for null and is only read.
    let other_ref = unsafe { &*other };
    Box::into_raw(Box::new(AvidaDoubleSumHandle {
        s1: other_ref.s1,
        s2: other_ref.s2,
        n: other_ref.n,
        max: other_ref.max,
    }))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_ds_free(handle: *mut AvidaDoubleSumHandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer came from Box::into_raw in this crate and is freed exactly once here.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub extern "C" fn avd_ds_clear(handle: *mut AvidaDoubleSumHandle) {
    with_ds_mut(handle, |h| h.clear());
}

#[no_mangle]
pub extern "C" fn avd_ds_add(handle: *mut AvidaDoubleSumHandle, value: c_double, weight: c_double) {
    with_ds_mut(handle, |h| h.add(value, weight));
}

#[no_mangle]
pub extern "C" fn avd_ds_subtract(
    handle: *mut AvidaDoubleSumHandle,
    value: c_double,
    weight: c_double,
) {
    with_ds_mut(handle, |h| h.subtract(value, weight));
}

#[no_mangle]
pub extern "C" fn avd_ds_count(handle: *const AvidaDoubleSumHandle) -> c_double {
    with_ds_ref(handle, 0.0, |h| h.n)
}

#[no_mangle]
pub extern "C" fn avd_ds_sum(handle: *const AvidaDoubleSumHandle) -> c_double {
    with_ds_ref(handle, 0.0, |h| h.s1)
}

#[no_mangle]
pub extern "C" fn avd_ds_max(handle: *const AvidaDoubleSumHandle) -> c_double {
    with_ds_ref(handle, 0.0, |h| h.max)
}

#[no_mangle]
pub extern "C" fn avd_ds_average(handle: *const AvidaDoubleSumHandle) -> c_double {
    with_ds_ref(handle, 0.0, |h| h.average())
}

#[no_mangle]
pub extern "C" fn avd_ds_variance(handle: *const AvidaDoubleSumHandle) -> c_double {
    with_ds_ref(handle, 0.0, |h| h.variance())
}

#[no_mangle]
pub extern "C" fn avd_ds_std_deviation(handle: *const AvidaDoubleSumHandle) -> c_double {
    with_ds_ref(handle, 0.0, |h| h.std_deviation())
}

#[no_mangle]
pub extern "C" fn avd_ds_std_error(handle: *const AvidaDoubleSumHandle) -> c_double {
    with_ds_ref(handle, 0.0, |h| h.std_error())
}
