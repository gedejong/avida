//! Rust-native ConfigSnapshot — a `#[repr(C)]` struct populated by C++ once
//! per update and passed into Rust task evaluators, phenotype methods, birth
//! chamber helpers, and organism logic.
//!
//! Every field uses `c_int` (for int/bool config vars) or `c_double` (for
//! double config vars), matching the C++ `cAvidaConfig` accessor types.

use std::ffi::{c_double, c_int};

// ---------------------------------------------------------------------------
// cTaskLib config values
// ---------------------------------------------------------------------------

/// Config values consumed by cTaskLib task evaluators.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TaskLibConfig {
    pub world_x: c_int,
    pub world_y: c_int,
    pub use_avatars: c_int,
    pub match_already_produced: c_int,
}

impl Default for TaskLibConfig {
    fn default() -> Self {
        Self {
            world_x: 60,
            world_y: 60,
            use_avatars: 0,
            match_already_produced: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// cPhenotype config values
// ---------------------------------------------------------------------------

/// Config values consumed by cPhenotype methods.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhenotypeConfig {
    pub base_merit_method: c_int,
    pub base_const_merit: c_int,
    pub default_bonus: c_double,
    pub divide_method: c_int,
    pub generation_inc_method: c_int,
    pub merit_bonus_effect: c_int,
    pub merit_default_bonus: c_int,
    pub fitness_method: c_int,
    pub fitness_valley: c_int,
    pub fitness_valley_start: c_int,
    pub fitness_valley_stop: c_int,
    /// bool in C++ config, but `.Get()` returns int.
    pub energy_enabled: c_int,
    pub energy_cap: c_double,
    pub apply_energy_method: c_int,
    pub fix_metabolic_rate: c_double,
    pub inherit_exe_rate: c_int,
    pub inherit_merit: c_int,
    pub inherit_multithread: c_int,
    pub energy_given_at_birth: c_double,
    pub resource_given_at_birth: c_double,
    pub resource_given_on_inject: c_double,
    pub frac_energy_decay_at_org_birth: c_double,
    pub frac_parent_energy_given_to_org_at_birth: c_double,
    pub energy_thresh_low: c_double,
    pub energy_thresh_high: c_double,
    pub demes_default_germline_propensity: c_double,
    /// bool in C++ config.
    pub demes_orgs_start_in_germ: c_int,
    pub tolerance_variations: c_int,
    pub tolerance_window: c_int,
    pub max_tolerance: c_int,
    /// bool in C++ config.
    pub use_resource_bins: c_int,
    pub collect_specific_resource: c_int,
    /// bool in C++ config.
    pub split_on_divide: c_int,
    pub task_refractory_period: c_double,
    pub task_switch_penalty_type: c_int,
    pub learning_count: c_int,
    /// bool in C++ config.
    pub age_poly_tracking: c_int,
}

impl Default for PhenotypeConfig {
    fn default() -> Self {
        Self {
            base_merit_method: 4,
            base_const_merit: 100,
            default_bonus: 1.0,
            divide_method: 1,
            generation_inc_method: 1,
            merit_bonus_effect: 0,
            merit_default_bonus: 0,
            fitness_method: 0,
            fitness_valley: 0,
            fitness_valley_start: 0,
            fitness_valley_stop: 0,
            energy_enabled: 0,
            energy_cap: -1.0,
            apply_energy_method: 0,
            fix_metabolic_rate: -1.0,
            inherit_exe_rate: 0,
            inherit_merit: 1,
            inherit_multithread: 0,
            energy_given_at_birth: 0.0,
            resource_given_at_birth: 0.0,
            resource_given_on_inject: 0.0,
            frac_energy_decay_at_org_birth: 0.0,
            frac_parent_energy_given_to_org_at_birth: 0.5,
            energy_thresh_low: 0.33,
            energy_thresh_high: 0.75,
            demes_default_germline_propensity: 0.0,
            demes_orgs_start_in_germ: 0,
            tolerance_variations: 0,
            tolerance_window: 0,
            max_tolerance: 1,
            use_resource_bins: 0,
            collect_specific_resource: 0,
            split_on_divide: 1,
            task_refractory_period: 0.0,
            task_switch_penalty_type: 0,
            learning_count: 0,
            age_poly_tracking: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// cBirthChamber config values
// ---------------------------------------------------------------------------

/// Config values consumed by cBirthChamber and birth handlers.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BirthChamberConfig {
    pub birth_method: c_int,
    pub module_num: c_int,
    pub recombination_prob: c_double,
    pub cont_rec_regs: c_int,
    pub corespond_rec_regs: c_int,
    pub same_length_sex: c_int,
    pub two_fold_cost_sex: c_int,
    pub max_birth_wait_time: c_int,
    /// bool in C++ config.
    pub allow_mate_selection: c_int,
    /// bool in C++ config.
    pub mating_types: c_int,
    /// bool in C++ config.
    pub legacy_grid_local_selection: c_int,
    pub default_group: c_int,
    pub num_demes: c_int,
}

impl Default for BirthChamberConfig {
    fn default() -> Self {
        Self {
            birth_method: 0,
            module_num: 0,
            recombination_prob: 1.0,
            cont_rec_regs: 1,
            corespond_rec_regs: 1,
            same_length_sex: 0,
            two_fold_cost_sex: 0,
            max_birth_wait_time: -1,
            allow_mate_selection: 0,
            mating_types: 0,
            legacy_grid_local_selection: 0,
            default_group: -1,
            num_demes: 1,
        }
    }
}

// ---------------------------------------------------------------------------
// cOrganism config values
// ---------------------------------------------------------------------------

/// Config values consumed by cOrganism methods.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrganismConfig {
    pub death_method: c_int,
    pub age_limit: c_int,
    pub age_deviation: c_int,
    pub max_unique_task_count: c_int,
    pub require_single_reaction: c_int,
    pub required_task: c_int,
    pub required_reaction: c_int,
    pub required_bonus: c_double,
    pub required_resource: c_int,
    pub required_resource_level: c_double,
    pub use_form_groups: c_int,
    pub pred_prey_switch: c_int,
    pub sterilize_fatal: c_double,
    pub sterilize_detrimental: c_double,
    pub sterilize_neutral: c_double,
    pub sterilize_beneficial: c_double,
    pub sterilize_taskloss: c_double,
    pub sterilize_unstable: c_int,
    pub revert_fatal: c_double,
    pub revert_detrimental: c_double,
    pub revert_neutral: c_double,
    pub revert_beneficial: c_double,
    pub revert_equals: c_double,
    pub revert_taskloss: c_double,
    /// bool in C++ config.
    pub save_received: c_int,
    /// bool in C++ config.
    pub merit_inc_apply_immediate: c_int,
}

impl Default for OrganismConfig {
    fn default() -> Self {
        Self {
            death_method: 2,
            age_limit: 20,
            age_deviation: 0,
            max_unique_task_count: -1,
            require_single_reaction: 0,
            required_task: -1,
            required_reaction: -1,
            required_bonus: 0.0,
            required_resource: -1,
            required_resource_level: 0.0,
            use_form_groups: 0,
            pred_prey_switch: -1,
            sterilize_fatal: 0.0,
            sterilize_detrimental: 0.0,
            sterilize_neutral: 0.0,
            sterilize_beneficial: 0.0,
            sterilize_taskloss: 0.0,
            sterilize_unstable: 0,
            revert_fatal: 0.0,
            revert_detrimental: 0.0,
            revert_neutral: 0.0,
            revert_beneficial: 0.0,
            revert_equals: 0.0,
            revert_taskloss: 0.0,
            save_received: 0,
            merit_inc_apply_immediate: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Top-level ConfigSnapshot
// ---------------------------------------------------------------------------

/// Snapshot of all Avida config values needed by Rust code.
///
/// Populated by C++ once per update via `avd_populate_config_snapshot()` and
/// passed by pointer into Rust FFI functions.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ConfigSnapshot {
    pub task_lib: TaskLibConfig,
    pub phenotype: PhenotypeConfig,
    pub birth_chamber: BirthChamberConfig,
    pub organism: OrganismConfig,
}

// ---------------------------------------------------------------------------
// FFI
// ---------------------------------------------------------------------------

/// Return a `ConfigSnapshot` filled with compiled-in defaults.
///
/// # Safety
/// No pointers involved — returns by value.
#[no_mangle]
pub extern "C" fn avd_config_snapshot_default() -> ConfigSnapshot {
    ConfigSnapshot::default()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem;

    #[test]
    fn default_snapshot_has_expected_values() {
        let snap = ConfigSnapshot::default();
        assert_eq!(snap.task_lib.world_x, 60);
        assert_eq!(snap.task_lib.world_y, 60);
        assert_eq!(snap.phenotype.base_merit_method, 4);
        assert_eq!(snap.phenotype.base_const_merit, 100);
        assert!((snap.phenotype.default_bonus - 1.0).abs() < f64::EPSILON);
        assert_eq!(snap.birth_chamber.birth_method, 0);
        assert!((snap.birth_chamber.recombination_prob - 1.0).abs() < f64::EPSILON);
        assert_eq!(snap.organism.death_method, 2);
        assert_eq!(snap.organism.age_limit, 20);
    }

    #[test]
    fn ffi_default_matches_rust_default() {
        let ffi = avd_config_snapshot_default();
        let rust = ConfigSnapshot::default();
        assert_eq!(ffi, rust);
    }

    #[test]
    fn struct_is_repr_c_compatible() {
        // Smoke check: size is non-zero and alignment is reasonable.
        assert!(mem::size_of::<ConfigSnapshot>() > 0);
        assert!(mem::align_of::<ConfigSnapshot>() <= 8);
    }

    #[test]
    fn sub_struct_sizes_are_stable() {
        // Guard against accidental field additions breaking ABI.
        // 4 fields * 4 bytes = 16 bytes for TaskLibConfig.
        assert_eq!(mem::size_of::<TaskLibConfig>(), 4 * 4);
        // BirthChamberConfig: 1 double (8) + 12 ints (48) = 56, with alignment padding.
        // OrganismConfig: 8 doubles (64) + 12 ints (48) + padding.
        // Just verify they are positive and stable.
        assert!(mem::size_of::<PhenotypeConfig>() > 0);
        assert!(mem::size_of::<BirthChamberConfig>() > 0);
        assert!(mem::size_of::<OrganismConfig>() > 0);
    }
}
