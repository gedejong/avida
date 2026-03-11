use std::ffi::{c_double, c_int};

use crate::{
    common::{with_hist_mut, with_hist_ref},
    AvidaHistogramHandle,
};

impl AvidaHistogramHandle {
    fn new(max_bin: i32, min_bin: i32) -> Option<Self> {
        if max_bin < min_bin {
            return None;
        }
        let num_bins = usize::try_from(max_bin - min_bin + 1).ok()?;
        Some(Self {
            bins: vec![0; num_bins],
            min_bin,
            max_bin,
            entry_count: 0,
            entry_total: 0,
        })
    }

    fn num_bins(&self) -> i32 {
        self.max_bin - self.min_bin + 1
    }

    fn idx(&self, value: i32) -> Option<usize> {
        if value < self.min_bin || value > self.max_bin {
            None
        } else {
            usize::try_from(value - self.min_bin).ok()
        }
    }

    fn resize(&mut self, new_max: i32, new_min: i32) {
        if new_max < new_min {
            return;
        }
        let new_num_bins = match usize::try_from(new_max - new_min + 1) {
            Ok(v) => v,
            Err(_) => return,
        };
        let mut new_bins = vec![0_i32; new_num_bins];
        let overlap_min = self.min_bin.max(new_min);
        let overlap_max = self.max_bin.min(new_max);
        if overlap_min <= overlap_max {
            for cur_bin in overlap_min..=overlap_max {
                let new_idx = usize::try_from(cur_bin - new_min).unwrap_or(0);
                let old_idx = usize::try_from(cur_bin - self.min_bin).unwrap_or(0);
                new_bins[new_idx] = self.bins[old_idx];
            }
        }
        self.bins = new_bins;
        self.max_bin = new_max;
        self.min_bin = new_min;

        let mut new_count = 0_i32;
        let mut new_total = 0_i32;
        for (i, count) in self.bins.iter().enumerate() {
            let value = i as i32 + self.min_bin;
            new_count += *count;
            new_total += *count * value;
        }
        self.entry_count = new_count;
        self.entry_total = new_total;
    }

    fn clear(&mut self) {
        self.bins.fill(0);
        self.entry_count = 0;
        self.entry_total = 0;
    }

    fn insert(&mut self, value: i32, count: i32) {
        if let Some(idx) = self.idx(value) {
            self.bins[idx] += count;
            self.entry_count += count;
            self.entry_total += value * count;
        }
    }

    fn remove(&mut self, value: i32) {
        if let Some(idx) = self.idx(value) {
            self.bins[idx] -= 1;
            self.entry_count -= 1;
            self.entry_total -= value;
        }
    }

    fn remove_bin(&mut self, value: i32) {
        if let Some(idx) = self.idx(value) {
            let old_size = self.bins[idx];
            self.bins[idx] = 0;
            self.entry_count -= old_size;
            self.entry_total -= value * old_size;
        }
    }

    fn average(&self) -> f64 {
        self.entry_total as f64 / self.entry_count as f64
    }

    fn count_average(&self) -> f64 {
        self.entry_count as f64 / self.num_bins() as f64
    }

    fn mode(&self) -> i32 {
        let mut mode = 0_usize;
        for i in 1..self.bins.len() {
            if self.bins[i] > self.bins[mode] {
                mode = i;
            }
        }
        mode as i32 + self.min_bin
    }

    fn variance(&self) -> f64 {
        if self.entry_count < 2 {
            return 0.0;
        }
        let mean = self.average();
        let mut var = 0.0;
        for (i, count) in self.bins.iter().enumerate() {
            let value = i as i32 + self.min_bin;
            let d = value as f64 - mean;
            var += *count as f64 * d * d;
        }
        var / (self.entry_count - 1) as f64
    }

    fn count_variance(&self) -> f64 {
        let num_bins = self.num_bins();
        if num_bins < 2 {
            return 0.0;
        }
        let mean = self.count_average();
        let mut var = 0.0;
        for count in &self.bins {
            let d = *count as f64 - mean;
            var += d * d;
        }
        var / (num_bins - 1) as f64
    }

    fn std_dev(&self) -> f64 {
        self.variance().sqrt()
    }

    fn count_std_dev(&self) -> f64 {
        self.count_variance().sqrt()
    }

    fn entropy(&self) -> f64 {
        let mut entropy = 0.0;
        for count in &self.bins {
            let prob = *count as f64 / self.entry_count as f64;
            entropy -= prob * prob.ln();
        }
        entropy
    }

    fn norm_entropy(&self) -> f64 {
        let mut entropy = 0.0;
        for count in &self.bins {
            let prob = *count as f64 / self.entry_count as f64;
            if prob != 0.0 {
                entropy -= prob * prob.ln();
            }
        }
        entropy / (self.num_bins() as f64).ln()
    }
}

#[no_mangle]
pub extern "C" fn avd_hist_new(max_bin: c_int, min_bin: c_int) -> *mut AvidaHistogramHandle {
    match AvidaHistogramHandle::new(max_bin, min_bin) {
        Some(h) => Box::into_raw(Box::new(h)),
        None => std::ptr::null_mut(),
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_hist_free(handle: *mut AvidaHistogramHandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer came from Box::into_raw in this crate and is freed exactly once here.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub extern "C" fn avd_hist_resize(
    handle: *mut AvidaHistogramHandle,
    new_max: c_int,
    new_min: c_int,
) {
    with_hist_mut(handle, |h| h.resize(new_max, new_min));
}

#[no_mangle]
pub extern "C" fn avd_hist_clear(handle: *mut AvidaHistogramHandle) {
    with_hist_mut(handle, |h| h.clear());
}

#[no_mangle]
pub extern "C" fn avd_hist_insert(handle: *mut AvidaHistogramHandle, value: c_int, count: c_int) {
    with_hist_mut(handle, |h| h.insert(value, count));
}

#[no_mangle]
pub extern "C" fn avd_hist_remove(handle: *mut AvidaHistogramHandle, value: c_int) {
    with_hist_mut(handle, |h| h.remove(value));
}

#[no_mangle]
pub extern "C" fn avd_hist_remove_bin(handle: *mut AvidaHistogramHandle, value: c_int) {
    with_hist_mut(handle, |h| h.remove_bin(value));
}

#[no_mangle]
pub extern "C" fn avd_hist_get_average(handle: *const AvidaHistogramHandle) -> c_double {
    with_hist_ref(handle, 0.0, |h| h.average())
}

#[no_mangle]
pub extern "C" fn avd_hist_get_count_average(handle: *const AvidaHistogramHandle) -> c_double {
    with_hist_ref(handle, 0.0, |h| h.count_average())
}

#[no_mangle]
pub extern "C" fn avd_hist_get_mode(handle: *const AvidaHistogramHandle) -> c_int {
    with_hist_ref(handle, 0, |h| h.mode())
}

#[no_mangle]
pub extern "C" fn avd_hist_get_variance(handle: *const AvidaHistogramHandle) -> c_double {
    with_hist_ref(handle, 0.0, |h| h.variance())
}

#[no_mangle]
pub extern "C" fn avd_hist_get_count_variance(handle: *const AvidaHistogramHandle) -> c_double {
    with_hist_ref(handle, 0.0, |h| h.count_variance())
}

#[no_mangle]
pub extern "C" fn avd_hist_get_std_dev(handle: *const AvidaHistogramHandle) -> c_double {
    with_hist_ref(handle, 0.0, |h| h.std_dev())
}

#[no_mangle]
pub extern "C" fn avd_hist_get_count_std_dev(handle: *const AvidaHistogramHandle) -> c_double {
    with_hist_ref(handle, 0.0, |h| h.count_std_dev())
}

#[no_mangle]
pub extern "C" fn avd_hist_get_entropy(handle: *const AvidaHistogramHandle) -> c_double {
    with_hist_ref(handle, 0.0, |h| h.entropy())
}

#[no_mangle]
pub extern "C" fn avd_hist_get_norm_entropy(handle: *const AvidaHistogramHandle) -> c_double {
    with_hist_ref(handle, 0.0, |h| h.norm_entropy())
}

#[no_mangle]
pub extern "C" fn avd_hist_get_count(handle: *const AvidaHistogramHandle) -> c_int {
    with_hist_ref(handle, 0, |h| h.entry_count)
}

#[no_mangle]
pub extern "C" fn avd_hist_get_count_for_value(
    handle: *const AvidaHistogramHandle,
    value: c_int,
) -> c_int {
    with_hist_ref(handle, 0, |h| h.idx(value).map_or(0, |i| h.bins[i]))
}

#[no_mangle]
pub extern "C" fn avd_hist_get_total(handle: *const AvidaHistogramHandle) -> c_int {
    with_hist_ref(handle, 0, |h| h.entry_total)
}

#[no_mangle]
pub extern "C" fn avd_hist_get_min_bin(handle: *const AvidaHistogramHandle) -> c_int {
    with_hist_ref(handle, 0, |h| h.min_bin)
}

#[no_mangle]
pub extern "C" fn avd_hist_get_max_bin(handle: *const AvidaHistogramHandle) -> c_int {
    with_hist_ref(handle, 0, |h| h.max_bin)
}

#[no_mangle]
pub extern "C" fn avd_hist_get_num_bins(handle: *const AvidaHistogramHandle) -> c_int {
    with_hist_ref(handle, 0, |h| h.num_bins())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn histogram_insert_remove_resize_and_stats() {
        let h = avd_hist_new(4, 0);
        assert!(!h.is_null());
        assert_eq!(avd_hist_get_num_bins(h), 5);
        assert_eq!(avd_hist_get_min_bin(h), 0);
        assert_eq!(avd_hist_get_max_bin(h), 4);

        avd_hist_insert(h, 0, 1);
        avd_hist_insert(h, 2, 3);
        avd_hist_insert(h, 4, 2);
        assert_eq!(avd_hist_get_count(h), 6);
        assert_eq!(avd_hist_get_total(h), 14);
        assert_eq!(avd_hist_get_mode(h), 2);
        assert_eq!(avd_hist_get_count_for_value(h, 2), 3);
        assert!(avd_hist_get_average(h) > 2.0);
        assert!(avd_hist_get_variance(h) > 0.0);
        assert!(avd_hist_get_count_variance(h) > 0.0);
        assert!(avd_hist_get_std_dev(h) > 0.0);
        assert!(avd_hist_get_count_std_dev(h) > 0.0);
        // Legacy behavior: entropy path does not skip zero-probability bins and may produce NaN.
        assert!(avd_hist_get_entropy(h).is_nan());
        assert!(avd_hist_get_norm_entropy(h).is_finite());

        avd_hist_remove(h, 4);
        assert_eq!(avd_hist_get_count_for_value(h, 4), 1);
        avd_hist_remove_bin(h, 2);
        assert_eq!(avd_hist_get_count_for_value(h, 2), 0);
        assert_eq!(avd_hist_get_count(h), 2);

        avd_hist_resize(h, 6, -1);
        assert_eq!(avd_hist_get_min_bin(h), -1);
        assert_eq!(avd_hist_get_max_bin(h), 6);
        assert_eq!(avd_hist_get_num_bins(h), 8);
        assert_eq!(avd_hist_get_count_for_value(h, 0), 1);
        assert_eq!(avd_hist_get_count_for_value(h, 4), 1);

        avd_hist_clear(h);
        assert_eq!(avd_hist_get_count(h), 0);
        assert_eq!(avd_hist_get_total(h), 0);
        avd_hist_free(h);
    }

    #[test]
    fn histogram_null_and_invalid_paths_are_safe() {
        assert!(avd_hist_new(0, 1).is_null());
        avd_hist_free(std::ptr::null_mut());
        avd_hist_resize(std::ptr::null_mut(), 1, 0);
        avd_hist_clear(std::ptr::null_mut());
        avd_hist_insert(std::ptr::null_mut(), 1, 1);
        avd_hist_remove(std::ptr::null_mut(), 1);
        avd_hist_remove_bin(std::ptr::null_mut(), 1);
        assert_eq!(avd_hist_get_count(std::ptr::null()), 0);
        assert_eq!(avd_hist_get_total(std::ptr::null()), 0);
        assert_eq!(avd_hist_get_average(std::ptr::null()), 0.0);
    }
}
