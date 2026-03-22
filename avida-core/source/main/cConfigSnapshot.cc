// C++ helper: populate a Rust ConfigSnapshot from cAvidaConfig.
//
// Called once per update (or on demand) to snapshot config values into the
// #[repr(C)] struct defined in Rust.

#include "cAvidaConfig.h"
#include "rust/running_stats_ffi.h"

void avd_populate_config_snapshot(ConfigSnapshot* snap, const cAvidaConfig& cfg)
{
  // -- TaskLib --
  snap->task_lib.world_x                = cfg.WORLD_X.Get();
  snap->task_lib.world_y                = cfg.WORLD_Y.Get();
  snap->task_lib.use_avatars             = cfg.USE_AVATARS.Get();
  snap->task_lib.match_already_produced  = cfg.MATCH_ALREADY_PRODUCED.Get();

  // -- Phenotype --
  snap->phenotype.base_merit_method      = cfg.BASE_MERIT_METHOD.Get();
  snap->phenotype.base_const_merit       = cfg.BASE_CONST_MERIT.Get();
  snap->phenotype.default_bonus          = cfg.DEFAULT_BONUS.Get();
  snap->phenotype.divide_method          = cfg.DIVIDE_METHOD.Get();
  snap->phenotype.generation_inc_method  = cfg.GENERATION_INC_METHOD.Get();
  snap->phenotype.merit_bonus_effect     = cfg.MERIT_BONUS_EFFECT.Get();
  snap->phenotype.merit_default_bonus    = cfg.MERIT_DEFAULT_BONUS.Get();
  snap->phenotype.fitness_method         = cfg.FITNESS_METHOD.Get();
  snap->phenotype.fitness_valley         = cfg.FITNESS_VALLEY.Get();
  snap->phenotype.fitness_valley_start   = cfg.FITNESS_VALLEY_START.Get();
  snap->phenotype.fitness_valley_stop    = cfg.FITNESS_VALLEY_STOP.Get();
  snap->phenotype.energy_enabled         = cfg.ENERGY_ENABLED.Get();
  snap->phenotype.energy_cap             = cfg.ENERGY_CAP.Get();
  snap->phenotype.apply_energy_method    = cfg.APPLY_ENERGY_METHOD.Get();
  snap->phenotype.fix_metabolic_rate     = cfg.FIX_METABOLIC_RATE.Get();
  snap->phenotype.inherit_exe_rate       = cfg.INHERIT_EXE_RATE.Get();
  snap->phenotype.inherit_merit          = cfg.INHERIT_MERIT.Get();
  snap->phenotype.inherit_multithread    = cfg.INHERIT_MULTITHREAD.Get();
  snap->phenotype.energy_given_at_birth  = cfg.ENERGY_GIVEN_AT_BIRTH.Get();
  snap->phenotype.resource_given_at_birth = cfg.RESOURCE_GIVEN_AT_BIRTH.Get();
  snap->phenotype.resource_given_on_inject = cfg.RESOURCE_GIVEN_ON_INJECT.Get();
  snap->phenotype.frac_energy_decay_at_org_birth = cfg.FRAC_ENERGY_DECAY_AT_ORG_BIRTH.Get();
  snap->phenotype.frac_parent_energy_given_to_org_at_birth = cfg.FRAC_PARENT_ENERGY_GIVEN_TO_ORG_AT_BIRTH.Get();
  snap->phenotype.energy_thresh_low      = cfg.ENERGY_THRESH_LOW.Get();
  snap->phenotype.energy_thresh_high     = cfg.ENERGY_THRESH_HIGH.Get();
  snap->phenotype.demes_default_germline_propensity = cfg.DEMES_DEFAULT_GERMLINE_PROPENSITY.Get();
  snap->phenotype.demes_orgs_start_in_germ = cfg.DEMES_ORGS_START_IN_GERM.Get();
  snap->phenotype.tolerance_variations   = cfg.TOLERANCE_VARIATIONS.Get();
  snap->phenotype.tolerance_window       = cfg.TOLERANCE_WINDOW.Get();
  snap->phenotype.max_tolerance          = cfg.MAX_TOLERANCE.Get();
  snap->phenotype.use_resource_bins      = cfg.USE_RESOURCE_BINS.Get();
  snap->phenotype.collect_specific_resource = cfg.COLLECT_SPECIFIC_RESOURCE.Get();
  snap->phenotype.split_on_divide        = cfg.SPLIT_ON_DIVIDE.Get();
  snap->phenotype.task_refractory_period = cfg.TASK_REFRACTORY_PERIOD.Get();
  snap->phenotype.task_switch_penalty_type = cfg.TASK_SWITCH_PENALTY_TYPE.Get();
  snap->phenotype.learning_count         = cfg.LEARNING_COUNT.Get();
  snap->phenotype.age_poly_tracking      = cfg.AGE_POLY_TRACKING.Get();

  // -- BirthChamber --
  snap->birth_chamber.birth_method       = cfg.BIRTH_METHOD.Get();
  snap->birth_chamber.module_num         = cfg.MODULE_NUM.Get();
  snap->birth_chamber.recombination_prob = cfg.RECOMBINATION_PROB.Get();
  snap->birth_chamber.cont_rec_regs      = cfg.CONT_REC_REGS.Get();
  snap->birth_chamber.corespond_rec_regs = cfg.CORESPOND_REC_REGS.Get();
  snap->birth_chamber.same_length_sex    = cfg.SAME_LENGTH_SEX.Get();
  snap->birth_chamber.two_fold_cost_sex  = cfg.TWO_FOLD_COST_SEX.Get();
  snap->birth_chamber.max_birth_wait_time = cfg.MAX_BIRTH_WAIT_TIME.Get();
  snap->birth_chamber.allow_mate_selection = cfg.ALLOW_MATE_SELECTION.Get();
  snap->birth_chamber.mating_types       = cfg.MATING_TYPES.Get();
  snap->birth_chamber.legacy_grid_local_selection = cfg.LEGACY_GRID_LOCAL_SELECTION.Get();
  snap->birth_chamber.default_group      = cfg.DEFAULT_GROUP.Get();
  snap->birth_chamber.num_demes          = cfg.NUM_DEMES.Get();

  // -- Organism --
  snap->organism.death_method            = cfg.DEATH_METHOD.Get();
  snap->organism.age_limit               = cfg.AGE_LIMIT.Get();
  snap->organism.age_deviation           = cfg.AGE_DEVIATION.Get();
  snap->organism.max_unique_task_count   = cfg.MAX_UNIQUE_TASK_COUNT.Get();
  snap->organism.require_single_reaction = cfg.REQUIRE_SINGLE_REACTION.Get();
  snap->organism.required_task           = cfg.REQUIRED_TASK.Get();
  snap->organism.required_reaction       = cfg.REQUIRED_REACTION.Get();
  snap->organism.required_bonus          = cfg.REQUIRED_BONUS.Get();
  snap->organism.required_resource       = cfg.REQUIRED_RESOURCE.Get();
  snap->organism.required_resource_level = cfg.REQUIRED_RESOURCE_LEVEL.Get();
  snap->organism.use_form_groups         = cfg.USE_FORM_GROUPS.Get();
  snap->organism.pred_prey_switch        = cfg.PRED_PREY_SWITCH.Get();
  snap->organism.sterilize_fatal         = cfg.STERILIZE_FATAL.Get();
  snap->organism.sterilize_detrimental   = cfg.STERILIZE_DETRIMENTAL.Get();
  snap->organism.sterilize_neutral       = cfg.STERILIZE_NEUTRAL.Get();
  snap->organism.sterilize_beneficial    = cfg.STERILIZE_BENEFICIAL.Get();
  snap->organism.sterilize_taskloss      = cfg.STERILIZE_TASKLOSS.Get();
  snap->organism.sterilize_unstable      = cfg.STERILIZE_UNSTABLE.Get();
  snap->organism.revert_fatal            = cfg.REVERT_FATAL.Get();
  snap->organism.revert_detrimental      = cfg.REVERT_DETRIMENTAL.Get();
  snap->organism.revert_neutral          = cfg.REVERT_NEUTRAL.Get();
  snap->organism.revert_beneficial       = cfg.REVERT_BENEFICIAL.Get();
  snap->organism.revert_equals           = cfg.REVERT_EQUALS.Get();
  snap->organism.revert_taskloss         = cfg.REVERT_TASKLOSS.Get();
  snap->organism.save_received           = cfg.SAVE_RECEIVED.Get();
  snap->organism.merit_inc_apply_immediate = cfg.MERIT_INC_APPLY_IMMEDIATE.Get();
}
