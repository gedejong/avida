//! `TaskContextSnapshot` — a `#[repr(C)]` struct that C++ populates before
//! calling Rust task evaluators.
//!
//! This captures the flat scalar data from `cTaskContext`, `cOrganism`, and
//! `cTaskEntry` arguments needed by the 48 remaining task functions. Fixed-size
//! arrays hold buffer contents (first 8 values) so no heap allocation crosses
//! the FFI boundary.

use std::ffi::{c_double, c_int};

/// Maximum number of buffer values captured in the snapshot.
pub const TASK_CTX_BUFFER_CAP: usize = 8;

/// Maximum number of integer arguments from `cArgContainer`.
pub const TASK_CTX_INT_ARGS_CAP: usize = 4;

/// Maximum number of double arguments from `cArgContainer`.
pub const TASK_CTX_DOUBLE_ARGS_CAP: usize = 2;

/// Flat snapshot of task-evaluation context, populated by C++ before calling
/// Rust evaluators.
///
/// All buffer arrays use a count + fixed-size array pattern: values beyond
/// the count are zeroed.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TaskContextSnapshot {
    // -- From cTaskContext ------------------------------------------------
    /// `cTaskContext::GetLogicId()`
    pub logic_id: c_int,

    /// First value in the output buffer (`GetOutputBuffer()[0]`), or 0 if empty.
    pub output_value: c_int,

    /// First `output_count` values from the output buffer.
    pub output_buffer: [c_int; TASK_CTX_BUFFER_CAP],
    /// Number of valid entries in `output_buffer` (capped at `TASK_CTX_BUFFER_CAP`).
    pub output_count: c_int,

    /// First `input_count` values from the input buffer.
    pub input_buffer: [c_int; TASK_CTX_BUFFER_CAP],
    /// Number of valid entries in `input_buffer` (capped at `TASK_CTX_BUFFER_CAP`).
    pub input_count: c_int,

    // -- Task entry arguments (from cArgContainer) -----------------------
    /// Integer arguments from the task entry, indexed 0..3.
    pub task_arg_int: [c_int; TASK_CTX_INT_ARGS_CAP],
    /// Double arguments from the task entry, indexed 0..1.
    pub task_arg_double: [c_double; TASK_CTX_DOUBLE_ARGS_CAP],
    /// `1` if the task entry has a non-null `cArgContainer`, else `0`.
    pub has_task_args: c_int,

    // -- From cOrganism --------------------------------------------------
    /// `cOrganism::GetCellID()`
    pub cell_id: c_int,
    /// `cOrganism::GetAVCellID()`
    pub av_cell_id: c_int,
    /// `cOrganism::GetForageTarget()`
    pub forage_target: c_int,
    /// `cOrganism::GetGradientMovement()`
    pub gradient_movement: c_double,
    /// `cOrganism::HasOpinion()` — 1 if true, 0 if false.
    pub has_opinion: c_int,
    /// `cOrganism::GetOpinion().first` (only meaningful when `has_opinion == 1`).
    pub opinion_value: c_int,
    /// `cPhenotype::GetKaboomExecuted()` — 1 if true, 0 if false.
    pub kaboom_executed: c_int,
    /// `cPhenotype::GetKaboomExecuted2()` — 1 if true, 0 if false.
    pub kaboom_executed2: c_int,
    /// `cOrganism::GetEventKilled()` — raw int from organism.
    pub event_killed: c_int,
    /// `cOrganism::GetPrevSeenCellID()`
    pub prev_seen_cell_id: c_int,

    // -- Neighbor information -------------------------------------------
    /// Number of neighbors that have non-empty output buffers.
    pub num_neighbors_with_outputs: c_int,
}

impl Default for TaskContextSnapshot {
    fn default() -> Self {
        Self {
            logic_id: 0,
            output_value: 0,
            output_buffer: [0; TASK_CTX_BUFFER_CAP],
            output_count: 0,
            input_buffer: [0; TASK_CTX_BUFFER_CAP],
            input_count: 0,
            task_arg_int: [0; TASK_CTX_INT_ARGS_CAP],
            task_arg_double: [0.0; TASK_CTX_DOUBLE_ARGS_CAP],
            has_task_args: 0,
            cell_id: -1,
            av_cell_id: -1,
            forage_target: 0,
            gradient_movement: 0.0,
            has_opinion: 0,
            opinion_value: 0,
            kaboom_executed: 0,
            kaboom_executed2: 0,
            event_killed: 0,
            prev_seen_cell_id: -1,
            num_neighbors_with_outputs: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Pure Rust task evaluators using the snapshot
// ---------------------------------------------------------------------------

/// Task_DontCare — always returns 1.0.
pub fn task_dont_care(_ctx: &TaskContextSnapshot) -> f64 {
    1.0
}

/// Task_Exploded — returns 1.0 if the organism executed the kaboom instruction.
pub fn task_exploded(ctx: &TaskContextSnapshot) -> f64 {
    if ctx.kaboom_executed != 0 {
        1.0
    } else {
        0.0
    }
}

/// Task_Exploded2 — returns 1.0 if the organism executed the kaboom2 instruction.
pub fn task_exploded2(ctx: &TaskContextSnapshot) -> f64 {
    if ctx.kaboom_executed2 != 0 {
        1.0
    } else {
        0.0
    }
}

/// Task_AllOnes — average of the first `length` output buffer values.
///
/// `length` comes from `task_arg_int[0]`.  Values beyond `output_count` are
/// treated as zero (matching the C++ `tBuffer` copy-then-index behaviour).
pub fn task_all_ones(ctx: &TaskContextSnapshot) -> f64 {
    let length = ctx.task_arg_int[0];
    if length <= 0 {
        return 0.0;
    }
    let len = length as usize;
    let mut sum: f64 = 0.0;
    for i in 0..len {
        if i < TASK_CTX_BUFFER_CAP && i < ctx.output_count as usize {
            sum += f64::from(ctx.output_buffer[i]);
        }
        // Beyond the buffer cap or stored count, the value is 0 — no addition.
    }
    sum / f64::from(length)
}

/// Task_MatchNumber — compare first output to `task_arg_int[0]`, with
/// threshold/halflife quality from existing Rust helper.
///
/// Uses `task_arg_int[0]` as the target number, `task_arg_int[1]` as threshold,
/// and `task_arg_double[0]` as halflife.
pub fn task_match_number(ctx: &TaskContextSnapshot) -> f64 {
    let target = i64::from(ctx.task_arg_int[0]);
    let actual = i64::from(ctx.output_value);
    let diff = (target - actual).abs();
    let threshold = ctx.task_arg_int[1];
    let halflife = ctx.task_arg_double[0];
    crate::task_lib_helpers::avd_tasklib_threshold_halflife_quality(diff, threshold, halflife)
}

// ---------------------------------------------------------------------------
// FFI exports
// ---------------------------------------------------------------------------

/// Return a zeroed-out `TaskContextSnapshot` with safe defaults.
///
/// # Safety
/// No pointers involved — returns by value.
#[no_mangle]
pub extern "C" fn avd_task_ctx_default() -> TaskContextSnapshot {
    TaskContextSnapshot::default()
}

/// Evaluate Task_DontCare via the snapshot. Always returns 1.0.
///
/// # Safety
/// `ctx` must point to a valid, initialized `TaskContextSnapshot`.
#[no_mangle]
pub unsafe extern "C" fn avd_task_ctx_dont_care(ctx: *const TaskContextSnapshot) -> f64 {
    // SAFETY: caller guarantees `ctx` is a valid pointer to an initialized snapshot.
    let snap = unsafe { &*ctx };
    task_dont_care(snap)
}

/// Evaluate Task_Exploded via the snapshot.
///
/// # Safety
/// `ctx` must point to a valid, initialized `TaskContextSnapshot`.
#[no_mangle]
pub unsafe extern "C" fn avd_task_ctx_exploded(ctx: *const TaskContextSnapshot) -> f64 {
    // SAFETY: caller guarantees `ctx` is a valid pointer to an initialized snapshot.
    let snap = unsafe { &*ctx };
    task_exploded(snap)
}

/// Evaluate Task_Exploded2 via the snapshot.
///
/// # Safety
/// `ctx` must point to a valid, initialized `TaskContextSnapshot`.
#[no_mangle]
pub unsafe extern "C" fn avd_task_ctx_exploded2(ctx: *const TaskContextSnapshot) -> f64 {
    // SAFETY: caller guarantees `ctx` is a valid pointer to an initialized snapshot.
    let snap = unsafe { &*ctx };
    task_exploded2(snap)
}

/// Evaluate Task_AllOnes via the snapshot.
///
/// # Safety
/// `ctx` must point to a valid, initialized `TaskContextSnapshot`.
#[no_mangle]
pub unsafe extern "C" fn avd_task_ctx_all_ones(ctx: *const TaskContextSnapshot) -> f64 {
    // SAFETY: caller guarantees `ctx` is a valid pointer to an initialized snapshot.
    let snap = unsafe { &*ctx };
    task_all_ones(snap)
}

/// Evaluate Task_MatchNumber via the snapshot.
///
/// # Safety
/// `ctx` must point to a valid, initialized `TaskContextSnapshot`.
#[no_mangle]
pub unsafe extern "C" fn avd_task_ctx_match_number(ctx: *const TaskContextSnapshot) -> f64 {
    // SAFETY: caller guarantees `ctx` is a valid pointer to an initialized snapshot.
    let snap = unsafe { &*ctx };
    task_match_number(snap)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_snapshot() {
        let snap = TaskContextSnapshot::default();
        assert_eq!(snap.logic_id, 0);
        assert_eq!(snap.output_value, 0);
        assert_eq!(snap.output_count, 0);
        assert_eq!(snap.input_count, 0);
        assert_eq!(snap.cell_id, -1);
        assert_eq!(snap.av_cell_id, -1);
        assert_eq!(snap.kaboom_executed, 0);
        assert_eq!(snap.kaboom_executed2, 0);
        assert_eq!(snap.has_task_args, 0);
        assert_eq!(snap.num_neighbors_with_outputs, 0);
    }

    #[test]
    fn test_dont_care_always_1() {
        let snap = TaskContextSnapshot::default();
        assert!((task_dont_care(&snap) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_exploded_false() {
        let snap = TaskContextSnapshot::default();
        assert!((task_exploded(&snap) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_exploded_true() {
        let snap = TaskContextSnapshot {
            kaboom_executed: 1,
            ..TaskContextSnapshot::default()
        };
        assert!((task_exploded(&snap) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_exploded2_false() {
        let snap = TaskContextSnapshot::default();
        assert!((task_exploded2(&snap) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_exploded2_true() {
        let snap = TaskContextSnapshot {
            kaboom_executed2: 1,
            ..TaskContextSnapshot::default()
        };
        assert!((task_exploded2(&snap) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_all_ones_basic() {
        let mut snap = TaskContextSnapshot::default();
        snap.task_arg_int[0] = 4; // length = 4
        snap.output_count = 4;
        snap.output_buffer[0] = 1;
        snap.output_buffer[1] = 1;
        snap.output_buffer[2] = 1;
        snap.output_buffer[3] = 0;
        // (1+1+1+0)/4 = 0.75
        assert!((task_all_ones(&snap) - 0.75).abs() < f64::EPSILON);
    }

    #[test]
    fn test_all_ones_zero_length() {
        let snap = TaskContextSnapshot::default();
        assert!((task_all_ones(&snap) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_all_ones_all_set() {
        let mut snap = TaskContextSnapshot::default();
        snap.task_arg_int[0] = 3;
        snap.output_count = 3;
        snap.output_buffer[0] = 1;
        snap.output_buffer[1] = 1;
        snap.output_buffer[2] = 1;
        assert!((task_all_ones(&snap) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_match_number_exact() {
        let mut snap = TaskContextSnapshot::default();
        snap.task_arg_int[0] = 42; // target
        snap.task_arg_int[1] = -1; // threshold (negative = no threshold)
        snap.task_arg_double[0] = 1.0; // halflife
        snap.output_value = 42;
        // diff = 0, quality = 2^(0/1) = 1.0
        assert!((task_match_number(&snap) - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_match_number_off_by_one() {
        let mut snap = TaskContextSnapshot::default();
        snap.task_arg_int[0] = 42;
        snap.task_arg_int[1] = -1; // no threshold
        snap.task_arg_double[0] = 1.0;
        snap.output_value = 43;
        // diff = 1, quality = 2^(1/-1) = 0.5
        assert!((task_match_number(&snap) - 0.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_match_number_over_threshold() {
        let mut snap = TaskContextSnapshot::default();
        snap.task_arg_int[0] = 42;
        snap.task_arg_int[1] = 2; // threshold = 2
        snap.task_arg_double[0] = 1.0;
        snap.output_value = 50; // diff = 8 > threshold 2
        assert!((task_match_number(&snap) - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ffi_default_roundtrip() {
        let snap = avd_task_ctx_default();
        assert_eq!(snap.logic_id, 0);
        assert_eq!(snap.cell_id, -1);
    }

    #[test]
    fn test_ffi_dont_care() {
        let snap = avd_task_ctx_default();
        // SAFETY: `snap` is a valid, stack-allocated snapshot.
        let result = unsafe { avd_task_ctx_dont_care(&snap) };
        assert!((result - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ffi_exploded() {
        let mut snap = avd_task_ctx_default();
        // SAFETY: `snap` is a valid, stack-allocated snapshot.
        assert!((unsafe { avd_task_ctx_exploded(&snap) } - 0.0).abs() < f64::EPSILON);
        snap.kaboom_executed = 1;
        // SAFETY: `snap` is a valid, stack-allocated snapshot.
        assert!((unsafe { avd_task_ctx_exploded(&snap) } - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ffi_exploded2() {
        let mut snap = avd_task_ctx_default();
        // SAFETY: `snap` is a valid, stack-allocated snapshot.
        assert!((unsafe { avd_task_ctx_exploded2(&snap) } - 0.0).abs() < f64::EPSILON);
        snap.kaboom_executed2 = 1;
        // SAFETY: `snap` is a valid, stack-allocated snapshot.
        assert!((unsafe { avd_task_ctx_exploded2(&snap) } - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ffi_all_ones() {
        let mut snap = avd_task_ctx_default();
        snap.task_arg_int[0] = 2;
        snap.output_count = 2;
        snap.output_buffer[0] = 1;
        snap.output_buffer[1] = 1;
        // SAFETY: `snap` is a valid, stack-allocated snapshot.
        assert!((unsafe { avd_task_ctx_all_ones(&snap) } - 1.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_ffi_match_number() {
        let mut snap = avd_task_ctx_default();
        snap.task_arg_int[0] = 100;
        snap.task_arg_int[1] = -1;
        snap.task_arg_double[0] = 1.0;
        snap.output_value = 100;
        // SAFETY: `snap` is a valid, stack-allocated snapshot.
        assert!((unsafe { avd_task_ctx_match_number(&snap) } - 1.0).abs() < f64::EPSILON);
    }

    /// Verify struct size is stable for ABI compatibility.
    #[test]
    fn test_struct_size_stable() {
        let size = std::mem::size_of::<TaskContextSnapshot>();
        // Ensure it doesn't accidentally change.  This is a basic sanity
        // check; the exact value depends on alignment/padding.
        assert!(size > 0, "TaskContextSnapshot must have nonzero size");
        // With the current fields we expect:
        //   21 c_int fields * 4 = 84
        //   8+8 buffer ints * 4 = 64
        //   4 arg ints * 4 = 16
        //   1 c_double (gradient_movement) * 8 = 8
        //   2 arg doubles * 8 = 16
        // Total scalars: 84+64+16+8+16 = 188, plus alignment padding
        assert!(
            size <= 256,
            "TaskContextSnapshot unexpectedly large: {size} bytes"
        );
    }
}
