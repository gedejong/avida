use std::ffi::c_int;

const CPU_DISPATCH_FAMILY_INVALID: c_int = -1;
const CPU_DISPATCH_FAMILY_NOP: c_int = 0;
const CPU_DISPATCH_FAMILY_LABEL: c_int = 1;
const CPU_DISPATCH_FAMILY_PROMOTER: c_int = 2;
const CPU_DISPATCH_FAMILY_STALL: c_int = 3;
const CPU_DISPATCH_FAMILY_DEFAULT: c_int = 4;

fn cpu_dispatch_family(
    is_nop: c_int,
    is_label: c_int,
    is_promoter: c_int,
    should_stall: c_int,
) -> c_int {
    let valid_bit = |value: c_int| value == 0 || value == 1;
    if !(valid_bit(is_nop)
        && valid_bit(is_label)
        && valid_bit(is_promoter)
        && valid_bit(should_stall))
    {
        return CPU_DISPATCH_FAMILY_INVALID;
    }

    if should_stall != 0 {
        CPU_DISPATCH_FAMILY_STALL
    } else if is_promoter != 0 {
        CPU_DISPATCH_FAMILY_PROMOTER
    } else if is_label != 0 {
        CPU_DISPATCH_FAMILY_LABEL
    } else if is_nop != 0 {
        CPU_DISPATCH_FAMILY_NOP
    } else {
        CPU_DISPATCH_FAMILY_DEFAULT
    }
}

fn cpu_dispatch_counted_opcode(opcode: c_int, dispatch_family: c_int) -> c_int {
    match dispatch_family {
        CPU_DISPATCH_FAMILY_NOP
        | CPU_DISPATCH_FAMILY_LABEL
        | CPU_DISPATCH_FAMILY_PROMOTER
        | CPU_DISPATCH_FAMILY_STALL
        | CPU_DISPATCH_FAMILY_DEFAULT => opcode,
        _ => opcode,
    }
}

#[no_mangle]
pub extern "C" fn avd_cpu_dispatch_family(
    is_nop: c_int,
    is_label: c_int,
    is_promoter: c_int,
    should_stall: c_int,
) -> c_int {
    cpu_dispatch_family(is_nop, is_label, is_promoter, should_stall)
}

#[no_mangle]
pub extern "C" fn avd_cpu_dispatch_counted_opcode(opcode: c_int, dispatch_family: c_int) -> c_int {
    cpu_dispatch_counted_opcode(opcode, dispatch_family)
}

// --- Thread count change classification ---
// After an instruction executes, the thread count may have changed.
// This classifies the change for the SingleProcess loop.
const CPU_THREAD_CHANGE_NONE: c_int = 0;
const CPU_THREAD_CHANGE_KILLED_ONE: c_int = 1;
const CPU_THREAD_CHANGE_DIVIDE: c_int = 2;
const CPU_THREAD_CHANGE_ERROR: c_int = 3;

/// Classifies post-instruction thread count change.
///
/// - `num_threads_before`: thread count before instruction
/// - `thread_size_after`: `m_threads.GetSize()` after instruction
///
/// Returns: 0=no change/grew, 1=killed one thread, 2=divide occurred, 3=error
#[no_mangle]
pub extern "C" fn avd_cpu_thread_change_kind(
    num_threads_before: c_int,
    thread_size_after: c_int,
) -> c_int {
    if num_threads_before == thread_size_after + 1 {
        CPU_THREAD_CHANGE_KILLED_ONE
    } else if num_threads_before > thread_size_after && thread_size_after == 1 {
        CPU_THREAD_CHANGE_DIVIDE
    } else if num_threads_before > thread_size_after {
        CPU_THREAD_CHANGE_ERROR
    } else {
        CPU_THREAD_CHANGE_NONE
    }
}

// --- Max-executed death policy ---

/// Returns 1 if the organism should die based on max-executed or to-die flag.
/// max_executed: organism's max allowed instructions (0 = unlimited)
/// time_used: instructions executed so far
/// to_die: phenotype.GetToDie() flag (0 or 1)
#[no_mangle]
pub extern "C" fn avd_cpu_should_die_max_executed(
    max_executed: c_int,
    time_used: c_int,
    to_die: c_int,
) -> c_int {
    if (max_executed > 0 && time_used >= max_executed) || to_die != 0 {
        1
    } else {
        0
    }
}

// --- No-active-promoter exec suppression ---

/// Returns 1 if execution should be suppressed because there's no active promoter
/// and the config mode is 2.
/// promoters_enabled: 0 or 1
/// no_active_promoter_effect: config value (effect mode)
/// promoter_index: current promoter index (-1 means none)
#[no_mangle]
pub extern "C" fn avd_cpu_should_suppress_no_promoter(
    promoters_enabled: c_int,
    no_active_promoter_effect: c_int,
    promoter_index: c_int,
) -> c_int {
    if promoters_enabled != 0 && no_active_promoter_effect == 2 && promoter_index == -1 {
        1
    } else {
        0
    }
}

// --- Promoter max-inst termination check ---

/// Returns 1 if the promoter should terminate because max instructions reached.
/// promoter_inst_max: config value (0 = unlimited)
/// promoter_inst_executed: instructions executed in current promoter run
#[no_mangle]
pub extern "C" fn avd_cpu_should_terminate_promoter(
    promoter_inst_max: c_int,
    promoter_inst_executed: c_int,
) -> c_int {
    if promoter_inst_max != 0 && promoter_inst_executed >= promoter_inst_max {
        1
    } else {
        0
    }
}

// --- Task switch penalty cost ---

/// Computes task switch penalty cost.
/// penalty_type: config value (0 = disabled)
/// num_new_unique_reactions: number of newly performed unique reactions
/// penalty_per_switch: config value for cost per switch
/// Returns: computed cost (0 if disabled or no new reactions)
#[no_mangle]
pub extern "C" fn avd_cpu_task_switch_penalty(
    penalty_type: c_int,
    num_new_unique_reactions: c_int,
    penalty_per_switch: c_int,
) -> c_int {
    if penalty_type != 0 && num_new_unique_reactions > 0 {
        num_new_unique_reactions * penalty_per_switch
    } else {
        0
    }
}

// --- TestCPU resource update gate ---

/// Returns 1 if the TestCPU should update resources this cycle.
/// Active when res_method >= RES_UPDATED_DEPLETABLE (2) AND cpu_cycles_used is on a time-slice boundary.
#[no_mangle]
pub extern "C" fn avd_cpu_should_update_test_resources(
    res_method: c_int,
    cpu_cycles_used: c_int,
    ave_time_slice: c_int,
) -> c_int {
    if ave_time_slice > 0 && res_method >= 2 && (cpu_cycles_used % ave_time_slice == 0) {
        1
    } else {
        0
    }
}

// --- Genome size clamping ---

/// Clamp a configured max genome size. If config is 0 or exceeds absolute max, use absolute max.
#[no_mangle]
pub extern "C" fn avd_cpu_clamp_max_genome_size(config_value: c_int, absolute_max: c_int) -> c_int {
    if config_value == 0 || config_value > absolute_max {
        absolute_max
    } else {
        config_value
    }
}

/// Clamp a configured min genome size. If config is 0 or below absolute min, use absolute min.
#[no_mangle]
pub extern "C" fn avd_cpu_clamp_min_genome_size(config_value: c_int, absolute_min: c_int) -> c_int {
    if config_value == 0 || config_value < absolute_min {
        absolute_min
    } else {
        config_value
    }
}

// --- Cardinal direction from gradient vectors ---

/// Classify gradient vectors (northerly, easterly) into 8 facing directions.
/// 0=N, 1=NE, 2=E, 3=SE, 4=S, 5=SW, 6=W, 7=NW, -1=zero vector.
#[no_mangle]
pub extern "C" fn avd_cpu_gradient_facing(northerly: c_int, easterly: c_int) -> c_int {
    match (northerly.signum(), easterly.signum()) {
        (1, 0) => 0,   // N
        (1, -1) => 1,  // NE
        (0, -1) => 2,  // E
        (-1, -1) => 3, // SE
        (-1, 0) => 4,  // S
        (-1, 1) => 5,  // SW
        (0, 1) => 6,   // W
        (1, 1) => 7,   // NW
        _ => -1,       // zero vector
    }
}

// --- Allocation validity check ---
const CPU_ALLOC_OK: c_int = 0;
const CPU_ALLOC_TOO_SMALL: c_int = 1;
const CPU_ALLOC_OUT_OF_RANGE: c_int = 2;
const CPU_ALLOC_TOO_LARGE: c_int = 3;
const CPU_ALLOC_PARENT_TOO_LARGE: c_int = 4;

/// Validate allocation request against size policies.
/// Returns 0=OK, 1=too small, 2=new_size out of range, 3=allocated too large, 4=old too large.
#[no_mangle]
pub extern "C" fn avd_cpu_alloc_validity(
    allocated_size: c_int,
    old_size: c_int,
    min_genome: c_int,
    max_genome: c_int,
    max_alloc_size: c_int,
    max_old_size: c_int,
) -> c_int {
    if allocated_size < 1 {
        return CPU_ALLOC_TOO_SMALL;
    }
    let new_size = old_size + allocated_size;
    if new_size > max_genome || new_size < min_genome {
        return CPU_ALLOC_OUT_OF_RANGE;
    }
    if allocated_size > max_alloc_size {
        return CPU_ALLOC_TOO_LARGE;
    }
    if old_size > max_old_size {
        return CPU_ALLOC_PARENT_TOO_LARGE;
    }
    CPU_ALLOC_OK
}

// --- Next/previous register wrap ---

/// Compute next register ID with wraparound.
#[no_mangle]
pub extern "C" fn avd_cpu_next_register(default_register: c_int, num_registers: c_int) -> c_int {
    if num_registers <= 0 {
        return default_register;
    }
    (default_register + 1) % num_registers
}

/// Compute previous register ID with wraparound.
#[no_mangle]
pub extern "C" fn avd_cpu_prev_register(default_register: c_int, num_registers: c_int) -> c_int {
    if num_registers <= 0 {
        return default_register;
    }
    (default_register + num_registers - 1) % num_registers
}

// --- Unary math domain guard (sqrt, log, log10) ---
const CPU_MATH_COMPUTE: c_int = 0;
const CPU_MATH_NOOP: c_int = 1;
const CPU_MATH_FAULT_NEGATIVE: c_int = 2;

/// Domain-check a unary math operation value.
///
/// - `value`: register value
/// - `threshold`: minimum value for computation (1 for log/log10, 2 for sqrt as `> 1`)
///
/// Returns: 0=compute, 1=no-op (value in [0, threshold)), 2=fault (negative)
#[no_mangle]
pub extern "C" fn avd_cpu_unary_math_domain(value: c_int, threshold: c_int) -> c_int {
    if value >= threshold {
        CPU_MATH_COMPUTE
    } else if value < 0 {
        CPU_MATH_FAULT_NEGATIVE
    } else {
        CPU_MATH_NOOP
    }
}

// --- Div/mod zero and overflow guard ---
const CPU_DIV_OK: c_int = 0;
const CPU_DIV_ZERO: c_int = 1;
const CPU_DIV_OVERFLOW: c_int = 2;

/// Guard a division/modulo operation.
///
/// - `op1`: numerator
/// - `op2`: denominator
/// - `int_min`: INT_MIN for the platform (to detect `INT_MIN / -1` overflow)
///
/// Returns: 0=OK to compute, 1=divide by zero, 2=overflow (INT_MIN / -1)
#[no_mangle]
pub extern "C" fn avd_cpu_div_guard(op1: c_int, op2: c_int, int_min: c_int) -> c_int {
    if op2 == 0 {
        CPU_DIV_ZERO
    } else if op1 == int_min && op2 == -1 {
        CPU_DIV_OVERFLOW
    } else {
        CPU_DIV_OK
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_dispatch_family_precedence_policy() {
        assert_eq!(
            avd_cpu_dispatch_family(1, 1, 1, 1),
            CPU_DISPATCH_FAMILY_STALL
        );
        assert_eq!(
            avd_cpu_dispatch_family(1, 1, 1, 0),
            CPU_DISPATCH_FAMILY_PROMOTER
        );
        assert_eq!(
            avd_cpu_dispatch_family(1, 1, 0, 0),
            CPU_DISPATCH_FAMILY_LABEL
        );
        assert_eq!(avd_cpu_dispatch_family(1, 0, 0, 0), CPU_DISPATCH_FAMILY_NOP);
        assert_eq!(
            avd_cpu_dispatch_family(0, 0, 0, 0),
            CPU_DISPATCH_FAMILY_DEFAULT
        );
    }

    #[test]
    fn cpu_dispatch_family_invalid_bit_guard() {
        assert_eq!(
            avd_cpu_dispatch_family(2, 0, 0, 0),
            CPU_DISPATCH_FAMILY_INVALID
        );
        assert_eq!(
            avd_cpu_dispatch_family(0, -1, 0, 0),
            CPU_DISPATCH_FAMILY_INVALID
        );
    }

    #[test]
    fn cpu_dispatch_counted_opcode_identity_policy() {
        assert_eq!(
            avd_cpu_dispatch_counted_opcode(77, CPU_DISPATCH_FAMILY_DEFAULT),
            77
        );
        assert_eq!(
            avd_cpu_dispatch_counted_opcode(11, CPU_DISPATCH_FAMILY_INVALID),
            11
        );
        assert_eq!(
            avd_cpu_dispatch_counted_opcode(3, CPU_DISPATCH_FAMILY_NOP),
            3
        );
    }

    // --- Thread change classification tests ---

    #[test]
    fn thread_change_killed_one() {
        assert_eq!(
            avd_cpu_thread_change_kind(5, 4),
            CPU_THREAD_CHANGE_KILLED_ONE
        );
        assert_eq!(
            avd_cpu_thread_change_kind(2, 1),
            CPU_THREAD_CHANGE_KILLED_ONE
        );
    }

    #[test]
    fn thread_change_divide() {
        // More than one thread lost, but ended at 1 => divide
        assert_eq!(avd_cpu_thread_change_kind(3, 1), CPU_THREAD_CHANGE_DIVIDE);
        assert_eq!(avd_cpu_thread_change_kind(10, 1), CPU_THREAD_CHANGE_DIVIDE);
    }

    #[test]
    fn thread_change_error() {
        // Lost more than one thread but didn't end at 1 => error
        assert_eq!(avd_cpu_thread_change_kind(5, 2), CPU_THREAD_CHANGE_ERROR);
        assert_eq!(avd_cpu_thread_change_kind(5, 3), CPU_THREAD_CHANGE_ERROR);
    }

    #[test]
    fn thread_change_none() {
        // No change or grew
        assert_eq!(avd_cpu_thread_change_kind(3, 3), CPU_THREAD_CHANGE_NONE);
        assert_eq!(avd_cpu_thread_change_kind(3, 4), CPU_THREAD_CHANGE_NONE);
        assert_eq!(avd_cpu_thread_change_kind(1, 1), CPU_THREAD_CHANGE_NONE);
    }

    // --- Max-executed death policy tests ---

    #[test]
    fn should_die_max_executed_policy() {
        // max_executed > 0 and time_used >= max_executed => die
        assert_eq!(avd_cpu_should_die_max_executed(100, 100, 0), 1);
        assert_eq!(avd_cpu_should_die_max_executed(100, 200, 0), 1);
        // to_die flag set => die
        assert_eq!(avd_cpu_should_die_max_executed(0, 0, 1), 1);
        // Neither condition => no die
        assert_eq!(avd_cpu_should_die_max_executed(100, 50, 0), 0);
        assert_eq!(avd_cpu_should_die_max_executed(0, 999, 0), 0);
    }

    // --- Promoter exec suppression tests ---

    #[test]
    fn should_suppress_no_promoter_policy() {
        // All three conditions met => suppress
        assert_eq!(avd_cpu_should_suppress_no_promoter(1, 2, -1), 1);
        // Missing any condition => don't suppress
        assert_eq!(avd_cpu_should_suppress_no_promoter(0, 2, -1), 0);
        assert_eq!(avd_cpu_should_suppress_no_promoter(1, 1, -1), 0);
        assert_eq!(avd_cpu_should_suppress_no_promoter(1, 2, 0), 0);
        assert_eq!(avd_cpu_should_suppress_no_promoter(1, 2, 5), 0);
    }

    // --- Promoter max-inst termination tests ---

    #[test]
    fn should_terminate_promoter_policy() {
        // max is set and executed >= max => terminate
        assert_eq!(avd_cpu_should_terminate_promoter(10, 10), 1);
        assert_eq!(avd_cpu_should_terminate_promoter(10, 15), 1);
        // max is 0 (unlimited) => no terminate
        assert_eq!(avd_cpu_should_terminate_promoter(0, 100), 0);
        // Not yet reached => no terminate
        assert_eq!(avd_cpu_should_terminate_promoter(10, 5), 0);
    }

    // --- Cardinal direction tests ---

    #[test]
    fn gradient_facing_all_directions() {
        assert_eq!(avd_cpu_gradient_facing(1, 0), 0); // N
        assert_eq!(avd_cpu_gradient_facing(1, -1), 1); // NE
        assert_eq!(avd_cpu_gradient_facing(0, -1), 2); // E
        assert_eq!(avd_cpu_gradient_facing(-1, -1), 3); // SE
        assert_eq!(avd_cpu_gradient_facing(-1, 0), 4); // S
        assert_eq!(avd_cpu_gradient_facing(-1, 1), 5); // SW
        assert_eq!(avd_cpu_gradient_facing(0, 1), 6); // W
        assert_eq!(avd_cpu_gradient_facing(1, 1), 7); // NW
        assert_eq!(avd_cpu_gradient_facing(0, 0), -1); // zero vector
    }

    // --- TestCPU resource update gate tests ---

    #[test]
    fn should_update_test_resources_policy() {
        // method >= 2 and on boundary
        assert_eq!(avd_cpu_should_update_test_resources(2, 100, 10), 1);
        assert_eq!(avd_cpu_should_update_test_resources(3, 50, 10), 1);
        // not on boundary
        assert_eq!(avd_cpu_should_update_test_resources(2, 101, 10), 0);
        // method too low
        assert_eq!(avd_cpu_should_update_test_resources(1, 100, 10), 0);
        assert_eq!(avd_cpu_should_update_test_resources(0, 100, 10), 0);
        // zero time slice guard
        assert_eq!(avd_cpu_should_update_test_resources(2, 100, 0), 0);
    }

    // --- Genome size clamping tests ---

    #[test]
    fn clamp_max_genome_size_policy() {
        assert_eq!(avd_cpu_clamp_max_genome_size(0, 1000), 1000); // 0 → absolute max
        assert_eq!(avd_cpu_clamp_max_genome_size(2000, 1000), 1000); // exceeds → absolute max
        assert_eq!(avd_cpu_clamp_max_genome_size(500, 1000), 500); // within range
    }

    #[test]
    fn clamp_min_genome_size_policy() {
        assert_eq!(avd_cpu_clamp_min_genome_size(0, 10), 10); // 0 → absolute min
        assert_eq!(avd_cpu_clamp_min_genome_size(5, 10), 10); // below → absolute min
        assert_eq!(avd_cpu_clamp_min_genome_size(50, 10), 50); // within range
    }

    #[test]
    fn gradient_facing_large_values() {
        assert_eq!(avd_cpu_gradient_facing(100, 0), 0);
        assert_eq!(avd_cpu_gradient_facing(-50, -200), 3);
    }

    // --- Allocation validity tests ---

    #[test]
    fn alloc_validity_ok() {
        assert_eq!(
            avd_cpu_alloc_validity(100, 100, 10, 500, 200, 200),
            CPU_ALLOC_OK
        );
    }

    #[test]
    fn alloc_validity_failures() {
        assert_eq!(
            avd_cpu_alloc_validity(0, 100, 10, 500, 200, 200),
            CPU_ALLOC_TOO_SMALL
        );
        assert_eq!(
            avd_cpu_alloc_validity(-5, 100, 10, 500, 200, 200),
            CPU_ALLOC_TOO_SMALL
        );
        // new_size = 100 + 500 = 600 > max_genome 500
        assert_eq!(
            avd_cpu_alloc_validity(500, 100, 10, 500, 600, 600),
            CPU_ALLOC_OUT_OF_RANGE
        );
        // allocated > max_alloc
        assert_eq!(
            avd_cpu_alloc_validity(201, 100, 10, 500, 200, 200),
            CPU_ALLOC_TOO_LARGE
        );
        // old > max_old
        assert_eq!(
            avd_cpu_alloc_validity(50, 201, 10, 500, 300, 200),
            CPU_ALLOC_PARENT_TOO_LARGE
        );
    }

    // --- Register wrap tests ---

    #[test]
    fn next_prev_register_wrap() {
        assert_eq!(avd_cpu_next_register(0, 3), 1);
        assert_eq!(avd_cpu_next_register(2, 3), 0); // wraps
        assert_eq!(avd_cpu_prev_register(0, 3), 2); // wraps
        assert_eq!(avd_cpu_prev_register(2, 3), 1);
    }

    #[test]
    fn register_wrap_guard() {
        assert_eq!(avd_cpu_next_register(0, 0), 0);
        assert_eq!(avd_cpu_prev_register(0, 0), 0);
    }

    // --- Unary math domain tests ---

    #[test]
    fn unary_math_domain_policy() {
        // sqrt threshold=2 (value > 1)
        assert_eq!(avd_cpu_unary_math_domain(5, 2), CPU_MATH_COMPUTE);
        assert_eq!(avd_cpu_unary_math_domain(1, 2), CPU_MATH_NOOP);
        assert_eq!(avd_cpu_unary_math_domain(0, 2), CPU_MATH_NOOP);
        assert_eq!(avd_cpu_unary_math_domain(-1, 2), CPU_MATH_FAULT_NEGATIVE);
        // log threshold=1
        assert_eq!(avd_cpu_unary_math_domain(5, 1), CPU_MATH_COMPUTE);
        assert_eq!(avd_cpu_unary_math_domain(1, 1), CPU_MATH_COMPUTE);
        assert_eq!(avd_cpu_unary_math_domain(0, 1), CPU_MATH_NOOP);
        assert_eq!(avd_cpu_unary_math_domain(-3, 1), CPU_MATH_FAULT_NEGATIVE);
    }

    // --- Div guard tests ---

    #[test]
    fn div_guard_policy() {
        assert_eq!(avd_cpu_div_guard(10, 3, i32::MIN), CPU_DIV_OK);
        assert_eq!(avd_cpu_div_guard(10, 0, i32::MIN), CPU_DIV_ZERO);
        assert_eq!(avd_cpu_div_guard(i32::MIN, -1, i32::MIN), CPU_DIV_OVERFLOW);
        assert_eq!(avd_cpu_div_guard(i32::MIN, 2, i32::MIN), CPU_DIV_OK);
    }

    // --- Task switch penalty tests ---

    #[test]
    fn task_switch_penalty_policy() {
        // Normal case
        assert_eq!(avd_cpu_task_switch_penalty(1, 3, 10), 30);
        // Disabled
        assert_eq!(avd_cpu_task_switch_penalty(0, 3, 10), 0);
        // No new reactions
        assert_eq!(avd_cpu_task_switch_penalty(1, 0, 10), 0);
        // Both disabled and no reactions
        assert_eq!(avd_cpu_task_switch_penalty(0, 0, 10), 0);
    }
}
