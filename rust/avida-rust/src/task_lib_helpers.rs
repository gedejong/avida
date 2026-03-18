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
}
