use std::ffi::c_int;

// --- Base merit computation ---

/// Compute initial deme merit from config.
/// If method == BASE_MERIT_CONST (0), returns const_merit; otherwise returns 1.0.
#[no_mangle]
pub extern "C" fn avd_deme_base_merit(method: c_int, const_merit: f64) -> f64 {
    if method == 0 {
        // BASE_MERIT_CONST == 0
        const_merit
    } else {
        1.0
    }
}

// --- Germline join gate ---

/// Returns 1 if the first organism in deme should join germline before mutation stats.
/// Active when DEMES_ORGANISM_SELECTION is not 7 and not 8.
#[no_mangle]
pub extern "C" fn avd_deme_should_join_germline_first(selection_method: c_int) -> c_int {
    if selection_method != 7 && selection_method != 8 {
        1
    } else {
        0
    }
}

// --- Reaction weight computation ---

/// Compute per-reaction weight for workload calculation.
/// If slope > 0, weight = slope * index; otherwise weight = 1.0.
/// First task (index 0) is always excluded by caller, so this just computes weight.
#[no_mangle]
pub extern "C" fn avd_deme_reaction_weight(slope: f64, index: c_int) -> f64 {
    if slope > 0.0 {
        slope * f64::from(index)
    } else {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_merit_const_method() {
        // BASE_MERIT_CONST == 0
        assert_eq!(avd_deme_base_merit(0, 100.0), 100.0);
        assert_eq!(avd_deme_base_merit(0, 0.5), 0.5);
    }

    #[test]
    fn base_merit_other_methods() {
        assert_eq!(avd_deme_base_merit(1, 100.0), 1.0);
        assert_eq!(avd_deme_base_merit(2, 100.0), 1.0);
        assert_eq!(avd_deme_base_merit(4, 100.0), 1.0);
    }

    #[test]
    fn should_join_germline_first_policy() {
        assert_eq!(avd_deme_should_join_germline_first(0), 1);
        assert_eq!(avd_deme_should_join_germline_first(6), 1);
        assert_eq!(avd_deme_should_join_germline_first(7), 0);
        assert_eq!(avd_deme_should_join_germline_first(8), 0);
        assert_eq!(avd_deme_should_join_germline_first(9), 1);
    }

    #[test]
    fn reaction_weight_with_slope() {
        assert!((avd_deme_reaction_weight(2.0, 3) - 6.0).abs() < f64::EPSILON);
        assert!((avd_deme_reaction_weight(0.5, 4) - 2.0).abs() < f64::EPSILON);
        // index 0 gives weight 0 with slope (caller excludes index 0)
        assert!((avd_deme_reaction_weight(2.0, 0)).abs() < f64::EPSILON);
    }

    #[test]
    fn reaction_weight_without_slope() {
        assert_eq!(avd_deme_reaction_weight(0.0, 3), 1.0);
        assert_eq!(avd_deme_reaction_weight(-1.0, 3), 1.0);
    }
}
