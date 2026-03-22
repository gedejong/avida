use crate::common::with_cstr;
use std::os::raw::{c_char, c_int, c_uint};

const TASKLIB_UNARY_OP_LOG: c_int = 0;
const TASKLIB_UNARY_OP_LOG2: c_int = 1;
const TASKLIB_UNARY_OP_LOG10: c_int = 2;
const TASKLIB_UNARY_OP_SQRT: c_int = 3;
const TASKLIB_UNARY_OP_SINE: c_int = 4;
const TASKLIB_UNARY_OP_COSINE: c_int = 5;
const TASKLIB_BINARY_OP_MULT: c_int = 0;
const TASKLIB_BINARY_OP_DIV: c_int = 1;

#[no_mangle]
pub extern "C" fn avd_tasklib_fractional_reward_bits(supplied: c_uint, correct: c_uint) -> f64 {
    let bit_diff = (supplied ^ correct).count_ones();
    f64::from(32u32 - bit_diff) / 32.0
}

#[no_mangle]
pub extern "C" fn avd_tasklib_is_logic3_or_math1_name(task_name: *const c_char) -> c_int {
    with_cstr(task_name, 0, |name| {
        let bytes = name.to_bytes();
        if bytes.starts_with(b"logic_3") || bytes.starts_with(b"math_1") {
            1
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn avd_tasklib_is_math2_or_math3_name(task_name: *const c_char) -> c_int {
    with_cstr(task_name, 0, |name| {
        let bytes = name.to_bytes();
        if bytes.starts_with(b"math_2") || bytes.starts_with(b"math_3") {
            1
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn avd_tasklib_is_fibonacci_name(task_name: *const c_char) -> c_int {
    with_cstr(task_name, 0, |name| {
        if name.to_bytes().starts_with(b"fib_") {
            1
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn avd_tasklib_is_matching_sequence_name(task_name: *const c_char) -> c_int {
    with_cstr(task_name, 0, |name| match name.to_bytes() {
        b"matchstr" | b"match_number" | b"matchprodstr" | b"sort_inputs" | b"fibonacci_seq" => 1,
        _ => 0,
    })
}

#[no_mangle]
pub extern "C" fn avd_tasklib_is_load_based_name(task_name: *const c_char) -> c_int {
    with_cstr(task_name, 0, |name| match name.to_bytes() {
        b"mult" | b"div" | b"log" | b"log2" | b"log10" | b"sqrt" | b"sine" | b"cosine"
        | b"optimize" | b"sg_path_traversal" | b"form-group" | b"form-group-id"
        | b"live-on-patch-id" | b"collect-odd-cell" | b"eat-target" | b"eat-target-echo"
        | b"eat-target-nand" | b"eat-target-and" | b"eat-target-orn" | b"eat-target-or"
        | b"eat-target-andn" | b"eat-target-nor" | b"eat-target-xor" | b"eat-target-equ"
        | b"move-ft" => 1,
        _ => 0,
    })
}

#[no_mangle]
pub extern "C" fn avd_tasklib_threshold_halflife_quality(
    diff: i64,
    threshold: c_int,
    halflife_arg: f64,
) -> f64 {
    if threshold >= 0 && diff > i64::from(threshold) {
        return 0.0;
    }

    let halflife = -halflife_arg.abs();
    2.0f64.powf((diff as f64) / halflife)
}

#[no_mangle]
pub extern "C" fn avd_tasklib_diff_scan_init() -> i64 {
    (i64::from(i32::MAX) + 1) * 2
}

#[no_mangle]
pub extern "C" fn avd_tasklib_diff_scan_update(current_min: i64, candidate: i64) -> i64 {
    if candidate < current_min {
        candidate
    } else {
        current_min
    }
}

#[no_mangle]
pub extern "C" fn avd_tasklib_unary_math_input_diff(
    input_value: c_int,
    test_output: i64,
    op_kind: c_int,
    cast_precision: f64,
) -> i64 {
    let x = f64::from(input_value);
    let transformed = match op_kind {
        TASKLIB_UNARY_OP_LOG => (if input_value == 0 { 1.0 } else { x }).abs().ln(),
        TASKLIB_UNARY_OP_LOG2 => (if input_value == 0 { 1.0 } else { x }).abs().log2(),
        TASKLIB_UNARY_OP_LOG10 => (if input_value == 0 { 1.0 } else { x }).abs().log10(),
        TASKLIB_UNARY_OP_SQRT => x.abs().sqrt(),
        TASKLIB_UNARY_OP_SINE => (x / cast_precision).sin() * cast_precision,
        TASKLIB_UNARY_OP_COSINE => (x / cast_precision).cos() * cast_precision,
        _ => return i64::MAX,
    };

    ((transformed as i64) - test_output).abs()
}

#[no_mangle]
pub extern "C" fn avd_tasklib_binary_pair_input_diff(
    lhs_value: c_int,
    rhs_value: c_int,
    test_output: i64,
    op_kind: c_int,
) -> i64 {
    let transformed = match op_kind {
        TASKLIB_BINARY_OP_MULT => i64::from(lhs_value) * i64::from(rhs_value),
        TASKLIB_BINARY_OP_DIV => {
            if rhs_value == 0 {
                return i64::MAX;
            }
            i64::from(lhs_value / rhs_value)
        }
        _ => return i64::MAX,
    };

    (transformed - test_output).abs()
}

// --- Fibonacci scoring helper ---

const FIBONACCI_VALUES: [c_int; 9] = [0, 1, 2, 3, 5, 8, 13, 21, 34];

/// Check if test_output matches the expected Fibonacci value for the given index (1-based).
/// fib_index: 1=Fib1(0), 2=Fib2(1), 4=Fib4(2), 5=Fib5(3), 6=Fib6(5), 7=Fib7(8),
///            8=Fib8(13), 9=Fib9(21), 10=Fib10(34)
/// Returns 1.0 if match, 0.0 otherwise. Returns 0.0 for invalid index.
#[no_mangle]
pub extern "C" fn avd_tasklib_fib_check(test_output: c_int, fib_index: c_int) -> f64 {
    let idx = match fib_index {
        1 => 0,
        2 => 1,
        4 => 2,
        5 => 3,
        6 => 4,
        7 => 5,
        8 => 6,
        9 => 7,
        10 => 8,
        _ => return 0.0,
    };
    if test_output == FIBONACCI_VALUES[idx] {
        1.0
    } else {
        0.0
    }
}

// --- Task name family classifiers for ungated blocks ---

/// Returns 1 if the task name is a basic logic/math name.
#[no_mangle]
pub extern "C" fn avd_tasklib_is_basic_name(task_name: *const c_char) -> c_int {
    crate::common::with_cstr(task_name, 0, |s| match s.to_bytes() {
        b"echo"
        | b"echo_dup"
        | b"add"
        | b"add3"
        | b"sub"
        | b"dontcare"
        | b"not"
        | b"not_dup"
        | b"nand"
        | b"nand_dup"
        | b"and"
        | b"and_dup"
        | b"orn"
        | b"orn_dup"
        | b"or"
        | b"or_dup"
        | b"andn"
        | b"andn_dup"
        | b"nor"
        | b"nor_dup"
        | b"xor"
        | b"xor_dup"
        | b"equ"
        | b"equ_dup"
        | b"xor-max"
        | b"nand-resourceDependent"
        | b"nor-resourceDependent" => 1,
        _ => 0,
    })
}

/// Returns 1 if the task name is a communication task name.
#[no_mangle]
pub extern "C" fn avd_tasklib_is_comm_name(task_name: *const c_char) -> c_int {
    crate::common::with_cstr(task_name, 0, |s| match s.to_bytes() {
        b"comm_echo" | b"comm_not" => 1,
        _ => 0,
    })
}

/// Returns 1 if the task name is a movement task name.
#[no_mangle]
pub extern "C" fn avd_tasklib_is_movement_name(task_name: *const c_char) -> c_int {
    crate::common::with_cstr(task_name, 0, |s| match s.to_bytes() {
        b"move_up_gradient"
        | b"move_neutral_gradient"
        | b"move_down_gradient"
        | b"move_not_up_gradient"
        | b"move_to_right_side"
        | b"move_to_left_side"
        | b"move"
        | b"movetotarget"
        | b"movetoevent"
        | b"movebetweenevent"
        | b"perfect_strings" => 1,
        _ => 0,
    })
}

/// Returns 1 if the task name is an event task name.
#[no_mangle]
pub extern "C" fn avd_tasklib_is_event_name(task_name: *const c_char) -> c_int {
    crate::common::with_cstr(task_name, 0, |s| match s.to_bytes() {
        b"move_to_event" | b"event_killed" => 1,
        _ => 0,
    })
}

/// Returns 1 if the task name is an altruism task name.
#[no_mangle]
pub extern "C" fn avd_tasklib_is_altruism_name(task_name: *const c_char) -> c_int {
    crate::common::with_cstr(task_name, 0, |s| match s.to_bytes() {
        b"exploded"
        | b"exploded2"
        | b"consume-public-good"
        | b"ai-display-cost"
        | b"produce-public-good" => 1,
        _ => 0,
    })
}

// --- Logic ID computation and Boolean logic task evaluation ---

/// Compute the logic ID from 3 input values and 1 output value.
///
/// This is a pure-function port of cTaskLib::SetupTests.
/// Returns the logic ID (0..255), or -1 if the output is inconsistent.
#[no_mangle]
pub extern "C" fn avd_task_compute_logic_id(
    input0: c_int,
    input1: c_int,
    input2: c_int,
    num_inputs: c_int,
    output: c_int,
) -> c_int {
    let mut test_inputs: [c_int; 3] = [input0, input1, input2];
    let mut test_output = output;

    // logic_out[i] == -1 means "not yet set"
    let mut logic_out: [c_int; 8] = [-1; 8];

    // Test all 32 bit positions for consistency
    let mut func_ok = true;
    for _ in 0..32 {
        let logic_pos = ((test_inputs[0] & 1)
            | ((test_inputs[1] & 1) << 1)
            | ((test_inputs[2] & 1) << 2)) as usize;

        let out_bit = test_output & 1;
        if logic_out[logic_pos] != -1 && logic_out[logic_pos] != out_bit {
            func_ok = false;
            break;
        }
        logic_out[logic_pos] = out_bit;

        test_output >>= 1;
        for inp in &mut test_inputs {
            *inp >>= 1;
        }
    }

    if !func_ok {
        return -1;
    }

    // Mirror bits for fewer than 3 inputs
    if num_inputs < 1 {
        // 000 -> 001
        logic_out[1] = logic_out[0];
    }
    if num_inputs < 2 {
        // 000 -> 010; 001 -> 011
        logic_out[2] = logic_out[0];
        logic_out[3] = logic_out[1];
    }
    if num_inputs < 3 {
        // 000->100; 001->101; 010->110; 011->111
        logic_out[4] = logic_out[0];
        logic_out[5] = logic_out[1];
        logic_out[6] = logic_out[2];
        logic_out[7] = logic_out[3];
    }

    // All bits must be 0 or 1 at this point (matches C++ assert in SetupTests).
    // With valid inputs and consistent output, all 8 positions will be filled.
    debug_assert!(logic_out.iter().all(|&v| v == 0 || v == 1));

    let mut logicid: c_int = 0;
    for (i, &val) in logic_out.iter().enumerate() {
        logicid += val << i;
    }
    logicid
}

/// Boolean logic task type constants for avd_task_eval_logic.
const TASK_NOT: c_int = 0;
const TASK_NAND: c_int = 1;
const TASK_AND: c_int = 2;
const TASK_ORNOT: c_int = 3;
const TASK_OR: c_int = 4;
const TASK_ANDNOT: c_int = 5;
const TASK_NOR: c_int = 6;
const TASK_XOR: c_int = 7;
const TASK_EQU: c_int = 8;

/// Evaluate a Boolean logic task given its type and logic_id.
/// Returns 1.0 if the logic_id matches any of the task's expected values, 0.0 otherwise.
#[no_mangle]
pub extern "C" fn avd_task_eval_logic(task_type: c_int, logic_id: c_int) -> f64 {
    let matches = match task_type {
        TASK_NOT => logic_id == 15 || logic_id == 51 || logic_id == 85,
        TASK_NAND => logic_id == 63 || logic_id == 95 || logic_id == 119,
        TASK_AND => logic_id == 136 || logic_id == 160 || logic_id == 192,
        TASK_ORNOT => {
            logic_id == 175
                || logic_id == 187
                || logic_id == 207
                || logic_id == 221
                || logic_id == 243
                || logic_id == 245
        }
        TASK_OR => logic_id == 238 || logic_id == 250 || logic_id == 252,
        TASK_ANDNOT => {
            logic_id == 10
                || logic_id == 12
                || logic_id == 34
                || logic_id == 48
                || logic_id == 68
                || logic_id == 80
        }
        TASK_NOR => logic_id == 3 || logic_id == 5 || logic_id == 17,
        TASK_XOR => logic_id == 60 || logic_id == 90 || logic_id == 102,
        TASK_EQU => logic_id == 153 || logic_id == 165 || logic_id == 195,
        _ => false,
    };
    if matches {
        1.0
    } else {
        0.0
    }
}

/// Check if any of the inputs matches the output (Task_Echo).
/// Returns 1.0 if output matches any input, 0.0 otherwise.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_task_eval_echo(
    inputs: *const c_int,
    num_inputs: c_int,
    output: c_int,
) -> f64 {
    if inputs.is_null() || num_inputs <= 0 {
        return 0.0;
    }
    // SAFETY: caller guarantees `inputs` points to at least `num_inputs` valid c_int values.
    // Null/zero checks above guard against invalid pointers.
    let slice = unsafe { std::slice::from_raw_parts(inputs, num_inputs as usize) };
    for &inp in slice {
        if inp == output {
            return 1.0;
        }
    }
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_uint;
    use std::ptr;

    fn reference_fractional_reward(supplied: c_uint, correct: c_uint) -> f64 {
        let mut diff = 0u32;
        let mut value = supplied ^ correct;
        for _ in 0..32 {
            diff += value & 1;
            value >>= 1;
        }
        f64::from(32 - diff) / 32.0
    }

    #[test]
    fn tasklib_fractional_reward_matches_reference_cases() {
        let cases: &[(c_uint, c_uint)] = &[
            (0, 0),
            (0xFFFFFFFF, 0xFFFFFFFF),
            (0, 0xFFFFFFFF),
            (0xAAAAAAAA, 0x55555555),
            (0xAAAAAAAA, 0xAAAAAAAA),
            (0xDEADBEEF, 0xDEADBEEF),
            (0xDEADBEEF, 0xDEADBEFF),
            (0x12345678, 0x87654321),
        ];
        for &(supplied, correct) in cases {
            let got = avd_tasklib_fractional_reward_bits(supplied, correct);
            let expected = reference_fractional_reward(supplied, correct);
            assert!((got - expected).abs() < 1e-15);
        }
    }

    #[test]
    fn tasklib_fractional_reward_properties() {
        assert!((avd_tasklib_fractional_reward_bits(0, 0) - 1.0).abs() < 1e-15);
        assert!((avd_tasklib_fractional_reward_bits(0, 1) - (31.0 / 32.0)).abs() < 1e-15);
        assert!((avd_tasklib_fractional_reward_bits(0, 0xFFFFFFFF) - 0.0).abs() < 1e-15);
        let a = avd_tasklib_fractional_reward_bits(0x1234ABCD, 0xABCD1234);
        let b = avd_tasklib_fractional_reward_bits(0xABCD1234, 0x1234ABCD);
        assert!((a - b).abs() < 1e-15);
    }

    #[test]
    fn tasklib_registration_family_classifier_logic3_math1() {
        let logic3 = CString::new("logic_3AA").expect("cstr");
        let math1 = CString::new("math_1AF").expect("cstr");
        let math2 = CString::new("math_2AA").expect("cstr");
        let echo = CString::new("echo").expect("cstr");
        assert_eq!(avd_tasklib_is_logic3_or_math1_name(logic3.as_ptr()), 1);
        assert_eq!(avd_tasklib_is_logic3_or_math1_name(math1.as_ptr()), 1);
        assert_eq!(avd_tasklib_is_logic3_or_math1_name(math2.as_ptr()), 0);
        assert_eq!(avd_tasklib_is_logic3_or_math1_name(echo.as_ptr()), 0);
        assert_eq!(avd_tasklib_is_logic3_or_math1_name(ptr::null()), 0);
    }

    #[test]
    fn tasklib_registration_family_classifier_math2_math3() {
        let math2 = CString::new("math_2AA").expect("cstr");
        let math3 = CString::new("math_3AF").expect("cstr");
        let math1 = CString::new("math_1AF").expect("cstr");
        let logic3 = CString::new("logic_3AA").expect("cstr");
        assert_eq!(avd_tasklib_is_math2_or_math3_name(math2.as_ptr()), 1);
        assert_eq!(avd_tasklib_is_math2_or_math3_name(math3.as_ptr()), 1);
        assert_eq!(avd_tasklib_is_math2_or_math3_name(math1.as_ptr()), 0);
        assert_eq!(avd_tasklib_is_math2_or_math3_name(logic3.as_ptr()), 0);
        assert_eq!(avd_tasklib_is_math2_or_math3_name(ptr::null()), 0);
    }

    #[test]
    fn tasklib_registration_family_classifier_fibonacci() {
        let fib = CString::new("fib_7").expect("cstr");
        let fib_seq = CString::new("fibonacci_seq").expect("cstr");
        let math3 = CString::new("math_3AF").expect("cstr");
        assert_eq!(avd_tasklib_is_fibonacci_name(fib.as_ptr()), 1);
        assert_eq!(avd_tasklib_is_fibonacci_name(fib_seq.as_ptr()), 0);
        assert_eq!(avd_tasklib_is_fibonacci_name(math3.as_ptr()), 0);
        assert_eq!(avd_tasklib_is_fibonacci_name(ptr::null()), 0);
    }

    #[test]
    fn tasklib_registration_family_classifier_matching_sequence() {
        let matchstr = CString::new("matchstr").expect("cstr");
        let sort_inputs = CString::new("sort_inputs").expect("cstr");
        let fib_seq = CString::new("fibonacci_seq").expect("cstr");
        let fib = CString::new("fib_7").expect("cstr");
        assert_eq!(avd_tasklib_is_matching_sequence_name(matchstr.as_ptr()), 1);
        assert_eq!(
            avd_tasklib_is_matching_sequence_name(sort_inputs.as_ptr()),
            1
        );
        assert_eq!(avd_tasklib_is_matching_sequence_name(fib_seq.as_ptr()), 1);
        assert_eq!(avd_tasklib_is_matching_sequence_name(fib.as_ptr()), 0);
        assert_eq!(avd_tasklib_is_matching_sequence_name(ptr::null()), 0);
    }

    #[test]
    fn tasklib_registration_family_classifier_load_based() {
        let mult = CString::new("mult").expect("cstr");
        let optimize = CString::new("optimize").expect("cstr");
        let eat_target = CString::new("eat-target").expect("cstr");
        let move_ft = CString::new("move-ft").expect("cstr");
        let event_name = CString::new("move_to_event").expect("cstr");
        assert_eq!(avd_tasklib_is_load_based_name(mult.as_ptr()), 1);
        assert_eq!(avd_tasklib_is_load_based_name(optimize.as_ptr()), 1);
        assert_eq!(avd_tasklib_is_load_based_name(eat_target.as_ptr()), 1);
        assert_eq!(avd_tasklib_is_load_based_name(move_ft.as_ptr()), 1);
        assert_eq!(avd_tasklib_is_load_based_name(event_name.as_ptr()), 0);
        assert_eq!(avd_tasklib_is_load_based_name(ptr::null()), 0);
    }

    #[test]
    fn tasklib_threshold_halflife_quality_threshold_policy() {
        let diff = 5i64;
        let threshold = 10;
        let halflife_arg: f64 = 4.0;
        let expected = 2.0f64.powf((diff as f64) / -halflife_arg.abs());
        let got = avd_tasklib_threshold_halflife_quality(diff, threshold, halflife_arg);
        assert!((got - expected).abs() < 1e-15);

        assert_eq!(avd_tasklib_threshold_halflife_quality(11, 10, 4.0), 0.0);
        assert_eq!(
            avd_tasklib_threshold_halflife_quality(11, -1, 4.0),
            2.0f64.powf(11.0 / -4.0)
        );
    }

    #[test]
    fn tasklib_threshold_halflife_quality_sign_and_edge_policy() {
        let pos = avd_tasklib_threshold_halflife_quality(7, -1, 3.0);
        let neg = avd_tasklib_threshold_halflife_quality(7, -1, -3.0);
        assert!((pos - neg).abs() < 1e-15);

        // Matches legacy C++ behavior with halflife_arg == 0.0.
        let zero_diff = avd_tasklib_threshold_halflife_quality(0, -1, 0.0);
        assert!(zero_diff.is_nan());
        let nonzero_diff = avd_tasklib_threshold_halflife_quality(2, -1, 0.0);
        assert_eq!(nonzero_diff, 0.0);
    }

    #[test]
    fn tasklib_unary_math_input_diff_opcode_policy() {
        assert_eq!(
            avd_tasklib_unary_math_input_diff(0, 0, TASKLIB_UNARY_OP_LOG, 100000.0),
            0
        );
        assert_eq!(
            avd_tasklib_unary_math_input_diff(1, 0, TASKLIB_UNARY_OP_LOG2, 100000.0),
            0
        );
        assert_eq!(
            avd_tasklib_unary_math_input_diff(1, 0, TASKLIB_UNARY_OP_LOG10, 100000.0),
            0
        );
        assert_eq!(
            avd_tasklib_unary_math_input_diff(-9, 3, TASKLIB_UNARY_OP_SQRT, 100000.0),
            0
        );
        assert_eq!(
            avd_tasklib_unary_math_input_diff(0, 0, TASKLIB_UNARY_OP_COSINE, 100000.0),
            100000
        );
        assert_eq!(
            avd_tasklib_unary_math_input_diff(1, 0, TASKLIB_UNARY_OP_SINE, 100000.0),
            0
        );
        assert_eq!(
            avd_tasklib_unary_math_input_diff(1, 0, -1, 100000.0),
            i64::MAX
        );
    }

    #[test]
    fn tasklib_binary_pair_input_diff_opcode_policy() {
        assert_eq!(
            avd_tasklib_binary_pair_input_diff(3, 4, 11, TASKLIB_BINARY_OP_MULT),
            1
        );
        assert_eq!(
            avd_tasklib_binary_pair_input_diff(8, 2, 3, TASKLIB_BINARY_OP_DIV),
            1
        );
        assert_eq!(
            avd_tasklib_binary_pair_input_diff(8, 0, 3, TASKLIB_BINARY_OP_DIV),
            i64::MAX
        );
        assert_eq!(avd_tasklib_binary_pair_input_diff(8, 2, 3, -1), i64::MAX);
    }

    // --- Fibonacci scoring tests ---

    #[test]
    fn tasklib_fib_check_matches() {
        assert_eq!(avd_tasklib_fib_check(0, 1), 1.0); // Fib1 = 0
        assert_eq!(avd_tasklib_fib_check(1, 2), 1.0); // Fib2 = 1
        assert_eq!(avd_tasklib_fib_check(2, 4), 1.0); // Fib4 = 2
        assert_eq!(avd_tasklib_fib_check(3, 5), 1.0); // Fib5 = 3
        assert_eq!(avd_tasklib_fib_check(5, 6), 1.0); // Fib6 = 5
        assert_eq!(avd_tasklib_fib_check(8, 7), 1.0); // Fib7 = 8
        assert_eq!(avd_tasklib_fib_check(13, 8), 1.0); // Fib8 = 13
        assert_eq!(avd_tasklib_fib_check(21, 9), 1.0); // Fib9 = 21
        assert_eq!(avd_tasklib_fib_check(34, 10), 1.0); // Fib10 = 34
    }

    #[test]
    fn tasklib_fib_check_mismatches() {
        assert_eq!(avd_tasklib_fib_check(1, 1), 0.0); // Fib1 expects 0
        assert_eq!(avd_tasklib_fib_check(0, 2), 0.0); // Fib2 expects 1
        assert_eq!(avd_tasklib_fib_check(99, 10), 0.0);
    }

    #[test]
    fn tasklib_fib_check_invalid_index() {
        assert_eq!(avd_tasklib_fib_check(0, 0), 0.0);
        assert_eq!(avd_tasklib_fib_check(0, 3), 0.0); // no Fib3
        assert_eq!(avd_tasklib_fib_check(0, 11), 0.0);
    }

    // --- Task name family classifier tests ---

    #[test]
    fn tasklib_basic_name_policy() {
        let yes_cases = ["echo", "not", "nand", "xor", "equ", "add", "sub", "xor-max"];
        for name in &yes_cases {
            let cs = CString::new(*name).unwrap();
            assert_eq!(
                avd_tasklib_is_basic_name(cs.as_ptr()),
                1,
                "expected basic for '{name}'"
            );
        }
        let no = CString::new("logic_3AA").unwrap();
        assert_eq!(avd_tasklib_is_basic_name(no.as_ptr()), 0);
        assert_eq!(avd_tasklib_is_basic_name(std::ptr::null()), 0);
    }

    #[test]
    fn tasklib_comm_name_policy() {
        let yes = CString::new("comm_echo").unwrap();
        assert_eq!(avd_tasklib_is_comm_name(yes.as_ptr()), 1);
        let no = CString::new("echo").unwrap();
        assert_eq!(avd_tasklib_is_comm_name(no.as_ptr()), 0);
    }

    #[test]
    fn tasklib_movement_name_policy() {
        let yes = CString::new("move_up_gradient").unwrap();
        assert_eq!(avd_tasklib_is_movement_name(yes.as_ptr()), 1);
        let yes2 = CString::new("movetotarget").unwrap();
        assert_eq!(avd_tasklib_is_movement_name(yes2.as_ptr()), 1);
        let no = CString::new("echo").unwrap();
        assert_eq!(avd_tasklib_is_movement_name(no.as_ptr()), 0);
    }

    #[test]
    fn tasklib_event_name_policy() {
        let yes = CString::new("move_to_event").unwrap();
        assert_eq!(avd_tasklib_is_event_name(yes.as_ptr()), 1);
        let no = CString::new("move").unwrap();
        assert_eq!(avd_tasklib_is_event_name(no.as_ptr()), 0);
    }

    #[test]
    fn tasklib_altruism_name_policy() {
        let yes = CString::new("exploded").unwrap();
        assert_eq!(avd_tasklib_is_altruism_name(yes.as_ptr()), 1);
        let no = CString::new("echo").unwrap();
        assert_eq!(avd_tasklib_is_altruism_name(no.as_ptr()), 0);
    }

    #[test]
    fn tasklib_diff_scan_reducer_policy() {
        assert_eq!(avd_tasklib_diff_scan_init(), 4_294_967_296);
        assert_eq!(avd_tasklib_diff_scan_update(10, 12), 10);
        assert_eq!(avd_tasklib_diff_scan_update(10, 7), 7);
        assert_eq!(
            avd_tasklib_diff_scan_update(avd_tasklib_diff_scan_init(), i64::MAX),
            avd_tasklib_diff_scan_init()
        );
    }

    // --- Logic ID computation tests ---

    #[test]
    fn logic_id_not_task_known_inputs() {
        // Use canonical inputs that cover all 8 logic positions:
        // input0 = 0x55555555 (bit0 alternates every 1 bit)
        // input1 = 0x33333333 (bit0 alternates every 2 bits)
        // input2 = 0x0F0F0F0F (bit0 alternates every 4 bits)
        // This ensures all 8 combinations of (A,B,C) low bits appear in the 32 positions.
        let input0 = 0x55555555_u32 as c_int;
        let input1 = 0x33333333_u32 as c_int;
        let input2 = 0x0F0F0F0F_i32;
        let output = !input0; // bitwise NOT of A
                              // NOT-A truth table: out[pos] = !A where A = pos & 1
                              // pos: 0→1, 1→0, 2→1, 3→0, 4→1, 5→0, 6→1, 7→0 = 0b01010101 = 85
        let logic_id = avd_task_compute_logic_id(input0, input1, input2, 3, output);
        assert_eq!(logic_id, 85, "NOT-A should produce logic_id 85");
        assert_eq!(avd_task_eval_logic(TASK_NOT, logic_id), 1.0);
    }

    #[test]
    fn logic_id_inconsistent_returns_minus_one() {
        // Construct inputs that produce an inconsistent output
        // If inputs cycle through all 8 logic positions but output bits disagree, we get -1
        // Simple case: input0=0, input1=0, input2=0 → logic_pos always 0
        // But output has different bits → still consistent since it always maps to pos 0.
        // We need to actually get an inconsistency. Use inputs where same logic_pos gets
        // different output bits.
        // input0=1 (bit0=1, bit1=0) → pos changes, but we need same pos different out.
        // Actually the simplest approach: all inputs 0 → logic_pos always 0,
        // but output alternates → bit 0 of output = 0, bit 1 = 1 → inconsistent at pos 0.
        let logic_id = avd_task_compute_logic_id(0, 0, 0, 3, 0b10); // bit0=0, bit1=1 at pos 0
        assert_eq!(logic_id, -1);
    }

    #[test]
    fn logic_id_echo_a() {
        // Echo of input A: output = input0
        // Truth table for A: [0,1,0,1,0,1,0,1] → 0b10101010 = 170
        let input0: c_int = 0x55555555_u32 as c_int; // alternating bits
        let output = input0;
        let logic_id =
            avd_task_compute_logic_id(input0, 0x33333333_u32 as c_int, 0x0F0F0F0F, 3, output);
        assert_eq!(logic_id, 170);
    }

    #[test]
    fn logic_id_all_256_exhaustive_task_coverage() {
        // For each of the 256 possible logic IDs, verify that each task returns
        // the correct result by checking against the known magic numbers.
        let not_ids: &[c_int] = &[15, 51, 85];
        let nand_ids: &[c_int] = &[63, 95, 119];
        let and_ids: &[c_int] = &[136, 160, 192];
        let ornot_ids: &[c_int] = &[175, 187, 207, 221, 243, 245];
        let or_ids: &[c_int] = &[238, 250, 252];
        let andnot_ids: &[c_int] = &[10, 12, 34, 48, 68, 80];
        let nor_ids: &[c_int] = &[3, 5, 17];
        let xor_ids: &[c_int] = &[60, 90, 102];
        let equ_ids: &[c_int] = &[153, 165, 195];

        for id in 0..256 {
            let id = id as c_int;
            assert_eq!(
                avd_task_eval_logic(TASK_NOT, id),
                if not_ids.contains(&id) { 1.0 } else { 0.0 },
                "NOT mismatch at logic_id={id}"
            );
            assert_eq!(
                avd_task_eval_logic(TASK_NAND, id),
                if nand_ids.contains(&id) { 1.0 } else { 0.0 },
                "NAND mismatch at logic_id={id}"
            );
            assert_eq!(
                avd_task_eval_logic(TASK_AND, id),
                if and_ids.contains(&id) { 1.0 } else { 0.0 },
                "AND mismatch at logic_id={id}"
            );
            assert_eq!(
                avd_task_eval_logic(TASK_ORNOT, id),
                if ornot_ids.contains(&id) { 1.0 } else { 0.0 },
                "ORNOT mismatch at logic_id={id}"
            );
            assert_eq!(
                avd_task_eval_logic(TASK_OR, id),
                if or_ids.contains(&id) { 1.0 } else { 0.0 },
                "OR mismatch at logic_id={id}"
            );
            assert_eq!(
                avd_task_eval_logic(TASK_ANDNOT, id),
                if andnot_ids.contains(&id) { 1.0 } else { 0.0 },
                "ANDNOT mismatch at logic_id={id}"
            );
            assert_eq!(
                avd_task_eval_logic(TASK_NOR, id),
                if nor_ids.contains(&id) { 1.0 } else { 0.0 },
                "NOR mismatch at logic_id={id}"
            );
            assert_eq!(
                avd_task_eval_logic(TASK_XOR, id),
                if xor_ids.contains(&id) { 1.0 } else { 0.0 },
                "XOR mismatch at logic_id={id}"
            );
            assert_eq!(
                avd_task_eval_logic(TASK_EQU, id),
                if equ_ids.contains(&id) { 1.0 } else { 0.0 },
                "EQU mismatch at logic_id={id}"
            );
        }
    }

    #[test]
    fn logic_id_invalid_task_type_returns_zero() {
        assert_eq!(avd_task_eval_logic(-1, 15), 0.0);
        assert_eq!(avd_task_eval_logic(9, 15), 0.0);
        assert_eq!(avd_task_eval_logic(100, 0), 0.0);
    }

    #[test]
    fn task_echo_basic() {
        let inputs: [c_int; 3] = [42, 77, 99];
        assert_eq!(avd_task_eval_echo(inputs.as_ptr(), 3, 42), 1.0);
        assert_eq!(avd_task_eval_echo(inputs.as_ptr(), 3, 77), 1.0);
        assert_eq!(avd_task_eval_echo(inputs.as_ptr(), 3, 99), 1.0);
        assert_eq!(avd_task_eval_echo(inputs.as_ptr(), 3, 100), 0.0);
    }

    #[test]
    fn task_echo_null_inputs() {
        assert_eq!(avd_task_eval_echo(std::ptr::null(), 0, 42), 0.0);
        assert_eq!(avd_task_eval_echo(std::ptr::null(), 3, 42), 0.0);
    }

    #[test]
    fn logic_id_fewer_inputs_mirroring() {
        // With 0 inputs: all inputs are 0, so logic_pos is always 0.
        // Output = 0xFFFFFFFF → bit0 = 1 consistently at pos 0.
        // Then mirror: logic_out[1]=logic_out[0]=1, [2]=[0]=1, [3]=[1]=1, [4..7]=[0..3]=1
        // All 1s → logic_id = 255
        let logic_id = avd_task_compute_logic_id(0, 0, 0, 0, -1); // -1 = 0xFFFFFFFF
        assert_eq!(logic_id, 255);

        // With 0 inputs and output = 0: all bits 0 → logic_id = 0
        let logic_id = avd_task_compute_logic_id(0, 0, 0, 0, 0);
        assert_eq!(logic_id, 0);
    }

    #[test]
    fn logic_id_two_inputs_mirroring() {
        // With 2 inputs: input2=0, so bits 4-7 mirror bits 0-3
        // Use input0 = 0x55555555 (alternating), input1 = 0x33333333
        // This should produce a valid logic_id with bit 4..7 mirrored from 0..3
        let input0 = 0x55555555_u32 as c_int;
        let input1 = 0x33333333_u32 as c_int;
        let output = input0 & input1; // AND
        let logic_id = avd_task_compute_logic_id(input0, input1, 0, 2, output);
        assert!(
            (0..=255).contains(&logic_id),
            "expected valid logic_id, got {logic_id}"
        );
        // With 2-input mirroring, bits 4-7 = bits 0-3
        assert_eq!(logic_id & 0xF0, (logic_id & 0x0F) << 4);
    }
}
