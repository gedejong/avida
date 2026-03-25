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

// ---------------------------------------------------------------------------
// Math3in operation codes (13 tasks)
// ---------------------------------------------------------------------------
const MATH3IN_AA: c_int = 0; // X^2+Y^2+Z^2
const MATH3IN_AB: c_int = 1; // sqrt(|X|)+sqrt(|Y|)+sqrt(|Z|)
const MATH3IN_AC: c_int = 2; // X+2Y+3Z
const MATH3IN_AD: c_int = 3; // XY^2+Z^3
const MATH3IN_AE: c_int = 4; // (X%Y)*Z
const MATH3IN_AF: c_int = 5; // (X+Y)^2+sqrt(|Y+Z|)
const MATH3IN_AG: c_int = 6; // (XY)%(YZ)
const MATH3IN_AH: c_int = 7; // X+Y+Z
const MATH3IN_AI: c_int = 8; // -X-Y-Z
const MATH3IN_AJ: c_int = 9; // (X-Y)^2+(Y-Z)^2+(Z-X)^2
const MATH3IN_AK: c_int = 10; // (X+Y)^2+(Y+Z)^2+(Z+X)^2
const MATH3IN_AL: c_int = 11; // (X-Y)^2+(X-Z)^2
const MATH3IN_AM: c_int = 12; // (X+Y)^2+(X+Z)^2 (comment says Y+Z but code uses X+Z)

/// Evaluate a three-input math expression for a single (x, y, z) triple.
///
/// Returns `Some(true)` if `output` matches, `Some(false)` if not,
/// or `None` when a guard prevents evaluation (division by zero, etc.).
fn math3in_check(x: c_int, y: c_int, z: c_int, output: c_int, op: c_int) -> Option<bool> {
    let result = match op {
        MATH3IN_AA => {
            output
                == x.wrapping_mul(x)
                    .wrapping_add(y.wrapping_mul(y))
                    .wrapping_add(z.wrapping_mul(z))
        }
        MATH3IN_AB => {
            output
                == (x.abs() as f64).sqrt() as i32
                    + (y.abs() as f64).sqrt() as i32
                    + (z.abs() as f64).sqrt() as i32
        }
        MATH3IN_AC => {
            output
                == x.wrapping_add(y.wrapping_mul(2))
                    .wrapping_add(z.wrapping_mul(3))
        }
        MATH3IN_AD => {
            output
                == x.wrapping_mul(y)
                    .wrapping_mul(y)
                    .wrapping_add(z.wrapping_mul(z).wrapping_mul(z))
        }
        MATH3IN_AE => {
            if y == 0 {
                return None;
            }
            output == x.wrapping_rem(y).wrapping_mul(z)
        }
        MATH3IN_AF => {
            let xy = x.wrapping_add(y);
            output
                == xy
                    .wrapping_mul(xy)
                    .wrapping_add((y.wrapping_add(z).abs() as f64).sqrt() as i32)
        }
        MATH3IN_AG => {
            let mod_base = y.wrapping_mul(z);
            if mod_base == 0 {
                return None;
            }
            output == x.wrapping_mul(y).wrapping_rem(mod_base)
        }
        MATH3IN_AH => output == x.wrapping_add(y).wrapping_add(z),
        MATH3IN_AI => output == 0i32.wrapping_sub(x).wrapping_sub(y).wrapping_sub(z),
        MATH3IN_AJ => {
            let xy = x.wrapping_sub(y);
            let yz = y.wrapping_sub(z);
            let zx = z.wrapping_sub(x);
            output
                == xy
                    .wrapping_mul(xy)
                    .wrapping_add(yz.wrapping_mul(yz))
                    .wrapping_add(zx.wrapping_mul(zx))
        }
        MATH3IN_AK => {
            let xy = x.wrapping_add(y);
            let yz = y.wrapping_add(z);
            let zx = z.wrapping_add(x);
            output
                == xy
                    .wrapping_mul(xy)
                    .wrapping_add(yz.wrapping_mul(yz))
                    .wrapping_add(zx.wrapping_mul(zx))
        }
        MATH3IN_AL => {
            let xy = x.wrapping_sub(y);
            let xz = x.wrapping_sub(z);
            output == xy.wrapping_mul(xy).wrapping_add(xz.wrapping_mul(xz))
        }
        MATH3IN_AM => {
            // C++ code: (input_buffer[i]+input_buffer[j])^2 + (input_buffer[i]+input_buffer[k])^2
            let xy = x.wrapping_add(y);
            let xz = x.wrapping_add(z);
            output == xy.wrapping_mul(xy).wrapping_add(xz.wrapping_mul(xz))
        }
        _ => return Some(false),
    };
    Some(result)
}

/// Evaluate a three-input math expression.
///
/// Returns 1.0 if `output` matches the expression applied to any triple
/// `(inputs[i], inputs[j], inputs[k])` where `i != j`, `j != k`, `i != k`,
/// otherwise 0.0.  Mirrors the C++ `Task_Math3in_*` family exactly.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_task_eval_math3in(
    inputs: *const c_int,
    num_inputs: c_int,
    output: c_int,
    op: c_int,
) -> f64 {
    if inputs.is_null() || num_inputs <= 0 {
        return 0.0;
    }
    let n = num_inputs as usize;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if i == j || j == k || i == k {
                    continue;
                }
                // SAFETY: inputs is non-null (checked above) and i,j,k < num_inputs.
                let (x, y, z) = unsafe { (*inputs.add(i), *inputs.add(j), *inputs.add(k)) };
                if math3in_check(x, y, z, output, op) == Some(true) {
                    return 1.0;
                }
            }
        }
    }
    0.0
}

// ---------------------------------------------------------------------------
// Simple arithmetic operation codes (11 tasks)
// ---------------------------------------------------------------------------
const ARITH_ADD: c_int = 0; // input[i]+input[j] (j<i)
const ARITH_ADD3: c_int = 1; // input[i]+input[i+1]+input[i+2]
const ARITH_SUB: c_int = 2; // input[i]-input[j] (i!=j)

/// Evaluate a simple arithmetic task (Add, Add3, Sub).
///
/// These have slightly different iteration patterns from each other, so they
/// are handled as a single dispatch.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_task_eval_simple_arith(
    inputs: *const c_int,
    num_inputs: c_int,
    output: c_int,
    op: c_int,
) -> f64 {
    if inputs.is_null() || num_inputs <= 0 {
        return 0.0;
    }
    let n = num_inputs as usize;
    match op {
        ARITH_ADD => {
            // C++: for i in 0..n, for j in 0..i: if output == input[i]+input[j]
            for i in 0..n {
                for j in 0..i {
                    // SAFETY: inputs is non-null (checked above) and i,j < num_inputs.
                    let (a, b) = unsafe { (*inputs.add(i), *inputs.add(j)) };
                    if output == a.wrapping_add(b) {
                        return 1.0;
                    }
                }
            }
        }
        ARITH_ADD3 => {
            // C++: for i in 0..(n-2): if output == input[i]+input[i+1]+input[i+2]
            if n >= 3 {
                for i in 0..n - 2 {
                    // SAFETY: inputs is non-null (checked above) and i+2 < num_inputs.
                    let (a, b, c) =
                        unsafe { (*inputs.add(i), *inputs.add(i + 1), *inputs.add(i + 2)) };
                    if output == a.wrapping_add(b).wrapping_add(c) {
                        return 1.0;
                    }
                }
            }
        }
        ARITH_SUB => {
            // C++: for i in 0..n, for j in 0..n, i!=j: if output == input[i]-input[j]
            for i in 0..n {
                for j in 0..n {
                    if i == j {
                        continue;
                    }
                    // SAFETY: inputs is non-null (checked above) and i,j < num_inputs.
                    let (a, b) = unsafe { (*inputs.add(i), *inputs.add(j)) };
                    if output == a.wrapping_sub(b) {
                        return 1.0;
                    }
                }
            }
        }
        _ => {}
    }
    0.0
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

/// Evaluate a Logic3in task: returns 1.0 if logic_id matches the target, 0.0 otherwise.
/// This handles single-target Logic3in_XX task variants with a single function.
#[no_mangle]
pub extern "C" fn avd_task_eval_logic3in(logic_id: c_int, target: c_int) -> f64 {
    if logic_id == target {
        1.0
    } else {
        0.0
    }
}

/// Evaluate a Logic3in task with 3 target values (OR condition).
/// Returns 1.0 if logic_id matches any of t1, t2, or t3.
#[no_mangle]
pub extern "C" fn avd_task_eval_logic3in_3(
    logic_id: c_int,
    t1: c_int,
    t2: c_int,
    t3: c_int,
) -> f64 {
    if logic_id == t1 || logic_id == t2 || logic_id == t3 {
        1.0
    } else {
        0.0
    }
}

/// Evaluate a Logic3in task with 4 target values (OR condition).
/// Returns 1.0 if logic_id matches any of t1, t2, t3, or t4.
#[no_mangle]
pub extern "C" fn avd_task_eval_logic3in_4(
    logic_id: c_int,
    t1: c_int,
    t2: c_int,
    t3: c_int,
    t4: c_int,
) -> f64 {
    if logic_id == t1 || logic_id == t2 || logic_id == t3 || logic_id == t4 {
        1.0
    } else {
        0.0
    }
}

/// Evaluate a Logic3in task with 6 target values (OR condition).
#[no_mangle]
pub extern "C" fn avd_task_eval_logic3in_6(
    logic_id: c_int,
    t1: c_int,
    t2: c_int,
    t3: c_int,
    t4: c_int,
    t5: c_int,
    t6: c_int,
) -> f64 {
    if logic_id == t1
        || logic_id == t2
        || logic_id == t3
        || logic_id == t4
        || logic_id == t5
        || logic_id == t6
    {
        1.0
    } else {
        0.0
    }
}

// ---------------------------------------------------------------------------
// Math1in operation codes (17 tasks)
// ---------------------------------------------------------------------------
const MATH1IN_AA: c_int = 0; // 2*x
const MATH1IN_AB: c_int = 1; // 2*x/3
const MATH1IN_AC: c_int = 2; // 5*x/4
const MATH1IN_AD: c_int = 3; // x*x
const MATH1IN_AE: c_int = 4; // x*x*x
const MATH1IN_AF: c_int = 5; // sqrt(abs(x))
const MATH1IN_AG: c_int = 6; // log(x) (x > 0)
const MATH1IN_AH: c_int = 7; // x^2 + x^3
const MATH1IN_AI: c_int = 8; // x^2 + sqrt(abs(x))
const MATH1IN_AJ: c_int = 9; // abs(x)
const MATH1IN_AK: c_int = 10; // x - 5
const MATH1IN_AL: c_int = 11; // -x (0 - x)
const MATH1IN_AM: c_int = 12; // 5*x
const MATH1IN_AN: c_int = 13; // x/4
const MATH1IN_AO: c_int = 14; // x - 6
const MATH1IN_AP: c_int = 15; // x - 7
const MATH1IN_AS: c_int = 16; // x * 3

/// Evaluate a single-input math expression.
///
/// Returns 1.0 if `output` matches the expression applied to any element of
/// `inputs[0..num_inputs]`, otherwise 0.0.  Mirrors the C++ `Task_Math1in_*`
/// family exactly, including integer truncation semantics.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_task_eval_math1in(
    inputs: *const c_int,
    num_inputs: c_int,
    output: c_int,
    op: c_int,
) -> f64 {
    if inputs.is_null() || num_inputs <= 0 {
        return 0.0;
    }
    let n = num_inputs as usize;
    for idx in 0..n {
        // SAFETY: inputs is non-null (checked above) and idx < num_inputs,
        // so the pointer arithmetic stays within the caller-provided buffer.
        let x = unsafe { *inputs.add(idx) };
        let matches = match op {
            MATH1IN_AA => output == x.wrapping_mul(2),
            MATH1IN_AB => output == x.wrapping_mul(2) / 3,
            MATH1IN_AC => output == x.wrapping_mul(5) / 4,
            MATH1IN_AD => output == x.wrapping_mul(x),
            MATH1IN_AE => output == x.wrapping_mul(x).wrapping_mul(x),
            MATH1IN_AF => output == ((x.abs() as f64).sqrt() as i32),
            MATH1IN_AG => {
                if x <= 0 {
                    false
                } else {
                    output == (x as f64).ln() as i32
                }
            }
            MATH1IN_AH => {
                output
                    == x.wrapping_mul(x)
                        .wrapping_add(x.wrapping_mul(x).wrapping_mul(x))
            }
            MATH1IN_AI => {
                output
                    == x.wrapping_mul(x)
                        .wrapping_add((x.abs() as f64).sqrt() as i32)
            }
            MATH1IN_AJ => output == x.abs(),
            MATH1IN_AK => output == x.wrapping_sub(5),
            MATH1IN_AL => output == 0i32.wrapping_sub(x),
            MATH1IN_AM => output == x.wrapping_mul(5),
            MATH1IN_AN => output == x / 4,
            MATH1IN_AO => output == x.wrapping_sub(6),
            MATH1IN_AP => output == x.wrapping_sub(7),
            MATH1IN_AS => output == x.wrapping_mul(3),
            _ => return 0.0,
        };
        if matches {
            return 1.0;
        }
    }
    0.0
}

// ---------------------------------------------------------------------------
// Math2in operation codes (26 tasks)
// ---------------------------------------------------------------------------
const MATH2IN_AA: c_int = 0; // sqrt(abs(x+y))
const MATH2IN_AB: c_int = 1; // (x+y)^2
const MATH2IN_AC: c_int = 2; // x%y
const MATH2IN_AD: c_int = 3; // 3x/2 + 5y/4
const MATH2IN_AE: c_int = 4; // abs(x-5) + abs(y-6)
const MATH2IN_AF: c_int = 5; // x*y - x/y
const MATH2IN_AG: c_int = 6; // (x-y)^2
const MATH2IN_AH: c_int = 7; // x^2 + y^2
const MATH2IN_AI: c_int = 8; // x^2 + y^3
const MATH2IN_AJ: c_int = 9; // (sqrt(abs(x)) + y) / (x - 7)
const MATH2IN_AK: c_int = 10; // log(abs(x/y))
const MATH2IN_AL: c_int = 11; // log(abs(x)) / y
const MATH2IN_AM: c_int = 12; // x / log(abs(y))
const MATH2IN_AN: c_int = 13; // x + y
const MATH2IN_AO: c_int = 14; // x - y
const MATH2IN_AP: c_int = 15; // x / y
const MATH2IN_AQ: c_int = 16; // x * y
const MATH2IN_AR: c_int = 17; // sqrt(abs(x)) + sqrt(abs(y))
const MATH2IN_AS: c_int = 18; // x + 2y
const MATH2IN_AT: c_int = 19; // x + 3y
const MATH2IN_AU: c_int = 20; // 2x + 3y
const MATH2IN_AV: c_int = 21; // x * y^2
const MATH2IN_AX: c_int = 22; // x + 3y (duplicate of AT in C++)
const MATH2IN_AY: c_int = 23; // 2x + y
const MATH2IN_AZ: c_int = 24; // 4x + 6y
const MATH2IN_AAA: c_int = 25; // 3x - 2y

/// Evaluate a two-input math expression applied to all distinct input pairs.
///
/// Returns `true` if `output` matches the op applied to `(x, y)`.
/// Mirrors the C++ guard checks for division-by-zero, overflow, and log domain.
fn math2in_check(x: c_int, y: c_int, output: c_int, op: c_int) -> Option<bool> {
    let result = match op {
        MATH2IN_AA => output == ((x.wrapping_add(y)).abs() as f64).sqrt() as i32,
        MATH2IN_AB => {
            let s = x.wrapping_add(y);
            output == s.wrapping_mul(s)
        }
        MATH2IN_AC => {
            if y == 0 {
                return None;
            }
            output == x.wrapping_rem(y)
        }
        MATH2IN_AD => output == x.wrapping_mul(3) / 2 + y.wrapping_mul(5) / 4,
        MATH2IN_AE => {
            output
                == (x.wrapping_sub(5))
                    .abs()
                    .wrapping_add((y.wrapping_sub(6)).abs())
        }
        MATH2IN_AF => {
            if y == 0 {
                return None;
            }
            // C++: if (0-INT_MAX > input_buffer[i] && input_buffer[j] == -1) continue;
            if (0i32.wrapping_sub(i32::MAX)) > x && y == -1 {
                return None;
            }
            output == x.wrapping_mul(y).wrapping_sub(x / y)
        }
        MATH2IN_AG => {
            let d = x.wrapping_sub(y);
            output == d.wrapping_mul(d)
        }
        MATH2IN_AH => output == x.wrapping_mul(x).wrapping_add(y.wrapping_mul(y)),
        MATH2IN_AI => {
            output
                == x.wrapping_mul(x)
                    .wrapping_add(y.wrapping_mul(y).wrapping_mul(y))
        }
        MATH2IN_AJ => {
            if x.wrapping_sub(7) == 0 {
                return None;
            }
            output == ((x.abs() as f64).sqrt() as i32).wrapping_add(y) / (x.wrapping_sub(7))
        }
        MATH2IN_AK => {
            if y == 0 {
                return None;
            }
            if (0i32.wrapping_sub(i32::MAX)) > x && y == -1 {
                return None;
            }
            let quot = x / y;
            if quot == 0 {
                return None;
            }
            output == (quot.abs() as f64).ln() as i32
        }
        MATH2IN_AL => {
            if y == 0 {
                return None;
            }
            output == (x.abs() as f64).ln() as i32 / y
        }
        MATH2IN_AM => {
            let log_abs_y = (y.abs() as f64).ln();
            if log_abs_y == 0.0 {
                return None;
            }
            // C++: if (0-INT_MAX > input_buffer[i] && log(...) == -1) continue;
            if (0i32.wrapping_sub(i32::MAX)) > x && log_abs_y == -1.0 {
                return None;
            }
            output == x / (log_abs_y as i32)
        }
        MATH2IN_AN => output == x.wrapping_add(y),
        MATH2IN_AO => output == x.wrapping_sub(y),
        MATH2IN_AP => {
            if y == 0 {
                return None;
            }
            if (0i32.wrapping_sub(i32::MAX)) > x && y == -1 {
                return None;
            }
            output == x / y
        }
        MATH2IN_AQ => output == x.wrapping_mul(y),
        MATH2IN_AR => {
            output == ((x.abs() as f64).sqrt() as i32).wrapping_add((y.abs() as f64).sqrt() as i32)
        }
        MATH2IN_AS => output == x.wrapping_add(y.wrapping_mul(2)),
        MATH2IN_AT => output == x.wrapping_add(y.wrapping_mul(3)),
        MATH2IN_AU => output == x.wrapping_mul(2).wrapping_add(y.wrapping_mul(3)),
        MATH2IN_AV => output == x.wrapping_mul(y).wrapping_mul(y),
        MATH2IN_AX => output == x.wrapping_add(y.wrapping_mul(3)),
        MATH2IN_AY => output == x.wrapping_mul(2).wrapping_add(y),
        MATH2IN_AZ => output == x.wrapping_mul(4).wrapping_add(y.wrapping_mul(6)),
        MATH2IN_AAA => output == x.wrapping_mul(3).wrapping_sub(y.wrapping_mul(2)),
        _ => return Some(false),
    };
    Some(result)
}

/// Evaluate a two-input math expression.
///
/// Returns 1.0 if `output` matches the expression applied to any distinct
/// pair `(inputs[i], inputs[j])` where `i != j`, otherwise 0.0.
/// Mirrors the C++ `Task_Math2in_*` family exactly.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_task_eval_math2in(
    inputs: *const c_int,
    num_inputs: c_int,
    output: c_int,
    op: c_int,
) -> f64 {
    if inputs.is_null() || num_inputs <= 0 {
        return 0.0;
    }
    let n = num_inputs as usize;
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            // SAFETY: inputs is non-null (checked above) and i,j < num_inputs,
            // so the pointer arithmetic stays within the caller-provided buffer.
            let (x, y) = unsafe { (*inputs.add(i), *inputs.add(j)) };
            if math2in_check(x, y, output, op) == Some(true) {
                return 1.0;
            }
        }
    }
    0.0
}

// ---------------------------------------------------------------------------
// FormSpatialGroup / FormSpatialGroupWithID — parametric FFI
// ---------------------------------------------------------------------------

/// Task_FormSpatialGroup: reward = max(0, 1 - ((t - g)^2 / t^2))
/// where t = ideal group size, g = actual group size.
///
/// # Safety
/// No pointers involved — pure scalar computation.
#[no_mangle]
pub extern "C" fn avd_task_eval_form_spatial_group(ideal_size: c_int, group_size: c_int) -> f64 {
    let t = f64::from(ideal_size);
    let g = f64::from(group_size);
    if t == 0.0 {
        return 0.0;
    }
    let num = (t - g) * (t - g);
    let denom = t * t;
    let reward = 1.0 - (num / denom);
    if reward < 0.0 {
        0.0
    } else {
        reward
    }
}

/// Task_FormSpatialGroupWithID: if organism's group matches desired group,
/// compute reward. If group_size < ideal_size, reward = 1.0. Otherwise
/// uses same quadratic penalty as FormSpatialGroup.
///
/// # Safety
/// No pointers involved — pure scalar computation.
#[no_mangle]
pub extern "C" fn avd_task_eval_form_spatial_group_with_id(
    ideal_size: c_int,
    group_id: c_int,
    desired_group_id: c_int,
    group_size: c_int,
) -> f64 {
    if group_id != desired_group_id {
        return 0.0;
    }
    let t = f64::from(ideal_size);
    let g = f64::from(group_size);
    if g < t {
        return 1.0;
    }
    if t == 0.0 {
        return 0.0;
    }
    let num = (t - g) * (t - g);
    let denom = t * t;
    let reward = 1.0 - (num / denom);
    if reward < 0.0 {
        0.0
    } else {
        reward
    }
}

// ---------------------------------------------------------------------------
// CommEcho / CommNot — neighbor-buffer FFI
// ---------------------------------------------------------------------------

/// Task_CommEcho: returns 1.0 if `output_value` matches any value in the
/// flattened neighbor input buffer.
///
/// # Safety
/// `neighbor_values` must point to at least `count` valid `c_int` values,
/// or be null (returns 0.0).
#[no_mangle]
pub unsafe extern "C" fn avd_task_eval_comm_echo(
    output_value: c_int,
    neighbor_values: *const c_int,
    count: c_int,
) -> f64 {
    if neighbor_values.is_null() || count <= 0 {
        return 0.0;
    }
    // SAFETY: caller guarantees neighbor_values points to at least count valid ints.
    let slice = unsafe { std::slice::from_raw_parts(neighbor_values, count as usize) };
    for &val in slice {
        if output_value == val {
            return 1.0;
        }
    }
    0.0
}

/// Task_CommNot: returns 1.0 if `output_value` matches `-(val + 1)` for any
/// value in the flattened neighbor input buffer (bitwise NOT for two's complement).
///
/// # Safety
/// `neighbor_values` must point to at least `count` valid `c_int` values,
/// or be null (returns 0.0).
#[no_mangle]
pub unsafe extern "C" fn avd_task_eval_comm_not(
    output_value: c_int,
    neighbor_values: *const c_int,
    count: c_int,
) -> f64 {
    if neighbor_values.is_null() || count <= 0 {
        return 0.0;
    }
    // SAFETY: caller guarantees neighbor_values points to at least count valid ints.
    let slice = unsafe { std::slice::from_raw_parts(neighbor_values, count as usize) };
    for &val in slice {
        if output_value == (0i32.wrapping_sub(val.wrapping_add(1))) {
            return 1.0;
        }
    }
    0.0
}

// ---------------------------------------------------------------------------
// Nand/Nor_ResourceDependent — parametric FFI
// ---------------------------------------------------------------------------

/// Task_Nand_ResourceDependent: returns 1.0 if logic_id is 63, 95, or 119
/// AND pheromone_amount < crossover_level (100.0).
#[no_mangle]
pub extern "C" fn avd_task_eval_nand_res_dep(logic_id: c_int, pheromone_amount: f64) -> f64 {
    const CROSSOVER: f64 = 100.0;
    if !(logic_id == 63 || logic_id == 95 || logic_id == 119) {
        return 0.0;
    }
    if pheromone_amount < CROSSOVER {
        1.0
    } else {
        0.0
    }
}

/// Task_Nor_ResourceDependent: returns 1.0 if logic_id is 3, 5, or 17
/// AND pheromone_amount > crossover_level (100.0).
#[no_mangle]
pub extern "C" fn avd_task_eval_nor_res_dep(logic_id: c_int, pheromone_amount: f64) -> f64 {
    const CROSSOVER: f64 = 100.0;
    if !(logic_id == 3 || logic_id == 5 || logic_id == 17) {
        return 0.0;
    }
    if pheromone_amount > CROSSOVER {
        1.0
    } else {
        0.0
    }
}

// ---------------------------------------------------------------------------
// MoveFT — parametric FFI
// ---------------------------------------------------------------------------

/// Task_MoveFT: returns 1.0 if organism moved (cell_id != prev_seen) AND
/// forage_target matches desired_target.
#[no_mangle]
pub extern "C" fn avd_task_eval_move_ft(
    effective_cell_id: c_int,
    prev_seen_cell_id: c_int,
    forage_target: c_int,
    desired_target: c_int,
) -> f64 {
    if effective_cell_id == prev_seen_cell_id {
        return 0.0;
    }
    if forage_target == desired_target {
        1.0
    } else {
        0.0
    }
}

// ---------------------------------------------------------------------------
// AllOnes — buffer-pointer FFI (variable-length output buffer)
// ---------------------------------------------------------------------------

/// Task_AllOnes: average of the first `length` output buffer values.
///
/// # Safety
/// `buf` must point to at least `buf_len` valid `c_int` values, or be null.
#[no_mangle]
pub unsafe extern "C" fn avd_task_eval_all_ones(
    buf: *const c_int,
    buf_len: c_int,
    length: c_int,
) -> f64 {
    if buf.is_null() || buf_len <= 0 || length <= 0 {
        return 0.0;
    }
    // SAFETY: caller guarantees buf points to at least buf_len valid ints.
    let slice = unsafe { std::slice::from_raw_parts(buf, buf_len as usize) };
    let mut sum: f64 = 0.0;
    for i in 0..length as usize {
        if i < slice.len() {
            sum += f64::from(slice[i]);
        }
    }
    sum / f64::from(length)
}

// ---------------------------------------------------------------------------
// Movement tasks — parametric FFI
// ---------------------------------------------------------------------------

/// Task_MoveToRightSide/MoveToLeftSide: reward if cell x-position matches edge.
/// `target_x` is 0 for left side, `world_x - 1` for right side.
#[no_mangle]
pub extern "C" fn avd_task_eval_move_to_side(cell_pos_x: c_int, target_x: c_int) -> f64 {
    if cell_pos_x == target_x {
        1.0
    } else {
        0.0
    }
}

/// Task_Move: returns 1.0 if organism moved (cell_id != prev_seen_cell_id).
#[no_mangle]
pub extern "C" fn avd_task_eval_move(effective_cell_id: c_int, prev_seen_cell_id: c_int) -> f64 {
    if effective_cell_id != prev_seen_cell_id {
        1.0
    } else {
        0.0
    }
}

/// Task_MoveToTarget: returns 1.0 if on a target cell (cell_data > 1) and
/// not the same as previously visited target cell.
/// Returns 0.0 if cell_data <= 0, cell_data == 1, or same target.
#[no_mangle]
pub extern "C" fn avd_task_eval_move_to_target(
    cell_data: c_int,
    current_cell: c_int,
    prev_target: c_int,
) -> f64 {
    if cell_data <= 0 {
        return 0.0;
    }
    if cell_data > 1 {
        if current_cell == prev_target {
            0.0
        } else {
            1.0
        }
    } else {
        0.0
    }
}

// ---------------------------------------------------------------------------
// RoyalRoad / RoyalRoadWithDitches — buffer-pointer FFI
// ---------------------------------------------------------------------------

/// Task_RoyalRoad: divide output buffer into `block_count` blocks of size
/// `floor(length / block_count)`. A block is "correct" if ALL its values are
/// non-zero (bitwise AND). Returns fraction of correct blocks.
///
/// # Safety
/// `buf` must point to at least `buf_len` valid `c_int` values, or be null
/// (in which case 0.0 is returned).
#[no_mangle]
pub unsafe extern "C" fn avd_task_eval_royal_road(
    buf: *const c_int,
    buf_len: c_int,
    length: c_int,
    block_count: c_int,
) -> f64 {
    if buf.is_null() || buf_len <= 0 || length <= 0 || block_count <= 0 {
        return 0.0;
    }
    let block_size = (length as f64 / block_count as f64).floor() as usize;
    if block_size == 0 {
        return 0.0;
    }
    // SAFETY: caller guarantees buf points to at least buf_len valid ints.
    let slice = unsafe { std::slice::from_raw_parts(buf, buf_len as usize) };
    let mut total_reward = 0.0;

    for i in 0..block_count as usize {
        let mut block_reward: c_int = 1;
        for j in 0..block_size {
            let idx = i * block_size + j;
            let val = if idx < slice.len() { slice[idx] } else { 0 };
            block_reward &= val;
        }
        if block_reward != 0 {
            total_reward += 1.0;
        }
    }

    total_reward / block_count as f64
}

/// Task_RoyalRoadWithDitches: state-machine variant of RoyalRoad.
///
/// Block types:
/// - A: all values non-zero (like basic RoyalRoad block)
/// - B: first `block_size - width` values non-zero, last `width` values zero
/// - X: anything else
///
/// State machine (starts in case 1):
/// - Case 1: X → case 2; A → reward = num_b_blocks + 2, case 3; B → num_b_blocks++
/// - Case 2: A → reward = num_b_blocks + 2 - height, case 3; else → case 3
/// - Case 3: terminal (no more reward changes)
///
/// # Safety
/// `buf` must point to at least `buf_len` valid `c_int` values, or be null.
#[no_mangle]
pub unsafe extern "C" fn avd_task_eval_royal_road_wd(
    buf: *const c_int,
    buf_len: c_int,
    length: c_int,
    block_count: c_int,
    width: c_int,
    height: c_int,
) -> f64 {
    if buf.is_null() || buf_len <= 0 || length <= 0 || block_count <= 0 {
        return 0.0;
    }
    let block_size = (length as f64 / block_count as f64).floor() as usize;
    if block_size == 0 {
        return 0.0;
    }
    // SAFETY: caller guarantees buf points to at least buf_len valid ints.
    let slice = unsafe { std::slice::from_raw_parts(buf, buf_len as usize) };
    let width = width as usize;

    let mut total_reward: f64 = 0.0;
    let mut num_b_blocks: i32 = 0;
    let mut next_case: i32 = 1;

    for i in 0..block_count as usize {
        // Determine block type: A, B, or X
        let mut block_type: i32 = -1; // -1 undefined, 0 X, 1 A, 2 B

        // Check for block A (all non-zero)
        let mut block_correct: c_int = 1;
        for j in 0..block_size {
            let idx = i * block_size + j;
            let val = if idx < slice.len() { slice[idx] } else { 0 };
            block_correct &= val;
        }
        if block_correct != 0 {
            block_type = 1; // A
        }

        // Check for block B if not A
        if block_type == -1 {
            block_correct = 1;
            for j in 0..block_size {
                let idx = i * block_size + j;
                let val = if idx < slice.len() { slice[idx] } else { 0 };
                if j < block_size.saturating_sub(width) {
                    if val == 0 {
                        block_correct = 0;
                    }
                } else if val == 1 {
                    block_correct = 0;
                }
                if block_correct == 0 {
                    break;
                }
            }
            if block_correct != 0 {
                block_type = 2; // B
            }
        }

        // Default to X
        if block_type == -1 {
            block_type = 0;
        }

        // State machine
        match next_case {
            1 => {
                if block_type == 0 {
                    next_case = 2;
                } else if block_type == 1 {
                    total_reward = (num_b_blocks + 2) as f64;
                    next_case = 3;
                } else if block_type == 2 {
                    num_b_blocks += 1;
                }
            }
            2 => {
                if block_type == 1 {
                    total_reward = (num_b_blocks + 2 - height) as f64;
                }
                next_case = 3;
            }
            _ => {} // case 3+: terminal
        }
    }

    total_reward / block_count as f64
}

// ---------------------------------------------------------------------------
// Task_AIDisplayCost — simple lyse display check
// ---------------------------------------------------------------------------

/// Task_AIDisplayCost: returns 1.0 if lyse_display is set, 0.0 otherwise.
#[no_mangle]
pub extern "C" fn avd_task_eval_ai_display_cost(lyse_display: c_int) -> f64 {
    if lyse_display != 0 {
        1.0
    } else {
        0.0
    }
}

// ---------------------------------------------------------------------------
// Task_MoveToEvent — match cell_data against deme events
// ---------------------------------------------------------------------------

/// Task_MoveToEvent: return 1.0 if cell_data matches any deme event ID.
///
/// `event_ids`: pointer to array of event IDs from deme.
/// `num_events`: number of events in the array.
/// `cell_data`: organism's cell data value.
///
/// # Safety
/// `event_ids` must point to at least `num_events` valid `c_int` values, or be null.
#[no_mangle]
pub unsafe extern "C" fn avd_task_eval_move_to_event(
    event_ids: *const c_int,
    num_events: c_int,
    cell_data: c_int,
) -> f64 {
    if cell_data <= 0 {
        return 0.0;
    }
    if event_ids.is_null() || num_events <= 0 {
        return 0.0;
    }
    // SAFETY: caller guarantees event_ids points to at least num_events valid ints.
    let ids = unsafe { std::slice::from_raw_parts(event_ids, num_events as usize) };
    for &id in ids {
        if id == cell_data {
            return 1.0;
        }
    }
    0.0
}

// ---------------------------------------------------------------------------
// FibonacciSequence — stateful task evaluator
// ---------------------------------------------------------------------------

/// Task_FibonacciSequence: evaluate if output matches next Fibonacci number.
///
/// State: `seq[2]` (last two values), `count` (matches so far).
/// Returns 1.0 on match, `penalty` if past target, 0.0 on miss.
///
/// # Safety
/// `seq` must point to at least 2 `c_int` values. `count` must be valid.
#[no_mangle]
pub unsafe extern "C" fn avd_task_eval_fibonacci_seq(
    seq: *mut c_int,
    count: *mut c_int,
    output_value: c_int,
    target_count: c_int,
    penalty: f64,
) -> f64 {
    if seq.is_null() || count.is_null() {
        return 0.0;
    }
    // SAFETY: caller guarantees seq points to 2 valid ints, count is valid.
    let (s, cnt) = unsafe { (std::slice::from_raw_parts_mut(seq, 2), &mut *count) };
    let next = s[0] + s[1];

    if output_value != next {
        return 0.0;
    }

    // Match found — advance state
    *cnt += 1;
    s[(*cnt as usize) % 2] = next;

    if *cnt > target_count {
        penalty
    } else {
        1.0
    }
}

// ---------------------------------------------------------------------------
// Task_MatchStr — string matching on output buffer bit patterns
// ---------------------------------------------------------------------------

/// Match a single integer value against a binary string pattern.
/// Compares bits of `test_output` against characters in `pattern` (right-to-left).
/// Returns number of matching bit positions.
fn match_str_bitwise(test_output: i32, pattern: &[u8]) -> i32 {
    let mut num_matched: i32 = 0;
    for (j, _) in pattern.iter().enumerate() {
        let string_index = pattern.len() - j - 1;
        let k = 1i32 << j;
        let ch = pattern[string_index];
        if (ch == b'0' && (test_output & k) == 0) || (ch == b'1' && (test_output & k) != 0) {
            num_matched += 1;
        }
    }
    num_matched
}

/// Task_MatchStr: compare output buffer against a binary string pattern.
///
/// Two modes:
/// - `binary == 0`: bitwise comparison of first output value against pattern
/// - `binary != 0`: element-wise comparison of output buffer against pattern (0/1/'9' wildcard)
///
/// Also checks received messages for better bitwise matches (non-binary mode only).
///
/// Returns bonus = pow(base_bonus, 2) where base_bonus = max_matched*2/len - 1 (or partial variant).
///
/// # Safety
/// - `output_buf` must point to `output_len` valid `c_int` values (or be null).
/// - `received_buf` must point to `received_len` valid `c_int` values (or be null).
/// - `pattern` must point to `pattern_len` valid bytes.
#[no_mangle]
pub unsafe extern "C" fn avd_task_eval_match_str(
    output_buf: *const c_int,
    output_len: c_int,
    received_buf: *const c_int,
    received_len: c_int,
    pattern: *const u8,
    pattern_len: c_int,
    partial: c_int,
    binary: c_int,
) -> f64 {
    if pattern.is_null() || pattern_len <= 0 {
        return 0.0;
    }
    // SAFETY: caller guarantees pattern points to pattern_len valid bytes.
    let pat = unsafe { std::slice::from_raw_parts(pattern, pattern_len as usize) };

    let mut max_num_matched: i32 = 0;
    let mut num_real: i32 = 0;

    if binary == 0 {
        // Non-binary mode: bitwise comparison of first output value
        if !output_buf.is_null() && output_len > 0 {
            // SAFETY: caller guarantees output_buf points to at least 1 valid int.
            let test_output = unsafe { *output_buf };
            max_num_matched = match_str_bitwise(test_output, pat);
        }
    } else {
        // Binary mode: element-wise comparison
        if !output_buf.is_null() {
            // SAFETY: caller guarantees output_buf points to output_len valid ints.
            let out = unsafe { std::slice::from_raw_parts(output_buf, output_len.max(0) as usize) };
            for (j, &p) in pat.iter().enumerate() {
                if p != b'9' {
                    num_real += 1;
                }
                if j < out.len() && ((p == b'0' && out[j] == 0) || (p == b'1' && out[j] == 1)) {
                    max_num_matched += 1;
                }
            }
        }
    }

    // Check received messages for better bitwise matches (non-binary mode only)
    if binary == 0 && !received_buf.is_null() && received_len > 0 {
        // SAFETY: caller guarantees received_buf points to received_len valid ints.
        let received = unsafe { std::slice::from_raw_parts(received_buf, received_len as usize) };
        for &msg in received {
            let num_matched = match_str_bitwise(msg, pat);
            if num_matched > max_num_matched {
                max_num_matched = num_matched;
            }
        }
    }

    // Compute bonus
    let str_len = pat.len() as f64;
    let base_bonus = if partial != 0 && num_real > 0 {
        (max_num_matched as f64) * 2.0 / (num_real as f64) - 1.0
    } else {
        (max_num_matched as f64) * 2.0 / str_len - 1.0
    };

    if base_bonus > 0.0 {
        base_bonus * base_bonus // pow(base_bonus, 2)
    } else {
        0.0
    }
}

// ---------------------------------------------------------------------------
// Task_Optimize — mathematical optimization with 23 function variants
// ---------------------------------------------------------------------------

/// Result from Task_Optimize containing both the quality score and the Fx value.
/// C++ needs Fx to call SetTaskValue().
#[repr(C)]
pub struct OptimizeResult {
    pub quality: f64,
    pub fx: f64,
}

/// Evaluate one of 23 optimization functions on the output buffer.
///
/// Arguments (matching C++ cArgContainer indices):
/// - `function`: int arg 0 — which function to evaluate (1-23, 18-19, 20-22)
/// - `binary`: int arg 1 — if nonzero, decode output buffer as binary variables
/// - `varlength`: int arg 2 — bit length per variable in binary mode
/// - `numvars`: int arg 3 — number of variables
/// - `basepow`: double arg 0 — base for binary decoding
/// - `max_fx`: double arg 1 — maximum Fx for quality scaling
/// - `min_fx`: double arg 2 — minimum Fx for quality scaling
/// - `thresh`: double arg 3 — threshold (< 0 means proportional; >= 0 means binary pass/fail)
/// - `thresh_max`: double arg 4 — upper threshold (< 0 means single threshold mode)
/// - `string_to_match`: pointer to pattern bytes (only for function 20)
/// - `string_len`: length of pattern (only for function 20)
///
/// # Safety
/// - `output_buf` must point to `output_len` valid `c_int` values.
/// - `string_to_match` must point to `string_len` valid bytes when function==20 (or be null).
#[no_mangle]
pub unsafe extern "C" fn avd_task_eval_optimize(
    output_buf: *const c_int,
    output_len: c_int,
    output_capacity: c_int,
    function: c_int,
    binary: c_int,
    varlength: c_int,
    numvars: c_int,
    basepow: f64,
    max_fx: f64,
    min_fx: f64,
    thresh: f64,
    thresh_max: f64,
    string_to_match: *const u8,
    string_len: c_int,
) -> OptimizeResult {
    let zero = OptimizeResult {
        quality: 0.0,
        fx: 0.0,
    };

    if output_buf.is_null() || output_len <= 0 {
        return zero;
    }

    // if the org hasn't output enough numbers, return 0
    if output_len < output_capacity {
        return zero;
    }

    // SAFETY: caller guarantees output_buf points to output_len valid ints.
    let out = unsafe { std::slice::from_raw_parts(output_buf, output_len as usize) };

    let nv = numvars as usize;
    let vl = varlength as usize;

    // C++ uses literal 3.14159 (not std::f64::consts::PI), so we match exactly.
    #[allow(clippy::approx_constant)]
    let pi_approx: f64 = 3.14159;

    let fx = match function {
        20 => {
            // String match: Fx = varlength - num_matched (0 best, varlength worst)
            if string_to_match.is_null() || string_len <= 0 {
                return zero;
            }
            // SAFETY: caller guarantees string_to_match points to string_len valid bytes.
            let pat = unsafe { std::slice::from_raw_parts(string_to_match, string_len as usize) };
            let mut matched = 0i32;
            let limit = vl.min(out.len()).min(pat.len());
            for i in 0..limit {
                if (pat[i] == b'0' && out[i] == 0) || (pat[i] == b'1' && out[i] == 1) {
                    matched += 1;
                }
            }
            (varlength - matched) as f64
        }
        21 => {
            // String with all 1's at beginning until pattern 0101
            let limit = vl.saturating_sub(3);
            let mut num_ones = 0i32;
            let mut pat_found = false;
            for i in 0..limit {
                if i < out.len() && out[i] == 1 {
                    num_ones += 1;
                } else if i < out.len() {
                    // Check 0-1-0-1 pattern
                    if i + 3 < out.len() && out[i + 1] == 1 && out[i + 2] == 0 && out[i + 3] == 1 {
                        pat_found = true;
                    }
                    break;
                }
            }
            if pat_found {
                (varlength - 4 - num_ones) as f64
            } else {
                varlength as f64
            }
        }
        22 => {
            // Count leading 1's: Fx = varlength - numOnes
            let mut num_ones = 0i32;
            for &val in out.iter().take(vl) {
                if val == 1 {
                    num_ones += 1;
                } else {
                    break;
                }
            }
            (varlength - num_ones) as f64
        }
        23 => {
            // Count leading 0's: Fx = varlength - numZeros
            let mut num_zeros = 0i32;
            for &val in out.iter().take(vl) {
                if val == 0 {
                    num_zeros += 1;
                } else {
                    break;
                }
            }
            (varlength - num_zeros) as f64
        }
        18 => {
            // Sum first 30 outputs + 1
            let mut tot = 0i32;
            for &val in out.iter().take(30) {
                tot += val;
            }
            (1 + tot) as f64
        }
        19 => {
            // Complex multi-variable function with g(x)
            let mut temp_vars = vec![0.0f64; nv];
            for &val in out.iter().take(30) {
                temp_vars[0] += val as f64;
            }
            let len = vl;
            for i in (0..len).rev() {
                for j in 1..nv {
                    let idx = 30 + i + len * (nv - j - 1);
                    if idx < out.len() {
                        temp_vars[j - 1] += out[idx] as f64;
                    }
                }
            }
            let mut gx = 0i32;
            for tv in temp_vars.iter().take(nv).skip(1) {
                if *tv == 5.0 {
                    gx += 1;
                } else {
                    gx += *tv as i32 + 2;
                }
            }
            (gx as f64) * (1.0 / (1.0 + temp_vars[0]))
        }
        _ => {
            // Standard continuous optimization functions (1-17)
            // Decode variables from output buffer
            let mut vars = vec![0.0f64; nv];
            if binary != 0 {
                let mut temp_vars = vec![0.0f64; nv];
                let mut tot = 0.0f64;
                let len = vl;
                #[allow(clippy::needless_range_loop)]
                for i in (0..len).rev() {
                    for j in 0..nv {
                        let idx = i + len * (nv - j - 1);
                        if idx < out.len() {
                            temp_vars[j] +=
                                (out[idx] as f64) * basepow.powf(((len - 1) - i) as f64);
                        }
                    }
                    tot += basepow.powf(i as f64);
                }
                if tot != 0.0 {
                    for (v, tv) in vars.iter_mut().zip(temp_vars.iter()) {
                        *v = *tv / tot;
                    }
                }
            } else {
                for (v, &o) in vars.iter_mut().zip(out.iter()) {
                    *v = (o as f64) / (0xFFFF_FFFFu32 as f64);
                }
            }

            // Clamp to [0, 1]
            for v in &mut vars {
                *v = v.clamp(0.0, 1.0);
            }

            match function {
                1 => vars[0], // F1
                2 => (1.0 + vars[1]) * (1.0 - (vars[0] / (1.0 + vars[1])).sqrt()),
                3 => (1.0 + vars[1]) * (1.0 - (vars[0] / (1.0 + vars[1])).powi(2)),
                4 => {
                    (1.0 + vars[1])
                        * (1.0
                            - (vars[0] / (1.0 + vars[1])).sqrt()
                            - (vars[0] / (1.0 + vars[1])) * (pi_approx * vars[0] * 10.0).sin())
                }
                5 => {
                    let x = vars[0] * -2.0;
                    x * x + vars[1] * vars[1]
                }
                6 => {
                    let x = vars[0] * -2.0;
                    (x + 2.0) * (x + 2.0) + vars[1] * vars[1]
                }
                7 => {
                    let x = vars[0] * 4.0;
                    x.sqrt() + vars[1]
                }
                8 => {
                    let x = vars[0] * 4.0;
                    (4.0 - x).sqrt() + vars[1]
                }
                9 => {
                    let denom = (nv as f64) - 1.0;
                    let sum = if denom > 0.0 {
                        vars.iter().skip(1).take(nv - 1).sum::<f64>() / denom
                    } else {
                        0.0
                    };
                    let gx = 1.0 + 9.0 * sum;
                    gx * (1.0 - (vars[0] / gx).sqrt())
                }
                10 => {
                    let denom = (nv as f64) - 1.0;
                    let sum = if denom > 0.0 {
                        vars.iter().skip(1).take(nv - 1).sum::<f64>() / denom
                    } else {
                        0.0
                    };
                    let gx = 1.0 + 9.0 * sum;
                    gx * (1.0 - (vars[0] / gx).powi(2))
                }
                11 => {
                    let denom = (nv as f64) - 1.0;
                    let sum = if denom > 0.0 {
                        vars.iter().skip(1).take(nv - 1).sum::<f64>() / denom
                    } else {
                        0.0
                    };
                    let gx = 1.0 + 9.0 * sum;
                    gx * (1.0
                        - (vars[0] / gx).sqrt()
                        - (vars[0] / gx) * (pi_approx * vars[0] * 10.0).sin())
                }
                12 => vars[0] * 0.9 + 0.1,
                13 => {
                    let x = vars[0] * 0.9 + 0.1;
                    let y = vars[1] * 5.0;
                    (1.0 + y) / x
                }
                14 => {
                    let x = vars[0] * 6.0 - 3.0;
                    let y = vars[1] * 6.0 - 3.0;
                    0.5 * (x * x + y * y) + (x * x + y * y).sin()
                }
                15 => {
                    let x = vars[0] * 6.0 - 3.0;
                    let y = vars[1] * 6.0 - 3.0;
                    (3.0 * x - 2.0 * y + 4.0).powi(2) / 8.0 + (x - y + 1.0).powi(2) / 27.0 + 15.0
                }
                16 => {
                    let x = vars[0] * 6.0 - 3.0;
                    let y = vars[1] * 6.0 - 3.0;
                    1.0 / (x * x + y * y + 1.0) - 1.1 * (-(x * x) - y * y).exp()
                }
                17 => {
                    let mut sum = 0.0f64;
                    for &vi_raw in vars.iter().skip(1).take(nv - 1) {
                        let vi = vi_raw * 6.0 - 3.0;
                        #[allow(clippy::approx_constant)]
                        let four_pi = 4.0 * 3.14159;
                        sum += (vi * vi - 10.0 * (four_pi * vi).cos()) / 10.0;
                    }
                    let gx = 10.0 + sum;
                    gx * (1.0 - (vars[0] / gx).sqrt())
                }
                _ => {
                    // Unknown function: return minimal quality
                    return OptimizeResult {
                        quality: 0.001,
                        fx: 0.0,
                    };
                }
            }
        }
    };

    // Compute quality from Fx
    let quality = if thresh < 0.0 {
        let q1 = max_fx - fx + 0.001;
        let q2 = max_fx - min_fx + 0.001;
        // C++ asserts q1 > 0 and q2 > 0 but we guard defensively
        if q1 <= 0.0 || q2 <= 0.0 {
            0.001
        } else {
            q1 / q2
        }
    } else if thresh_max < 0.0 {
        // Single threshold mode
        if fx <= (max_fx - min_fx) * thresh + min_fx {
            1.0
        } else {
            0.0
        }
    } else {
        // Dual threshold (band) mode
        let val = fx;
        let low = (max_fx - min_fx) * thresh + min_fx;
        let high = (max_fx - min_fx) * thresh_max + min_fx;
        if val >= low && val <= high {
            1.0
        } else {
            0.0
        }
    };

    let final_quality = if quality > 1.0 {
        quality // C++ prints warning but doesn't cap
    } else if quality < 0.001 {
        0.001
    } else {
        quality
    };

    OptimizeResult {
        quality: final_quality,
        fx,
    }
}

// ---------------------------------------------------------------------------
// Task_SortInputs — validate sorted output of organism inputs
// ---------------------------------------------------------------------------

/// Task_SortInputs: validate that organism output is a sorted version of its inputs.
///
/// Uses insertion sort scoring to measure how sorted the output is.
/// Returns quality based on halflife scoring when < 50% of max moves needed.
///
/// # Safety
/// - `output_buf` must point to `output_len` valid `c_int` values.
/// - `input_buf` must point to `input_len` valid `c_int` values (organism inputs).
#[no_mangle]
pub unsafe extern "C" fn avd_task_eval_sort_inputs(
    output_buf: *const c_int,
    output_len: c_int,
    input_buf: *const c_int,
    input_len: c_int,
    size: c_int,
    direction: c_int,
    contiguous: c_int,
    halflife: f64,
) -> f64 {
    if output_buf.is_null() || input_buf.is_null() || output_len <= 0 || input_len <= 0 {
        return 0.0;
    }

    let stored = output_len as usize;
    let sz = size as usize;

    // if less than half, can't possibly reach threshold
    if stored <= sz / 2 {
        return 0.0;
    }

    // SAFETY: caller guarantees output_buf points to `stored` valid ints.
    let out = unsafe { std::slice::from_raw_parts(output_buf, stored) };
    // SAFETY: caller guarantees input_buf points to `input_len` valid ints.
    let inp = unsafe { std::slice::from_raw_parts(input_buf, input_len as usize) };

    // Build value map: valid inputs map to -1 (unseen)
    use std::collections::HashMap;
    let mut valmap: HashMap<i32, i32> = HashMap::new();
    for i in 0..sz {
        if i < inp.len() {
            valmap.insert(inp[i], -1);
        }
    }

    let span_start: isize;
    let span_end: usize;

    if contiguous != 0 {
        // Scan for the largest contiguous span of valid inputs
        let mut best_start: isize = -1;
        let mut best_end: usize = 0;
        let mut i = 0usize;
        while i < stored {
            if valmap.contains_key(&out[i]) {
                let t_start = i;
                i += 1;
                while i < stored && valmap.contains_key(&out[i]) {
                    i += 1;
                }
                if best_start == -1 || (i - t_start) > (best_end - best_start as usize) {
                    best_start = t_start as isize;
                    best_end = i;
                }
            } else {
                i += 1;
            }
        }

        if best_start == -1 {
            return 0.0;
        }
        span_start = best_start;
        span_end = best_end;
    } else {
        // Scattered mode: exact reproduction of C++ logic:
        //   int span_start = -1;
        //   while (++span_start < stored && valmap.Has(output[span_start])) ;
        // This finds the first output position that is NOT a valid input.
        // If all outputs are valid inputs, span_start == stored -> return 0.0.
        let mut ss: isize = -1;
        loop {
            ss += 1;
            if ss as usize >= stored {
                break;
            }
            if !valmap.contains_key(&out[ss as usize]) {
                break;
            }
        }

        if ss as usize >= stored {
            return 0.0;
        }
        span_start = ss;
        span_end = stored;
    }

    // Again, if span is less than half the size can't possibly reach threshold
    if (span_end as isize - span_start) as usize <= sz / 2 {
        return 0.0;
    }

    // Insertion sort scoring
    let ascending = direction >= 0;
    let mut sorted_arr = vec![0i32; sz];
    let mut count: usize = 1;
    let mut score: usize = 0;
    let mut maxscore: usize = 0;

    // Store first value
    let first_idx = span_start as usize;
    valmap.insert(out[first_idx], first_idx as i32);
    sorted_arr[0] = out[first_idx];

    // Iterate over the remaining span
    #[allow(clippy::needless_range_loop)]
    for i in (first_idx + 1)..span_end {
        let value = out[i];

        // Check for dup or invalid output
        match valmap.get(&value) {
            Some(-1) => {} // valid unseen input, proceed
            _ => continue, // dup or not in inputs, skip
        }

        maxscore += count;
        count += 1;
        valmap.insert(value, i as i32);

        // Insertion sort, counting moves
        let mut j = count as isize - 2;
        while j >= 0
            && ((ascending && sorted_arr[j as usize] > value)
                || (!ascending && sorted_arr[j as usize] < value))
        {
            sorted_arr[(j + 1) as usize] = sorted_arr[j as usize];
            j -= 1;
            score += 1;
        }
        sorted_arr[(j + 1) as usize] = value;
    }

    // If not all inputs were observed, penalize missing ones
    if count < sz {
        for input_val in inp.iter().take(sz.min(inp.len())) {
            if let Some(&idx) = valmap.get(input_val) {
                if idx == -1 {
                    maxscore += count;
                    score += count;
                    count += 1;
                }
            }
        }
    }

    if maxscore == 0 {
        return 0.0;
    }

    // Score of 50% expected with random output
    let ratio = score as f64 / maxscore as f64;
    if ratio < 0.5 {
        let hl = -halflife.abs();
        2.0f64.powf(score as f64 / hl)
    } else {
        0.0
    }
}

// ---------------------------------------------------------------------------
// Task_MatchProdStr — pure string-matching core
// ---------------------------------------------------------------------------

/// Compute the number of matching bits between a target string and an output value.
///
/// In non-binary mode (`binary == 0`), compares bits of `test_output` against the
/// string characters ('0'/'1') from the least-significant bit upward.
///
/// In binary mode (`binary != 0`), compares element-wise: `string[j]` vs
/// `output_buf[j]`, where '0' matches 0 and '1' matches 1 (characters '9'
/// are wildcards that don't count toward `num_real`).
///
/// Returns `(max_num_matched, num_real)` packed as `max_num_matched * 1000 + num_real`.
///
/// # Safety
/// - `string_ptr` must point to `string_len` valid bytes (the match-target string).
/// - `output_buf` must point to `output_len` valid `c_int` values.
#[no_mangle]
pub unsafe extern "C" fn avd_task_match_prod_str_core(
    string_ptr: *const u8,
    string_len: c_int,
    output_buf: *const c_int,
    output_len: c_int,
    binary: c_int,
) -> i64 {
    if string_ptr.is_null() || string_len <= 0 {
        return 0;
    }

    let slen = string_len as usize;
    // SAFETY: caller guarantees string_ptr is valid for slen bytes.
    let string_bytes = unsafe { std::slice::from_raw_parts(string_ptr, slen) };

    let mut num_matched: i32 = 0;
    let mut num_real: i32 = 0;

    if binary == 0 {
        // Non-binary mode: compare bits of first output value
        let test_output = if !output_buf.is_null() && output_len > 0 {
            // SAFETY: we just checked output_buf is non-null and output_len > 0.
            unsafe { *output_buf }
        } else {
            return 0;
        };

        for j in 0..slen {
            let string_index = slen - j - 1; // start with last char
            let k: i32 = 1 << j;
            let ch = string_bytes[string_index];
            if (ch == b'0' && (test_output & k) == 0) || (ch == b'1' && (test_output & k) != 0) {
                num_matched += 1;
            }
        }
        num_real = slen as i32;
    } else {
        // Binary mode: element-wise comparison
        if output_buf.is_null() || output_len <= 0 {
            return 0;
        }
        // SAFETY: we just checked output_buf is non-null and output_len > 0.
        let out = unsafe { std::slice::from_raw_parts(output_buf, output_len as usize) };

        for j in 0..slen {
            let ch = string_bytes[j];
            if ch != b'9' {
                num_real += 1;
            }
            let buf_val = if j < out.len() { out[j] } else { 0 };
            if (ch == b'0' && buf_val == 0) || (ch == b'1' && buf_val == 1) {
                num_matched += 1;
            }
        }
    }

    // Pack both values: matched * 1000 + num_real
    i64::from(num_matched) * 1000 + i64::from(num_real)
}

/// Compute the bonus for Task_MatchProdStr given match results.
///
/// `max_num_matched`: how many bits matched (possibly overridden to string_len if already produced).
/// `string_len`: length of the target string.
/// `num_real`: count of non-wildcard characters (only used when `partial != 0`).
/// `partial`: if nonzero, use num_real instead of string_len for base_bonus.
/// `mypow`: exponent for the bonus calculation.
#[no_mangle]
pub extern "C" fn avd_task_match_prod_str_bonus(
    max_num_matched: c_int,
    string_len: c_int,
    num_real: c_int,
    partial: c_int,
    mypow: f64,
) -> f64 {
    let matched = f64::from(max_num_matched);
    let base_bonus = if partial != 0 {
        if num_real == 0 {
            return 0.0;
        }
        matched * 2.0 / f64::from(num_real) - 1.0
    } else {
        if string_len == 0 {
            return 0.0;
        }
        matched * 2.0 / f64::from(string_len) - 1.0
    };

    if base_bonus > 0.0 {
        base_bonus.powf(mypow)
    } else {
        0.0
    }
}

// ---------------------------------------------------------------------------
// Task_SGPathTraversal — pure traversal quality computation
// ---------------------------------------------------------------------------

/// Compute the path traversal quality for Task_SGPathTraversal.
///
/// Given a history of visited grid cells, a state grid, and a "poison" state,
/// counts how many unique non-poison cells were visited, subtracts the
/// poison-touch count, and returns `max(0, traversed) / target_count`.
///
/// # Safety
/// - `history_ptr` must point to `history_len` valid `c_int` values (cell IDs from ext_mem).
/// - `grid_states_ptr` must point to `grid_len` valid `c_int` values (the state grid data).
#[no_mangle]
pub unsafe extern "C" fn avd_task_sg_path_traversal_quality(
    history_ptr: *const c_int,
    history_len: c_int,
    grid_states_ptr: *const c_int,
    grid_len: c_int,
    poison_state: c_int,
    poison_touch_count: c_int,
    target_count: c_int,
) -> f64 {
    if history_ptr.is_null() || history_len <= 0 || target_count <= 0 {
        return 0.0;
    }
    if grid_states_ptr.is_null() || grid_len <= 0 {
        return 0.0;
    }

    let hlen = history_len as usize;
    // SAFETY: caller guarantees history_ptr is valid for hlen elements.
    let history_raw = unsafe { std::slice::from_raw_parts(history_ptr, hlen) };
    // SAFETY: caller guarantees grid_states_ptr is valid for grid_len elements.
    let grid = unsafe { std::slice::from_raw_parts(grid_states_ptr, grid_len as usize) };

    // Sort history to count unique cells
    let mut history: Vec<i32> = history_raw.to_vec();
    history.sort_unstable();

    let mut traversed: i32 = 0;
    let mut last: i32 = -1;
    for &cell_id in &history {
        if cell_id == last {
            continue;
        }
        last = cell_id;
        // Check bounds and count non-poison cells
        if cell_id >= 0 && (cell_id as usize) < grid.len() && grid[cell_id as usize] != poison_state
        {
            traversed += 1;
        }
    }

    traversed -= poison_touch_count;

    let effective = if traversed >= 0 { traversed } else { 0 };
    f64::from(effective) / f64::from(target_count)
}

// ---------------------------------------------------------------------------
// cBirthChamber — merit blending for recombination
// ---------------------------------------------------------------------------

/// Result of merit blending: two new merit values and whether a genome swap is needed.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MeritBlendResult {
    pub merit0: f64,
    pub merit1: f64,
    /// 1 if genomes should be swapped (majority of genome went to the other child).
    pub swap: c_int,
}

/// Blend two merit values based on crossover fractions.
///
/// Computes:
/// - `new_merit0 = merit0 * stay_frac + merit1 * cut_frac`
/// - `new_merit1 = merit1 * stay_frac + merit0 * cut_frac`
/// - swap = 1 if stay_frac < cut_frac (majority of genome crossed over)
#[no_mangle]
pub extern "C" fn avd_birth_blend_merits(
    merit0: f64,
    merit1: f64,
    cut_frac: f64,
    stay_frac: f64,
) -> MeritBlendResult {
    let new_merit0 = merit0 * stay_frac + merit1 * cut_frac;
    let new_merit1 = merit1 * stay_frac + merit0 * cut_frac;
    let swap = if stay_frac < cut_frac { 1 } else { 0 };
    MeritBlendResult {
        merit0: new_merit0,
        merit1: new_merit1,
        swap,
    }
}

/// Compute cut_frac and stay_frac from start_frac and end_frac (after ordering).
///
/// Ensures start <= end, then returns `(cut_frac, stay_frac)` packed as
/// `cut_frac` in out_cut and `stay_frac` in out_stay.
/// Also computes the four region boundaries for two genomes.
///
/// Returns: start0, end0, start1, end1 packed into a `RecombRegion`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct RecombRegion {
    pub start0: c_int,
    pub end0: c_int,
    pub start1: c_int,
    pub end1: c_int,
    pub cut_frac: f64,
    pub stay_frac: f64,
}

/// Compute recombination region boundaries and fractions from two random doubles.
///
/// `frac_a` and `frac_b` are raw random doubles in [0, 1).
/// `genome0_size` and `genome1_size` are the lengths of the two genomes.
#[no_mangle]
pub extern "C" fn avd_birth_recomb_region(
    frac_a: f64,
    frac_b: f64,
    genome0_size: c_int,
    genome1_size: c_int,
) -> RecombRegion {
    let (start_frac, end_frac) = if frac_a > frac_b {
        (frac_b, frac_a)
    } else {
        (frac_a, frac_b)
    };

    let cut_frac = end_frac - start_frac;
    let stay_frac = 1.0 - cut_frac;

    let g0 = f64::from(genome0_size);
    let g1 = f64::from(genome1_size);

    RecombRegion {
        start0: (start_frac * g0) as c_int,
        end0: (end_frac * g0) as c_int,
        start1: (start_frac * g1) as c_int,
        end1: (end_frac * g1) as c_int,
        cut_frac,
        stay_frac,
    }
}

#[cfg(test)]
#[allow(clippy::undocumented_unsafe_blocks)]
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

    // --- Math1in tests ---

    #[test]
    fn math1in_aa_2x() {
        let inputs: [c_int; 2] = [5, 10];
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 2, 10, MATH1IN_AA),
            1.0
        );
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 2, 20, MATH1IN_AA),
            1.0
        );
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 2, 11, MATH1IN_AA),
            0.0
        );
    }

    #[test]
    fn math1in_ab_2x_div3() {
        let inputs: [c_int; 1] = [9];
        // 2*9/3 = 6 (integer division)
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 6, MATH1IN_AB),
            1.0
        );
        let inputs2: [c_int; 1] = [10];
        // 2*10/3 = 20/3 = 6
        assert_eq!(
            avd_task_eval_math1in(inputs2.as_ptr(), 1, 6, MATH1IN_AB),
            1.0
        );
    }

    #[test]
    fn math1in_ac_5x_div4() {
        let inputs: [c_int; 1] = [8];
        // 5*8/4 = 10
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 10, MATH1IN_AC),
            1.0
        );
    }

    #[test]
    fn math1in_ad_x_squared() {
        let inputs: [c_int; 1] = [7];
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 49, MATH1IN_AD),
            1.0
        );
    }

    #[test]
    fn math1in_ae_x_cubed() {
        let inputs: [c_int; 1] = [3];
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 27, MATH1IN_AE),
            1.0
        );
    }

    #[test]
    fn math1in_af_sqrt_abs() {
        let inputs: [c_int; 1] = [16];
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 4, MATH1IN_AF),
            1.0
        );
        let neg: [c_int; 1] = [-25];
        assert_eq!(avd_task_eval_math1in(neg.as_ptr(), 1, 5, MATH1IN_AF), 1.0);
    }

    #[test]
    fn math1in_ag_log() {
        let inputs: [c_int; 1] = [100];
        // ln(100) ~ 4.605 => 4
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 4, MATH1IN_AG),
            1.0
        );
        // x <= 0 is skipped
        let neg: [c_int; 1] = [-5];
        assert_eq!(avd_task_eval_math1in(neg.as_ptr(), 1, 0, MATH1IN_AG), 0.0);
        let zero: [c_int; 1] = [0];
        assert_eq!(avd_task_eval_math1in(zero.as_ptr(), 1, 0, MATH1IN_AG), 0.0);
    }

    #[test]
    fn math1in_ah_x2_plus_x3() {
        let inputs: [c_int; 1] = [3];
        // 9 + 27 = 36
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 36, MATH1IN_AH),
            1.0
        );
    }

    #[test]
    fn math1in_ai_x2_plus_sqrt_abs() {
        let inputs: [c_int; 1] = [16];
        // 256 + 4 = 260
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 260, MATH1IN_AI),
            1.0
        );
    }

    #[test]
    fn math1in_aj_abs() {
        let inputs: [c_int; 1] = [-42];
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 42, MATH1IN_AJ),
            1.0
        );
    }

    #[test]
    fn math1in_ak_x_minus_5() {
        let inputs: [c_int; 1] = [12];
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 7, MATH1IN_AK),
            1.0
        );
    }

    #[test]
    fn math1in_al_negate() {
        let inputs: [c_int; 1] = [42];
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, -42, MATH1IN_AL),
            1.0
        );
    }

    #[test]
    fn math1in_am_5x() {
        let inputs: [c_int; 1] = [6];
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 30, MATH1IN_AM),
            1.0
        );
    }

    #[test]
    fn math1in_an_x_div4() {
        let inputs: [c_int; 1] = [17];
        // 17/4 = 4
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 4, MATH1IN_AN),
            1.0
        );
    }

    #[test]
    fn math1in_ao_x_minus_6() {
        let inputs: [c_int; 1] = [10];
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 4, MATH1IN_AO),
            1.0
        );
    }

    #[test]
    fn math1in_ap_x_minus_7() {
        let inputs: [c_int; 1] = [10];
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 3, MATH1IN_AP),
            1.0
        );
    }

    #[test]
    fn math1in_as_3x() {
        let inputs: [c_int; 1] = [7];
        assert_eq!(
            avd_task_eval_math1in(inputs.as_ptr(), 1, 21, MATH1IN_AS),
            1.0
        );
    }

    #[test]
    fn math1in_null_inputs_returns_zero() {
        assert_eq!(
            avd_task_eval_math1in(std::ptr::null(), 3, 0, MATH1IN_AA),
            0.0
        );
    }

    #[test]
    fn math1in_invalid_op_returns_zero() {
        let inputs: [c_int; 1] = [5];
        assert_eq!(avd_task_eval_math1in(inputs.as_ptr(), 1, 10, 99), 0.0);
    }

    // --- Math2in tests ---

    #[test]
    fn math2in_aa_sqrt_sum() {
        let inputs: [c_int; 2] = [9, 16];
        // sqrt(abs(9+16)) = sqrt(25) = 5
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 5, MATH2IN_AA),
            1.0
        );
    }

    #[test]
    fn math2in_ab_sum_squared() {
        let inputs: [c_int; 2] = [3, 4];
        // (3+4)^2 = 49
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 49, MATH2IN_AB),
            1.0
        );
    }

    #[test]
    fn math2in_ac_modulo() {
        let inputs: [c_int; 2] = [17, 5];
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 2, MATH2IN_AC),
            1.0
        );
        // Division by zero skipped, but (0 % 17) = 0 still matches
        let inputs2: [c_int; 2] = [17, 0];
        assert_eq!(
            avd_task_eval_math2in(inputs2.as_ptr(), 2, 0, MATH2IN_AC),
            1.0
        );
        // No match at all
        assert_eq!(
            avd_task_eval_math2in(inputs2.as_ptr(), 2, 3, MATH2IN_AC),
            0.0
        );
    }

    #[test]
    fn math2in_ad_linear_combo() {
        let inputs: [c_int; 2] = [10, 8];
        // 3*10/2 + 5*8/4 = 15 + 10 = 25
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 25, MATH2IN_AD),
            1.0
        );
    }

    #[test]
    fn math2in_ae_abs_offsets() {
        let inputs: [c_int; 2] = [3, 10];
        // abs(3-5) + abs(10-6) = 2 + 4 = 6
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 6, MATH2IN_AE),
            1.0
        );
    }

    #[test]
    fn math2in_af_xy_minus_x_div_y() {
        let inputs: [c_int; 2] = [10, 3];
        // 10*3 - 10/3 = 30 - 3 = 27
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 27, MATH2IN_AF),
            1.0
        );
    }

    #[test]
    fn math2in_ag_diff_squared() {
        let inputs: [c_int; 2] = [10, 3];
        // (10-3)^2 = 49
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 49, MATH2IN_AG),
            1.0
        );
    }

    #[test]
    fn math2in_ah_sum_of_squares() {
        let inputs: [c_int; 2] = [3, 4];
        // 9 + 16 = 25
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 25, MATH2IN_AH),
            1.0
        );
    }

    #[test]
    fn math2in_ai_x2_plus_y3() {
        let inputs: [c_int; 2] = [3, 2];
        // 9 + 8 = 17
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 17, MATH2IN_AI),
            1.0
        );
    }

    #[test]
    fn math2in_aj_sqrt_y_div() {
        let inputs: [c_int; 2] = [16, 3];
        // (sqrt(16) + 3) / (16 - 7) = (4 + 3) / 9 = 0
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 0, MATH2IN_AJ),
            1.0
        );
        // x-7 == 0 skipped
        let inputs2: [c_int; 2] = [7, 3];
        assert_eq!(
            avd_task_eval_math2in(inputs2.as_ptr(), 2, 0, MATH2IN_AJ),
            0.0
        );
    }

    #[test]
    fn math2in_an_sum() {
        let inputs: [c_int; 2] = [10, 20];
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 30, MATH2IN_AN),
            1.0
        );
    }

    #[test]
    fn math2in_ao_diff() {
        let inputs: [c_int; 2] = [20, 10];
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 10, MATH2IN_AO),
            1.0
        );
    }

    #[test]
    fn math2in_ap_div() {
        let inputs: [c_int; 2] = [20, 4];
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 5, MATH2IN_AP),
            1.0
        );
    }

    #[test]
    fn math2in_aq_product() {
        let inputs: [c_int; 2] = [6, 7];
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 42, MATH2IN_AQ),
            1.0
        );
    }

    #[test]
    fn math2in_ar_sqrt_sum() {
        let inputs: [c_int; 2] = [16, 25];
        // sqrt(16) + sqrt(25) = 4 + 5 = 9
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 9, MATH2IN_AR),
            1.0
        );
    }

    #[test]
    fn math2in_as_x_plus_2y() {
        let inputs: [c_int; 2] = [5, 3];
        // 5 + 2*3 = 11
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 11, MATH2IN_AS),
            1.0
        );
    }

    #[test]
    fn math2in_at_x_plus_3y() {
        let inputs: [c_int; 2] = [5, 3];
        // 5 + 3*3 = 14
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 14, MATH2IN_AT),
            1.0
        );
    }

    #[test]
    fn math2in_au_2x_plus_3y() {
        let inputs: [c_int; 2] = [5, 3];
        // 10 + 9 = 19
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 19, MATH2IN_AU),
            1.0
        );
    }

    #[test]
    fn math2in_av_xy2() {
        let inputs: [c_int; 2] = [2, 5];
        // 2 * 5 * 5 = 50
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 50, MATH2IN_AV),
            1.0
        );
    }

    #[test]
    fn math2in_ax_x_plus_3y() {
        let inputs: [c_int; 2] = [5, 3];
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 14, MATH2IN_AX),
            1.0
        );
    }

    #[test]
    fn math2in_ay_2x_plus_y() {
        let inputs: [c_int; 2] = [5, 3];
        // 10 + 3 = 13
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 13, MATH2IN_AY),
            1.0
        );
    }

    #[test]
    fn math2in_az_4x_plus_6y() {
        let inputs: [c_int; 2] = [5, 3];
        // 20 + 18 = 38
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 38, MATH2IN_AZ),
            1.0
        );
    }

    #[test]
    fn math2in_aaa_3x_minus_2y() {
        let inputs: [c_int; 2] = [5, 3];
        // 15 - 6 = 9
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 9, MATH2IN_AAA),
            1.0
        );
    }

    #[test]
    fn math2in_null_inputs_returns_zero() {
        assert_eq!(
            avd_task_eval_math2in(std::ptr::null(), 3, 0, MATH2IN_AA),
            0.0
        );
    }

    #[test]
    fn math2in_invalid_op_returns_zero() {
        let inputs: [c_int; 2] = [5, 3];
        assert_eq!(avd_task_eval_math2in(inputs.as_ptr(), 2, 0, 99), 0.0);
    }

    #[test]
    fn math2in_single_input_always_zero() {
        // With only 1 input, i always equals j, so all pairs are skipped
        let inputs: [c_int; 1] = [5];
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 1, 10, MATH2IN_AN),
            0.0
        );
    }

    #[test]
    fn math2in_ak_log_abs_quot() {
        let inputs: [c_int; 2] = [100, 2];
        // log(abs(100/2)) = log(50) ~ 3.91 => 3
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 3, MATH2IN_AK),
            1.0
        );
    }

    #[test]
    fn math2in_al_log_abs_x_div_y() {
        let inputs: [c_int; 2] = [100, 2];
        // log(abs(100)) / 2 = 4.605/2 = 4/2 = 2
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 2, MATH2IN_AL),
            1.0
        );
    }

    #[test]
    fn math2in_am_x_div_log_abs_y() {
        let inputs: [c_int; 2] = [20, 100];
        // 20 / (int)log(100) = 20 / 4 = 5
        assert_eq!(
            avd_task_eval_math2in(inputs.as_ptr(), 2, 5, MATH2IN_AM),
            1.0
        );
    }

    // --- Math3in tests ---

    #[test]
    fn math3in_aa_x2_y2_z2() {
        let inputs: [c_int; 3] = [2, 3, 4];
        // 4+9+16 = 29
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 29, MATH3IN_AA),
            1.0
        );
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 30, MATH3IN_AA),
            0.0
        );
    }

    #[test]
    fn math3in_ab_sqrt_sum() {
        let inputs: [c_int; 3] = [4, 9, 16];
        // sqrt(4)+sqrt(9)+sqrt(16) = 2+3+4 = 9
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 9, MATH3IN_AB),
            1.0
        );
    }

    #[test]
    fn math3in_ac_x_2y_3z() {
        let inputs: [c_int; 3] = [1, 2, 3];
        // 1+4+9=14, 1+6+6=13, 2+2+9=13, 2+6+3=11, 3+2+6=11, 3+4+3=10
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 14, MATH3IN_AC),
            1.0
        );
    }

    #[test]
    fn math3in_ad_xy2_z3() {
        let inputs: [c_int; 3] = [2, 3, 1];
        // 2*3*3+1*1*1 = 18+1 = 19
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 19, MATH3IN_AD),
            1.0
        );
    }

    #[test]
    fn math3in_ae_xmody_times_z() {
        let inputs: [c_int; 3] = [7, 3, 2];
        // 7%3*2 = 1*2 = 2
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 2, MATH3IN_AE),
            1.0
        );
    }

    #[test]
    fn math3in_ae_div_by_zero_guard() {
        let inputs: [c_int; 3] = [7, 0, 2];
        // y=0 should be skipped; only permutations with non-zero y succeed
        // Try (7,2,0): 7%2*0 = 1*0 = 0
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 0, MATH3IN_AE),
            1.0
        );
    }

    #[test]
    fn math3in_af_xpy2_sqrt_ypz() {
        let inputs: [c_int; 3] = [2, 3, 6];
        // (2+3)^2 + sqrt(|3+6|) = 25 + 3 = 28
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 28, MATH3IN_AF),
            1.0
        );
    }

    #[test]
    fn math3in_ag_xy_mod_yz() {
        let inputs: [c_int; 3] = [5, 3, 2];
        // (5*3) % (3*2) = 15 % 6 = 3
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 3, MATH3IN_AG),
            1.0
        );
    }

    #[test]
    fn math3in_ag_div_by_zero_guard() {
        let inputs: [c_int; 3] = [5, 0, 2];
        // y*z = 0 -> skip; permutations with non-zero mod_base:
        // (0,5,2): 0*5 % 5*2 = 0%10 = 0
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 0, MATH3IN_AG),
            1.0
        );
    }

    #[test]
    fn math3in_ah_sum() {
        let inputs: [c_int; 3] = [10, 20, 30];
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 60, MATH3IN_AH),
            1.0
        );
    }

    #[test]
    fn math3in_ai_neg_sum() {
        let inputs: [c_int; 3] = [10, 20, 30];
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, -60, MATH3IN_AI),
            1.0
        );
    }

    #[test]
    fn math3in_aj_sq_diffs() {
        let inputs: [c_int; 3] = [1, 2, 4];
        // (1-2)^2+(2-4)^2+(4-1)^2 = 1+4+9 = 14
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 14, MATH3IN_AJ),
            1.0
        );
    }

    #[test]
    fn math3in_ak_sq_sums() {
        let inputs: [c_int; 3] = [1, 2, 3];
        // (1+2)^2+(2+3)^2+(3+1)^2 = 9+25+16 = 50
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 50, MATH3IN_AK),
            1.0
        );
    }

    #[test]
    fn math3in_al_sq_diff_xz() {
        let inputs: [c_int; 3] = [5, 2, 1];
        // (5-2)^2+(5-1)^2 = 9+16 = 25
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 25, MATH3IN_AL),
            1.0
        );
    }

    #[test]
    fn math3in_am_sq_sum_xz() {
        let inputs: [c_int; 3] = [1, 2, 3];
        // (1+2)^2+(1+3)^2 = 9+16 = 25
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 3, 25, MATH3IN_AM),
            1.0
        );
    }

    #[test]
    fn math3in_null_inputs_returns_zero() {
        assert_eq!(
            avd_task_eval_math3in(std::ptr::null(), 3, 0, MATH3IN_AA),
            0.0
        );
    }

    #[test]
    fn math3in_invalid_op_returns_zero() {
        let inputs: [c_int; 3] = [1, 2, 3];
        assert_eq!(avd_task_eval_math3in(inputs.as_ptr(), 3, 0, 99), 0.0);
    }

    #[test]
    fn math3in_needs_3_distinct_indices() {
        // With only 2 inputs, all triples require i!=j!=k, impossible with 2
        let inputs: [c_int; 2] = [1, 2];
        assert_eq!(
            avd_task_eval_math3in(inputs.as_ptr(), 2, 0, MATH3IN_AH),
            0.0
        );
    }

    // --- Simple arithmetic tests ---

    #[test]
    fn arith_add_basic() {
        let inputs: [c_int; 3] = [10, 20, 30];
        // Add: for j < i, so pairs (1,0)=30, (2,0)=40, (2,1)=50
        assert_eq!(
            avd_task_eval_simple_arith(inputs.as_ptr(), 3, 30, ARITH_ADD),
            1.0
        );
        assert_eq!(
            avd_task_eval_simple_arith(inputs.as_ptr(), 3, 50, ARITH_ADD),
            1.0
        );
        assert_eq!(
            avd_task_eval_simple_arith(inputs.as_ptr(), 3, 99, ARITH_ADD),
            0.0
        );
    }

    #[test]
    fn arith_add3_basic() {
        let inputs: [c_int; 4] = [1, 2, 3, 4];
        // i=0: 1+2+3=6, i=1: 2+3+4=9
        assert_eq!(
            avd_task_eval_simple_arith(inputs.as_ptr(), 4, 6, ARITH_ADD3),
            1.0
        );
        assert_eq!(
            avd_task_eval_simple_arith(inputs.as_ptr(), 4, 9, ARITH_ADD3),
            1.0
        );
        assert_eq!(
            avd_task_eval_simple_arith(inputs.as_ptr(), 4, 10, ARITH_ADD3),
            0.0
        );
    }

    #[test]
    fn arith_add3_too_few_inputs() {
        let inputs: [c_int; 2] = [1, 2];
        assert_eq!(
            avd_task_eval_simple_arith(inputs.as_ptr(), 2, 3, ARITH_ADD3),
            0.0
        );
    }

    #[test]
    fn arith_sub_basic() {
        let inputs: [c_int; 2] = [10, 3];
        assert_eq!(
            avd_task_eval_simple_arith(inputs.as_ptr(), 2, 7, ARITH_SUB),
            1.0
        );
        assert_eq!(
            avd_task_eval_simple_arith(inputs.as_ptr(), 2, -7, ARITH_SUB),
            1.0
        );
    }

    #[test]
    fn arith_null_inputs() {
        assert_eq!(
            avd_task_eval_simple_arith(std::ptr::null(), 3, 0, ARITH_ADD),
            0.0
        );
    }

    #[test]
    fn arith_invalid_op() {
        let inputs: [c_int; 2] = [1, 2];
        assert_eq!(avd_task_eval_simple_arith(inputs.as_ptr(), 2, 3, 99), 0.0);
    }

    // --- Movement task tests ---

    #[test]
    fn move_to_side_right() {
        assert!((avd_task_eval_move_to_side(59, 59) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_to_side_not_right() {
        assert!((avd_task_eval_move_to_side(30, 59) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_to_side_left() {
        assert!((avd_task_eval_move_to_side(0, 0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_yes() {
        assert!((avd_task_eval_move(5, 3) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_no() {
        assert!((avd_task_eval_move(5, 5) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_to_target_on_target() {
        // cell_data > 1, different from prev target
        assert!((avd_task_eval_move_to_target(2, 10, 5) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_to_target_same_target() {
        assert!((avd_task_eval_move_to_target(2, 10, 10) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_to_target_no_data() {
        assert!((avd_task_eval_move_to_target(0, 10, 5) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_to_target_data_one() {
        assert!((avd_task_eval_move_to_target(1, 10, 5) - 0.0).abs() < f64::EPSILON);
    }

    // --- FormSpatialGroup tests ---

    #[test]
    fn form_spatial_group_exact() {
        // t=10, g=10 → 1 - 0/100 = 1.0
        assert!((avd_task_eval_form_spatial_group(10, 10) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn form_spatial_group_half() {
        // t=10, g=5 → 1 - 25/100 = 0.75
        assert!((avd_task_eval_form_spatial_group(10, 5) - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn form_spatial_group_zero_ideal() {
        assert!((avd_task_eval_form_spatial_group(0, 5) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn form_spatial_group_with_id_match() {
        // group matches, g < t → 1.0
        assert!((avd_task_eval_form_spatial_group_with_id(10, 3, 3, 5) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn form_spatial_group_with_id_no_match() {
        assert!((avd_task_eval_form_spatial_group_with_id(10, 3, 5, 5) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn form_spatial_group_with_id_over_size() {
        // group matches, g=12 > t=10 → 1 - 4/100 = 0.96
        assert!((avd_task_eval_form_spatial_group_with_id(10, 3, 3, 12) - 0.96).abs() < 1e-10);
    }

    // --- CommEcho/CommNot tests ---

    #[test]
    fn comm_echo_match() {
        let vals: [c_int; 3] = [10, 42, 20];
        // SAFETY: vals is a valid stack-allocated array.
        let result = unsafe { avd_task_eval_comm_echo(42, vals.as_ptr(), 3) };
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn comm_echo_no_match() {
        let vals: [c_int; 3] = [10, 20, 30];
        // SAFETY: vals is a valid stack-allocated array.
        let result = unsafe { avd_task_eval_comm_echo(42, vals.as_ptr(), 3) };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn comm_echo_null() {
        // SAFETY: null is handled gracefully.
        let result = unsafe { avd_task_eval_comm_echo(42, std::ptr::null(), 0) };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn comm_not_match() {
        // NOT of 42 in two's complement: -(42+1) = -43
        let vals: [c_int; 2] = [10, 42];
        // SAFETY: vals is a valid stack-allocated array.
        let result = unsafe { avd_task_eval_comm_not(-43, vals.as_ptr(), 2) };
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn comm_not_no_match() {
        let vals: [c_int; 2] = [10, 42];
        // SAFETY: vals is a valid stack-allocated array.
        let result = unsafe { avd_task_eval_comm_not(99, vals.as_ptr(), 2) };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    // --- Nand/Nor_ResourceDependent tests ---

    #[test]
    fn nand_res_dep_below_threshold() {
        assert!((avd_task_eval_nand_res_dep(63, 50.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn nand_res_dep_above_threshold() {
        assert!((avd_task_eval_nand_res_dep(63, 150.0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn nand_res_dep_wrong_logic_id() {
        assert!((avd_task_eval_nand_res_dep(42, 50.0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn nor_res_dep_above_threshold() {
        assert!((avd_task_eval_nor_res_dep(3, 150.0) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn nor_res_dep_below_threshold() {
        assert!((avd_task_eval_nor_res_dep(3, 50.0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn nor_res_dep_wrong_logic_id() {
        assert!((avd_task_eval_nor_res_dep(42, 150.0) - 0.0).abs() < f64::EPSILON);
    }

    // --- MoveFT tests ---

    #[test]
    fn move_ft_moved_right_target() {
        assert!((avd_task_eval_move_ft(5, 3, 2, 2) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_ft_didnt_move() {
        assert!((avd_task_eval_move_ft(5, 5, 2, 2) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_ft_wrong_target() {
        assert!((avd_task_eval_move_ft(5, 3, 2, 7) - 0.0).abs() < f64::EPSILON);
    }

    // --- RoyalRoad tests ---

    #[test]
    fn royal_road_all_ones() {
        // 4 blocks of size 2, all 1s → 4/4 = 1.0
        let buf: [c_int; 8] = [1, 1, 1, 1, 1, 1, 1, 1];
        // SAFETY: buf is a valid stack-allocated array with 8 elements.
        let result = unsafe { avd_task_eval_royal_road(buf.as_ptr(), 8, 8, 4) };
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn royal_road_half_correct() {
        // 4 blocks of size 2: [1,1], [1,0], [1,1], [0,1]
        // block 0: 1&1=1, block 1: 1&0=0, block 2: 1&1=1, block 3: 0&1=0
        // reward = 2/4 = 0.5
        let buf: [c_int; 8] = [1, 1, 1, 0, 1, 1, 0, 1];
        // SAFETY: buf is a valid stack-allocated array with 8 elements.
        let result = unsafe { avd_task_eval_royal_road(buf.as_ptr(), 8, 8, 4) };
        assert!((result - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn royal_road_all_zeros() {
        let buf: [c_int; 8] = [0; 8];
        // SAFETY: buf is a valid stack-allocated array with 8 elements.
        let result = unsafe { avd_task_eval_royal_road(buf.as_ptr(), 8, 8, 4) };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn royal_road_null_buf() {
        // SAFETY: null pointer is handled gracefully by the function.
        let result = unsafe { avd_task_eval_royal_road(std::ptr::null(), 0, 8, 4) };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn royal_road_zero_block_count() {
        let buf: [c_int; 4] = [1; 4];
        // SAFETY: buf is a valid stack-allocated array with 4 elements.
        let result = unsafe { avd_task_eval_royal_road(buf.as_ptr(), 4, 4, 0) };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    // --- RoyalRoadWithDitches tests ---

    #[test]
    fn royal_road_wd_all_a_blocks() {
        // 3 blocks of size 2, all 1s → type A blocks
        // State machine: case 1, block A → total_reward = 0 + 2 = 2, next_case = 3
        // Then remaining blocks are case 3 (no-op)
        // Result: 2/3
        let buf: [c_int; 6] = [1, 1, 1, 1, 1, 1];
        // SAFETY: buf is a valid stack-allocated array with 6 elements.
        let result = unsafe { avd_task_eval_royal_road_wd(buf.as_ptr(), 6, 6, 3, 1, 1) };
        assert!((result - 2.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn royal_road_wd_null_buf() {
        // SAFETY: null pointer is handled gracefully by the function.
        let result = unsafe { avd_task_eval_royal_road_wd(std::ptr::null(), 0, 4, 2, 1, 1) };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    // --- FibonacciSequence tests ---

    #[test]
    fn fib_seq_first_match() {
        let mut seq = [1i32, 0i32];
        let mut count = 0i32;
        // SAFETY: seq and count are valid stack-allocated variables.
        let result =
            unsafe { avd_task_eval_fibonacci_seq(seq.as_mut_ptr(), &mut count, 1, 10, 0.5) };
        assert!((result - 1.0).abs() < f64::EPSILON);
        assert_eq!(count, 1);
    }

    #[test]
    fn fib_seq_no_match() {
        let mut seq = [1i32, 0i32];
        let mut count = 0i32;
        // SAFETY: seq and count are valid stack-allocated variables.
        let result =
            unsafe { avd_task_eval_fibonacci_seq(seq.as_mut_ptr(), &mut count, 42, 10, 0.5) };
        assert!((result - 0.0).abs() < f64::EPSILON);
        assert_eq!(count, 0);
    }

    #[test]
    fn fib_seq_past_target() {
        let mut seq = [1i32, 1i32];
        let mut count = 5i32;
        // SAFETY: seq and count are valid stack-allocated variables.
        let result =
            unsafe { avd_task_eval_fibonacci_seq(seq.as_mut_ptr(), &mut count, 2, 5, 0.25) };
        assert!((result - 0.25).abs() < f64::EPSILON);
    }

    // --- AIDisplayCost tests ---

    #[test]
    fn ai_display_cost_off() {
        assert!((avd_task_eval_ai_display_cost(0) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn ai_display_cost_on() {
        assert!((avd_task_eval_ai_display_cost(1) - 1.0).abs() < f64::EPSILON);
    }

    // --- MoveToEvent tests ---

    #[test]
    fn move_to_event_match() {
        let events = [3i32, 5, 7];
        // SAFETY: events is a valid stack-allocated array.
        let result = unsafe { avd_task_eval_move_to_event(events.as_ptr(), 3, 5) };
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_to_event_no_match() {
        let events = [3i32, 5, 7];
        // SAFETY: events is a valid stack-allocated array.
        let result = unsafe { avd_task_eval_move_to_event(events.as_ptr(), 3, 9) };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn move_to_event_negative_data() {
        let events = [3i32, 5];
        // SAFETY: events is a valid stack-allocated array.
        let result = unsafe { avd_task_eval_move_to_event(events.as_ptr(), 2, -1) };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    // --- MatchStr tests ---

    #[test]
    fn match_str_bitwise_perfect() {
        // Pattern "101" (3 bits): output value 5 = 0b101
        // Bit 0: pat[2]='1', val&1=1 -> match
        // Bit 1: pat[1]='0', val&2=0 -> match
        // Bit 2: pat[0]='1', val&4=4 -> match
        // 3 matched out of 3 -> base_bonus = 3*2/3 - 1 = 1.0 -> bonus = 1.0
        let out = [5i32];
        let pat = b"101";
        let result = unsafe {
            avd_task_eval_match_str(out.as_ptr(), 1, ptr::null(), 0, pat.as_ptr(), 3, 0, 0)
        };
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn match_str_bitwise_no_match() {
        // Pattern "111" (3 bits): output value 0 = 0b000
        // 0 matched out of 3 -> base_bonus = 0*2/3 - 1 = -1.0 -> bonus = 0.0
        let out = [0i32];
        let pat = b"111";
        let result = unsafe {
            avd_task_eval_match_str(out.as_ptr(), 1, ptr::null(), 0, pat.as_ptr(), 3, 0, 0)
        };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn match_str_bitwise_half_match() {
        // Pattern "11" (2 bits): output value 1 = 0b01
        // Bit 0: pat[1]='1', val&1=1 -> match
        // Bit 1: pat[0]='1', val&2=0 -> no match
        // 1 matched out of 2 -> base_bonus = 1*2/2 - 1 = 0.0 -> bonus = 0.0
        let out = [1i32];
        let pat = b"11";
        let result = unsafe {
            avd_task_eval_match_str(out.as_ptr(), 1, ptr::null(), 0, pat.as_ptr(), 2, 0, 0)
        };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn match_str_binary_mode_perfect() {
        // Binary mode: element-wise comparison
        // Pattern "101", output [1, 0, 1] -> 3 matched, no '9' so num_real=3
        // base_bonus = 3*2/3 - 1 = 1.0 -> bonus = 1.0
        let out = [1i32, 0, 1];
        let pat = b"101";
        let result = unsafe {
            avd_task_eval_match_str(out.as_ptr(), 3, ptr::null(), 0, pat.as_ptr(), 3, 0, 1)
        };
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn match_str_binary_mode_with_wildcard() {
        // Binary mode with partial: pattern "19" has 1 real char, output [1, 0]
        // pat[0]='1', out[0]=1 -> match; pat[1]='9' (wildcard, not counted)
        // num_real = 1, matched = 1
        // partial mode: base_bonus = 1*2/1 - 1 = 1.0 -> bonus = 1.0
        let out = [1i32, 0];
        let pat = b"19";
        let result = unsafe {
            avd_task_eval_match_str(out.as_ptr(), 2, ptr::null(), 0, pat.as_ptr(), 2, 1, 1)
        };
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn match_str_received_better() {
        // Output matches 1/3, but received message matches 3/3
        // Pattern "101": output 0 matches 0 bits, received [5] matches 3 bits
        let out = [0i32];
        let received = [5i32]; // 0b101
        let pat = b"101";
        let result = unsafe {
            avd_task_eval_match_str(out.as_ptr(), 1, received.as_ptr(), 1, pat.as_ptr(), 3, 0, 0)
        };
        // max_matched = 3, base_bonus = 3*2/3 - 1 = 1.0
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn match_str_null_pattern() {
        let out = [5i32];
        let result = unsafe {
            avd_task_eval_match_str(out.as_ptr(), 1, ptr::null(), 0, ptr::null(), 0, 0, 0)
        };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn match_str_empty_output_nonbinary() {
        // No output stored, non-binary mode -> max_matched stays 0
        let pat = b"101";
        let result = unsafe {
            avd_task_eval_match_str(ptr::null(), 0, ptr::null(), 0, pat.as_ptr(), 3, 0, 0)
        };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    // --- Optimize tests ---

    #[test]
    fn optimize_not_enough_output() {
        let out = [1i32, 2];
        let result = unsafe {
            avd_task_eval_optimize(
                out.as_ptr(),
                2,
                3, // capacity > len
                1,
                0,
                8,
                2,
                2.0,
                1.0,
                0.0,
                -1.0,
                -1.0,
                ptr::null(),
                0,
            )
        };
        assert!((result.quality - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn optimize_func1_proportional() {
        // Function 1: Fx = vars[0], non-binary mode
        // vars[0] = out[0] / 0xFFFFFFFF, vars[1] = out[1] / 0xFFFFFFFF
        // With out[0] = 0, vars[0] = 0, Fx = 0
        // quality = (maxFx - Fx + 0.001) / (maxFx - minFx + 0.001)
        //         = (1.0 - 0.0 + 0.001) / (1.0 - 0.0 + 0.001) = 1.0
        let out = [0i32, 0];
        let result = unsafe {
            avd_task_eval_optimize(
                out.as_ptr(),
                2,
                2,
                1,
                0,
                8,
                2,
                2.0,
                1.0,
                0.0,
                -1.0,
                -1.0,
                ptr::null(),
                0,
            )
        };
        assert!((result.fx - 0.0).abs() < f64::EPSILON);
        assert!((result.quality - 1.0).abs() < 1e-10);
    }

    #[test]
    fn optimize_func22_leading_ones() {
        // Function 22: count leading 1s, Fx = varlength - numOnes
        // varlength=4, output [1,1,1,0] -> 3 leading ones -> Fx = 4-3 = 1
        let out = [1i32, 1, 1, 0];
        let result = unsafe {
            avd_task_eval_optimize(
                out.as_ptr(),
                4,
                4,
                22,
                0,
                4,
                1,
                2.0,
                4.0,
                0.0,
                -1.0,
                -1.0,
                ptr::null(),
                0,
            )
        };
        assert!((result.fx - 1.0).abs() < f64::EPSILON);
        // quality = (4 - 1 + 0.001) / (4 - 0 + 0.001) ≈ 3.001/4.001
        let expected_q = 3.001 / 4.001;
        assert!((result.quality - expected_q).abs() < 1e-10);
    }

    #[test]
    fn optimize_func23_leading_zeros() {
        // Function 23: count leading 0s, Fx = varlength - numZeros
        // varlength=4, output [0,0,1,0] -> 2 leading zeros -> Fx = 4-2 = 2
        let out = [0i32, 0, 1, 0];
        let result = unsafe {
            avd_task_eval_optimize(
                out.as_ptr(),
                4,
                4,
                23,
                0,
                4,
                1,
                2.0,
                4.0,
                0.0,
                -1.0,
                -1.0,
                ptr::null(),
                0,
            )
        };
        assert!((result.fx - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn optimize_func18_sum_plus_one() {
        // Function 18: Fx = 1 + sum(first 30 outputs)
        // 4 outputs: [1, 2, 3, 4] -> sum = 10, Fx = 11
        let mut out = [0i32; 30];
        out[0] = 1;
        out[1] = 2;
        out[2] = 3;
        out[3] = 4;
        let result = unsafe {
            avd_task_eval_optimize(
                out.as_ptr(),
                30,
                30,
                18,
                0,
                8,
                2,
                2.0,
                100.0,
                0.0,
                -1.0,
                -1.0,
                ptr::null(),
                0,
            )
        };
        assert!((result.fx - 11.0).abs() < f64::EPSILON);
    }

    #[test]
    fn optimize_threshold_pass() {
        // Function 22: Fx = varlength - numOnes = 4 - 4 = 0
        // Single threshold: thresh=0.5, Fx <= (max-min)*thresh + min = (4-0)*0.5+0 = 2.0
        // 0 <= 2.0 -> quality = 1.0
        let out = [1i32, 1, 1, 1];
        let result = unsafe {
            avd_task_eval_optimize(
                out.as_ptr(),
                4,
                4,
                22,
                0,
                4,
                1,
                2.0,
                4.0,
                0.0,
                0.5,
                -1.0,
                ptr::null(),
                0,
            )
        };
        assert!((result.quality - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn optimize_threshold_fail() {
        // Function 22: Fx = 4 - 0 = 4
        // Single threshold: Fx <= 2.0? No -> quality = 0.0 -> clamped to 0.001
        let out = [0i32, 0, 0, 0];
        let result = unsafe {
            avd_task_eval_optimize(
                out.as_ptr(),
                4,
                4,
                22,
                0,
                4,
                1,
                2.0,
                4.0,
                0.0,
                0.5,
                -1.0,
                ptr::null(),
                0,
            )
        };
        assert!((result.quality - 0.001).abs() < f64::EPSILON);
    }

    #[test]
    fn optimize_dual_threshold_in_band() {
        // Function 22: Fx = 4 - 2 = 2
        // Band: low = (4-0)*0.25+0 = 1.0, high = (4-0)*0.75+0 = 3.0
        // 2 in [1, 3] -> quality = 1.0
        let out = [1i32, 1, 0, 0];
        let result = unsafe {
            avd_task_eval_optimize(
                out.as_ptr(),
                4,
                4,
                22,
                0,
                4,
                1,
                2.0,
                4.0,
                0.0,
                0.25,
                0.75,
                ptr::null(),
                0,
            )
        };
        assert!((result.quality - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn optimize_unknown_function() {
        // Unknown function returns quality 0.001
        let out = [0i32, 0];
        let result = unsafe {
            avd_task_eval_optimize(
                out.as_ptr(),
                2,
                2,
                99,
                0,
                8,
                2,
                2.0,
                1.0,
                0.0,
                -1.0,
                -1.0,
                ptr::null(),
                0,
            )
        };
        assert!((result.quality - 0.001).abs() < f64::EPSILON);
    }

    #[test]
    fn optimize_func20_string_match() {
        // Function 20: string match, Fx = varlength - matched
        // Pattern "1010", output [1,0,1,0] -> all match -> Fx = 0
        let out = [1i32, 0, 1, 0];
        let pat = b"1010";
        let result = unsafe {
            avd_task_eval_optimize(
                out.as_ptr(),
                4,
                4,
                20,
                0,
                4,
                1,
                2.0,
                4.0,
                0.0,
                -1.0,
                -1.0,
                pat.as_ptr(),
                4,
            )
        };
        assert!((result.fx - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn optimize_null_buf() {
        let result = unsafe {
            avd_task_eval_optimize(
                ptr::null(),
                0,
                2,
                1,
                0,
                8,
                2,
                2.0,
                1.0,
                0.0,
                -1.0,
                -1.0,
                ptr::null(),
                0,
            )
        };
        assert!((result.quality - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn optimize_binary_mode_func1() {
        // Binary mode: 2 vars, varlength=2, basepow=2
        // Output: [1, 1, 0, 0] (var0 bits: [1,1], var1 bits: [0,0])
        // For var0: indices 0+2*(2-0-1)=2, 1+2*(2-0-1)=3 -> out[2]=0, out[3]=0
        //   Wait, let me trace: len=2, numvars=2
        //   i goes from 1 down to 0:
        //     i=1: j=0: idx=1+2*(2-0-1)=1+2=3, tempVars[0] += out[3]*2^0 = 0*1 = 0
        //           j=1: idx=1+2*(2-1-1)=1+0=1, tempVars[1] += out[1]*2^0 = 1*1 = 1
        //     i=0: j=0: idx=0+2*(2-0-1)=0+2=2, tempVars[0] += out[2]*2^1 = 0*2 = 0
        //           j=1: idx=0+2*(2-1-1)=0+0=0, tempVars[1] += out[0]*2^1 = 1*2 = 2
        //   tot = 2^1 + 2^0 = 3
        //   vars[0] = 0/3, vars[1] = 3/3 = 1.0
        //   Fx = vars[0] = 0.0
        let out = [1i32, 1, 0, 0];
        let result = unsafe {
            avd_task_eval_optimize(
                out.as_ptr(),
                4,
                4,
                1,
                1,
                2,
                2,
                2.0,
                1.0,
                0.0,
                -1.0,
                -1.0,
                ptr::null(),
                0,
            )
        };
        assert!((result.fx - 0.0).abs() < 1e-10);
    }

    // --- SortInputs tests ---

    #[test]
    fn sort_inputs_perfect_ascending() {
        // Inputs: [5, 3, 1, 4, 2], size=5
        // Output: [1, 2, 3, 4, 5] (perfectly sorted ascending)
        let inputs = [5i32, 3, 1, 4, 2];
        let output = [1i32, 2, 3, 4, 5];
        let result = unsafe {
            avd_task_eval_sort_inputs(
                output.as_ptr(),
                5,
                inputs.as_ptr(),
                5,
                5,
                1,   // ascending
                0,   // scattered
                5.0, // halflife
            )
        };
        // Perfect sort: score = 0, quality = 2^(0/(-5)) = 1.0
        // But scattered mode skips valid entries from start, so span_start
        // will land on first non-input entry. Since all outputs ARE inputs,
        // span_start reaches stored=5, returns 0.0.
        // Actually this is the scattered bug - it returns 0 when all outputs are valid.
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sort_inputs_contiguous_perfect() {
        // Contiguous mode: inputs [5, 3, 1, 4, 2], size=5
        // Output: [1, 2, 3, 4, 5]
        let inputs = [5i32, 3, 1, 4, 2];
        let output = [1i32, 2, 3, 4, 5];
        let result = unsafe {
            avd_task_eval_sort_inputs(
                output.as_ptr(),
                5,
                inputs.as_ptr(),
                5,
                5,
                1,   // ascending
                1,   // contiguous
                5.0, // halflife
            )
        };
        // Contiguous: all valid, span = [0, 5)
        // Insertion sort: each element goes in order, score = 0, maxscore > 0
        // quality = 2^(0 / -5) = 1.0
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sort_inputs_too_few_outputs() {
        // size=10, output only has 3 items -> 3 <= 10/2=5 -> return 0
        let inputs = [1i32, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let output = [1i32, 2, 3];
        let result = unsafe {
            avd_task_eval_sort_inputs(output.as_ptr(), 3, inputs.as_ptr(), 10, 10, 1, 1, 5.0)
        };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sort_inputs_null_output() {
        let inputs = [1i32, 2, 3];
        let result =
            unsafe { avd_task_eval_sort_inputs(ptr::null(), 0, inputs.as_ptr(), 3, 3, 1, 1, 5.0) };
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sort_inputs_contiguous_reversed() {
        // Descending sort should give perfect score when direction < 0
        let inputs = [1i32, 2, 3, 4, 5];
        let output = [5i32, 4, 3, 2, 1]; // perfectly sorted descending
        let result = unsafe {
            avd_task_eval_sort_inputs(
                output.as_ptr(),
                5,
                inputs.as_ptr(),
                5,
                5,
                -1, // descending
                1,  // contiguous
                5.0,
            )
        };
        // Perfect descending sort: score = 0 -> quality = 1.0
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sort_inputs_contiguous_with_invalid() {
        // Output has some non-input values
        // Inputs: [10, 20, 30], size=3
        // Output: [99, 10, 20, 30, 99] -> contiguous span [1,4) covers [10,20,30]
        let inputs = [10i32, 20, 30];
        let output = [99i32, 10, 20, 30, 99];
        let result = unsafe {
            avd_task_eval_sort_inputs(
                output.as_ptr(),
                5,
                inputs.as_ptr(),
                3,
                3,
                1, // ascending
                1, // contiguous
                5.0,
            )
        };
        // span = [1,4), all 3 inputs present and sorted
        // score = 0, maxscore > 0, quality = 1.0
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sort_inputs_contiguous_wrong_order() {
        // Inputs: [1, 2, 3, 4], size=4
        // Output: [4, 3, 2, 1] ascending but they're reversed
        let inputs = [1i32, 2, 3, 4];
        let output = [4i32, 3, 2, 1];
        let result = unsafe {
            avd_task_eval_sort_inputs(
                output.as_ptr(),
                4,
                inputs.as_ptr(),
                4,
                4,
                1, // ascending
                1, // contiguous
                5.0,
            )
        };
        // Fully reversed ascending: maximum disorder
        // score/maxscore >= 0.5, so quality = 0.0
        assert!((result - 0.0).abs() < f64::EPSILON);
    }

    // -----------------------------------------------------------------------
    // Task_MatchProdStr core tests
    // -----------------------------------------------------------------------

    #[test]
    fn match_prod_str_nonbinary_perfect() {
        // String "1010", output value = 0b1010 = 10
        let string = b"1010";
        let output = [10i32]; // binary 1010
        let packed =
            unsafe { avd_task_match_prod_str_core(string.as_ptr(), 4, output.as_ptr(), 1, 0) };
        let matched = (packed / 1000) as i32;
        assert_eq!(matched, 4);
    }

    #[test]
    fn match_prod_str_nonbinary_none() {
        // String "1111", output value = 0 (all zeros)
        let string = b"1111";
        let output = [0i32];
        let packed =
            unsafe { avd_task_match_prod_str_core(string.as_ptr(), 4, output.as_ptr(), 1, 0) };
        let matched = (packed / 1000) as i32;
        assert_eq!(matched, 0);
    }

    #[test]
    fn match_prod_str_binary_mode() {
        // String "1091", output buffer [1, 0, 0, 1]
        // '1' matches 1, '0' matches 0, '9' is wildcard (not counted), '1' matches 1
        let string = b"1091";
        let output = [1i32, 0, 0, 1];
        let packed =
            unsafe { avd_task_match_prod_str_core(string.as_ptr(), 4, output.as_ptr(), 4, 1) };
        let matched = (packed / 1000) as i32;
        let num_real = (packed % 1000) as i32;
        assert_eq!(matched, 3); // '1'=1, '0'=0, '1'=1 match; '9' ignored for matching but doesn't count
        assert_eq!(num_real, 3); // 3 non-'9' chars
    }

    #[test]
    fn match_prod_str_binary_wildcard() {
        // String "99", output [1, 0]
        let string = b"99";
        let output = [1i32, 0];
        let packed =
            unsafe { avd_task_match_prod_str_core(string.as_ptr(), 2, output.as_ptr(), 2, 1) };
        let matched = (packed / 1000) as i32;
        let num_real = (packed % 1000) as i32;
        assert_eq!(matched, 0); // '9' never matches
        assert_eq!(num_real, 0); // all wildcards
    }

    #[test]
    fn match_prod_str_bonus_full() {
        // 4 out of 4 matched, string_len=4, partial=0, mypow=1.0
        // base_bonus = 4*2/4 - 1 = 1.0, bonus = 1.0^1.0 = 1.0
        let bonus = avd_task_match_prod_str_bonus(4, 4, 4, 0, 1.0);
        assert!((bonus - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn match_prod_str_bonus_half() {
        // 2 out of 4 matched: base_bonus = 2*2/4 - 1 = 0.0, bonus = 0.0
        let bonus = avd_task_match_prod_str_bonus(2, 4, 4, 0, 1.0);
        assert!((bonus - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn match_prod_str_bonus_partial() {
        // 3 out of 3 real chars matched, string_len=4, partial=1, mypow=2.0
        // base_bonus = 3*2/3 - 1 = 1.0, bonus = 1.0^2.0 = 1.0
        let bonus = avd_task_match_prod_str_bonus(3, 4, 3, 1, 2.0);
        assert!((bonus - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn match_prod_str_null_string() {
        let output = [10i32];
        let packed = unsafe { avd_task_match_prod_str_core(ptr::null(), 0, output.as_ptr(), 1, 0) };
        assert_eq!(packed, 0);
    }

    // -----------------------------------------------------------------------
    // Task_SGPathTraversal tests
    // -----------------------------------------------------------------------

    #[test]
    fn sg_path_traversal_basic() {
        // Grid: 4 cells, states [0, 1, 0, 1], poison_state = 1
        // History: [0, 2, 0, 3] -> unique: {0, 2, 3}
        // Cell 0 state=0 (non-poison), cell 2 state=0 (non-poison), cell 3 state=1 (poison)
        // traversed = 2, minus poison_touch_count=0 -> 2
        // quality = 2/4 = 0.5
        let grid = [0i32, 1, 0, 1];
        let history = [0i32, 2, 0, 3];
        let q = unsafe {
            avd_task_sg_path_traversal_quality(
                history.as_ptr(),
                4,
                grid.as_ptr(),
                4,
                1, // poison_state
                0, // poison_touch_count
                4, // target_count
            )
        };
        assert!((q - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn sg_path_traversal_with_poison_touch() {
        // Same as above but poison_touch_count=1 -> traversed = 2 - 1 = 1
        let grid = [0i32, 1, 0, 1];
        let history = [0i32, 2, 0, 3];
        let q = unsafe {
            avd_task_sg_path_traversal_quality(history.as_ptr(), 4, grid.as_ptr(), 4, 1, 1, 4)
        };
        assert!((q - 0.25).abs() < f64::EPSILON);
    }

    #[test]
    fn sg_path_traversal_empty_history() {
        let grid = [0i32, 1, 0, 1];
        let q = unsafe {
            avd_task_sg_path_traversal_quality(ptr::null(), 0, grid.as_ptr(), 4, 1, 0, 4)
        };
        assert!((q - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn sg_path_traversal_all_poison() {
        // All cells are poison state
        let grid = [1i32, 1, 1, 1];
        let history = [0i32, 1, 2, 3];
        let q = unsafe {
            avd_task_sg_path_traversal_quality(history.as_ptr(), 4, grid.as_ptr(), 4, 1, 0, 4)
        };
        assert!((q - 0.0).abs() < f64::EPSILON);
    }

    // -----------------------------------------------------------------------
    // BirthChamber merit blending tests
    // -----------------------------------------------------------------------

    #[test]
    fn merit_blend_equal() {
        // 50/50 split: cut_frac=0.5, stay_frac=0.5
        let r = avd_birth_blend_merits(100.0, 200.0, 0.5, 0.5);
        assert!((r.merit0 - 150.0).abs() < f64::EPSILON);
        assert!((r.merit1 - 150.0).abs() < f64::EPSILON);
        assert_eq!(r.swap, 0); // exactly equal, no swap
    }

    #[test]
    fn merit_blend_no_crossover() {
        // No crossover: cut_frac=0, stay_frac=1
        let r = avd_birth_blend_merits(100.0, 200.0, 0.0, 1.0);
        assert!((r.merit0 - 100.0).abs() < f64::EPSILON);
        assert!((r.merit1 - 200.0).abs() < f64::EPSILON);
        assert_eq!(r.swap, 0);
    }

    #[test]
    fn merit_blend_full_crossover() {
        // Full crossover: cut_frac=1, stay_frac=0 -> swap
        let r = avd_birth_blend_merits(100.0, 200.0, 1.0, 0.0);
        assert!((r.merit0 - 200.0).abs() < f64::EPSILON);
        assert!((r.merit1 - 100.0).abs() < f64::EPSILON);
        assert_eq!(r.swap, 1);
    }

    #[test]
    fn merit_blend_majority_crossover() {
        // Majority crosses: cut_frac=0.7, stay_frac=0.3 -> swap
        let r = avd_birth_blend_merits(100.0, 200.0, 0.7, 0.3);
        assert!((r.merit0 - (100.0 * 0.3 + 200.0 * 0.7)).abs() < 1e-10);
        assert!((r.merit1 - (200.0 * 0.3 + 100.0 * 0.7)).abs() < 1e-10);
        assert_eq!(r.swap, 1);
    }

    // -----------------------------------------------------------------------
    // BirthChamber recombination region tests
    // -----------------------------------------------------------------------

    #[test]
    fn recomb_region_ordered() {
        let r = avd_birth_recomb_region(0.2, 0.8, 100, 200);
        assert_eq!(r.start0, 20);
        assert_eq!(r.end0, 80);
        assert_eq!(r.start1, 40);
        assert_eq!(r.end1, 160);
        assert!((r.cut_frac - 0.6).abs() < 1e-10);
        assert!((r.stay_frac - 0.4).abs() < 1e-10);
    }

    #[test]
    fn recomb_region_reversed() {
        // frac_a > frac_b: should swap
        let r = avd_birth_recomb_region(0.9, 0.1, 100, 100);
        assert_eq!(r.start0, 10);
        assert_eq!(r.end0, 90);
        assert!((r.cut_frac - 0.8).abs() < 1e-10);
    }

    #[test]
    fn recomb_region_equal() {
        let r = avd_birth_recomb_region(0.5, 0.5, 100, 100);
        assert_eq!(r.start0, r.end0);
        assert!((r.cut_frac - 0.0).abs() < 1e-10);
        assert!((r.stay_frac - 1.0).abs() < 1e-10);
    }
}
