use std::ffi::c_double;

use crate::{
    common::{boxed_free, boxed_new, with_ds_mut, with_ds_ref},
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
    boxed_new(AvidaDoubleSumHandle::new())
}

#[no_mangle]
pub extern "C" fn avd_ds_clone(other: *const AvidaDoubleSumHandle) -> *mut AvidaDoubleSumHandle {
    with_ds_ref(other, std::ptr::null_mut(), |other_ref| {
        boxed_new(AvidaDoubleSumHandle {
            s1: other_ref.s1,
            s2: other_ref.s2,
            n: other_ref.n,
            max: other_ref.max,
        })
    })
}

#[no_mangle]
pub extern "C" fn avd_ds_free(handle: *mut AvidaDoubleSumHandle) {
    boxed_free(handle);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_sum_tracks_weighted_stats_and_clear() {
        let h = avd_ds_new();
        assert!(!h.is_null());
        assert_eq!(avd_ds_count(h), 0.0);
        assert_eq!(avd_ds_sum(h), 0.0);

        avd_ds_add(h, 2.0, 1.0);
        avd_ds_add(h, 4.0, 2.0);
        assert!((avd_ds_count(h) - 3.0).abs() < 1e-12);
        assert!((avd_ds_sum(h) - 10.0).abs() < 1e-12);
        assert!((avd_ds_average(h) - (10.0 / 3.0)).abs() < 1e-12);
        assert_eq!(avd_ds_max(h), 4.0);
        assert!(avd_ds_variance(h) > 0.0);
        assert!(avd_ds_std_deviation(h) > 0.0);
        assert!(avd_ds_std_error(h) > 0.0);

        avd_ds_subtract(h, 4.0, 1.0);
        assert!((avd_ds_count(h) - 2.0).abs() < 1e-12);
        assert!((avd_ds_sum(h) - 6.0).abs() < 1e-12);

        let cloned = avd_ds_clone(h);
        assert!(!cloned.is_null());
        assert!((avd_ds_sum(cloned) - 6.0).abs() < 1e-12);
        avd_ds_clear(cloned);
        assert_eq!(avd_ds_count(cloned), 0.0);
        assert_eq!(avd_ds_sum(cloned), 0.0);

        avd_ds_free(cloned);
        avd_ds_free(h);
    }

    #[test]
    fn double_sum_handles_nulls_and_zero_variance_paths() {
        assert!(avd_ds_clone(std::ptr::null()).is_null());
        avd_ds_free(std::ptr::null_mut());
        avd_ds_clear(std::ptr::null_mut());
        avd_ds_add(std::ptr::null_mut(), 1.0, 1.0);
        avd_ds_subtract(std::ptr::null_mut(), 1.0, 1.0);
        assert_eq!(avd_ds_count(std::ptr::null()), 0.0);
        assert_eq!(avd_ds_average(std::ptr::null()), 0.0);
        assert_eq!(avd_ds_variance(std::ptr::null()), 0.0);

        let h = avd_ds_new();
        avd_ds_add(h, 7.0, 1.0);
        assert_eq!(avd_ds_variance(h), 0.0);
        assert_eq!(avd_ds_std_error(h), 0.0);
        avd_ds_free(h);
    }
}
