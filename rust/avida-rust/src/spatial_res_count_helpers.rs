use crate::common::set_out;
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
}
