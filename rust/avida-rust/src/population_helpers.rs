use std::ffi::c_int;

const CPOP_DEME_BLOCK_SKIP: c_int = 0;
const CPOP_DEME_BLOCK_RUN: c_int = 1;
const CPOP_ROUTING_MODE_PROCESS_STEP: c_int = 0;
const CPOP_ROUTING_MODE_SPECULATIVE_STEP: c_int = 1;

fn cpop_should_check_implicit_deme_repro(num_demes: c_int) -> c_int {
    if num_demes >= 1 {
        CPOP_DEME_BLOCK_RUN
    } else {
        CPOP_DEME_BLOCK_SKIP
    }
}

fn cpop_should_run_speculative_deme_block(num_demes: c_int) -> c_int {
    cpop_should_run_multi_deme_block(num_demes)
}

fn cpop_should_run_multi_deme_block(num_demes: c_int) -> c_int {
    if num_demes > 1 {
        CPOP_DEME_BLOCK_RUN
    } else {
        CPOP_DEME_BLOCK_SKIP
    }
}

fn cpop_should_update_deme_counters(num_demes: c_int) -> c_int {
    if num_demes > 0 {
        CPOP_DEME_BLOCK_RUN
    } else {
        CPOP_DEME_BLOCK_SKIP
    }
}

fn cpop_deme_routing_short_circuit_kind(routing_mode: c_int, num_demes: c_int) -> c_int {
    match routing_mode {
        CPOP_ROUTING_MODE_PROCESS_STEP => cpop_should_check_implicit_deme_repro(num_demes),
        CPOP_ROUTING_MODE_SPECULATIVE_STEP => cpop_should_run_multi_deme_block(num_demes),
        _ => CPOP_DEME_BLOCK_SKIP,
    }
}

#[no_mangle]
pub extern "C" fn avd_cpop_should_check_implicit_deme_repro(num_demes: c_int) -> c_int {
    cpop_should_check_implicit_deme_repro(num_demes)
}

#[no_mangle]
pub extern "C" fn avd_cpop_should_run_speculative_deme_block(num_demes: c_int) -> c_int {
    cpop_should_run_speculative_deme_block(num_demes)
}

#[no_mangle]
pub extern "C" fn avd_cpop_should_update_deme_counters(num_demes: c_int) -> c_int {
    cpop_should_update_deme_counters(num_demes)
}

#[no_mangle]
pub extern "C" fn avd_cpop_should_run_multi_deme_block(num_demes: c_int) -> c_int {
    cpop_should_run_multi_deme_block(num_demes)
}

#[no_mangle]
pub extern "C" fn avd_cpop_deme_routing_short_circuit_kind(
    routing_mode: c_int,
    num_demes: c_int,
) -> c_int {
    cpop_deme_routing_short_circuit_kind(routing_mode, num_demes)
}

// --- Pred/prey tracking gate ---

/// Returns 1 if pred/prey forager tracking is active.
/// Active when `pred_prey_switch == -2` or `pred_prey_switch > -1`.
#[no_mangle]
pub extern "C" fn avd_cpop_is_pred_prey_tracking_active(pred_prey_switch: c_int) -> c_int {
    if pred_prey_switch == -2 || pred_prey_switch > -1 {
        1
    } else {
        0
    }
}

// --- Forager type classification ---
const CPOP_FORAGER_TYPE_PREY: c_int = 0;
const CPOP_FORAGER_TYPE_TOP_PRED: c_int = 1;
const CPOP_FORAGER_TYPE_PRED: c_int = 2;

/// Classifies an organism's forager type for counter tracking.
///
/// - `is_prey_ft`: 1 if organism IsPreyFT()
/// - `is_top_pred_ft`: 1 if organism IsTopPredFT()
///
/// Returns: 0=prey, 1=top_pred, 2=pred
#[no_mangle]
pub extern "C" fn avd_cpop_forager_type_kind(is_prey_ft: c_int, is_top_pred_ft: c_int) -> c_int {
    if is_prey_ft != 0 {
        CPOP_FORAGER_TYPE_PREY
    } else if is_top_pred_ft != 0 {
        CPOP_FORAGER_TYPE_TOP_PRED
    } else {
        CPOP_FORAGER_TYPE_PRED
    }
}

// --- Deadly boundary detection ---

/// Returns 1 if the cell at (dest_x, dest_y) is on a deadly world boundary.
/// Only applies when deadly_boundaries is enabled (1) and geometry is GRID (1).
#[no_mangle]
pub extern "C" fn avd_cpop_is_deadly_boundary(
    deadly_boundaries: c_int,
    geometry: c_int,
    dest_x: c_int,
    dest_y: c_int,
    world_x: c_int,
    world_y: c_int,
) -> c_int {
    if deadly_boundaries == 1
        && geometry == 1
        && (dest_x == 0 || dest_y == 0 || dest_x == world_x - 1 || dest_y == world_y - 1)
    {
        1
    } else {
        0
    }
}

// --- Prey target exclusion filter ---

/// Returns 1 if the organism is a valid prey target (not excluded).
/// Excludes adult predators and juveniles with predatory parents.
/// Includes organisms with forage_target > -1, or forage_target == -1 with parent_ft > -2.
#[no_mangle]
pub extern "C" fn avd_cpop_is_valid_prey_target(forage_target: c_int, parent_ft: c_int) -> c_int {
    if forage_target > -1 || (forage_target == -1 && parent_ft > -2) {
        1
    } else {
        0
    }
}

// --- Merit bonus instruction gate ---

/// Returns 1 if merit bonus instruction counting is enabled (rewarded_instruction != -1).
#[no_mangle]
pub extern "C" fn avd_cpop_is_merit_bonus_enabled(rewarded_instruction: c_int) -> c_int {
    if rewarded_instruction != -1 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpop_implicit_repro_policy() {
        assert_eq!(
            avd_cpop_should_check_implicit_deme_repro(-1),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(
            avd_cpop_should_check_implicit_deme_repro(0),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(
            avd_cpop_should_check_implicit_deme_repro(1),
            CPOP_DEME_BLOCK_RUN
        );
        assert_eq!(
            avd_cpop_should_check_implicit_deme_repro(2),
            CPOP_DEME_BLOCK_RUN
        );
    }

    #[test]
    fn cpop_speculative_deme_block_policy() {
        assert_eq!(
            avd_cpop_should_run_speculative_deme_block(-1),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(
            avd_cpop_should_run_speculative_deme_block(0),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(
            avd_cpop_should_run_speculative_deme_block(1),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(
            avd_cpop_should_run_speculative_deme_block(2),
            CPOP_DEME_BLOCK_RUN
        );
    }

    #[test]
    fn cpop_deme_counter_update_policy() {
        assert_eq!(
            avd_cpop_should_update_deme_counters(-1),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(
            avd_cpop_should_update_deme_counters(0),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(avd_cpop_should_update_deme_counters(1), CPOP_DEME_BLOCK_RUN);
        assert_eq!(avd_cpop_should_update_deme_counters(2), CPOP_DEME_BLOCK_RUN);
    }

    #[test]
    fn cpop_multi_deme_block_policy() {
        assert_eq!(
            avd_cpop_should_run_multi_deme_block(-1),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(
            avd_cpop_should_run_multi_deme_block(0),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(
            avd_cpop_should_run_multi_deme_block(1),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(avd_cpop_should_run_multi_deme_block(2), CPOP_DEME_BLOCK_RUN);
    }

    #[test]
    fn cpop_speculative_and_multi_deme_alignment_policy() {
        assert_eq!(
            avd_cpop_should_run_speculative_deme_block(-1),
            avd_cpop_should_run_multi_deme_block(-1)
        );
        assert_eq!(
            avd_cpop_should_run_speculative_deme_block(0),
            avd_cpop_should_run_multi_deme_block(0)
        );
        assert_eq!(
            avd_cpop_should_run_speculative_deme_block(1),
            avd_cpop_should_run_multi_deme_block(1)
        );
        assert_eq!(
            avd_cpop_should_run_speculative_deme_block(2),
            avd_cpop_should_run_multi_deme_block(2)
        );
    }

    // --- Pred/prey tracking gate tests ---

    #[test]
    fn cpop_pred_prey_tracking_active_policy() {
        assert_eq!(avd_cpop_is_pred_prey_tracking_active(-2), 1);
        assert_eq!(avd_cpop_is_pred_prey_tracking_active(0), 1);
        assert_eq!(avd_cpop_is_pred_prey_tracking_active(5), 1);
        assert_eq!(avd_cpop_is_pred_prey_tracking_active(-1), 0);
        assert_eq!(avd_cpop_is_pred_prey_tracking_active(-3), 0);
    }

    // --- Forager type classification tests ---

    #[test]
    fn cpop_forager_type_kind_policy() {
        assert_eq!(avd_cpop_forager_type_kind(1, 0), CPOP_FORAGER_TYPE_PREY);
        assert_eq!(avd_cpop_forager_type_kind(1, 1), CPOP_FORAGER_TYPE_PREY); // prey takes precedence
        assert_eq!(avd_cpop_forager_type_kind(0, 1), CPOP_FORAGER_TYPE_TOP_PRED);
        assert_eq!(avd_cpop_forager_type_kind(0, 0), CPOP_FORAGER_TYPE_PRED);
    }

    // --- Deadly boundary tests ---

    #[test]
    fn cpop_deadly_boundary_edges() {
        // On edges of a 10x10 world (GRID geometry=1)
        assert_eq!(avd_cpop_is_deadly_boundary(1, 1, 0, 5, 10, 10), 1);
        assert_eq!(avd_cpop_is_deadly_boundary(1, 1, 5, 0, 10, 10), 1);
        assert_eq!(avd_cpop_is_deadly_boundary(1, 1, 9, 5, 10, 10), 1);
        assert_eq!(avd_cpop_is_deadly_boundary(1, 1, 5, 9, 10, 10), 1);
        // Interior cell
        assert_eq!(avd_cpop_is_deadly_boundary(1, 1, 5, 5, 10, 10), 0);
    }

    #[test]
    fn cpop_deadly_boundary_disabled() {
        // Not enabled
        assert_eq!(avd_cpop_is_deadly_boundary(0, 1, 0, 0, 10, 10), 0);
        // Wrong geometry
        assert_eq!(avd_cpop_is_deadly_boundary(1, 2, 0, 0, 10, 10), 0);
    }

    // --- Prey target exclusion tests ---

    #[test]
    fn cpop_valid_prey_target_policy() {
        // forage_target > -1 => valid (prey organisms)
        assert_eq!(avd_cpop_is_valid_prey_target(0, -5), 1);
        assert_eq!(avd_cpop_is_valid_prey_target(3, -3), 1);
        // forage_target == -1 with parent_ft > -2 => valid (juvenile with non-predatory parent)
        assert_eq!(avd_cpop_is_valid_prey_target(-1, -1), 1);
        assert_eq!(avd_cpop_is_valid_prey_target(-1, 0), 1);
        // forage_target == -1 with parent_ft <= -2 => excluded (juvenile with predatory parent)
        assert_eq!(avd_cpop_is_valid_prey_target(-1, -2), 0);
        assert_eq!(avd_cpop_is_valid_prey_target(-1, -3), 0);
        // forage_target < -1 => excluded (predator)
        assert_eq!(avd_cpop_is_valid_prey_target(-2, 0), 0);
        assert_eq!(avd_cpop_is_valid_prey_target(-3, 0), 0);
    }

    // --- Merit bonus gate tests ---

    #[test]
    fn cpop_merit_bonus_enabled_policy() {
        assert_eq!(avd_cpop_is_merit_bonus_enabled(-1), 0);
        assert_eq!(avd_cpop_is_merit_bonus_enabled(0), 1);
        assert_eq!(avd_cpop_is_merit_bonus_enabled(5), 1);
        assert_eq!(avd_cpop_is_merit_bonus_enabled(-2), 1);
    }

    #[test]
    fn cpop_deme_routing_short_circuit_kind_policy() {
        assert_eq!(
            avd_cpop_deme_routing_short_circuit_kind(CPOP_ROUTING_MODE_PROCESS_STEP, -1),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(
            avd_cpop_deme_routing_short_circuit_kind(CPOP_ROUTING_MODE_PROCESS_STEP, 1),
            CPOP_DEME_BLOCK_RUN
        );
        assert_eq!(
            avd_cpop_deme_routing_short_circuit_kind(CPOP_ROUTING_MODE_SPECULATIVE_STEP, 1),
            CPOP_DEME_BLOCK_SKIP
        );
        assert_eq!(
            avd_cpop_deme_routing_short_circuit_kind(CPOP_ROUTING_MODE_SPECULATIVE_STEP, 2),
            CPOP_DEME_BLOCK_RUN
        );
        assert_eq!(
            avd_cpop_deme_routing_short_circuit_kind(-1, 2),
            CPOP_DEME_BLOCK_SKIP
        );
    }
}
