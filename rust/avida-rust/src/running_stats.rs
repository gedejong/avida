use std::ffi::c_double;

use crate::{
    common::{boxed_free, boxed_new, with_mut, with_ref},
    AvidaRunningStatsHandle,
};

impl AvidaRunningStatsHandle {
    fn new() -> Self {
        Self {
            n: 0.0,
            m1: 0.0,
            m2: 0.0,
            m3: 0.0,
            m4: 0.0,
        }
    }

    fn clear(&mut self) {
        self.n = 0.0;
        self.m1 = 0.0;
        self.m2 = 0.0;
        self.m3 = 0.0;
        self.m4 = 0.0;
    }

    fn push(&mut self, x: f64) {
        self.n += 1.0;
        let d = x - self.m1;
        let d_n = d / self.n;
        let d_n2 = d_n * d_n;

        self.m4 += d * d_n2 * d_n * ((self.n - 1.0) * ((self.n * self.n) - 3.0 * self.n + 3.0))
            + 6.0 * d_n2 * self.m2
            - 4.0 * d_n * self.m3;
        self.m3 += d * d_n2 * ((self.n - 1.0) * (self.n - 2.0)) - 3.0 * d_n * self.m2;
        self.m2 += d * d_n * (self.n - 1.0);
        self.m1 += d_n;
    }

    fn variance(&self) -> f64 {
        if self.n > 1.0 {
            self.m2 / (self.n - 1.0)
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

    fn skewness(&self) -> f64 {
        self.n.sqrt() * self.m3 / self.m2.powf(1.5)
    }

    fn kurtosis(&self) -> f64 {
        self.n * self.m4 / (self.m2 * self.m2)
    }
}

#[no_mangle]
pub extern "C" fn avd_rs_new() -> *mut AvidaRunningStatsHandle {
    boxed_new(AvidaRunningStatsHandle::new())
}

#[no_mangle]
pub extern "C" fn avd_rs_clone(
    other: *const AvidaRunningStatsHandle,
) -> *mut AvidaRunningStatsHandle {
    with_ref(other, std::ptr::null_mut(), |other_ref| {
        boxed_new(AvidaRunningStatsHandle {
            n: other_ref.n,
            m1: other_ref.m1,
            m2: other_ref.m2,
            m3: other_ref.m3,
            m4: other_ref.m4,
        })
    })
}

#[no_mangle]
pub extern "C" fn avd_rs_free(handle: *mut AvidaRunningStatsHandle) {
    boxed_free(handle);
}

#[no_mangle]
pub extern "C" fn avd_rs_clear(handle: *mut AvidaRunningStatsHandle) {
    with_mut(handle, |h| h.clear());
}

#[no_mangle]
pub extern "C" fn avd_rs_push(handle: *mut AvidaRunningStatsHandle, x: c_double) {
    with_mut(handle, |h| h.push(x));
}

#[no_mangle]
pub extern "C" fn avd_rs_n(handle: *const AvidaRunningStatsHandle) -> c_double {
    with_ref(handle, 0.0, |h| h.n)
}

#[no_mangle]
pub extern "C" fn avd_rs_mean(handle: *const AvidaRunningStatsHandle) -> c_double {
    with_ref(handle, 0.0, |h| h.m1)
}

#[no_mangle]
pub extern "C" fn avd_rs_variance(handle: *const AvidaRunningStatsHandle) -> c_double {
    with_ref(handle, 0.0, |h| h.variance())
}

#[no_mangle]
pub extern "C" fn avd_rs_std_deviation(handle: *const AvidaRunningStatsHandle) -> c_double {
    with_ref(handle, 0.0, |h| h.std_deviation())
}

#[no_mangle]
pub extern "C" fn avd_rs_std_error(handle: *const AvidaRunningStatsHandle) -> c_double {
    with_ref(handle, 0.0, |h| h.std_error())
}

#[no_mangle]
pub extern "C" fn avd_rs_skewness(handle: *const AvidaRunningStatsHandle) -> c_double {
    with_ref(handle, 0.0, |h| h.skewness())
}

#[no_mangle]
pub extern "C" fn avd_rs_kurtosis(handle: *const AvidaRunningStatsHandle) -> c_double {
    with_ref(handle, 0.0, |h| h.kurtosis())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn running_stats_matches_reference_values() {
        let h = avd_rs_new();
        assert!(!h.is_null());
        for v in [1.0_f64, 2.0, 3.0, 4.0] {
            avd_rs_push(h, v);
        }
        assert_eq!(avd_rs_n(h), 4.0);
        assert!((avd_rs_mean(h) - 2.5).abs() < 1e-12);
        assert!(avd_rs_variance(h) > 0.0);
        assert!(avd_rs_std_deviation(h) > 0.0);
        assert!(avd_rs_std_error(h) > 0.0);
        assert!(avd_rs_skewness(h).is_finite());
        assert!(avd_rs_kurtosis(h).is_finite());

        let c = avd_rs_clone(h);
        assert!(!c.is_null());
        assert_eq!(avd_rs_n(c), 4.0);

        avd_rs_clear(c);
        assert_eq!(avd_rs_n(c), 0.0);
        assert_eq!(avd_rs_variance(c), 0.0);
        assert_eq!(avd_rs_std_error(c), 0.0);

        avd_rs_free(c);
        avd_rs_free(h);
    }

    #[test]
    fn running_stats_null_safety_paths() {
        assert!(avd_rs_clone(std::ptr::null()).is_null());
        avd_rs_free(std::ptr::null_mut());
        avd_rs_clear(std::ptr::null_mut());
        avd_rs_push(std::ptr::null_mut(), 1.0);
        assert_eq!(avd_rs_n(std::ptr::null()), 0.0);
        assert_eq!(avd_rs_mean(std::ptr::null()), 0.0);
        assert_eq!(avd_rs_variance(std::ptr::null()), 0.0);
        assert_eq!(avd_rs_std_deviation(std::ptr::null()), 0.0);
    }
}
