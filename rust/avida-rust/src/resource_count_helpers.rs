use crate::common::{with_cstr, with_mut_slice, with_slice};
use std::ffi::{c_char, c_int};

const RC_GEOMETRY_GLOBAL: c_int = 0;
const RC_GEOMETRY_GRID: c_int = 1;
const RC_GEOMETRY_TORUS: c_int = 2;
const RC_GEOMETRY_PARTIAL: c_int = 5;
const RC_DISPATCH_NONE: c_int = 0;
const RC_DISPATCH_NONSPATIAL: c_int = 1;
const RC_DISPATCH_SPATIAL: c_int = 2;
const RC_WRAPPER_GLOBAL: c_int = 0;
const RC_READ_PATH_GLOBAL: c_int = 0;
const RC_READ_PATH_SPATIAL: c_int = 1;
const RC_SETCELL_GLOBAL_NOOP: c_int = 0;
const RC_SETCELL_SPATIAL_WRITE: c_int = 1;
const RC_SETUP_PATH_GLOBAL: c_int = 0;
const RC_SETUP_PATH_PARTIAL: c_int = 1;
const RC_SETUP_PATH_SPATIAL: c_int = 2;
const RC_GRAD_SETTER_PEAK_X: c_int = 0;
const RC_GRAD_SETTER_PEAK_Y: c_int = 1;
const RC_GRAD_SETTER_HEIGHT: c_int = 2;
const RC_GRAD_SETTER_SPREAD: c_int = 3;
const RC_GRAD_SETTER_PLATEAU: c_int = 4;
const RC_GRAD_SETTER_INITIAL_PLAT: c_int = 5;
const RC_GRAD_SETTER_DECAY: c_int = 6;
const RC_GRAD_SETTER_MAX_X: c_int = 7;
const RC_GRAD_SETTER_MAX_Y: c_int = 8;
const RC_GRAD_SETTER_MIN_X: c_int = 9;
const RC_GRAD_SETTER_MIN_Y: c_int = 10;
const RC_GRAD_SETTER_MOVE_SCALER: c_int = 11;
const RC_GRAD_SETTER_UPDATE_STEP: c_int = 12;
const RC_GRAD_SETTER_IS_HALO: c_int = 13;
const RC_GRAD_SETTER_HALO_INNER_RADIUS: c_int = 14;
const RC_GRAD_SETTER_HALO_WIDTH: c_int = 15;
const RC_GRAD_SETTER_HALO_ANCHOR_X: c_int = 16;
const RC_GRAD_SETTER_HALO_ANCHOR_Y: c_int = 17;
const RC_GRAD_SETTER_MOVE_SPEED: c_int = 18;
const RC_GRAD_SETTER_MOVE_RESISTANCE: c_int = 19;
const RC_GRAD_SETTER_PLATEAU_INFLOW: c_int = 20;
const RC_GRAD_SETTER_PLATEAU_OUTFLOW: c_int = 21;
const RC_GRAD_SETTER_CONE_INFLOW: c_int = 22;
const RC_GRAD_SETTER_CONE_OUTFLOW: c_int = 23;
const RC_GRAD_SETTER_GRADIENT_INFLOW: c_int = 24;
const RC_GRAD_SETTER_PLATEAU_COMMON: c_int = 25;
const RC_GRAD_SETTER_FLOOR: c_int = 26;
const RC_GRAD_SETTER_HABITAT: c_int = 27;
const RC_GRAD_SETTER_MIN_SIZE: c_int = 28;
const RC_GRAD_SETTER_MAX_SIZE: c_int = 29;
const RC_GRAD_SETTER_CONFIG: c_int = 30;
const RC_GRAD_SETTER_COUNT: c_int = 31;
const RC_GRAD_SETTER_RESISTANCE: c_int = 32;
const RC_GRAD_SETTER_DAMAGE: c_int = 33;
const RC_GRAD_SETTER_THRESHOLD: c_int = 34;
const RC_GRAD_SETTER_REFUGE: c_int = 35;
const RC_GRAD_SETTER_DEATH_ODDS: c_int = 36;
const RC_GRAD_SETTER_INVALID: c_int = -1;
const RC_GRADIENT_SETTER_SEQUENCE: [c_int; 37] = [
    RC_GRAD_SETTER_PEAK_X,
    RC_GRAD_SETTER_PEAK_Y,
    RC_GRAD_SETTER_HEIGHT,
    RC_GRAD_SETTER_SPREAD,
    RC_GRAD_SETTER_PLATEAU,
    RC_GRAD_SETTER_INITIAL_PLAT,
    RC_GRAD_SETTER_DECAY,
    RC_GRAD_SETTER_MAX_X,
    RC_GRAD_SETTER_MAX_Y,
    RC_GRAD_SETTER_MIN_X,
    RC_GRAD_SETTER_MIN_Y,
    RC_GRAD_SETTER_MOVE_SCALER,
    RC_GRAD_SETTER_UPDATE_STEP,
    RC_GRAD_SETTER_IS_HALO,
    RC_GRAD_SETTER_HALO_INNER_RADIUS,
    RC_GRAD_SETTER_HALO_WIDTH,
    RC_GRAD_SETTER_HALO_ANCHOR_X,
    RC_GRAD_SETTER_HALO_ANCHOR_Y,
    RC_GRAD_SETTER_MOVE_SPEED,
    RC_GRAD_SETTER_MOVE_RESISTANCE,
    RC_GRAD_SETTER_PLATEAU_INFLOW,
    RC_GRAD_SETTER_PLATEAU_OUTFLOW,
    RC_GRAD_SETTER_CONE_INFLOW,
    RC_GRAD_SETTER_CONE_OUTFLOW,
    RC_GRAD_SETTER_GRADIENT_INFLOW,
    RC_GRAD_SETTER_PLATEAU_COMMON,
    RC_GRAD_SETTER_FLOOR,
    RC_GRAD_SETTER_HABITAT,
    RC_GRAD_SETTER_MIN_SIZE,
    RC_GRAD_SETTER_MAX_SIZE,
    RC_GRAD_SETTER_CONFIG,
    RC_GRAD_SETTER_COUNT,
    RC_GRAD_SETTER_RESISTANCE,
    RC_GRAD_SETTER_DAMAGE,
    RC_GRAD_SETTER_THRESHOLD,
    RC_GRAD_SETTER_REFUGE,
    RC_GRAD_SETTER_DEATH_ODDS,
];

#[no_mangle]
pub extern "C" fn avd_rc_lookup_resource_index(
    names: *const *const c_char,
    count: c_int,
    query: *const c_char,
) -> c_int {
    let query_bytes = with_cstr(query, None::<Vec<u8>>, |q| Some(q.to_bytes().to_vec()));
    let Some(query_bytes) = query_bytes else {
        return -1;
    };

    with_slice(names, count, -1, |name_ptrs| {
        for (i, name_ptr) in name_ptrs.iter().enumerate() {
            if name_ptr.is_null() {
                return -1;
            }
            let is_match = with_cstr(*name_ptr, false, |name| {
                name.to_bytes() == query_bytes.as_slice()
            });
            if is_match {
                return i as c_int;
            }
        }
        -1
    })
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

fn update_time_delta(in_time: f64) -> f64 {
    in_time
}

fn wrapper_global_only_flag(wrapper_mode: c_int) -> c_int {
    if wrapper_mode == RC_WRAPPER_GLOBAL {
        1
    } else {
        0
    }
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

fn spatial_step_iterations(num_updates: c_int) -> c_int {
    num_updates.max(0)
}

fn use_cell_list_branch(cell_list_size: c_int) -> c_int {
    if cell_list_size > 0 {
        1
    } else {
        0
    }
}

fn is_spatial_geometry(geometry: c_int) -> c_int {
    if geometry == RC_GEOMETRY_GLOBAL || geometry == RC_GEOMETRY_PARTIAL {
        0
    } else {
        1
    }
}

fn dispatch_action(is_spatial: c_int, global_only: c_int) -> c_int {
    if is_spatial == 0 {
        RC_DISPATCH_NONSPATIAL
    } else if global_only != 0 {
        RC_DISPATCH_NONE
    } else {
        RC_DISPATCH_SPATIAL
    }
}

fn should_advance_last_updated(global_only: c_int) -> c_int {
    if global_only == 0 {
        1
    } else {
        0
    }
}

fn read_path_kind(geometry: c_int) -> c_int {
    if is_spatial_geometry(geometry) != 0 {
        RC_READ_PATH_SPATIAL
    } else {
        RC_READ_PATH_GLOBAL
    }
}

fn setcell_write_path_kind(geometry: c_int) -> c_int {
    if is_spatial_geometry(geometry) != 0 {
        RC_SETCELL_SPATIAL_WRITE
    } else {
        RC_SETCELL_GLOBAL_NOOP
    }
}

fn setup_path_kind(geometry: c_int) -> c_int {
    if geometry == RC_GEOMETRY_GLOBAL {
        RC_SETUP_PATH_GLOBAL
    } else if geometry == RC_GEOMETRY_PARTIAL {
        RC_SETUP_PATH_PARTIAL
    } else {
        RC_SETUP_PATH_SPATIAL
    }
}

fn should_log_spatial_rectangles(geometry: c_int) -> c_int {
    if geometry == RC_GEOMETRY_GRID || geometry == RC_GEOMETRY_TORUS {
        1
    } else {
        0
    }
}

fn resize_cell_count(world_x: c_int, world_y: c_int) -> c_int {
    world_x.wrapping_mul(world_y)
}

fn gradient_setter_count() -> c_int {
    RC_GRADIENT_SETTER_SEQUENCE.len() as c_int
}

fn gradient_setter_opcode(index: c_int) -> c_int {
    let Ok(index) = usize::try_from(index) else {
        return RC_GRAD_SETTER_INVALID;
    };
    RC_GRADIENT_SETTER_SEQUENCE
        .get(index)
        .copied()
        .unwrap_or(RC_GRAD_SETTER_INVALID)
}

fn apply_nonspatial_steps_internal(
    mut current: f64,
    decay_precalc: &[f64],
    inflow_precalc: &[f64],
    precalc_distance: c_int,
    num_steps: c_int,
) -> f64 {
    if precalc_distance < 0 || num_steps < 0 {
        return current;
    }
    let p = precalc_distance as usize;
    if p == 0 {
        return current * decay_precalc[0] + inflow_precalc[0];
    }
    let mut remaining = num_steps as usize;
    while remaining > p {
        current = current * decay_precalc[p] + inflow_precalc[p];
        remaining -= p;
    }
    current * decay_precalc[remaining] + inflow_precalc[remaining]
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
pub extern "C" fn avd_rc_spatial_step_iterations(num_updates: c_int) -> c_int {
    spatial_step_iterations(num_updates)
}

#[no_mangle]
pub extern "C" fn avd_rc_use_cell_list_branch(cell_list_size: c_int) -> c_int {
    use_cell_list_branch(cell_list_size)
}

#[no_mangle]
pub extern "C" fn avd_rc_is_spatial_geometry(geometry: c_int) -> c_int {
    is_spatial_geometry(geometry)
}

#[no_mangle]
pub extern "C" fn avd_rc_dispatch_action(is_spatial: c_int, global_only: c_int) -> c_int {
    dispatch_action(is_spatial, global_only)
}

#[no_mangle]
pub extern "C" fn avd_rc_should_advance_last_updated(global_only: c_int) -> c_int {
    should_advance_last_updated(global_only)
}

#[no_mangle]
pub extern "C" fn avd_rc_read_path_kind(geometry: c_int) -> c_int {
    read_path_kind(geometry)
}

#[no_mangle]
pub extern "C" fn avd_rc_setcell_write_path_kind(geometry: c_int) -> c_int {
    setcell_write_path_kind(geometry)
}

#[no_mangle]
pub extern "C" fn avd_rc_setup_path_kind(geometry: c_int) -> c_int {
    setup_path_kind(geometry)
}

#[no_mangle]
pub extern "C" fn avd_rc_should_log_spatial_rectangles(geometry: c_int) -> c_int {
    should_log_spatial_rectangles(geometry)
}

#[no_mangle]
pub extern "C" fn avd_rc_resize_cell_count(world_x: c_int, world_y: c_int) -> c_int {
    resize_cell_count(world_x, world_y)
}

#[no_mangle]
pub extern "C" fn avd_rc_gradient_setter_count() -> c_int {
    gradient_setter_count()
}

#[no_mangle]
pub extern "C" fn avd_rc_gradient_setter_opcode(index: c_int) -> c_int {
    gradient_setter_opcode(index)
}

#[no_mangle]
pub extern "C" fn avd_rc_update_time_delta(in_time: f64) -> f64 {
    update_time_delta(in_time)
}

#[no_mangle]
pub extern "C" fn avd_rc_wrapper_global_only_flag(wrapper_mode: c_int) -> c_int {
    wrapper_global_only_flag(wrapper_mode)
}

#[no_mangle]
pub extern "C" fn avd_rc_remainder_update_time(
    update_time: f64,
    update_step: f64,
    num_steps: c_int,
) -> f64 {
    update_time - (num_steps as f64) * update_step
}

#[no_mangle]
pub extern "C" fn avd_rc_apply_nonspatial_steps(
    current: f64,
    decay_precalc: *const f64,
    inflow_precalc: *const f64,
    precalc_distance: c_int,
    num_steps: c_int,
) -> f64 {
    if precalc_distance < 0 || num_steps < 0 {
        return current;
    }
    let count = match precalc_distance.checked_add(1) {
        Some(v) => v,
        None => return current,
    };
    with_slice(decay_precalc, count, current, |decay_slice| {
        with_slice(inflow_precalc, count, current, |inflow_slice| {
            apply_nonspatial_steps_internal(
                current,
                decay_slice,
                inflow_slice,
                precalc_distance,
                num_steps,
            )
        })
    })
}

#[no_mangle]
pub extern "C" fn avd_rc_fill_precalc_tables(
    decay_rate: f64,
    inflow: f64,
    update_step: f64,
    precalc_distance: c_int,
    out_decay: *mut f64,
    out_inflow: *mut f64,
) {
    if precalc_distance < 0 || out_decay.is_null() || out_inflow.is_null() {
        return;
    }
    let count = match precalc_distance.checked_add(1) {
        Some(v) => v,
        None => return,
    };
    let mut prepared_decay = vec![0.0_f64; count as usize];
    let mut prepared_inflow = vec![0.0_f64; count as usize];

    let step_decay = avd_rc_step_decay(decay_rate, update_step);
    let step_inflow = avd_rc_step_inflow(inflow, update_step);

    prepared_decay[0] = 1.0;
    prepared_inflow[0] = 0.0;
    for i in 1..prepared_decay.len() {
        prepared_decay[i] = avd_rc_decay_precalc_next(prepared_decay[i - 1], step_decay);
        prepared_inflow[i] =
            avd_rc_inflow_precalc_next(prepared_inflow[i - 1], step_decay, step_inflow);
    }

    with_mut_slice(out_decay, count, |decay_slice| {
        decay_slice.copy_from_slice(&prepared_decay);
    });
    with_mut_slice(out_inflow, count, |inflow_slice| {
        inflow_slice.copy_from_slice(&prepared_inflow);
    });
}

#[no_mangle]
pub extern "C" fn avd_rc_fill_inflow_precalc_table(
    decay_rate: f64,
    inflow: f64,
    update_step: f64,
    precalc_distance: c_int,
    out_inflow: *mut f64,
) {
    if precalc_distance < 0 || out_inflow.is_null() {
        return;
    }
    let count = match precalc_distance.checked_add(1) {
        Some(v) => v,
        None => return,
    };
    let mut prepared_inflow = vec![0.0_f64; count as usize];

    let step_decay = avd_rc_step_decay(decay_rate, update_step);
    let step_inflow = avd_rc_step_inflow(inflow, update_step);
    prepared_inflow[0] = 0.0;
    for i in 1..prepared_inflow.len() {
        prepared_inflow[i] =
            avd_rc_inflow_precalc_next(prepared_inflow[i - 1], step_decay, step_inflow);
    }

    with_mut_slice(out_inflow, count, |inflow_slice| {
        inflow_slice.copy_from_slice(&prepared_inflow);
    });
}

#[no_mangle]
pub extern "C" fn avd_rc_fill_decay_precalc_table(
    decay_rate: f64,
    update_step: f64,
    precalc_distance: c_int,
    out_decay: *mut f64,
) {
    if precalc_distance < 0 || out_decay.is_null() {
        return;
    }
    let count = match precalc_distance.checked_add(1) {
        Some(v) => v,
        None => return,
    };
    let mut prepared_decay = vec![0.0_f64; count as usize];

    let step_decay = avd_rc_step_decay(decay_rate, update_step);
    prepared_decay[0] = 1.0;
    for i in 1..prepared_decay.len() {
        prepared_decay[i] = avd_rc_decay_precalc_next(prepared_decay[i - 1], step_decay);
    }

    with_mut_slice(out_decay, count, |decay_slice| {
        decay_slice.copy_from_slice(&prepared_decay);
    });
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
        let a = CString::new("resA").expect("literal has no NUL");
        let names = [a.as_ptr()];
        assert_eq!(
            avd_rc_lookup_resource_index(names.as_ptr(), names.len() as c_int, std::ptr::null()),
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
        assert_eq!(avd_rc_spatial_step_iterations(6), 6);
        assert_eq!(avd_rc_spatial_step_iterations(0), 0);
        assert_eq!(avd_rc_spatial_step_iterations(-6), 0);
        assert_eq!(avd_rc_use_cell_list_branch(4), 1);
        assert_eq!(avd_rc_use_cell_list_branch(0), 0);
        assert_eq!(avd_rc_use_cell_list_branch(-2), 0);
    }

    #[test]
    fn rc_dispatch_policy_matrix_matches_do_updates_semantics() {
        assert_eq!(avd_rc_is_spatial_geometry(0), 0);
        assert_eq!(avd_rc_is_spatial_geometry(5), 0);
        assert_eq!(avd_rc_is_spatial_geometry(1), 1);
        assert_eq!(avd_rc_is_spatial_geometry(2), 1);
        assert_eq!(avd_rc_is_spatial_geometry(42), 1);
        assert_eq!(avd_rc_is_spatial_geometry(-7), 1);

        assert_eq!(avd_rc_dispatch_action(0, 0), RC_DISPATCH_NONSPATIAL);
        assert_eq!(avd_rc_dispatch_action(0, 1), RC_DISPATCH_NONSPATIAL);
        assert_eq!(avd_rc_dispatch_action(1, 0), RC_DISPATCH_SPATIAL);
        assert_eq!(avd_rc_dispatch_action(1, 1), RC_DISPATCH_NONE);
        assert_eq!(avd_rc_dispatch_action(-1, 0), RC_DISPATCH_SPATIAL);
        assert_eq!(avd_rc_dispatch_action(2, 1), RC_DISPATCH_NONE);

        assert_eq!(avd_rc_should_advance_last_updated(0), 1);
        assert_eq!(avd_rc_should_advance_last_updated(1), 0);
        assert_eq!(avd_rc_should_advance_last_updated(-1), 0);
        assert_eq!(avd_rc_read_path_kind(0), RC_READ_PATH_GLOBAL);
        assert_eq!(avd_rc_read_path_kind(5), RC_READ_PATH_GLOBAL);
        assert_eq!(avd_rc_read_path_kind(1), RC_READ_PATH_SPATIAL);
        assert_eq!(avd_rc_read_path_kind(2), RC_READ_PATH_SPATIAL);
        assert_eq!(avd_rc_read_path_kind(42), RC_READ_PATH_SPATIAL);
        assert_eq!(avd_rc_setcell_write_path_kind(0), RC_SETCELL_GLOBAL_NOOP);
        assert_eq!(avd_rc_setcell_write_path_kind(5), RC_SETCELL_GLOBAL_NOOP);
        assert_eq!(avd_rc_setcell_write_path_kind(1), RC_SETCELL_SPATIAL_WRITE);
        assert_eq!(avd_rc_setcell_write_path_kind(2), RC_SETCELL_SPATIAL_WRITE);
        assert_eq!(avd_rc_setcell_write_path_kind(42), RC_SETCELL_SPATIAL_WRITE);
        assert_eq!(avd_rc_setup_path_kind(0), RC_SETUP_PATH_GLOBAL);
        assert_eq!(avd_rc_setup_path_kind(5), RC_SETUP_PATH_PARTIAL);
        assert_eq!(avd_rc_setup_path_kind(1), RC_SETUP_PATH_SPATIAL);
        assert_eq!(avd_rc_setup_path_kind(2), RC_SETUP_PATH_SPATIAL);
        assert_eq!(avd_rc_setup_path_kind(42), RC_SETUP_PATH_SPATIAL);
        assert_eq!(avd_rc_should_log_spatial_rectangles(0), 0);
        assert_eq!(avd_rc_should_log_spatial_rectangles(5), 0);
        assert_eq!(avd_rc_should_log_spatial_rectangles(1), 1);
        assert_eq!(avd_rc_should_log_spatial_rectangles(2), 1);
        assert_eq!(avd_rc_should_log_spatial_rectangles(42), 0);
        assert_eq!(avd_rc_resize_cell_count(40, 30), 1200);
        assert_eq!(avd_rc_resize_cell_count(0, 30), 0);
        assert_eq!(avd_rc_resize_cell_count(-2, 7), -14);
    }

    #[test]
    fn rc_gradient_setter_sequence_policy() {
        assert_eq!(
            avd_rc_gradient_setter_count(),
            RC_GRADIENT_SETTER_SEQUENCE.len() as c_int
        );
        for (i, expected) in RC_GRADIENT_SETTER_SEQUENCE.iter().enumerate() {
            assert_eq!(avd_rc_gradient_setter_opcode(i as c_int), *expected);
        }
    }

    #[test]
    fn rc_gradient_setter_sequence_guards() {
        assert_eq!(avd_rc_gradient_setter_opcode(-1), RC_GRAD_SETTER_INVALID);
        assert_eq!(
            avd_rc_gradient_setter_opcode(avd_rc_gradient_setter_count()),
            RC_GRAD_SETTER_INVALID
        );
        assert_eq!(avd_rc_gradient_setter_opcode(999), RC_GRAD_SETTER_INVALID);
    }

    #[test]
    fn rc_update_time_and_wrapper_policy_matrix() {
        let time_cases = [0.0, -0.25, f64::NAN, f64::INFINITY, f64::NEG_INFINITY];
        for input in time_cases {
            let got = avd_rc_update_time_delta(input);
            if input.is_nan() {
                assert!(got.is_nan());
            } else {
                assert_eq!(got, input);
            }
        }

        assert_eq!(avd_rc_wrapper_global_only_flag(RC_WRAPPER_GLOBAL), 1);
        assert_eq!(avd_rc_wrapper_global_only_flag(1), 0);
        assert_eq!(avd_rc_wrapper_global_only_flag(2), 0);
        assert_eq!(avd_rc_wrapper_global_only_flag(-1), 0);
        assert_eq!(avd_rc_wrapper_global_only_flag(99), 0);
    }

    #[test]
    fn rc_fill_precalc_tables_matches_reference_math() {
        let distance = 12;
        let mut decay = vec![0.0; (distance + 1) as usize];
        let mut inflow = vec![0.0; (distance + 1) as usize];
        let decay_rate = 0.91;
        let inflow_rate = 1.75;
        let step = 1.0 / 10000.0;
        avd_rc_fill_precalc_tables(
            decay_rate,
            inflow_rate,
            step,
            distance,
            decay.as_mut_ptr(),
            inflow.as_mut_ptr(),
        );
        assert!((decay[0] - 1.0).abs() < 1e-15);
        assert!((inflow[0] - 0.0).abs() < 1e-15);

        let step_decay = decay_rate.powf(step);
        let step_inflow = inflow_rate * step;
        let mut decay_ref = 1.0;
        let mut inflow_ref = 0.0;
        for i in 1..=distance as usize {
            decay_ref *= step_decay;
            inflow_ref = inflow_ref * step_decay + step_inflow;
            assert!((decay[i] - decay_ref).abs() < 1e-12);
            assert!((inflow[i] - inflow_ref).abs() < 1e-12);
        }
    }

    #[test]
    fn rc_fill_precalc_tables_rejects_invalid_inputs() {
        let mut decay = vec![123.0; 4];
        let mut inflow = vec![456.0; 4];
        avd_rc_fill_precalc_tables(0.9, 1.0, 0.1, -1, decay.as_mut_ptr(), inflow.as_mut_ptr());
        assert_eq!(decay, vec![123.0; 4]);
        assert_eq!(inflow, vec![456.0; 4]);

        avd_rc_fill_precalc_tables(0.9, 1.0, 0.1, 3, std::ptr::null_mut(), inflow.as_mut_ptr());
        assert_eq!(inflow, vec![456.0; 4]);
    }

    #[test]
    fn rc_fill_inflow_precalc_table_matches_reference_math() {
        let distance = 8;
        let mut inflow = vec![-1.0; (distance + 1) as usize];
        let decay_rate = 0.87;
        let inflow_rate = 1.25;
        let step = 0.1;
        avd_rc_fill_inflow_precalc_table(
            decay_rate,
            inflow_rate,
            step,
            distance,
            inflow.as_mut_ptr(),
        );

        let step_decay = decay_rate.powf(step);
        let step_inflow = inflow_rate * step;
        let mut inflow_ref = 0.0;
        assert!((inflow[0] - inflow_ref).abs() < 1e-15);
        for value in inflow.iter().take(distance as usize + 1).skip(1) {
            inflow_ref = inflow_ref * step_decay + step_inflow;
            assert!((*value - inflow_ref).abs() < 1e-12);
        }
    }

    #[test]
    fn rc_fill_decay_precalc_table_matches_reference_math() {
        let distance = 8;
        let mut decay = vec![-1.0; (distance + 1) as usize];
        let decay_rate = 0.93;
        let step = 0.125;
        avd_rc_fill_decay_precalc_table(decay_rate, step, distance, decay.as_mut_ptr());

        let step_decay = decay_rate.powf(step);
        let mut decay_ref = 1.0;
        assert!((decay[0] - decay_ref).abs() < 1e-15);
        for value in decay.iter().take(distance as usize + 1).skip(1) {
            decay_ref *= step_decay;
            assert!((*value - decay_ref).abs() < 1e-12);
        }
    }

    #[test]
    fn rc_fill_setter_precalc_tables_reject_invalid_inputs() {
        let mut decay = vec![123.0; 4];
        let mut inflow = vec![456.0; 4];

        avd_rc_fill_inflow_precalc_table(0.9, 1.0, 0.1, -1, inflow.as_mut_ptr());
        avd_rc_fill_decay_precalc_table(0.9, 0.1, -1, decay.as_mut_ptr());
        avd_rc_fill_inflow_precalc_table(0.9, 1.0, 0.1, 3, std::ptr::null_mut());
        avd_rc_fill_decay_precalc_table(0.9, 0.1, 3, std::ptr::null_mut());

        assert_eq!(decay, vec![123.0; 4]);
        assert_eq!(inflow, vec![456.0; 4]);
    }

    #[test]
    fn rc_apply_nonspatial_steps_matches_reference_cxx_loop() {
        let distance = 100;
        let mut decay = vec![0.0; (distance + 1) as usize];
        let mut inflow = vec![0.0; (distance + 1) as usize];
        avd_rc_fill_precalc_tables(
            0.91,
            1.75,
            1.0 / 10000.0,
            distance,
            decay.as_mut_ptr(),
            inflow.as_mut_ptr(),
        );

        let mut reference = 12.5;
        let mut remaining = 237;
        while remaining > distance {
            reference = reference * decay[distance as usize] + inflow[distance as usize];
            remaining -= distance;
        }
        reference = reference * decay[remaining as usize] + inflow[remaining as usize];

        let got =
            avd_rc_apply_nonspatial_steps(12.5, decay.as_ptr(), inflow.as_ptr(), distance, 237);
        assert!((got - reference).abs() < 1e-12);
    }

    #[test]
    fn rc_apply_nonspatial_steps_guards_invalid_inputs() {
        let decay = [1.0, 0.9];
        let inflow = [0.0, 0.1];
        assert_eq!(
            avd_rc_apply_nonspatial_steps(5.0, std::ptr::null(), inflow.as_ptr(), 1, 1),
            5.0
        );
        assert_eq!(
            avd_rc_apply_nonspatial_steps(5.0, decay.as_ptr(), std::ptr::null(), 1, 1),
            5.0
        );
        assert_eq!(
            avd_rc_apply_nonspatial_steps(5.0, decay.as_ptr(), inflow.as_ptr(), -1, 1),
            5.0
        );
        assert_eq!(
            avd_rc_apply_nonspatial_steps(5.0, decay.as_ptr(), inflow.as_ptr(), 1, -1),
            5.0
        );
    }
}
