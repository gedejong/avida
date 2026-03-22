//! Rust-native PhenotypeCoreMetrics type replacing the core scalar fields of
//! cPhenotype (section 1: "values calculated at last divide" + section 2
//! in-progress scalars `cur_bonus` / `cur_energy_bonus`).
//!
//! This is Slice 1 of the cPhenotype migration (issue #48).

use std::ffi::{c_double, c_int};

use crate::merit::Merit;

/// Core scalar metrics extracted from cPhenotype.
///
/// Layout must remain `repr(C)` so C++ can embed it directly.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PhenotypeCoreMetrics {
    /// Relative speed of CPU (already #[repr(C)] Rust type).
    pub merit: Merit,
    /// Ratio of current execution merit over base execution merit.
    pub execution_ratio: c_double,
    /// Amount of energy. Determines relative speed of CPU when turned on.
    pub energy_store: c_double,
    /// Number of instructions in genome.
    pub genome_length: c_int,
    /// Number of times MERIT_BONUS_INT is in genome.
    pub bonus_instruction_count: c_int,
    /// Instructions copied into genome.
    pub copied_size: c_int,
    /// Instructions executed from genome.
    pub executed_size: c_int,
    /// CPU cycles to produce offspring (including additional time costs).
    pub gestation_time: c_int,
    /// Total instructions executed at last divide.
    pub gestation_start: c_int,
    /// Relative effective replication rate.
    pub fitness: c_double,
    /// Type of the divide command used.
    pub div_type: c_double,
    /// Current bonus (in-progress).
    pub cur_bonus: c_double,
    /// Current energy bonus (in-progress).
    pub cur_energy_bonus: c_double,
}

impl Default for PhenotypeCoreMetrics {
    fn default() -> Self {
        Self {
            merit: Merit::default(),
            execution_ratio: 1.0,
            energy_store: 0.0,
            genome_length: 0,
            bonus_instruction_count: 0,
            copied_size: 0,
            executed_size: 0,
            gestation_time: 0,
            gestation_start: 0,
            fitness: 0.0,
            div_type: 1.0,
            cur_bonus: 1.0,
            cur_energy_bonus: 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// CalcSizeMerit — pure function, no world/config dependency
// ---------------------------------------------------------------------------

/// Base-merit method constants (mirrors C++ eBASE_MERIT).
const BASE_MERIT_CONST: c_int = 0;
const BASE_MERIT_COPIED_SIZE: c_int = 1;
const BASE_MERIT_EXE_SIZE: c_int = 2;
const BASE_MERIT_FULL_SIZE: c_int = 3;
const BASE_MERIT_LEAST_SIZE: c_int = 4;
const BASE_MERIT_SQRT_LEAST_SIZE: c_int = 5;
const BASE_MERIT_NUM_BONUS_INST: c_int = 6;
const BASE_MERIT_GESTATION_TIME: c_int = 7;

/// Compute base merit size from phenotype metrics.
///
/// This is the pure-function extraction of `cPhenotype::CalcSizeMerit`.
/// The `fitness_valley_*` and `merit_bonus_effect` parameters are only used
/// when `base_merit_method == BASE_MERIT_NUM_BONUS_INST`; pass 0 otherwise.
///
/// # Safety
/// `metrics` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_calc_size_merit(
    metrics: *const PhenotypeCoreMetrics,
    base_merit_method: c_int,
    base_const_merit: c_int,
    cpu_cycles_used: c_int,
    fitness_valley: c_int,
    fitness_valley_start: c_int,
    fitness_valley_stop: c_int,
    merit_bonus_effect: c_int,
) -> c_int {
    // SAFETY: Caller guarantees `metrics` is a valid pointer to `PhenotypeCoreMetrics`.
    let m = unsafe { &*metrics };

    match base_merit_method {
        BASE_MERIT_COPIED_SIZE => m.copied_size,
        BASE_MERIT_EXE_SIZE => m.executed_size,
        BASE_MERIT_FULL_SIZE => m.genome_length,
        BASE_MERIT_LEAST_SIZE => {
            let mut out = m.genome_length;
            if out > m.copied_size {
                out = m.copied_size;
            }
            if out > m.executed_size {
                out = m.executed_size;
            }
            out
        }
        BASE_MERIT_SQRT_LEAST_SIZE => {
            let mut out = m.genome_length;
            if out > m.copied_size {
                out = m.copied_size;
            }
            if out > m.executed_size {
                out = m.executed_size;
            }
            (f64::from(out).sqrt()) as c_int
        }
        BASE_MERIT_NUM_BONUS_INST => {
            if fitness_valley != 0
                && m.bonus_instruction_count >= fitness_valley_start
                && m.bonus_instruction_count <= fitness_valley_stop
            {
                return 1;
            }
            if merit_bonus_effect > 0 {
                1 + m.bonus_instruction_count
            } else if merit_bonus_effect < 0 {
                m.genome_length - (m.bonus_instruction_count - 1)
            } else {
                1
            }
        }
        BASE_MERIT_GESTATION_TIME => cpu_cycles_used,
        BASE_MERIT_CONST => base_const_merit,
        _ => base_const_merit, // unknown defaults to CONST behavior
    }
}

// ---------------------------------------------------------------------------
// PhenotypeStatusFlags — Slice 2 of cPhenotype migration (issue #48)
// ---------------------------------------------------------------------------

/// Status flags and counters extracted from cPhenotype sections 5 + 6.
///
/// All bool fields are represented as `c_int` for FFI safety (C++ `bool` has
/// platform-dependent size; `int` matches the existing Avida FFI convention).
/// 0 = false, non-zero = true.
///
/// Layout must remain `repr(C)` so C++ can embed it directly.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct PhenotypeStatusFlags {
    // --- Section 5: Status Flags (updated at each divide) ---
    pub to_die: c_int,
    pub to_delete: c_int,
    pub make_random_resource: c_int,
    pub is_injected: c_int,
    pub is_clone: c_int,

    // Donor flags
    pub is_donor_cur: c_int,
    pub is_donor_last: c_int,
    pub is_donor_rand: c_int,
    pub is_donor_rand_last: c_int,
    pub is_donor_null: c_int,
    pub is_donor_null_last: c_int,
    pub is_donor_kin: c_int,
    pub is_donor_kin_last: c_int,
    pub is_donor_edit: c_int,
    pub is_donor_edit_last: c_int,
    pub is_donor_gbg: c_int,
    pub is_donor_gbg_last: c_int,
    pub is_donor_truegb: c_int,
    pub is_donor_truegb_last: c_int,
    pub is_donor_threshgb: c_int,
    pub is_donor_threshgb_last: c_int,
    pub is_donor_quanta_threshgb: c_int,
    pub is_donor_quanta_threshgb_last: c_int,
    pub is_donor_shadedgb: c_int,
    pub is_donor_shadedgb_last: c_int,

    // Energy flags
    pub is_energy_requestor: c_int,
    pub is_energy_donor: c_int,
    pub is_energy_receiver: c_int,
    pub has_used_donated_energy: c_int,
    pub has_open_energy_request: c_int,

    // Donation counters
    pub num_thresh_gb_donations: c_int,
    pub num_thresh_gb_donations_last: c_int,
    pub num_quanta_thresh_gb_donations: c_int,
    pub num_quanta_thresh_gb_donations_last: c_int,
    pub num_shaded_gb_donations: c_int,
    pub num_shaded_gb_donations_last: c_int,
    pub num_donations_locus: c_int,
    pub num_donations_locus_last: c_int,

    // Receiver flags
    pub is_receiver: c_int,
    pub is_receiver_last: c_int,
    pub is_receiver_rand: c_int,
    pub is_receiver_kin: c_int,
    pub is_receiver_kin_last: c_int,
    pub is_receiver_edit: c_int,
    pub is_receiver_edit_last: c_int,
    pub is_receiver_gbg: c_int,
    pub is_receiver_truegb: c_int,
    pub is_receiver_truegb_last: c_int,
    pub is_receiver_threshgb: c_int,
    pub is_receiver_threshgb_last: c_int,
    pub is_receiver_quanta_threshgb: c_int,
    pub is_receiver_quanta_threshgb_last: c_int,
    pub is_receiver_shadedgb: c_int,
    pub is_receiver_shadedgb_last: c_int,
    pub is_receiver_gb_same_locus: c_int,
    pub is_receiver_gb_same_locus_last: c_int,

    // General status
    pub is_modifier: c_int,
    pub is_modified: c_int,
    pub is_fertile: c_int,
    pub is_mutated: c_int,
    pub is_multi_thread: c_int,
    pub parent_true: c_int,
    pub parent_sex: c_int,
    pub parent_cross_num: c_int,
    pub born_parent_group: c_int,
    pub kaboom_executed: c_int,
    pub kaboom_executed2: c_int,

    // --- Section 6: Child information ---
    pub copy_true: c_int,
    pub divide_sex: c_int,
    pub child_fertile: c_int,
    pub last_child_fertile: c_int,
    pub mate_select_id: c_int,
    pub cross_num: c_int,
    pub child_copied_size: c_int,
}

// ---------------------------------------------------------------------------
// PhenotypeLifetimeData — Slice 4 of cPhenotype migration (issue #48)
// ---------------------------------------------------------------------------

/// Lifetime records, mating, reproduction, and remaining scalars from
/// cPhenotype sections 2 (in-progress extras), 3 (last divide), 4 (lifetime),
/// and 7 (set once).
///
/// All `bool` fields are represented as `c_int` for FFI safety.
/// `fault_desc` (cString) stays in C++; `testCPU_inst_count` (AvidaArray)
/// stays in C++.
///
/// Layout must remain `repr(C)` so C++ can embed it directly.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PhenotypeLifetimeData {
    // --- Section 2 in-progress extras (not in CoreMetrics) ---
    pub cur_num_errors: c_int,
    pub cur_num_donates: c_int,
    pub trial_time_used: c_int,
    pub trial_cpu_cycles_used: c_int,
    pub last_child_germline_propensity: c_double,
    pub mating_type: c_int,
    pub mate_preference: c_int,
    pub cur_mating_display_a: c_int,
    pub cur_mating_display_b: c_int,

    // --- Section 3: last divide scalars ---
    pub last_merit_base: c_double,
    pub last_bonus: c_double,
    pub last_energy_bonus: c_double,
    pub last_num_errors: c_int,
    pub last_num_donates: c_int,
    pub last_fitness: c_double,
    pub last_cpu_cycles_used: c_int,
    pub cur_child_germline_propensity: c_double,
    pub last_mating_display_a: c_int,
    pub last_mating_display_b: c_int,

    // --- Section 4: lifetime records ---
    pub num_divides_failed: c_int,
    pub num_divides: c_int,
    pub generation: c_int,
    pub cpu_cycles_used: c_int,
    pub time_used: c_int,
    pub num_execs: c_int,
    pub age: c_int,
    // fault_desc (cString) stays in C++
    pub neutral_metric: c_double,
    pub life_fitness: c_double,
    pub exec_time_born: c_int,
    pub gmu_exec_time_born: c_double,
    pub birth_update: c_int,
    pub birth_cell_id: c_int,
    pub av_birth_cell_id: c_int,
    pub birth_group_id: c_int,
    pub birth_forager_type: c_int,
    pub last_task_id: c_int,
    pub num_new_unique_reactions: c_int,
    pub res_consumed: c_double,
    pub is_germ_cell: c_int, // bool as c_int
    pub last_task_time: c_int,

    // --- Section 7: set once ---
    pub permanent_germline_propensity: c_double,
}

impl Default for PhenotypeLifetimeData {
    fn default() -> Self {
        Self {
            // Section 2 in-progress extras
            cur_num_errors: 0,
            cur_num_donates: 0,
            trial_time_used: 0,
            trial_cpu_cycles_used: 0,
            last_child_germline_propensity: 0.0,
            mating_type: -1,    // MATING_TYPE_JUVENILE
            mate_preference: 0, // MATE_PREFERENCE_RANDOM
            cur_mating_display_a: 0,
            cur_mating_display_b: 0,

            // Section 3 last divide
            last_merit_base: 0.0,
            last_bonus: 0.0,
            last_energy_bonus: 0.0,
            last_num_errors: 0,
            last_num_donates: 0,
            last_fitness: 0.0,
            last_cpu_cycles_used: 0,
            cur_child_germline_propensity: 0.0,
            last_mating_display_a: 0,
            last_mating_display_b: 0,

            // Section 4 lifetime
            num_divides_failed: 0,
            num_divides: 0,
            generation: 0,
            cpu_cycles_used: 0,
            time_used: 0,
            num_execs: 0,
            age: 0,
            neutral_metric: 0.0,
            life_fitness: 0.0,
            exec_time_born: 0,
            gmu_exec_time_born: 0.0,
            birth_update: 0,
            birth_cell_id: 0,
            av_birth_cell_id: 0,
            birth_group_id: 0,
            birth_forager_type: -1,
            last_task_id: -1,
            num_new_unique_reactions: 0,
            res_consumed: 0.0,
            is_germ_cell: 0,
            last_task_time: 0,

            // Section 7 set once
            permanent_germline_propensity: 0.0,
        }
    }
}

// ---------------------------------------------------------------------------
// FFI: construction / default
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn avd_pheno_core_default() -> PhenotypeCoreMetrics {
    PhenotypeCoreMetrics::default()
}

#[no_mangle]
pub extern "C" fn avd_pheno_flags_default() -> PhenotypeStatusFlags {
    PhenotypeStatusFlags::default()
}

#[no_mangle]
pub extern "C" fn avd_pheno_lifetime_default() -> PhenotypeLifetimeData {
    PhenotypeLifetimeData::default()
}

// ---------------------------------------------------------------------------
// FFI: getters
// ---------------------------------------------------------------------------

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_merit(p: *const PhenotypeCoreMetrics) -> Merit {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).merit }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_execution_ratio(p: *const PhenotypeCoreMetrics) -> c_double {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).execution_ratio }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_energy_store(p: *const PhenotypeCoreMetrics) -> c_double {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).energy_store }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_genome_length(p: *const PhenotypeCoreMetrics) -> c_int {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).genome_length }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_bonus_instruction_count(p: *const PhenotypeCoreMetrics) -> c_int {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).bonus_instruction_count }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_copied_size(p: *const PhenotypeCoreMetrics) -> c_int {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).copied_size }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_executed_size(p: *const PhenotypeCoreMetrics) -> c_int {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).executed_size }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_gestation_time(p: *const PhenotypeCoreMetrics) -> c_int {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).gestation_time }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_gestation_start(p: *const PhenotypeCoreMetrics) -> c_int {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).gestation_start }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_fitness(p: *const PhenotypeCoreMetrics) -> c_double {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).fitness }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_div_type(p: *const PhenotypeCoreMetrics) -> c_double {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).div_type }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_cur_bonus(p: *const PhenotypeCoreMetrics) -> c_double {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).cur_bonus }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_get_cur_energy_bonus(p: *const PhenotypeCoreMetrics) -> c_double {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe { (*p).cur_energy_bonus }
}

// ---------------------------------------------------------------------------
// FFI: setters
// ---------------------------------------------------------------------------

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_merit(p: *mut PhenotypeCoreMetrics, m: Merit) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).merit = m;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_execution_ratio(p: *mut PhenotypeCoreMetrics, v: c_double) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).execution_ratio = v;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_energy_store(p: *mut PhenotypeCoreMetrics, v: c_double) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).energy_store = v;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_genome_length(p: *mut PhenotypeCoreMetrics, v: c_int) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).genome_length = v;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_bonus_instruction_count(p: *mut PhenotypeCoreMetrics, v: c_int) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).bonus_instruction_count = v;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_copied_size(p: *mut PhenotypeCoreMetrics, v: c_int) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).copied_size = v;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_executed_size(p: *mut PhenotypeCoreMetrics, v: c_int) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).executed_size = v;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_gestation_time(p: *mut PhenotypeCoreMetrics, v: c_int) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).gestation_time = v;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_gestation_start(p: *mut PhenotypeCoreMetrics, v: c_int) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).gestation_start = v;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_fitness(p: *mut PhenotypeCoreMetrics, v: c_double) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).fitness = v;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_div_type(p: *mut PhenotypeCoreMetrics, v: c_double) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).div_type = v;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_cur_bonus(p: *mut PhenotypeCoreMetrics, v: c_double) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).cur_bonus = v;
    }
}

/// # Safety
/// `p` must point to a valid `PhenotypeCoreMetrics`.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn avd_pheno_set_cur_energy_bonus(p: *mut PhenotypeCoreMetrics, v: c_double) {
    // SAFETY: Caller guarantees `p` is valid.
    unsafe {
        (*p).cur_energy_bonus = v;
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_values_match_cpp_constructor() {
        let m = PhenotypeCoreMetrics::default();
        assert_eq!(m.merit.get_double(), 0.0);
        assert_eq!(m.execution_ratio, 1.0);
        assert_eq!(m.energy_store, 0.0);
        assert_eq!(m.genome_length, 0);
        assert_eq!(m.bonus_instruction_count, 0);
        assert_eq!(m.copied_size, 0);
        assert_eq!(m.executed_size, 0);
        assert_eq!(m.gestation_time, 0);
        assert_eq!(m.gestation_start, 0);
        assert_eq!(m.fitness, 0.0);
        assert_eq!(m.div_type, 1.0);
        assert_eq!(m.cur_bonus, 1.0);
        assert_eq!(m.cur_energy_bonus, 0.0);
    }

    #[test]
    fn ffi_default_roundtrip() {
        let m = avd_pheno_core_default();
        assert_eq!(m.fitness, 0.0);
        assert_eq!(m.cur_bonus, 1.0);
    }

    #[test]
    fn ffi_getters() {
        let m = PhenotypeCoreMetrics {
            genome_length: 100,
            copied_size: 80,
            executed_size: 90,
            fitness: 42.5,
            cur_bonus: 2.0,
            ..PhenotypeCoreMetrics::default()
        };

        assert_eq!(avd_pheno_get_genome_length(&m), 100);
        assert_eq!(avd_pheno_get_copied_size(&m), 80);
        assert_eq!(avd_pheno_get_executed_size(&m), 90);
        assert!((avd_pheno_get_fitness(&m) - 42.5).abs() < f64::EPSILON);
        assert!((avd_pheno_get_cur_bonus(&m) - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn ffi_setters() {
        let mut m = PhenotypeCoreMetrics::default();
        avd_pheno_set_genome_length(&mut m, 200);
        avd_pheno_set_fitness(&mut m, 99.9);
        avd_pheno_set_cur_bonus(&mut m, 3.0);
        avd_pheno_set_gestation_time(&mut m, 50);

        assert_eq!(m.genome_length, 200);
        assert!((m.fitness - 99.9).abs() < f64::EPSILON);
        assert!((m.cur_bonus - 3.0).abs() < f64::EPSILON);
        assert_eq!(m.gestation_time, 50);
    }

    fn make_metrics(
        genome_length: c_int,
        copied_size: c_int,
        executed_size: c_int,
    ) -> PhenotypeCoreMetrics {
        PhenotypeCoreMetrics {
            genome_length,
            copied_size,
            executed_size,
            ..PhenotypeCoreMetrics::default()
        }
    }

    #[test]
    fn calc_size_merit_copied_size() {
        let m = make_metrics(100, 80, 90);
        let result = avd_pheno_calc_size_merit(&m, 1, 0, 0, 0, 0, 0, 0); // COPIED_SIZE
        assert_eq!(result, 80);
    }

    #[test]
    fn calc_size_merit_exe_size() {
        let m = make_metrics(100, 80, 90);
        let result = avd_pheno_calc_size_merit(&m, 2, 0, 0, 0, 0, 0, 0); // EXE_SIZE
        assert_eq!(result, 90);
    }

    #[test]
    fn calc_size_merit_full_size() {
        let m = make_metrics(100, 80, 90);
        let result = avd_pheno_calc_size_merit(&m, 3, 0, 0, 0, 0, 0, 0); // FULL_SIZE
        assert_eq!(result, 100);
    }

    #[test]
    fn calc_size_merit_least_size() {
        let m = make_metrics(100, 80, 90);
        let result = avd_pheno_calc_size_merit(&m, 4, 0, 0, 0, 0, 0, 0); // LEAST_SIZE
        assert_eq!(result, 80);
    }

    #[test]
    fn calc_size_merit_sqrt_least_size() {
        let m = make_metrics(100, 100, 100);
        let result = avd_pheno_calc_size_merit(&m, 5, 0, 0, 0, 0, 0, 0); // SQRT_LEAST_SIZE
        assert_eq!(result, 10); // sqrt(100) == 10
    }

    #[test]
    fn calc_size_merit_num_bonus_inst_positive() {
        let m = PhenotypeCoreMetrics {
            genome_length: 100,
            bonus_instruction_count: 5,
            ..PhenotypeCoreMetrics::default()
        };
        let result = avd_pheno_calc_size_merit(&m, 6, 0, 0, 0, 0, 0, 1); // NUM_BONUS_INST, effect>0
        assert_eq!(result, 6); // 1 + 5
    }

    #[test]
    fn calc_size_merit_num_bonus_inst_negative() {
        let m = PhenotypeCoreMetrics {
            genome_length: 100,
            bonus_instruction_count: 5,
            ..PhenotypeCoreMetrics::default()
        };
        let result = avd_pheno_calc_size_merit(&m, 6, 0, 0, 0, 0, 0, -1); // effect<0
        assert_eq!(result, 96); // 100 - (5 - 1)
    }

    #[test]
    fn calc_size_merit_num_bonus_inst_zero_effect() {
        let m = PhenotypeCoreMetrics {
            genome_length: 100,
            bonus_instruction_count: 5,
            ..PhenotypeCoreMetrics::default()
        };
        let result = avd_pheno_calc_size_merit(&m, 6, 0, 0, 0, 0, 0, 0); // effect==0
        assert_eq!(result, 1);
    }

    #[test]
    fn calc_size_merit_fitness_valley() {
        let m = PhenotypeCoreMetrics {
            genome_length: 100,
            bonus_instruction_count: 5,
            ..PhenotypeCoreMetrics::default()
        };
        // fitness_valley=1, start=3, stop=7 -> bonus_instruction_count(5) is in [3,7]
        let result = avd_pheno_calc_size_merit(&m, 6, 0, 0, 1, 3, 7, 1);
        assert_eq!(result, 1); // valley hit
    }

    #[test]
    fn calc_size_merit_gestation_time() {
        let m = PhenotypeCoreMetrics::default();
        let result = avd_pheno_calc_size_merit(&m, 7, 0, 42, 0, 0, 0, 0); // GESTATION_TIME
        assert_eq!(result, 42);
    }

    #[test]
    fn calc_size_merit_const() {
        let m = PhenotypeCoreMetrics::default();
        let result = avd_pheno_calc_size_merit(&m, 0, 100, 0, 0, 0, 0, 0); // CONST
        assert_eq!(result, 100);
    }

    #[test]
    fn calc_size_merit_unknown_defaults_to_const() {
        let m = PhenotypeCoreMetrics::default();
        let result = avd_pheno_calc_size_merit(&m, 99, 77, 0, 0, 0, 0, 0);
        assert_eq!(result, 77);
    }

    #[test]
    fn merit_getter_setter_roundtrip() {
        let mut m = PhenotypeCoreMetrics::default();
        let merit = Merit::new(42.0);
        avd_pheno_set_merit(&mut m, merit);
        let got = avd_pheno_get_merit(&m);
        assert!((got.get_double() - 42.0).abs() < f64::EPSILON);
    }

    #[test]
    fn all_setters_roundtrip() {
        let mut m = PhenotypeCoreMetrics::default();

        avd_pheno_set_execution_ratio(&mut m, 2.5);
        assert!((avd_pheno_get_execution_ratio(&m) - 2.5).abs() < f64::EPSILON);

        avd_pheno_set_energy_store(&mut m, 100.0);
        assert!((avd_pheno_get_energy_store(&m) - 100.0).abs() < f64::EPSILON);

        avd_pheno_set_bonus_instruction_count(&mut m, 7);
        assert_eq!(avd_pheno_get_bonus_instruction_count(&m), 7);

        avd_pheno_set_copied_size(&mut m, 50);
        assert_eq!(avd_pheno_get_copied_size(&m), 50);

        avd_pheno_set_executed_size(&mut m, 60);
        assert_eq!(avd_pheno_get_executed_size(&m), 60);

        avd_pheno_set_gestation_start(&mut m, 10);
        assert_eq!(avd_pheno_get_gestation_start(&m), 10);

        avd_pheno_set_div_type(&mut m, 2.0);
        assert!((avd_pheno_get_div_type(&m) - 2.0).abs() < f64::EPSILON);

        avd_pheno_set_cur_energy_bonus(&mut m, 5.5);
        assert!((avd_pheno_get_cur_energy_bonus(&m) - 5.5).abs() < f64::EPSILON);
    }

    #[test]
    fn struct_is_repr_c_and_sized() {
        // Verify the struct has a known, fixed size for FFI.
        let size = std::mem::size_of::<PhenotypeCoreMetrics>();
        // Merit(4+4+4+8=20 bytes padded to 24) + 2*f64 + 4*i32 + 2*f64 + 2*f64
        // Exact size depends on alignment; just verify it is non-zero and reasonable.
        assert!(size > 0);
        assert!(size <= 256); // sanity bound
    }

    // -----------------------------------------------------------------------
    // PhenotypeStatusFlags tests
    // -----------------------------------------------------------------------

    #[test]
    fn flags_default_all_zero() {
        let f = PhenotypeStatusFlags::default();
        assert_eq!(f.to_die, 0);
        assert_eq!(f.to_delete, 0);
        assert_eq!(f.make_random_resource, 0);
        assert_eq!(f.is_injected, 0);
        assert_eq!(f.is_clone, 0);
        assert_eq!(f.is_donor_cur, 0);
        assert_eq!(f.is_donor_last, 0);
        assert_eq!(f.is_energy_requestor, 0);
        assert_eq!(f.is_receiver, 0);
        assert_eq!(f.is_modifier, 0);
        assert_eq!(f.is_modified, 0);
        assert_eq!(f.is_fertile, 0);
        assert_eq!(f.is_mutated, 0);
        assert_eq!(f.is_multi_thread, 0);
        assert_eq!(f.parent_true, 0);
        assert_eq!(f.parent_sex, 0);
        assert_eq!(f.parent_cross_num, 0);
        assert_eq!(f.born_parent_group, 0);
        assert_eq!(f.kaboom_executed, 0);
        assert_eq!(f.kaboom_executed2, 0);
        assert_eq!(f.copy_true, 0);
        assert_eq!(f.divide_sex, 0);
        assert_eq!(f.child_fertile, 0);
        assert_eq!(f.last_child_fertile, 0);
        assert_eq!(f.mate_select_id, 0);
        assert_eq!(f.cross_num, 0);
        assert_eq!(f.child_copied_size, 0);
        assert_eq!(f.num_thresh_gb_donations, 0);
        assert_eq!(f.num_quanta_thresh_gb_donations, 0);
        assert_eq!(f.num_shaded_gb_donations, 0);
        assert_eq!(f.num_donations_locus, 0);
    }

    #[test]
    fn flags_ffi_default_roundtrip() {
        let f = avd_pheno_flags_default();
        assert_eq!(f.to_die, 0);
        assert_eq!(f.is_fertile, 0);
        assert_eq!(f.child_copied_size, 0);
    }

    #[test]
    fn flags_field_mutation() {
        let f = PhenotypeStatusFlags {
            is_donor_cur: 1,
            is_fertile: 1,
            num_thresh_gb_donations: 5,
            mate_select_id: -1,
            child_copied_size: 100,
            ..PhenotypeStatusFlags::default()
        };

        assert_eq!(f.is_donor_cur, 1);
        assert_eq!(f.is_fertile, 1);
        assert_eq!(f.num_thresh_gb_donations, 5);
        assert_eq!(f.mate_select_id, -1);
        assert_eq!(f.child_copied_size, 100);
        // Other fields remain zero
        assert_eq!(f.is_donor_last, 0);
        assert_eq!(f.to_die, 0);
    }

    #[test]
    fn flags_struct_is_repr_c_and_sized() {
        let size = std::mem::size_of::<PhenotypeStatusFlags>();
        // 74 c_int fields * 4 bytes = 296 bytes (no padding needed, all same type)
        assert_eq!(size, 74 * 4);
    }

    #[test]
    fn flags_clone_is_independent() {
        let f = PhenotypeStatusFlags {
            is_donor_cur: 1,
            ..PhenotypeStatusFlags::default()
        };
        let f2 = f;
        assert_eq!(f2.is_donor_cur, 1);
        // Mutating original does not affect copy (Copy semantics)
        let mut f3 = f;
        f3.is_donor_cur = 0;
        assert_eq!(f.is_donor_cur, 1);
        assert_eq!(f3.is_donor_cur, 0);
    }

    // -----------------------------------------------------------------------
    // PhenotypeLifetimeData tests
    // -----------------------------------------------------------------------

    #[test]
    fn lifetime_default_values() {
        let d = PhenotypeLifetimeData::default();
        // Section 2
        assert_eq!(d.cur_num_errors, 0);
        assert_eq!(d.cur_num_donates, 0);
        assert_eq!(d.trial_time_used, 0);
        assert_eq!(d.trial_cpu_cycles_used, 0);
        assert!((d.last_child_germline_propensity - 0.0).abs() < f64::EPSILON);
        assert_eq!(d.mating_type, -1); // MATING_TYPE_JUVENILE
        assert_eq!(d.mate_preference, 0); // MATE_PREFERENCE_RANDOM
        assert_eq!(d.cur_mating_display_a, 0);
        assert_eq!(d.cur_mating_display_b, 0);
        // Section 3
        assert!((d.last_merit_base - 0.0).abs() < f64::EPSILON);
        assert!((d.last_bonus - 0.0).abs() < f64::EPSILON);
        assert!((d.last_energy_bonus - 0.0).abs() < f64::EPSILON);
        assert_eq!(d.last_num_errors, 0);
        assert_eq!(d.last_num_donates, 0);
        assert!((d.last_fitness - 0.0).abs() < f64::EPSILON);
        assert_eq!(d.last_cpu_cycles_used, 0);
        assert!((d.cur_child_germline_propensity - 0.0).abs() < f64::EPSILON);
        assert_eq!(d.last_mating_display_a, 0);
        assert_eq!(d.last_mating_display_b, 0);
        // Section 4
        assert_eq!(d.num_divides_failed, 0);
        assert_eq!(d.num_divides, 0);
        assert_eq!(d.generation, 0);
        assert_eq!(d.cpu_cycles_used, 0);
        assert_eq!(d.time_used, 0);
        assert_eq!(d.num_execs, 0);
        assert_eq!(d.age, 0);
        assert!((d.neutral_metric - 0.0).abs() < f64::EPSILON);
        assert!((d.life_fitness - 0.0).abs() < f64::EPSILON);
        assert_eq!(d.exec_time_born, 0);
        assert!((d.gmu_exec_time_born - 0.0).abs() < f64::EPSILON);
        assert_eq!(d.birth_update, 0);
        assert_eq!(d.birth_cell_id, 0);
        assert_eq!(d.av_birth_cell_id, 0);
        assert_eq!(d.birth_group_id, 0);
        assert_eq!(d.birth_forager_type, -1);
        assert_eq!(d.last_task_id, -1);
        assert_eq!(d.num_new_unique_reactions, 0);
        assert!((d.res_consumed - 0.0).abs() < f64::EPSILON);
        assert_eq!(d.is_germ_cell, 0);
        assert_eq!(d.last_task_time, 0);
        // Section 7
        assert!((d.permanent_germline_propensity - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn lifetime_ffi_default_roundtrip() {
        let d = avd_pheno_lifetime_default();
        assert_eq!(d.cur_num_errors, 0);
        assert_eq!(d.mating_type, -1);
        assert_eq!(d.birth_forager_type, -1);
        assert_eq!(d.last_task_id, -1);
        assert!((d.permanent_germline_propensity - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn lifetime_field_mutation() {
        let d = PhenotypeLifetimeData {
            num_divides: 5,
            generation: 12,
            cpu_cycles_used: 1000,
            last_fitness: 2.5,
            is_germ_cell: 1,
            ..PhenotypeLifetimeData::default()
        };
        assert_eq!(d.num_divides, 5);
        assert_eq!(d.generation, 12);
        assert_eq!(d.cpu_cycles_used, 1000);
        assert!((d.last_fitness - 2.5).abs() < f64::EPSILON);
        assert_eq!(d.is_germ_cell, 1);
        // Other fields remain default
        assert_eq!(d.cur_num_errors, 0);
        assert_eq!(d.mating_type, -1);
    }

    #[test]
    fn lifetime_struct_is_repr_c_and_sized() {
        let size = std::mem::size_of::<PhenotypeLifetimeData>();
        assert!(size > 0);
        // 22 c_int fields * 4 = 88 bytes for ints
        // 12 c_double fields * 8 = 96 bytes for doubles
        // Total with potential padding: should be reasonable
        assert!(size <= 512); // sanity bound
    }

    #[test]
    fn lifetime_clone_is_independent() {
        let d = PhenotypeLifetimeData {
            num_divides: 7,
            ..PhenotypeLifetimeData::default()
        };
        let d2 = d;
        assert_eq!(d2.num_divides, 7);
        let mut d3 = d;
        d3.num_divides = 0;
        assert_eq!(d.num_divides, 7);
        assert_eq!(d3.num_divides, 0);
    }
}
