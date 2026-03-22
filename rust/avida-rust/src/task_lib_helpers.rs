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
}
