use crate::common::{set_out, with_slice};
use std::ffi::c_int;

fn clamp_to_bound(value: c_int, bound: c_int) -> c_int {
    if value < 0 {
        0
    } else if value > bound {
        bound
    } else {
        value
    }
}

fn normalize_span_internal(start: c_int, end: c_int, bound: c_int) -> (c_int, c_int) {
    let normalized_start = clamp_to_bound(start, bound);
    let mut normalized_end = clamp_to_bound(end, bound);
    if normalized_end < normalized_start {
        normalized_end = normalized_end.saturating_add(bound);
    }
    (normalized_start, normalized_end)
}

fn source_per_cell_internal(amount: f64, x1: c_int, x2: c_int, y1: c_int, y2: c_int) -> f64 {
    let width = f64::from(x2 - x1 + 1);
    let height = f64::from(y2 - y1 + 1);
    amount / (width * height)
}

fn sink_delta_internal(current_amount: f64, decay: f64) -> f64 {
    (current_amount * (1.0 - decay)).max(0.0)
}

fn cell_outflow_delta_internal(current_amount: f64, outflow: f64) -> f64 {
    (current_amount * outflow).max(0.0)
}

fn wrap_coord(coord: c_int, bound: c_int) -> Option<c_int> {
    if bound <= 0 {
        return None;
    }
    let rem = coord % bound;
    Some(if rem < 0 { rem + bound } else { rem })
}

fn wrapped_elem_index_internal(x: c_int, y: c_int, world_x: c_int, world_y: c_int) -> c_int {
    let Some(mx) = wrap_coord(x, world_x) else {
        return -1;
    };
    let Some(my) = wrap_coord(y, world_y) else {
        return -1;
    };
    let index = i64::from(my) * i64::from(world_x) + i64::from(mx);
    if index < 0 || index > i64::from(c_int::MAX) {
        return -1;
    }
    index as c_int
}

const GEOMETRY_GRID: c_int = 1;
const RESOURCE_NONE: c_int = -99;

fn slot_deltas(slot: c_int) -> Option<(c_int, c_int, f64)> {
    let sqrt2 = 2.0_f64.sqrt();
    match slot {
        0 => Some((-1, -1, sqrt2)),
        1 => Some((0, -1, 1.0)),
        2 => Some((1, -1, sqrt2)),
        3 => Some((1, 0, 1.0)),
        4 => Some((1, 1, sqrt2)),
        5 => Some((0, 1, 1.0)),
        6 => Some((-1, 1, sqrt2)),
        7 => Some((-1, 0, 1.0)),
        _ => None,
    }
}

fn is_grid_masked_neighbor(cell_id: c_int, world_x: c_int, world_y: c_int, slot: c_int) -> bool {
    if world_x <= 0 || world_y <= 0 {
        return true;
    }
    let num_cells = world_x.saturating_mul(world_y);
    if num_cells <= 0 {
        return true;
    }
    if cell_id < 0 || cell_id >= num_cells {
        return true;
    }

    let row = cell_id / world_x;
    let col = cell_id % world_x;
    let on_top = row == 0;
    let on_bottom = row == world_y - 1;
    let on_left = col == 0;
    let on_right = col == world_x - 1;

    (on_top && (slot == 0 || slot == 1 || slot == 2))
        || (on_bottom && (slot == 4 || slot == 5 || slot == 6))
        || (on_left && (slot == 0 || slot == 7 || slot == 6))
        || (on_right && (slot == 2 || slot == 3 || slot == 4))
}

fn setpointer_entry_internal(
    cell_id: c_int,
    world_x: c_int,
    world_y: c_int,
    geometry: c_int,
    slot: c_int,
) -> Option<(c_int, c_int, c_int, f64)> {
    if world_x <= 0 || world_y <= 0 || cell_id < 0 || cell_id >= world_x.saturating_mul(world_y) {
        return None;
    }
    let (xdist, ydist, dist) = slot_deltas(slot)?;

    if geometry == GEOMETRY_GRID && is_grid_masked_neighbor(cell_id, world_x, world_y, slot) {
        return Some((
            RESOURCE_NONE,
            RESOURCE_NONE,
            RESOURCE_NONE,
            f64::from(RESOURCE_NONE),
        ));
    }

    let x = cell_id % world_x;
    let y = cell_id / world_x;
    let nx = wrap_coord(x + xdist, world_x)?;
    let ny = wrap_coord(y + ydist, world_y)?;
    Some((ny * world_x + nx, xdist, ydist, dist))
}

fn cell_id_in_bounds_strict_internal(cell_id: c_int, grid_size: c_int) -> c_int {
    if cell_id >= 0 && cell_id < grid_size {
        1
    } else {
        0
    }
}

fn cell_id_in_bounds_legacy_setcell_internal(cell_id: c_int, grid_size: c_int) -> c_int {
    if cell_id >= 0 && cell_id <= grid_size {
        1
    } else {
        0
    }
}

fn state_fold_internal(amount: f64, delta: f64) -> (f64, f64) {
    (amount + delta, 0.0)
}

fn rate_next_delta_internal(current_delta: f64, rate_in: f64) -> f64 {
    current_delta + rate_in
}

fn reset_amount_internal(res_initial: f64, cell_initial: f64) -> f64 {
    res_initial + cell_initial
}

fn setcell_apply_initial_internal(amount: f64, delta: f64, cell_initial: f64) -> (f64, f64) {
    (amount + delta + cell_initial, 0.0)
}

#[allow(clippy::too_many_arguments)]
fn compute_flow_scalar_internal(
    elem1_amount: f64,
    elem2_amount: f64,
    inxdiffuse: f64,
    inydiffuse: f64,
    inxgravity: f64,
    inygravity: f64,
    xdist: c_int,
    ydist: c_int,
    dist: f64,
) -> f64 {
    // Preserve existing legacy guard semantics exactly.
    if elem1_amount == 0.0 && elem2_amount == 0.0 && dist < 0.0 {
        return 0.0;
    }

    let diff = elem1_amount - elem2_amount;
    let (xdiffuse, xgravity) = if xdist != 0 {
        let xgravity = if (xdist > 0 && inxgravity > 0.0) || (xdist < 0 && inxgravity < 0.0) {
            elem1_amount * inxgravity.abs() / 3.0
        } else {
            -elem2_amount * inxgravity.abs() / 3.0
        };
        let xdiffuse = inxdiffuse * diff / 16.0;
        (xdiffuse, xgravity)
    } else {
        (0.0, 0.0)
    };

    let (ydiffuse, ygravity) = if ydist != 0 {
        let ygravity = if (ydist > 0 && inygravity > 0.0) || (ydist < 0 && inygravity < 0.0) {
            elem1_amount * inygravity.abs() / 3.0
        } else {
            -elem2_amount * inygravity.abs() / 3.0
        };
        let ydiffuse = inydiffuse * diff / 16.0;
        (ydiffuse, ygravity)
    } else {
        (0.0, 0.0)
    };

    ((xdiffuse + ydiffuse + xgravity + ygravity) / (f64::from(xdist.abs() + ydist.abs()))) / dist
}

#[no_mangle]
pub extern "C" fn avd_src_normalize_span(
    start: c_int,
    end: c_int,
    bound: c_int,
    out_start: *mut c_int,
    out_end: *mut c_int,
) -> c_int {
    if out_start.is_null() || out_end.is_null() {
        return 0;
    }
    let (normalized_start, normalized_end) = normalize_span_internal(start, end, bound);
    if !set_out(out_start, normalized_start) {
        return 0;
    }
    if !set_out(out_end, normalized_end) {
        return 0;
    }
    1
}

#[no_mangle]
pub extern "C" fn avd_src_compute_flow_scalar(
    elem1_amount: f64,
    elem2_amount: f64,
    inxdiffuse: f64,
    inydiffuse: f64,
    inxgravity: f64,
    inygravity: f64,
    xdist: c_int,
    ydist: c_int,
    dist: f64,
) -> f64 {
    compute_flow_scalar_internal(
        elem1_amount,
        elem2_amount,
        inxdiffuse,
        inydiffuse,
        inxgravity,
        inygravity,
        xdist,
        ydist,
        dist,
    )
}

#[allow(clippy::too_many_arguments)]
#[no_mangle]
pub extern "C" fn avd_src_compute_flow_pair_deltas(
    elem1_amount: f64,
    elem2_amount: f64,
    inxdiffuse: f64,
    inydiffuse: f64,
    inxgravity: f64,
    inygravity: f64,
    xdist: c_int,
    ydist: c_int,
    dist: f64,
    out_elem1_delta: *mut f64,
    out_elem2_delta: *mut f64,
) -> c_int {
    if out_elem1_delta.is_null() || out_elem2_delta.is_null() {
        return 0;
    }
    let flowamt = compute_flow_scalar_internal(
        elem1_amount,
        elem2_amount,
        inxdiffuse,
        inydiffuse,
        inxgravity,
        inygravity,
        xdist,
        ydist,
        dist,
    );
    if !set_out(out_elem1_delta, -flowamt) {
        return 0;
    }
    if !set_out(out_elem2_delta, flowamt) {
        return 0;
    }
    1
}

#[no_mangle]
pub extern "C" fn avd_src_source_per_cell(
    amount: f64,
    x1: c_int,
    x2: c_int,
    y1: c_int,
    y2: c_int,
) -> f64 {
    source_per_cell_internal(amount, x1, x2, y1, y2)
}

#[no_mangle]
pub extern "C" fn avd_src_sink_delta(current_amount: f64, decay: f64) -> f64 {
    sink_delta_internal(current_amount, decay)
}

#[no_mangle]
pub extern "C" fn avd_src_cell_outflow_delta(current_amount: f64, outflow: f64) -> f64 {
    cell_outflow_delta_internal(current_amount, outflow)
}

#[no_mangle]
pub extern "C" fn avd_src_wrapped_elem_index(
    x: c_int,
    y: c_int,
    world_x: c_int,
    world_y: c_int,
) -> c_int {
    wrapped_elem_index_internal(x, y, world_x, world_y)
}

#[no_mangle]
pub extern "C" fn avd_src_cell_id_in_bounds_strict(cell_id: c_int, grid_size: c_int) -> c_int {
    cell_id_in_bounds_strict_internal(cell_id, grid_size)
}

#[no_mangle]
pub extern "C" fn avd_src_cell_id_in_bounds_legacy_setcell(
    cell_id: c_int,
    grid_size: c_int,
) -> c_int {
    cell_id_in_bounds_legacy_setcell_internal(cell_id, grid_size)
}

#[no_mangle]
pub extern "C" fn avd_src_setpointer_entry(
    cell_id: c_int,
    world_x: c_int,
    world_y: c_int,
    geometry: c_int,
    slot: c_int,
    out_elempt: *mut c_int,
    out_xdist: *mut c_int,
    out_ydist: *mut c_int,
    out_dist: *mut f64,
) -> c_int {
    if out_elempt.is_null() || out_xdist.is_null() || out_ydist.is_null() || out_dist.is_null() {
        return 0;
    }
    let Some((elempt, xdist, ydist, dist)) =
        setpointer_entry_internal(cell_id, world_x, world_y, geometry, slot)
    else {
        return 0;
    };

    if !set_out(out_elempt, elempt) {
        return 0;
    }
    if !set_out(out_xdist, xdist) {
        return 0;
    }
    if !set_out(out_ydist, ydist) {
        return 0;
    }
    if !set_out(out_dist, dist) {
        return 0;
    }
    1
}

#[no_mangle]
pub extern "C" fn avd_src_state_fold(
    amount: f64,
    delta: f64,
    out_amount: *mut f64,
    out_delta: *mut f64,
) -> c_int {
    if out_amount.is_null() || out_delta.is_null() {
        return 0;
    }
    let (next_amount, next_delta) = state_fold_internal(amount, delta);
    if !set_out(out_amount, next_amount) {
        return 0;
    }
    if !set_out(out_delta, next_delta) {
        return 0;
    }
    1
}

#[no_mangle]
pub extern "C" fn avd_src_sum_amounts(values: *const f64, count: c_int) -> f64 {
    with_slice(values, count, 0.0, |slice| slice.iter().sum())
}

#[no_mangle]
pub extern "C" fn avd_src_rate_next_delta(
    current_delta: f64,
    rate_in: f64,
    out_delta: *mut f64,
) -> c_int {
    if out_delta.is_null() {
        return 0;
    }
    if !set_out(out_delta, rate_next_delta_internal(current_delta, rate_in)) {
        return 0;
    }
    1
}

#[no_mangle]
pub extern "C" fn avd_src_reset_amount(
    res_initial: f64,
    cell_initial: f64,
    out_amount: *mut f64,
) -> c_int {
    if out_amount.is_null() {
        return 0;
    }
    if !set_out(out_amount, reset_amount_internal(res_initial, cell_initial)) {
        return 0;
    }
    1
}

#[no_mangle]
pub extern "C" fn avd_src_setcell_apply_initial(
    amount: f64,
    delta: f64,
    cell_initial: f64,
    out_amount: *mut f64,
    out_delta: *mut f64,
) -> c_int {
    if out_amount.is_null() || out_delta.is_null() {
        return 0;
    }
    let (next_amount, next_delta) = setcell_apply_initial_internal(amount, delta, cell_initial);
    if !set_out(out_amount, next_amount) {
        return 0;
    }
    if !set_out(out_delta, next_delta) {
        return 0;
    }
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_span_clamps_and_wraps() {
        let mut start = -1;
        let mut end = -1;
        assert_eq!(avd_src_normalize_span(-5, 12, 10, &mut start, &mut end), 1);
        assert_eq!(start, 0);
        assert_eq!(end, 10);

        assert_eq!(avd_src_normalize_span(8, 3, 10, &mut start, &mut end), 1);
        assert_eq!(start, 8);
        assert_eq!(end, 13);
    }

    #[test]
    fn normalize_span_guards_null_outputs() {
        let mut out_end = 0;
        assert_eq!(
            avd_src_normalize_span(1, 2, 10, std::ptr::null_mut(), &mut out_end),
            0
        );
    }

    #[test]
    fn flow_scalar_matches_legacy_reference_cases() {
        let scalar =
            avd_src_compute_flow_scalar(10.0, 4.0, 1.0, 1.0, 0.5, -0.25, 1, -1, 2.0_f64.sqrt());
        let diff = 6.0;
        let xgravity = 10.0 * 0.5 / 3.0;
        let ygravity = 10.0 * 0.25 / 3.0;
        let expected = ((diff / 16.0 + diff / 16.0 + xgravity + ygravity) / 2.0) / 2.0_f64.sqrt();
        assert!((scalar - expected).abs() < 1e-12);

        let guarded = avd_src_compute_flow_scalar(0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1, 0, -1.0);
        assert_eq!(guarded, 0.0);
    }

    #[test]
    fn flow_pair_deltas_match_scalar_and_conserve_mass() {
        let mut d1 = 0.0;
        let mut d2 = 0.0;
        assert_eq!(
            avd_src_compute_flow_pair_deltas(
                10.0,
                4.0,
                1.0,
                1.0,
                0.5,
                -0.25,
                1,
                -1,
                2.0_f64.sqrt(),
                &mut d1,
                &mut d2
            ),
            1
        );
        let flow =
            avd_src_compute_flow_scalar(10.0, 4.0, 1.0, 1.0, 0.5, -0.25, 1, -1, 2.0_f64.sqrt());
        assert!((d1 + flow).abs() < 1e-12);
        assert!((d2 - flow).abs() < 1e-12);
        assert!((d1 + d2).abs() < 1e-12);

        assert_eq!(
            avd_src_compute_flow_pair_deltas(
                0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1, 0, -1.0, &mut d1, &mut d2
            ),
            1
        );
        assert_eq!(d1, 0.0);
        assert_eq!(d2, 0.0);
    }

    #[test]
    fn flow_pair_deltas_guard_null_outputs() {
        let mut d2 = 0.0;
        assert_eq!(
            avd_src_compute_flow_pair_deltas(
                1.0,
                2.0,
                1.0,
                1.0,
                0.0,
                0.0,
                1,
                0,
                1.0,
                std::ptr::null_mut(),
                &mut d2
            ),
            0
        );
    }

    #[test]
    fn source_per_cell_matches_reference_formula() {
        assert!((avd_src_source_per_cell(12.0, 0, 1, 0, 2) - 2.0).abs() < 1e-15);
        assert!((avd_src_source_per_cell(5.0, 2, 2, 3, 3) - 5.0).abs() < 1e-15);
    }

    #[test]
    fn sink_and_outflow_delta_match_legacy_clamp() {
        assert!((avd_src_sink_delta(10.0, 0.2) - 8.0).abs() < 1e-15);
        assert_eq!(avd_src_sink_delta(10.0, 1.5), 0.0);
        assert!((avd_src_cell_outflow_delta(10.0, 0.2) - 2.0).abs() < 1e-15);
        assert_eq!(avd_src_cell_outflow_delta(10.0, -0.2), 0.0);
    }

    #[test]
    fn wrapped_elem_index_wraps_and_matches_reference() {
        let world_x = 5;
        let world_y = 4;
        assert_eq!(avd_src_wrapped_elem_index(2, 1, world_x, world_y), 7);
        assert_eq!(avd_src_wrapped_elem_index(-1, 0, world_x, world_y), 4);
        assert_eq!(avd_src_wrapped_elem_index(6, -1, world_x, world_y), 16);
        assert_eq!(avd_src_wrapped_elem_index(-13, 9, world_x, world_y), 7);
    }

    #[test]
    fn wrapped_elem_index_rejects_invalid_dimensions() {
        assert_eq!(avd_src_wrapped_elem_index(1, 2, 0, 4), -1);
        assert_eq!(avd_src_wrapped_elem_index(1, 2, 4, 0), -1);
        assert_eq!(avd_src_wrapped_elem_index(1, 2, -4, 4), -1);
        assert_eq!(avd_src_wrapped_elem_index(1, 2, 4, -4), -1);
    }

    #[test]
    fn cell_id_bounds_policies_match_legacy_callsite_rules() {
        assert_eq!(avd_src_cell_id_in_bounds_strict(-1, 5), 0);
        assert_eq!(avd_src_cell_id_in_bounds_strict(0, 5), 1);
        assert_eq!(avd_src_cell_id_in_bounds_strict(4, 5), 1);
        assert_eq!(avd_src_cell_id_in_bounds_strict(5, 5), 0);
        assert_eq!(avd_src_cell_id_in_bounds_strict(0, 0), 0);

        assert_eq!(avd_src_cell_id_in_bounds_legacy_setcell(-1, 5), 0);
        assert_eq!(avd_src_cell_id_in_bounds_legacy_setcell(0, 5), 1);
        assert_eq!(avd_src_cell_id_in_bounds_legacy_setcell(5, 5), 1);
        assert_eq!(avd_src_cell_id_in_bounds_legacy_setcell(6, 5), 0);
        assert_eq!(avd_src_cell_id_in_bounds_legacy_setcell(0, 0), 1);
        assert_eq!(avd_src_cell_id_in_bounds_legacy_setcell(1, 0), 0);
    }

    #[test]
    fn setpointer_entry_matches_torus_and_grid_masks() {
        let mut elempt = 0;
        let mut xdist = 0;
        let mut ydist = 0;
        let mut dist = 0.0;

        assert_eq!(
            avd_src_setpointer_entry(
                4,
                3,
                3,
                2,
                0,
                &mut elempt,
                &mut xdist,
                &mut ydist,
                &mut dist
            ),
            1
        );
        assert_eq!(elempt, 0);
        assert_eq!(xdist, -1);
        assert_eq!(ydist, -1);
        assert!((dist - 2.0_f64.sqrt()).abs() < 1e-12);

        assert_eq!(
            avd_src_setpointer_entry(
                0,
                3,
                3,
                GEOMETRY_GRID,
                0,
                &mut elempt,
                &mut xdist,
                &mut ydist,
                &mut dist
            ),
            1
        );
        assert_eq!(elempt, RESOURCE_NONE);
        assert_eq!(xdist, RESOURCE_NONE);
        assert_eq!(ydist, RESOURCE_NONE);
        assert_eq!(dist, f64::from(RESOURCE_NONE));
    }

    #[test]
    fn setpointer_entry_guard_matrix() {
        let mut elempt = 0;
        let mut xdist = 0;
        let mut ydist = 0;
        let mut dist = 0.0;
        assert_eq!(
            avd_src_setpointer_entry(
                -1,
                3,
                3,
                GEOMETRY_GRID,
                0,
                &mut elempt,
                &mut xdist,
                &mut ydist,
                &mut dist
            ),
            0
        );
        assert_eq!(
            avd_src_setpointer_entry(
                0,
                0,
                3,
                GEOMETRY_GRID,
                0,
                &mut elempt,
                &mut xdist,
                &mut ydist,
                &mut dist
            ),
            0
        );
        assert_eq!(
            avd_src_setpointer_entry(
                0,
                3,
                3,
                GEOMETRY_GRID,
                8,
                &mut elempt,
                &mut xdist,
                &mut ydist,
                &mut dist
            ),
            0
        );
        assert_eq!(
            avd_src_setpointer_entry(
                0,
                3,
                3,
                GEOMETRY_GRID,
                0,
                std::ptr::null_mut(),
                &mut xdist,
                &mut ydist,
                &mut dist
            ),
            0
        );
    }

    #[test]
    fn setpointer_entry_degenerate_grid_dimensions() {
        let mut elempt = 0;
        let mut xdist = 0;
        let mut ydist = 0;
        let mut dist = 0.0;

        assert_eq!(
            avd_src_setpointer_entry(
                0,
                1,
                1,
                GEOMETRY_GRID,
                3,
                &mut elempt,
                &mut xdist,
                &mut ydist,
                &mut dist
            ),
            1
        );
        assert_eq!(elempt, RESOURCE_NONE);
        assert_eq!(xdist, RESOURCE_NONE);
        assert_eq!(ydist, RESOURCE_NONE);
        assert_eq!(dist, f64::from(RESOURCE_NONE));

        assert_eq!(
            avd_src_setpointer_entry(
                1,
                1,
                3,
                GEOMETRY_GRID,
                1,
                &mut elempt,
                &mut xdist,
                &mut ydist,
                &mut dist
            ),
            1
        );
        assert_eq!(elempt, 0);
        assert_eq!(xdist, 0);
        assert_eq!(ydist, -1);
        assert!((dist - 1.0).abs() < 1e-12);

        assert_eq!(
            avd_src_setpointer_entry(
                1,
                3,
                1,
                GEOMETRY_GRID,
                3,
                &mut elempt,
                &mut xdist,
                &mut ydist,
                &mut dist
            ),
            1
        );
        assert_eq!(elempt, 2);
        assert_eq!(xdist, 1);
        assert_eq!(ydist, 0);
        assert!((dist - 1.0).abs() < 1e-12);
    }

    #[test]
    fn state_fold_matches_legacy_semantics_and_guards() {
        let mut out_amount = -1.0;
        let mut out_delta = -1.0;
        assert_eq!(
            avd_src_state_fold(7.25, -2.0, &mut out_amount, &mut out_delta),
            1
        );
        assert!((out_amount - 5.25).abs() < 1e-15);
        assert_eq!(out_delta, 0.0);

        assert_eq!(
            avd_src_state_fold(1.0, 2.0, std::ptr::null_mut(), &mut out_delta),
            0
        );
    }

    #[test]
    fn sum_amounts_matches_reference_and_defaults_zero() {
        let values = [1.5, -2.0, 4.25];
        assert!((avd_src_sum_amounts(values.as_ptr(), values.len() as c_int) - 3.75).abs() < 1e-15);
        assert_eq!(avd_src_sum_amounts(std::ptr::null(), 3), 0.0);
        assert_eq!(avd_src_sum_amounts(values.as_ptr(), 0), 0.0);
    }

    #[test]
    fn rate_next_delta_matches_reference_and_guards() {
        let mut out = -1.0;
        assert_eq!(avd_src_rate_next_delta(1.25, -0.5, &mut out), 1);
        assert!((out - 0.75).abs() < 1e-15);
        assert_eq!(avd_src_rate_next_delta(1.0, 2.0, std::ptr::null_mut()), 0);
    }

    #[test]
    fn reset_amount_matches_reference_and_guards() {
        let mut out = -1.0;
        assert_eq!(avd_src_reset_amount(2.5, 1.25, &mut out), 1);
        assert!((out - 3.75).abs() < 1e-15);
        assert_eq!(avd_src_reset_amount(1.0, 2.0, std::ptr::null_mut()), 0);
    }

    #[test]
    fn setcell_apply_initial_matches_reference_and_guards() {
        let mut out_amount = -1.0;
        let mut out_delta = -1.0;
        assert_eq!(
            avd_src_setcell_apply_initial(3.0, -0.25, 1.5, &mut out_amount, &mut out_delta),
            1
        );
        assert!((out_amount - 4.25).abs() < 1e-15);
        assert_eq!(out_delta, 0.0);
        assert_eq!(
            avd_src_setcell_apply_initial(1.0, 2.0, 3.0, std::ptr::null_mut(), &mut out_delta),
            0
        );
    }
}
