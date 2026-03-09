use std::ffi::c_double;

use crate::{
    common::{with_mut, with_ref},
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
    Box::into_raw(Box::new(AvidaRunningStatsHandle::new()))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rs_clone(
    other: *const AvidaRunningStatsHandle,
) -> *mut AvidaRunningStatsHandle {
    if other.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: pointer was checked for null and is only read.
    let other_ref = unsafe { &*other };
    Box::into_raw(Box::new(AvidaRunningStatsHandle {
        n: other_ref.n,
        m1: other_ref.m1,
        m2: other_ref.m2,
        m3: other_ref.m3,
        m4: other_ref.m4,
    }))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rs_free(handle: *mut AvidaRunningStatsHandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer came from Box::into_raw in this crate and is freed exactly once here.
    unsafe {
        drop(Box::from_raw(handle));
    }
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
        let mut rs = AvidaRunningStatsHandle::new();
        for v in [1.0_f64, 2.0, 3.0, 4.0] {
            rs.push(v);
        }
        assert_eq!(rs.n, 4.0);
        assert!((rs.m1 - 2.5).abs() < 1e-12);
    }
}
