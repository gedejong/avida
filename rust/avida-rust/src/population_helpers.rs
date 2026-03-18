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

// --- Phenotype config gates ---

// --- Birth method gate ---

/// Returns 1 if birth method is FULL_SOUP_ELDEST (5).
#[no_mangle]
pub extern "C" fn avd_cpop_is_birth_method_eldest(birth_method: c_int) -> c_int {
    if birth_method == 5 {
        1
    } else {
        0
    }
}

// DIVIDE_METHOD_OFFSPRING=0, DIVIDE_METHOD_SPLIT=1, DIVIDE_METHOD_BIRTH=2
// GENERATION_INC_OFFSPRING=0, GENERATION_INC_BOTH=1

/// Returns 1 if divide method is SPLIT (1).
#[no_mangle]
pub extern "C" fn avd_cpop_is_divide_method_split(divide_method: c_int) -> c_int {
    if divide_method == 1 {
        1
    } else {
        0
    }
}

/// Returns 1 if generation increment method is BOTH (1).
#[no_mangle]
pub extern "C" fn avd_cpop_is_generation_inc_both(gen_inc_method: c_int) -> c_int {
    if gen_inc_method == 1 {
        1
    } else {
        0
    }
}

/// Returns 1 if divide method is SPLIT (1) or BIRTH (2).
#[no_mangle]
pub extern "C" fn avd_cpop_is_divide_method_split_or_birth(divide_method: c_int) -> c_int {
    if divide_method == 1 || divide_method == 2 {
        1
    } else {
        0
    }
}

// --- CopyParentFT loophole guard ---

/// Returns 1 if the parent forage target should be copied to the offspring.
/// Returns 0 (block copy) when the loophole condition is met:
/// pred_prey_switch <= 0 or == 2, AND parent is prey (> -2), AND current ft is predatory (< -1).
#[no_mangle]
pub extern "C" fn avd_cpop_should_copy_parent_ft(
    pred_prey_switch: c_int,
    parent_ft: c_int,
    forage_target: c_int,
) -> c_int {
    if (pred_prey_switch <= 0 || pred_prey_switch == 2) && parent_ft > -2 && forage_target < -1 {
        0 // block copy
    } else {
        1 // allow copy
    }
}

// --- Max-pred kill gate ---

/// Returns 1 if a random predator should be killed to enforce max_pred cap.
/// Active when parent_ft <= -2 AND max_pred > 0 AND num_total_pred >= max_pred.
#[no_mangle]
pub extern "C" fn avd_cpop_should_kill_rand_pred(
    parent_ft: c_int,
    max_pred: c_int,
    num_total_pred: c_int,
) -> c_int {
    if parent_ft <= -2 && max_pred > 0 && num_total_pred >= max_pred {
        1
    } else {
        0
    }
}

// --- Message buffer behavior classification ---
const CPOP_MSG_BUFFER_DROP_OLDEST: c_int = 0;
const CPOP_MSG_BUFFER_DROP_NEW: c_int = 1;
const CPOP_MSG_BUFFER_INVALID: c_int = -1;

/// Classify message receive buffer overflow behavior.
/// 0 = drop oldest, 1 = drop new message, other = invalid/error.
#[no_mangle]
pub extern "C" fn avd_cpop_msg_buffer_overflow_action(behavior: c_int) -> c_int {
    match behavior {
        0 => CPOP_MSG_BUFFER_DROP_OLDEST,
        1 => CPOP_MSG_BUFFER_DROP_NEW,
        _ => CPOP_MSG_BUFFER_INVALID,
    }
}

// --- Message buffer full check ---

/// Returns 1 if the message receive buffer is full and needs overflow handling.
/// buffer_size == -1 means unlimited. Returns 0 if not full or unlimited.
#[no_mangle]
pub extern "C" fn avd_cpop_is_msg_buffer_full(buffer_size: c_int, current_count: c_int) -> c_int {
    if buffer_size != -1 && buffer_size <= current_count {
        1
    } else {
        0
    }
}

// --- Forage target transition classification ---
// Given (new_ft, old_ft), classify the population counter transition.
const CPOP_FT_TRANSITION_NONE: c_int = 0;
const CPOP_FT_TRANSITION_PREY_TO_PRED: c_int = 1;
const CPOP_FT_TRANSITION_TOP_PRED_TO_PRED: c_int = 2;
const CPOP_FT_TRANSITION_PREY_TO_TOP_PRED: c_int = 3;
const CPOP_FT_TRANSITION_PRED_TO_TOP_PRED: c_int = 4;
const CPOP_FT_TRANSITION_PRED_TO_PREY: c_int = 5;
const CPOP_FT_TRANSITION_TOP_PRED_TO_PREY: c_int = 6;

/// Classify forage-target transition for population counter updates.
///
/// - `new_ft`: new forage target
/// - `old_ft`: current forage target
///
/// Returns transition kind (which counters to increment/decrement).
/// prey: ft > -2, pred: ft == -2, top_pred: ft < -2
#[no_mangle]
pub extern "C" fn avd_cpop_forage_target_transition(new_ft: c_int, old_ft: c_int) -> c_int {
    let new_is_prey = new_ft > -2;
    let new_is_pred = new_ft == -2;
    let new_is_top = new_ft < -2;
    let old_is_prey = old_ft > -2;
    let old_is_pred = old_ft == -2;
    let old_is_top = old_ft < -2;

    if new_is_pred && old_is_prey {
        CPOP_FT_TRANSITION_PREY_TO_PRED
    } else if new_is_pred && old_is_top {
        CPOP_FT_TRANSITION_TOP_PRED_TO_PRED
    } else if new_is_top && old_is_prey {
        CPOP_FT_TRANSITION_PREY_TO_TOP_PRED
    } else if new_is_top && old_is_pred {
        CPOP_FT_TRANSITION_PRED_TO_TOP_PRED
    } else if new_is_prey && old_is_pred {
        CPOP_FT_TRANSITION_PRED_TO_PREY
    } else if new_is_prey && old_is_top {
        CPOP_FT_TRANSITION_TOP_PRED_TO_PREY
    } else {
        CPOP_FT_TRANSITION_NONE
    }
}

// --- Deme resource reset policy ---
const CPOP_DEME_RESET_BOTH: c_int = 0;
const CPOP_DEME_RESET_TARGET_ONLY: c_int = 1;
const CPOP_DEME_RESET_NEITHER: c_int = 2;
const CPOP_DEME_RESET_INVALID: c_int = -1;

/// Classifies deme resource reset policy from config value.
/// 0=reset both, 1=reset target only, 2=reset neither, other=-1 (invalid).
#[no_mangle]
pub extern "C" fn avd_cpop_deme_reset_resources_kind(config_value: c_int) -> c_int {
    match config_value {
        0 => CPOP_DEME_RESET_BOTH,
        1 => CPOP_DEME_RESET_TARGET_ONLY,
        2 => CPOP_DEME_RESET_NEITHER,
        _ => CPOP_DEME_RESET_INVALID,
    }
}

// --- Max prey kill gate ---

/// Returns 1 if a random prey should be killed to enforce max_prey cap.
/// Active when max_prey > 0 AND num_prey >= max_prey AND parent is prey.
#[no_mangle]
pub extern "C" fn avd_cpop_should_kill_rand_prey(
    max_prey: c_int,
    num_prey: c_int,
    is_prey_ft: c_int,
) -> c_int {
    if max_prey > 0 && num_prey >= max_prey && is_prey_ft != 0 {
        1
    } else {
        0
    }
}

// --- Test-birth kill gate ---

/// Returns 1 if offspring should be killed for test-birth methods (12 or 13) when not injecting.
#[no_mangle]
pub extern "C" fn avd_cpop_should_kill_test_birth(birth_method: c_int, is_inject: c_int) -> c_int {
    if (birth_method == 12 || birth_method == 13) && is_inject == 0 {
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

    // --- Birth method gate tests ---

    #[test]
    fn birth_method_eldest_policy() {
        assert_eq!(avd_cpop_is_birth_method_eldest(5), 1);
        assert_eq!(avd_cpop_is_birth_method_eldest(0), 0);
        assert_eq!(avd_cpop_is_birth_method_eldest(4), 0);
    }

    // --- Phenotype config gate tests ---

    #[test]
    fn divide_method_split_policy() {
        assert_eq!(avd_cpop_is_divide_method_split(1), 1); // SPLIT
        assert_eq!(avd_cpop_is_divide_method_split(0), 0); // OFFSPRING
        assert_eq!(avd_cpop_is_divide_method_split(2), 0); // BIRTH
    }

    #[test]
    fn generation_inc_both_policy() {
        assert_eq!(avd_cpop_is_generation_inc_both(1), 1); // BOTH
        assert_eq!(avd_cpop_is_generation_inc_both(0), 0); // OFFSPRING
    }

    #[test]
    fn divide_method_split_or_birth_policy() {
        assert_eq!(avd_cpop_is_divide_method_split_or_birth(1), 1); // SPLIT
        assert_eq!(avd_cpop_is_divide_method_split_or_birth(2), 1); // BIRTH
        assert_eq!(avd_cpop_is_divide_method_split_or_birth(0), 0); // OFFSPRING
    }

    // --- CopyParentFT loophole guard tests ---

    #[test]
    fn should_copy_parent_ft_policy() {
        // Loophole blocked: switch<=0 or ==2, parent prey (>-2), current predatory (<-1)
        assert_eq!(avd_cpop_should_copy_parent_ft(0, 0, -2), 0);
        assert_eq!(avd_cpop_should_copy_parent_ft(-2, 1, -2), 0);
        assert_eq!(avd_cpop_should_copy_parent_ft(2, 0, -2), 0);
        // forage_target == -1 is NOT < -1, so copy is allowed
        assert_eq!(avd_cpop_should_copy_parent_ft(0, 0, -1), 1);
        // Allowed: switch == 1 (not matching condition)
        assert_eq!(avd_cpop_should_copy_parent_ft(1, 0, -2), 1);
        // Allowed: parent is predator
        assert_eq!(avd_cpop_should_copy_parent_ft(0, -2, -2), 1);
        // Allowed: current is prey
        assert_eq!(avd_cpop_should_copy_parent_ft(0, 0, 0), 1);
    }

    // --- Max-pred kill gate tests ---

    #[test]
    fn should_kill_rand_pred_policy() {
        assert_eq!(avd_cpop_should_kill_rand_pred(-2, 50, 50), 1);
        assert_eq!(avd_cpop_should_kill_rand_pred(-3, 50, 60), 1);
        assert_eq!(avd_cpop_should_kill_rand_pred(-1, 50, 50), 0); // not pred parent
        assert_eq!(avd_cpop_should_kill_rand_pred(-2, 0, 50), 0); // disabled
        assert_eq!(avd_cpop_should_kill_rand_pred(-2, 50, 49), 0); // under cap
    }

    // --- Message buffer tests ---

    #[test]
    fn msg_buffer_overflow_action_policy() {
        assert_eq!(
            avd_cpop_msg_buffer_overflow_action(0),
            CPOP_MSG_BUFFER_DROP_OLDEST
        );
        assert_eq!(
            avd_cpop_msg_buffer_overflow_action(1),
            CPOP_MSG_BUFFER_DROP_NEW
        );
        assert_eq!(
            avd_cpop_msg_buffer_overflow_action(2),
            CPOP_MSG_BUFFER_INVALID
        );
    }

    #[test]
    fn msg_buffer_full_check_policy() {
        assert_eq!(avd_cpop_is_msg_buffer_full(10, 10), 1);
        assert_eq!(avd_cpop_is_msg_buffer_full(10, 15), 1);
        assert_eq!(avd_cpop_is_msg_buffer_full(10, 5), 0);
        assert_eq!(avd_cpop_is_msg_buffer_full(-1, 100), 0); // unlimited
    }

    // --- Forage target transition tests ---

    #[test]
    fn forage_target_transition_policy() {
        // prey(>-2) to pred(==-2)
        assert_eq!(
            avd_cpop_forage_target_transition(-2, 0),
            CPOP_FT_TRANSITION_PREY_TO_PRED
        );
        // top_pred(<-2) to pred(==-2)
        assert_eq!(
            avd_cpop_forage_target_transition(-2, -3),
            CPOP_FT_TRANSITION_TOP_PRED_TO_PRED
        );
        // prey to top_pred
        assert_eq!(
            avd_cpop_forage_target_transition(-3, 0),
            CPOP_FT_TRANSITION_PREY_TO_TOP_PRED
        );
        // pred to top_pred
        assert_eq!(
            avd_cpop_forage_target_transition(-3, -2),
            CPOP_FT_TRANSITION_PRED_TO_TOP_PRED
        );
        // pred to prey
        assert_eq!(
            avd_cpop_forage_target_transition(0, -2),
            CPOP_FT_TRANSITION_PRED_TO_PREY
        );
        // top_pred to prey
        assert_eq!(
            avd_cpop_forage_target_transition(0, -3),
            CPOP_FT_TRANSITION_TOP_PRED_TO_PREY
        );
        // same type => NONE
        assert_eq!(
            avd_cpop_forage_target_transition(0, 1),
            CPOP_FT_TRANSITION_NONE
        );
        assert_eq!(
            avd_cpop_forage_target_transition(-2, -2),
            CPOP_FT_TRANSITION_NONE
        );
        assert_eq!(
            avd_cpop_forage_target_transition(-3, -4),
            CPOP_FT_TRANSITION_NONE
        );
    }

    // --- Deme resource reset tests ---

    #[test]
    fn cpop_deme_reset_resources_policy() {
        assert_eq!(avd_cpop_deme_reset_resources_kind(0), CPOP_DEME_RESET_BOTH);
        assert_eq!(
            avd_cpop_deme_reset_resources_kind(1),
            CPOP_DEME_RESET_TARGET_ONLY
        );
        assert_eq!(
            avd_cpop_deme_reset_resources_kind(2),
            CPOP_DEME_RESET_NEITHER
        );
        assert_eq!(
            avd_cpop_deme_reset_resources_kind(3),
            CPOP_DEME_RESET_INVALID
        );
        assert_eq!(
            avd_cpop_deme_reset_resources_kind(-1),
            CPOP_DEME_RESET_INVALID
        );
    }

    // --- Max prey kill gate tests ---

    #[test]
    fn cpop_should_kill_rand_prey_policy() {
        assert_eq!(avd_cpop_should_kill_rand_prey(100, 100, 1), 1);
        assert_eq!(avd_cpop_should_kill_rand_prey(100, 200, 1), 1);
        assert_eq!(avd_cpop_should_kill_rand_prey(100, 99, 1), 0); // under cap
        assert_eq!(avd_cpop_should_kill_rand_prey(0, 100, 1), 0); // disabled
        assert_eq!(avd_cpop_should_kill_rand_prey(100, 100, 0), 0); // not prey
    }

    // --- Test-birth kill gate tests ---

    #[test]
    fn cpop_should_kill_test_birth_policy() {
        assert_eq!(avd_cpop_should_kill_test_birth(12, 0), 1);
        assert_eq!(avd_cpop_should_kill_test_birth(13, 0), 1);
        assert_eq!(avd_cpop_should_kill_test_birth(12, 1), 0); // is_inject
        assert_eq!(avd_cpop_should_kill_test_birth(0, 0), 0); // normal birth
        assert_eq!(avd_cpop_should_kill_test_birth(5, 0), 0);
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
