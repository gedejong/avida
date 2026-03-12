use std::ffi::{c_double, c_int};

use crate::{
    common::{boxed_free, boxed_new, with_ra_mut, with_ra_ref},
    AvidaRunningAverageHandle,
};

impl AvidaRunningAverageHandle {
    fn new(window_size: usize) -> Self {
        Self {
            values: vec![0.0; window_size],
            s1: 0.0,
            s2: 0.0,
            window_size,
            pointer: 0,
            n: 0,
        }
    }

    fn clear(&mut self) {
        self.s1 = 0.0;
        self.s2 = 0.0;
        self.pointer = 0;
        self.n = 0;
    }

    fn add(&mut self, value: f64) {
        self.s1 += value;
        self.s2 += value * value;
        if self.n < self.window_size {
            self.values[self.n] = value;
            self.n += 1;
        } else {
            let out_v = self.values[self.pointer];
            self.s1 -= out_v;
            self.s2 -= out_v * out_v;
            self.values[self.pointer] = value;
            self.pointer += 1;
            if self.pointer == self.window_size {
                self.pointer = 0;
            }
        }
    }

    fn average(&self) -> f64 {
        if self.n == self.window_size {
            self.s1 / self.n as f64
        } else {
            0.0
        }
    }

    fn variance(&self) -> f64 {
        if self.n == self.window_size {
            (self.s2 - self.s1 * self.s1 / self.n as f64) / (self.n as f64 - 1.0)
        } else {
            0.0
        }
    }

    fn std_deviation(&self) -> f64 {
        self.variance().sqrt()
    }

    fn std_error(&self) -> f64 {
        if self.n == self.window_size {
            (self.s2 - self.s1 * self.s1 / self.n as f64 / (self.n as f64 * (self.n as f64 - 1.0)))
                .sqrt()
        } else {
            0.0
        }
    }
}

#[no_mangle]
pub extern "C" fn avd_ra_new(window_size: c_int) -> *mut AvidaRunningAverageHandle {
    if window_size <= 1 {
        return std::ptr::null_mut();
    }
    let ws = usize::try_from(window_size).ok();
    match ws {
        Some(valid_ws) => boxed_new(AvidaRunningAverageHandle::new(valid_ws)),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn avd_ra_free(handle: *mut AvidaRunningAverageHandle) {
    boxed_free(handle);
}

#[no_mangle]
pub extern "C" fn avd_ra_clear(handle: *mut AvidaRunningAverageHandle) {
    with_ra_mut(handle, |h| h.clear());
}

#[no_mangle]
pub extern "C" fn avd_ra_add(handle: *mut AvidaRunningAverageHandle, value: c_double) {
    with_ra_mut(handle, |h| h.add(value));
}

#[no_mangle]
pub extern "C" fn avd_ra_sum(handle: *const AvidaRunningAverageHandle) -> c_double {
    with_ra_ref(handle, 0.0, |h| h.s1)
}

#[no_mangle]
pub extern "C" fn avd_ra_sum_of_squares(handle: *const AvidaRunningAverageHandle) -> c_double {
    with_ra_ref(handle, 0.0, |h| h.s2)
}

#[no_mangle]
pub extern "C" fn avd_ra_average(handle: *const AvidaRunningAverageHandle) -> c_double {
    with_ra_ref(handle, 0.0, |h| h.average())
}

#[no_mangle]
pub extern "C" fn avd_ra_variance(handle: *const AvidaRunningAverageHandle) -> c_double {
    with_ra_ref(handle, 0.0, |h| h.variance())
}

#[no_mangle]
pub extern "C" fn avd_ra_std_deviation(handle: *const AvidaRunningAverageHandle) -> c_double {
    with_ra_ref(handle, 0.0, |h| h.std_deviation())
}

#[no_mangle]
pub extern "C" fn avd_ra_std_error(handle: *const AvidaRunningAverageHandle) -> c_double {
    with_ra_ref(handle, 0.0, |h| h.std_error())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn running_average_warmup_and_wrap_behavior() {
        let h = avd_ra_new(3);
        assert!(!h.is_null());
        avd_ra_add(h, 1.0);
        avd_ra_add(h, 2.0);
        assert_eq!(avd_ra_average(h), 0.0);

        avd_ra_add(h, 3.0);
        assert!((avd_ra_average(h) - 2.0).abs() < 1e-12);
        assert!(avd_ra_variance(h) > 0.0);
        assert!(avd_ra_std_deviation(h) > 0.0);
        assert!(avd_ra_std_error(h) > 0.0);

        // Force wrap-around path.
        avd_ra_add(h, 4.0);
        avd_ra_add(h, 5.0);
        assert!((avd_ra_average(h) - 4.0).abs() < 1e-12);
        assert!(avd_ra_sum(h) > 0.0);
        assert!(avd_ra_sum_of_squares(h) > 0.0);

        avd_ra_clear(h);
        assert_eq!(avd_ra_average(h), 0.0);
        assert_eq!(avd_ra_variance(h), 0.0);
        avd_ra_free(h);
    }

    #[test]
    fn running_average_null_and_invalid_paths() {
        assert!(avd_ra_new(1).is_null());
        assert!(avd_ra_new(0).is_null());
        avd_ra_free(std::ptr::null_mut());
        avd_ra_clear(std::ptr::null_mut());
        avd_ra_add(std::ptr::null_mut(), 1.0);
        assert_eq!(avd_ra_average(std::ptr::null()), 0.0);
        assert_eq!(avd_ra_std_error(std::ptr::null()), 0.0);
    }
}
