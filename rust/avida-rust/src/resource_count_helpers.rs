use std::ffi::{c_char, c_int, CStr};

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rc_lookup_resource_index(
    names: *const *const c_char,
    count: c_int,
    query: *const c_char,
) -> c_int {
    if names.is_null() || query.is_null() || count <= 0 {
        return -1;
    }
    // SAFETY: query was checked for null and is only read.
    let query_bytes = unsafe { CStr::from_ptr(query) }.to_bytes();
    for i in 0..count {
        // SAFETY: names was checked for null and i is within [0, count).
        let name_ptr = unsafe { *names.add(i as usize) };
        if name_ptr.is_null() {
            return -1;
        }
        // SAFETY: name_ptr validated non-null and is only read.
        let name_bytes = unsafe { CStr::from_ptr(name_ptr) }.to_bytes();
        if name_bytes == query_bytes {
            return i;
        }
    }
    -1
}

#[no_mangle]
pub extern "C" fn avd_rc_step_inflow(inflow: f64, update_step: f64) -> f64 {
    inflow * update_step
}

#[no_mangle]
pub extern "C" fn avd_rc_step_decay(decay_rate: f64, update_step: f64) -> f64 {
    decay_rate.powf(update_step)
}

#[no_mangle]
pub extern "C" fn avd_rc_inflow_precalc_next(
    previous: f64,
    step_decay: f64,
    step_inflow: f64,
) -> f64 {
    previous * step_decay + step_inflow
}

#[no_mangle]
pub extern "C" fn avd_rc_decay_precalc_next(previous: f64, step_decay: f64) -> f64 {
    previous * step_decay
}

#[no_mangle]
pub extern "C" fn avd_rc_accumulate_update_time(current: f64, delta: f64) -> f64 {
    current + delta
}

fn num_steps_from_ratio(update_time: f64, update_step: f64) -> c_int {
    // Scheduling only supports finite, positive step sizes.
    if !update_step.is_finite() || update_step <= 0.0 {
        return 0;
    }
    // Mirror legacy cast behavior for non-finite update_time.
    if update_time.is_nan() {
        return 0;
    }

    let ratio = update_time / update_step;
    if ratio >= c_int::MAX as f64 {
        return c_int::MAX;
    }
    if ratio <= c_int::MIN as f64 {
        return c_int::MIN;
    }
    ratio.trunc() as c_int
}

fn saturating_update_delta(current_update: c_int, previous_update: c_int) -> c_int {
    current_update.saturating_sub(previous_update)
}

#[no_mangle]
pub extern "C" fn avd_rc_num_steps(update_time: f64, update_step: f64) -> c_int {
    num_steps_from_ratio(update_time, update_step)
}

#[no_mangle]
pub extern "C" fn avd_rc_num_spatial_updates(
    current_update: c_int,
    previous_update: c_int,
) -> c_int {
    saturating_update_delta(current_update, previous_update)
}

#[no_mangle]
pub extern "C" fn avd_rc_remainder_update_time(
    update_time: f64,
    update_step: f64,
    num_steps: c_int,
) -> f64 {
    update_time - (num_steps as f64) * update_step
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn lookup_handles_empty_and_null_inputs() {
        let query = CString::new("resA").expect("literal has no NUL");
        assert_eq!(
            avd_rc_lookup_resource_index(std::ptr::null(), 1, query.as_ptr()),
            -1
        );
        assert_eq!(
            avd_rc_lookup_resource_index(std::ptr::null(), 0, query.as_ptr()),
            -1
        );
        assert_eq!(
            avd_rc_lookup_resource_index(std::ptr::null(), 0, std::ptr::null()),
            -1
        );
    }

    #[test]
    fn lookup_returns_first_match_and_not_found() {
        let a = CString::new("resA").expect("literal has no NUL");
        let b = CString::new("resB").expect("literal has no NUL");
        let dup = CString::new("resA").expect("literal has no NUL");
        let query_a = CString::new("resA").expect("literal has no NUL");
        let query_b = CString::new("resB").expect("literal has no NUL");
        let query_x = CString::new("resX").expect("literal has no NUL");
        let names = [a.as_ptr(), b.as_ptr(), dup.as_ptr()];

        assert_eq!(
            avd_rc_lookup_resource_index(names.as_ptr(), names.len() as c_int, query_a.as_ptr()),
            0
        );
        assert_eq!(
            avd_rc_lookup_resource_index(names.as_ptr(), names.len() as c_int, query_b.as_ptr()),
            1
        );
        assert_eq!(
            avd_rc_lookup_resource_index(names.as_ptr(), names.len() as c_int, query_x.as_ptr()),
            -1
        );
    }

    #[test]
    fn lookup_rejects_null_name_entries() {
        let a = CString::new("resA").expect("literal has no NUL");
        let query_b = CString::new("resB").expect("literal has no NUL");
        let names = [a.as_ptr(), std::ptr::null()];
        assert_eq!(
            avd_rc_lookup_resource_index(names.as_ptr(), names.len() as c_int, query_b.as_ptr()),
            -1
        );
    }

    #[test]
    fn rc_precalc_helpers_match_reference_math() {
        let step = avd_rc_step_decay(0.95, 1.0 / 10000.0);
        assert!((step - 0.95_f64.powf(1.0 / 10000.0)).abs() < 1e-15);
        assert!((avd_rc_step_inflow(2.5, 0.1) - 0.25).abs() < 1e-15);

        let mut inflow_prev = 0.0;
        let mut decay_prev = 1.0;
        let step_inflow = avd_rc_step_inflow(1.75, 1.0 / 10000.0);
        let step_decay = avd_rc_step_decay(0.9, 1.0 / 10000.0);

        for _ in 0..64 {
            inflow_prev = avd_rc_inflow_precalc_next(inflow_prev, step_decay, step_inflow);
            decay_prev = avd_rc_decay_precalc_next(decay_prev, step_decay);
        }

        let mut inflow_ref = 0.0;
        let mut decay_ref = 1.0;
        for _ in 0..64 {
            inflow_ref = inflow_ref * step_decay + step_inflow;
            decay_ref *= step_decay;
        }

        assert!((inflow_prev - inflow_ref).abs() < 1e-12);
        assert!((decay_prev - decay_ref).abs() < 1e-12);
    }

    #[test]
    fn rc_scheduling_helpers_match_reference_math() {
        let step = 1.0 / 10000.0;
        assert!((avd_rc_accumulate_update_time(0.25, 0.125) - 0.375).abs() < 1e-15);

        let cases = [
            (0.0, 0),
            (0.5 * step, 0),
            (-0.5 * step, 0),
            (step, 1),
            (2.9 * step, 2),
            (-2.9 * step, -2),
            (25.0 * step, 25),
        ];
        for (time, expected_steps) in cases {
            let got_steps = avd_rc_num_steps(time, step);
            assert_eq!(got_steps, expected_steps);
            let rem = avd_rc_remainder_update_time(time, step, got_steps);
            let expected = time - (expected_steps as f64) * step;
            assert!((rem - expected).abs() < 1e-15);
        }

        assert_eq!(avd_rc_num_steps(1.0, 0.0), 0);
        assert_eq!(avd_rc_num_steps(1.0, -step), 0);
        assert_eq!(avd_rc_num_steps(1.0, f64::INFINITY), 0);
        assert_eq!(avd_rc_num_steps(f64::NAN, step), 0);
        assert_eq!(avd_rc_num_steps(f64::INFINITY, step), c_int::MAX);
        assert_eq!(avd_rc_num_steps(f64::NEG_INFINITY, step), c_int::MIN);
    }

    #[test]
    fn rc_spatial_scheduling_delta_saturates() {
        assert_eq!(avd_rc_num_spatial_updates(10, 4), 6);
        assert_eq!(avd_rc_num_spatial_updates(4, 10), -6);
        assert_eq!(avd_rc_num_spatial_updates(c_int::MAX, -1), c_int::MAX);
        assert_eq!(avd_rc_num_spatial_updates(c_int::MIN, 1), c_int::MIN);
    }
}
