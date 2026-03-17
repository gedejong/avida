use std::ffi::c_int;

// --- Search type normalization ---

/// Normalize a sensor search_type based on habitat and organism context.
///
/// - `habitat_used`: current habitat (-2=organisms, 0+=resource habitats)
/// - `search_type`: raw search type from register
/// - `pred_experiment`: 1 if PRED_PREY_SWITCH > -1
/// - `is_predator`: 1 if organism forage_target <= -2
///
/// Returns normalized search_type.
#[no_mangle]
pub extern "C" fn avd_sensor_normalize_search_type(
    habitat_used: c_int,
    search_type: c_int,
    pred_experiment: c_int,
    is_predator: c_int,
) -> c_int {
    if habitat_used != -2 {
        // Looking for env resources: clamp to [0,1], default 0
        if !(0..=1).contains(&search_type) {
            return 0;
        }
    } else if pred_experiment != 0 && is_predator == 0 {
        // Looking for orgs, pred experiment, organism is prey: clamp to [-2,2], default 1
        if !(-2..=2).contains(&search_type) {
            return 1;
        }
    } else if pred_experiment != 0 && is_predator != 0 {
        // Looking for orgs, pred experiment, organism is predator: clamp to [-2,2], default -1
        if !(-2..=2).contains(&search_type) {
            return -1;
        }
    } else {
        // Looking for orgs, non-predator experiment: clamp to [-2,0], default 0
        if !(-2..=0).contains(&search_type) {
            return 0;
        }
    }
    search_type
}

// --- Distance clamping ---

/// Clamp a sensor distance_sought to [1, max_dist].
/// If distance_sought < 0, returns 1.
/// If distance_sought > max_dist, returns max_dist.
/// Otherwise returns distance_sought unchanged.
#[no_mangle]
pub extern "C" fn avd_sensor_clamp_distance(distance_sought: c_int, max_dist: c_int) -> c_int {
    if distance_sought < 0 {
        1
    } else if distance_sought > max_dist {
        max_dist
    } else {
        distance_sought
    }
}

// --- Max distance computation ---

/// Compute max look distance from config and world dimensions.
/// If look_dist_config == -1, use long_axis; otherwise use look_dist_config.
/// Then cap at max(world_x, world_y).
#[no_mangle]
pub extern "C" fn avd_sensor_max_distance(
    look_dist_config: c_int,
    world_x: c_int,
    world_y: c_int,
) -> c_int {
    let long_axis = {
        let max_dim = if world_x > world_y { world_x } else { world_y };
        (max_dim as f64 * 0.5 + 0.5) as c_int
    };
    let max_dist = if look_dist_config != -1 {
        look_dist_config
    } else {
        long_axis
    };
    let world_max = if world_x > world_y { world_x } else { world_y };
    if max_dist > world_max {
        world_max
    } else {
        max_dist
    }
}

// --- ID sought clamp ---

/// Clamp id_sought: if < -1, return -1.
#[no_mangle]
pub extern "C" fn avd_sensor_clamp_id_sought(id_sought: c_int) -> c_int {
    if id_sought < -1 {
        -1
    } else {
        id_sought
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Search type normalization tests ---

    #[test]
    fn search_type_env_resource() {
        // habitat != -2, search_type out of [0,1] => default 0
        assert_eq!(avd_sensor_normalize_search_type(0, -1, 0, 0), 0);
        assert_eq!(avd_sensor_normalize_search_type(0, 2, 0, 0), 0);
        // valid search_type unchanged
        assert_eq!(avd_sensor_normalize_search_type(0, 0, 0, 0), 0);
        assert_eq!(avd_sensor_normalize_search_type(0, 1, 0, 0), 1);
    }

    #[test]
    fn search_type_prey_in_pred_experiment() {
        // pred_experiment=1, is_predator=0 (prey), out of [-2,2] => default 1
        assert_eq!(avd_sensor_normalize_search_type(-2, -3, 1, 0), 1);
        assert_eq!(avd_sensor_normalize_search_type(-2, 3, 1, 0), 1);
        // valid range unchanged
        assert_eq!(avd_sensor_normalize_search_type(-2, 2, 1, 0), 2);
        assert_eq!(avd_sensor_normalize_search_type(-2, -2, 1, 0), -2);
    }

    #[test]
    fn search_type_predator_in_pred_experiment() {
        // pred_experiment=1, is_predator=1, out of [-2,2] => default -1
        assert_eq!(avd_sensor_normalize_search_type(-2, -3, 1, 1), -1);
        assert_eq!(avd_sensor_normalize_search_type(-2, 3, 1, 1), -1);
        // valid range unchanged
        assert_eq!(avd_sensor_normalize_search_type(-2, 0, 1, 1), 0);
    }

    #[test]
    fn search_type_non_pred_experiment() {
        // pred_experiment=0, habitat=-2, out of [-2,0] => default 0
        assert_eq!(avd_sensor_normalize_search_type(-2, 1, 0, 0), 0);
        assert_eq!(avd_sensor_normalize_search_type(-2, -3, 0, 0), 0);
        // valid range unchanged
        assert_eq!(avd_sensor_normalize_search_type(-2, -1, 0, 0), -1);
        assert_eq!(avd_sensor_normalize_search_type(-2, 0, 0, 0), 0);
    }

    // --- Distance clamping tests ---

    #[test]
    fn clamp_distance_policy() {
        assert_eq!(avd_sensor_clamp_distance(-1, 10), 1);
        assert_eq!(avd_sensor_clamp_distance(15, 10), 10);
        assert_eq!(avd_sensor_clamp_distance(5, 10), 5);
        assert_eq!(avd_sensor_clamp_distance(0, 10), 0);
    }

    // --- Max distance tests ---

    #[test]
    fn max_distance_policy() {
        // config -1 => use long_axis = (max(60,40)*0.5+0.5) = 30
        assert_eq!(avd_sensor_max_distance(-1, 60, 40), 30);
        // config != -1, within world_max
        assert_eq!(avd_sensor_max_distance(20, 60, 40), 20);
        // config exceeds world_max => cap
        assert_eq!(avd_sensor_max_distance(100, 60, 40), 60);
    }

    // --- ID sought clamp tests ---

    #[test]
    fn clamp_id_sought_policy() {
        assert_eq!(avd_sensor_clamp_id_sought(-2), -1);
        assert_eq!(avd_sensor_clamp_id_sought(-100), -1);
        assert_eq!(avd_sensor_clamp_id_sought(-1), -1);
        assert_eq!(avd_sensor_clamp_id_sought(0), 0);
        assert_eq!(avd_sensor_clamp_id_sought(5), 5);
    }
}
