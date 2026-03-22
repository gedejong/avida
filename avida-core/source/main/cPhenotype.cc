/*
 *  cPhenotype.cc
 *  Avida
 *
 *  Called "phenotype.cc" prior to 12/5/05.
 *  Copyright 1999-2011 Michigan State University. All rights reserved.
 *  Copyright 1993-2003 California Institute of Technology.
 *
 *
 *  This file is part of Avida.
 *
 *  Avida is free software; you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
 *  as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 *  Avida is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser General Public License along with Avida.
 *  If not, see <http://www.gnu.org/licenses/>.
 *
 */

#include "cPhenotype.h"
#include "avida/systematics/Types.h"
#include "cContextPhenotype.h"
#include "cEnvironment.h"
#include "cDeme.h"
#include "cOrganism.h"
#include "cReactionResult.h"
#include "cTaskState.h"
#include "cWorld.h"

#include "rust/running_stats_ffi.h"
#include "tList.h"

#include <fstream>

using namespace std;


cPhenotype::cPhenotype(cWorld* world, int parent_generation, int num_nops)
: m_world(world)
, initialized(false)
, m_core(avd_pheno_core_default())
, cur_task_count(m_world->GetEnvironment().GetNumTasks())
, cur_para_tasks(m_world->GetEnvironment().GetNumTasks())
, cur_host_tasks(m_world->GetEnvironment().GetNumTasks())
, cur_internal_task_count(m_world->GetEnvironment().GetNumTasks())
, eff_task_count(m_world->GetEnvironment().GetNumTasks())
, cur_task_quality(m_world->GetEnvironment().GetNumTasks())  
, cur_task_value(m_world->GetEnvironment().GetNumTasks())  
, cur_internal_task_quality(m_world->GetEnvironment().GetNumTasks())
, cur_rbins_total(m_world->GetEnvironment().GetResourceLib().GetSize())
, cur_rbins_avail(m_world->GetEnvironment().GetResourceLib().GetSize())
, cur_reaction_count(m_world->GetEnvironment().GetReactionLib().GetSize())
, first_reaction_cycles(m_world->GetEnvironment().GetReactionLib().GetSize())
, first_reaction_execs(m_world->GetEnvironment().GetReactionLib().GetSize())
, cur_stolen_reaction_count(m_world->GetEnvironment().GetReactionLib().GetSize())
, cur_reaction_add_reward(m_world->GetEnvironment().GetReactionLib().GetSize())
, cur_sense_count(m_world->GetStats().GetSenseSize())
, sensed_resources(m_world->GetEnvironment().GetResourceLib().GetSize())
, cur_task_time(m_world->GetEnvironment().GetNumTasks())   // Added for tracking time; WRE 03-18-07
, m_tolerance_immigrants()
, m_tolerance_offspring_own()
, m_tolerance_offspring_others()
, m_intolerances((m_world->GetConfig().TOLERANCE_VARIATIONS.Get() > 0) ? 1 : 3)
, m_lifetime(avd_pheno_lifetime_default())
, m_reaction_result(NULL)
, last_task_count(m_world->GetEnvironment().GetNumTasks())
, last_para_tasks(m_world->GetEnvironment().GetNumTasks())
, last_host_tasks(m_world->GetEnvironment().GetNumTasks())
, last_internal_task_count(m_world->GetEnvironment().GetNumTasks())
, last_task_quality(m_world->GetEnvironment().GetNumTasks())
, last_task_value(m_world->GetEnvironment().GetNumTasks())
, last_internal_task_quality(m_world->GetEnvironment().GetNumTasks())
, last_rbins_total(m_world->GetEnvironment().GetResourceLib().GetSize())
, last_rbins_avail(m_world->GetEnvironment().GetResourceLib().GetSize())
, last_collect_spec_counts()
, last_reaction_count(m_world->GetEnvironment().GetReactionLib().GetSize())
, last_reaction_add_reward(m_world->GetEnvironment().GetReactionLib().GetSize())  
, last_sense_count(m_world->GetStats().GetSenseSize())

{
  m_lifetime.is_germ_cell = m_world->GetConfig().DEMES_ORGS_START_IN_GERM.Get() ? 1 : 0;
  if (parent_generation >= 0) {
    m_lifetime.generation = parent_generation;
    if (!avd_cpop_is_generation_inc_both(m_world->GetConfig().GENERATION_INC_METHOD.Get())) m_lifetime.generation++;
  }
  
  double num_resources = m_world->GetEnvironment().GetResourceLib().GetSize();
  if (num_resources <= 0 || num_nops <= 0) return;
  double most_nops_needed = ceil(log(num_resources) / log((double)num_nops));
  cur_collect_spec_counts.Resize(int((pow((double)num_nops, most_nops_needed + 1.0) - 1.0) / ((double)num_nops - 1.0)));
}

cPhenotype::~cPhenotype()
{
  // Remove Task States
  for (Apto::Map<void*, cTaskState*>::ValueIterator it = m_task_states.Values(); it.Next();) delete (*it.Get());
  delete m_reaction_result;
}


cPhenotype::cPhenotype(const cPhenotype& in_phen) : m_reaction_result(NULL)
{
  *this = in_phen;
}


cPhenotype& cPhenotype::operator=(const cPhenotype& in_phen)
{
  
  m_world                  = in_phen.m_world;
  initialized              = in_phen.initialized;
  
  
  // 1+2. Core metrics (single struct copy)
  m_core                   = in_phen.m_core;
  energy_tobe_applied      = in_phen.energy_tobe_applied;
  energy_testament         = in_phen.energy_testament;
  energy_received_buffer   = in_phen.energy_received_buffer;

  // 2+3+4+7. Lifetime data (single struct copy)
  m_lifetime               = in_phen.m_lifetime;
  fault_desc               = in_phen.fault_desc;

  cur_task_count           = in_phen.cur_task_count;
  cur_para_tasks           = in_phen.cur_para_tasks;
  cur_host_tasks           = in_phen.cur_host_tasks;
  eff_task_count           = in_phen.eff_task_count;
  cur_internal_task_count  = in_phen.cur_internal_task_count;
  cur_task_quality         = in_phen.cur_task_quality;
  cur_internal_task_quality= in_phen.cur_internal_task_quality;
  cur_task_value           = in_phen.cur_task_value;
  cur_rbins_total          = in_phen.cur_rbins_total;
  cur_rbins_avail          = in_phen.cur_rbins_avail;
  cur_collect_spec_counts  = in_phen.cur_collect_spec_counts;
  cur_reaction_count       = in_phen.cur_reaction_count;
  first_reaction_cycles    = in_phen.first_reaction_cycles;
  first_reaction_execs     = in_phen.first_reaction_execs;
  cur_reaction_add_reward  = in_phen.cur_reaction_add_reward;
  cur_inst_count           = in_phen.cur_inst_count;
  cur_from_sensor_count    = in_phen.cur_from_sensor_count;
  cur_group_attack_count    = in_phen.cur_group_attack_count;
  cur_top_pred_group_attack_count    = in_phen.cur_top_pred_group_attack_count;
  cur_killed_targets       = in_phen.cur_killed_targets;
  cur_attacks              = in_phen.cur_attacks;
  cur_kills                 = in_phen.cur_kills;
  cur_sense_count          = in_phen.cur_sense_count;
  sensed_resources         = in_phen.sensed_resources;
  cur_task_time            = in_phen.cur_task_time;
  m_tolerance_immigrants          = in_phen.m_tolerance_immigrants;
  m_tolerance_offspring_own       = in_phen.m_tolerance_offspring_own;
  m_tolerance_offspring_others    = in_phen.m_tolerance_offspring_others;
  m_intolerances                  = in_phen.m_intolerances;
  cur_stolen_reaction_count       = in_phen.cur_stolen_reaction_count;

  cur_from_message_count    = in_phen.cur_from_message_count;

  // Dynamically allocated m_task_states requires special handling
  for (Apto::Map<void*, cTaskState*>::ConstIterator it = in_phen.m_task_states.Begin(); it.Next();) {
    cTaskState* new_ts = new cTaskState(**((*it.Get()).Value2()));
    m_task_states.Set((*it.Get()).Value1(), new_ts);
  }

  // 3. Dynamic arrays from "last divide"
  last_task_count          = in_phen.last_task_count;
  last_host_tasks          = in_phen.last_host_tasks;
  last_para_tasks          = in_phen.last_para_tasks;
  last_internal_task_count = in_phen.last_internal_task_count;
  last_task_quality        = in_phen.last_task_quality;
  last_internal_task_quality=in_phen.last_internal_task_quality;
  last_task_value          = in_phen.last_task_value;
  last_rbins_total         = in_phen.last_rbins_total;
  last_rbins_avail         = in_phen.last_rbins_avail;
  last_collect_spec_counts = in_phen.last_collect_spec_counts;
  last_reaction_count      = in_phen.last_reaction_count;
  last_reaction_add_reward = in_phen.last_reaction_add_reward;
  last_inst_count          = in_phen.last_inst_count;
  last_from_sensor_count   = in_phen.last_from_sensor_count;
  last_group_attack_count   = in_phen.last_group_attack_count;
  last_top_pred_group_attack_count   = in_phen.last_top_pred_group_attack_count;
  last_killed_targets      = in_phen.last_killed_targets;
  last_attacks             = in_phen.last_attacks;
  last_kills                = in_phen.last_kills;
  last_sense_count         = in_phen.last_sense_count;
  total_energy_donated     = in_phen.total_energy_donated;
  total_energy_received    = in_phen.total_energy_received;
  total_energy_applied     = in_phen.total_energy_applied;

  last_from_message_count   = in_phen.last_from_message_count;

  // 5+6. Status flags + child info (single struct copy)
  m_flags                 = in_phen.m_flags;
  is_donor_locus          = in_phen.is_donor_locus;
  is_donor_locus_last     = in_phen.is_donor_locus_last;

  total_energy_donated    = in_phen.total_energy_donated;
  total_energy_received   = in_phen.total_energy_received;
  total_energy_applied    = in_phen.total_energy_applied;
  
  return *this;
}


/**
 * This function is run whenever a new organism is being constructed inside
 * of its parent.
 *
 * Assumptions:
 *     - parent_phenotype has had DivideReset run on it already!
 *     - this is the first method run on an otherwise freshly built phenotype.
 **/

void cPhenotype::SetupOffspring(const cPhenotype& parent_phenotype, const InstructionSequence& _genome)
{
  // Copy divide values from parent, which should already be setup.
  reinterpret_cast<cMerit&>(m_core.merit) = reinterpret_cast<const cMerit&>(parent_phenotype.m_core.merit);
  if(m_world->GetConfig().INHERIT_EXE_RATE.Get() == 0)
    m_core.execution_ratio = 1.0;
  else
    m_core.execution_ratio = parent_phenotype.m_core.execution_ratio;

  m_core.energy_store    = min(m_core.energy_store, (double)m_world->GetConfig().ENERGY_CAP.Get());
  energy_tobe_applied = 0.0;
  energy_testament = 0.0;
  energy_received_buffer = 0.0;
  m_core.genome_length   = _genome.GetSize();
  m_core.copied_size     = parent_phenotype.m_flags.child_copied_size;
  m_core.executed_size   = parent_phenotype.m_core.executed_size;

  m_core.gestation_time  = parent_phenotype.m_core.gestation_time;
  m_core.gestation_start = 0;
  m_lifetime.cpu_cycles_used = 0;
  m_core.fitness         = parent_phenotype.m_core.fitness;
  m_core.div_type        = parent_phenotype.m_core.div_type;

  assert(m_core.genome_length > 0);
  assert(m_core.copied_size > 0);
  assert(m_core.gestation_time >= 0); //@JEB 0 valid for some fitness methods
  assert(m_core.div_type > 0);

  // Initialize current values, as neeeded.
  m_core.cur_bonus       = m_world->GetConfig().DEFAULT_BONUS.Get();
  m_core.cur_energy_bonus = 0.0;
  m_lifetime.cur_num_errors  = 0;
  m_lifetime.cur_num_donates  = 0;
  cur_task_count.SetAll(0);
  cur_internal_task_count.SetAll(0);
  eff_task_count.SetAll(0);
  cur_host_tasks.SetAll(0);
  cur_para_tasks.SetAll(0);
  cur_task_quality.SetAll(0);
  cur_task_value.SetAll(0);
  cur_internal_task_quality.SetAll(0);
  cur_rbins_total.SetAll(0);  // total resources collected in lifetime
  // parent's resources have already been halved or reset in DivideReset;
  // offspring gets that value (half or 0) too.
  cur_rbins_avail.SetAll(0);
  if (m_world->GetConfig().SPLIT_ON_DIVIDE.Get()) {
    for (int i = 0; i < cur_rbins_avail.GetSize(); i++) cur_rbins_avail[i] = parent_phenotype.cur_rbins_avail[i];
  }
  if (m_world->GetConfig().RESOURCE_GIVEN_AT_BIRTH.Get() > 0.0) {
    const int resource = m_world->GetConfig().COLLECT_SPECIFIC_RESOURCE.Get();
    cur_rbins_avail[resource] += m_world->GetConfig().RESOURCE_GIVEN_AT_BIRTH.Get();
  }
  
  cur_collect_spec_counts.SetAll(0);
  cur_reaction_count.SetAll(0);
  first_reaction_cycles.SetAll(-1);
  first_reaction_execs.SetAll(-1);
  cur_stolen_reaction_count.SetAll(0);
  cur_reaction_add_reward.SetAll(0);
  cur_inst_count.SetAll(0);
  cur_from_sensor_count.SetAll(0);
  cur_from_message_count.SetAll(0);
  for (int r = 0; r < cur_group_attack_count.GetSize(); r++) {
    cur_group_attack_count[r].SetAll(0);
    cur_top_pred_group_attack_count[r].SetAll(0);
  }
  cur_killed_targets.SetAll(0);
  cur_attacks = 0;
  cur_kills = 0;
  cur_sense_count.SetAll(0);
  cur_task_time.SetAll(0.0);  // Added for time tracking; WRE 03-18-07
  for (int j = 0; j < sensed_resources.GetSize(); j++) {
    sensed_resources[j] =  parent_phenotype.sensed_resources[j];
  }
  cur_trial_fitnesses.Resize(0); 
  cur_trial_bonuses.Resize(0); 
  cur_trial_times_used.Resize(0); 
  m_lifetime.trial_time_used = 0;
  m_lifetime.trial_cpu_cycles_used = 0;
  m_tolerance_immigrants.Clear();        
  m_tolerance_offspring_own.Clear();     
  m_tolerance_offspring_others.Clear();  
  m_intolerances.SetAll(make_pair(-1, -1));  
  m_lifetime.cur_child_germline_propensity = m_world->GetConfig().DEMES_DEFAULT_GERMLINE_PROPENSITY.Get();
  m_lifetime.mating_type = MATING_TYPE_JUVENILE; //@CHC
  m_lifetime.mate_preference = MATE_PREFERENCE_RANDOM; //@CHC
  
  m_lifetime.cur_mating_display_a = 0;
  m_lifetime.cur_mating_display_b = 0;
  m_lifetime.last_mating_display_a = 0;
  m_lifetime.last_mating_display_b = 0;
  
  // Copy last values from parent
  m_lifetime.last_merit_base          = parent_phenotype.m_lifetime.last_merit_base;
  m_lifetime.last_bonus               = parent_phenotype.m_lifetime.last_bonus;
  m_lifetime.last_cpu_cycles_used      = parent_phenotype.m_lifetime.last_cpu_cycles_used;
  m_lifetime.last_num_errors           = parent_phenotype.m_lifetime.last_num_errors;
  m_lifetime.last_num_donates          = parent_phenotype.m_lifetime.last_num_donates;
  last_task_count           = parent_phenotype.last_task_count;
  last_host_tasks           = parent_phenotype.last_host_tasks;
  last_para_tasks           = parent_phenotype.last_para_tasks;
  last_internal_task_count  = parent_phenotype.last_internal_task_count;
  last_task_quality         = parent_phenotype.last_task_quality;
  last_task_value           = parent_phenotype.last_task_value;
  last_internal_task_quality= parent_phenotype.last_internal_task_quality;
  last_rbins_total          = parent_phenotype.last_rbins_total;
  last_rbins_avail          = parent_phenotype.last_rbins_avail;
  last_collect_spec_counts  = parent_phenotype.last_collect_spec_counts;
  last_reaction_count       = parent_phenotype.last_reaction_count;
  last_reaction_add_reward  = parent_phenotype.last_reaction_add_reward;
  last_inst_count           = parent_phenotype.last_inst_count;
  last_from_sensor_count    = parent_phenotype.last_from_sensor_count;
  last_group_attack_count    = parent_phenotype.last_group_attack_count;
  last_top_pred_group_attack_count    = parent_phenotype.last_top_pred_group_attack_count;
  last_killed_targets       = parent_phenotype.last_killed_targets;
  last_attacks              = parent_phenotype.last_attacks;
  last_kills                = parent_phenotype.last_kills;
  last_sense_count          = parent_phenotype.last_sense_count;
  m_lifetime.last_fitness              = CalcFitness(m_lifetime.last_merit_base, m_lifetime.last_bonus, m_core.gestation_time, m_lifetime.last_cpu_cycles_used);
  m_lifetime.last_child_germline_propensity = parent_phenotype.m_lifetime.last_child_germline_propensity;   // chance of child being a germline cell; @JEB

  last_from_message_count    = parent_phenotype.last_from_message_count;

  // Setup other miscellaneous values...
  m_lifetime.num_divides     = 0;
  m_lifetime.num_divides_failed = 0;
  m_lifetime.generation      = parent_phenotype.m_lifetime.generation;
  if (!avd_cpop_is_generation_inc_both(m_world->GetConfig().GENERATION_INC_METHOD.Get())) m_lifetime.generation++;
  m_lifetime.cpu_cycles_used = 0;
  m_lifetime.time_used       = 0;
  m_lifetime.num_execs       = 0;
  m_lifetime.age             = 0;
  fault_desc      = "";
  m_lifetime.neutral_metric  = parent_phenotype.m_lifetime.neutral_metric + m_world->GetRandom().GetRandNormal();
  m_lifetime.life_fitness    = m_core.fitness;
  m_lifetime.exec_time_born  = parent_phenotype.m_lifetime.exec_time_born;  //@MRR treating offspring and parent as siblings; already set in DivideReset
  m_lifetime.birth_update    = parent_phenotype.m_lifetime.birth_update;    
  m_lifetime.num_new_unique_reactions = 0;
  m_lifetime.last_task_id             = -1;
  m_lifetime.res_consumed             = 0;
  m_lifetime.is_germ_cell             = parent_phenotype.m_lifetime.is_germ_cell;
  m_lifetime.last_task_time           = 0; 
  
  m_flags.num_thresh_gb_donations = 0;
  m_flags.num_thresh_gb_donations_last = parent_phenotype.m_flags.num_thresh_gb_donations_last;
  m_flags.num_quanta_thresh_gb_donations = 0;
  m_flags.num_quanta_thresh_gb_donations_last = parent_phenotype.m_flags.num_thresh_gb_donations_last;
  m_flags.num_shaded_gb_donations = 0;
  m_flags.num_shaded_gb_donations_last = parent_phenotype.m_flags.num_shaded_gb_donations_last;
  m_flags.num_donations_locus = 0;
  m_flags.num_donations_locus_last = parent_phenotype.m_flags.num_donations_locus_last;

  // Setup flags...
  m_flags.is_injected   = 0;
  m_flags.is_clone   = 0;
  m_flags.is_donor_cur  = 0;
  m_flags.is_donor_last = parent_phenotype.m_flags.is_donor_last;
  m_flags.is_donor_rand = 0;
  m_flags.is_donor_rand_last = parent_phenotype.m_flags.is_donor_rand_last;
  m_flags.is_donor_null = 0;
  m_flags.is_donor_null_last = parent_phenotype.m_flags.is_donor_null_last;
  m_flags.is_donor_kin  = 0;
  m_flags.is_donor_kin_last = parent_phenotype.m_flags.is_donor_kin_last;
  m_flags.is_donor_edit  = 0;
  m_flags.is_donor_edit_last = parent_phenotype.m_flags.is_donor_edit_last;
  m_flags.is_donor_gbg  = 0;
  m_flags.is_donor_gbg_last = parent_phenotype.m_flags.is_donor_gbg_last;
  m_flags.is_donor_truegb  = 0;
  m_flags.is_donor_truegb_last = parent_phenotype.m_flags.is_donor_truegb_last;
  m_flags.is_donor_threshgb  = 0;
  m_flags.is_donor_threshgb_last = parent_phenotype.m_flags.is_donor_threshgb_last;
  m_flags.is_donor_quanta_threshgb  = 0;
  m_flags.is_donor_quanta_threshgb_last = parent_phenotype.m_flags.is_donor_quanta_threshgb_last;
  m_flags.is_donor_shadedgb  = 0;
  m_flags.is_donor_shadedgb_last = parent_phenotype.m_flags.is_donor_shadedgb_last;
  is_donor_locus.SetAll(false);
  is_donor_locus_last = parent_phenotype.is_donor_locus_last;

  m_flags.is_receiver   = 0;
  m_flags.is_receiver_last = parent_phenotype.m_flags.is_receiver_last;
  m_flags.is_receiver_rand   = 0;
  m_flags.is_receiver_kin    = 0;
  m_flags.is_receiver_kin_last    = parent_phenotype.m_flags.is_receiver_kin_last;
  m_flags.is_receiver_edit   = 0;
  m_flags.is_receiver_edit_last    = parent_phenotype.m_flags.is_receiver_edit_last;
  m_flags.is_receiver_gbg    = 0;
  m_flags.is_receiver_truegb = 0;
  m_flags.is_receiver_truegb_last = parent_phenotype.m_flags.is_receiver_truegb_last;
  m_flags.is_receiver_threshgb = 0;
  m_flags.is_receiver_threshgb_last = parent_phenotype.m_flags.is_receiver_threshgb_last;
  m_flags.is_receiver_quanta_threshgb = 0;
  m_flags.is_receiver_quanta_threshgb_last = parent_phenotype.m_flags.is_receiver_quanta_threshgb_last;
  m_flags.is_receiver_shadedgb = 0;
  m_flags.is_receiver_shadedgb_last = parent_phenotype.m_flags.is_receiver_shadedgb_last;
  m_flags.is_receiver_gb_same_locus = 0;
  m_flags.is_receiver_gb_same_locus_last = parent_phenotype.m_flags.is_receiver_gb_same_locus;

  m_flags.is_modifier   = 0;
  m_flags.is_modified   = 0;
  m_flags.is_fertile    = parent_phenotype.m_flags.last_child_fertile;
  m_flags.is_mutated    = 0;
  m_flags.kaboom_executed = 0;
  m_flags.kaboom_executed2 = 0;
  if (m_world->GetConfig().INHERIT_MULTITHREAD.Get()) {
    m_flags.is_multi_thread = parent_phenotype.m_flags.is_multi_thread;
  } else {
    m_flags.is_multi_thread = 0;
  }

  m_flags.parent_true   = parent_phenotype.m_flags.copy_true;
  m_flags.parent_sex    = parent_phenotype.m_flags.divide_sex;
  m_flags.parent_cross_num    = parent_phenotype.m_flags.cross_num;
  m_flags.make_random_resource = 0;
  m_flags.to_die = 0;
  m_flags.to_delete = 0;

  m_flags.is_energy_requestor = 0;
  m_flags.is_energy_donor = 0;
  m_flags.is_energy_receiver = 0;
  m_flags.has_used_donated_energy = 0;
  m_flags.has_open_energy_request = 0;
  total_energy_donated = 0.0;
  total_energy_received = 0.0;
  total_energy_applied = 0.0;

  // Setup child info...
  m_flags.copy_true          = 0;
  m_flags.divide_sex         = 0;
  m_flags.mate_select_id     = -1;
  m_flags.cross_num          = 0;
  m_flags.last_child_fertile = m_flags.is_fertile;
  m_flags.child_fertile      = 1;
  m_flags.child_copied_size  = 0;
  
  // permanently set germline propensity of org (since DivideReset is called first, it is now in the "last" slot...)
  m_lifetime.permanent_germline_propensity  = parent_phenotype.m_lifetime.last_child_germline_propensity;
  
  initialized = true;
}


/**
 * This function is run whenever a new organism is being constructed via
 * some form of injection into the population, or in a test environment.
 *
 * Assumptions:
 *     - Updates to these values (i.e. resetting of merit) will occur afterward
 *     - This is the first method run on an otherwise freshly built phenotype.
 **/
void cPhenotype::SetupInject(const InstructionSequence& _genome)
{
  // Setup reasonable initial values injected organism...
  m_core.genome_length   = _genome.GetSize();
  avd_merit_set(&m_core.merit, m_core.genome_length);
  m_core.copied_size     = m_core.genome_length;
  m_core.executed_size   = m_core.genome_length;
  m_core.energy_store    = min(m_world->GetConfig().ENERGY_GIVEN_ON_INJECT.Get(), m_world->GetConfig().ENERGY_CAP.Get());
  energy_tobe_applied = 0.0;
  energy_testament = 0.0;
  energy_received_buffer = 0.0;
  m_core.execution_ratio = 1.0;
  m_core.gestation_time  = 0;
  m_core.gestation_start = 0;
  m_core.fitness         = 0;
  m_core.div_type        = 1;

  // Initialize current values, as neeeded.
  m_core.cur_bonus       = m_world->GetConfig().DEFAULT_BONUS.Get();
  m_core.cur_energy_bonus = 0.0;
  m_lifetime.cur_num_errors  = 0;
  m_lifetime.cur_num_donates  = 0;
  cur_task_count.SetAll(0);
  cur_para_tasks.SetAll(0);
  cur_host_tasks.SetAll(0);
  cur_internal_task_count.SetAll(0);
  eff_task_count.SetAll(0);
  cur_task_quality.SetAll(0);
  cur_task_value.SetAll(0);
  cur_internal_task_quality.SetAll(0);
  cur_rbins_total.SetAll(0);
  if (m_world->GetConfig().RESOURCE_GIVEN_ON_INJECT.Get() > 0.0) {   
    const int resource = m_world->GetConfig().COLLECT_SPECIFIC_RESOURCE.Get();
    cur_rbins_avail[resource] = m_world->GetConfig().RESOURCE_GIVEN_ON_INJECT.Get();
  }
  else cur_rbins_avail.SetAll(0);
  cur_collect_spec_counts.SetAll(0);
  cur_reaction_count.SetAll(0);
  first_reaction_cycles.SetAll(-1);
  first_reaction_execs.SetAll(-1);
  cur_stolen_reaction_count.SetAll(0);
  cur_reaction_add_reward.SetAll(0);
  cur_inst_count.SetAll(0);
  cur_from_sensor_count.SetAll(0);
  cur_from_message_count.SetAll(0);
  for (int r = 0; r < cur_group_attack_count.GetSize(); r++) {
    cur_group_attack_count[r].SetAll(0);
    cur_top_pred_group_attack_count[r].SetAll(0);
  }
  cur_killed_targets.SetAll(0);
  cur_attacks = 0;
  cur_kills = 0;
  sensed_resources.SetAll(0);
  cur_sense_count.SetAll(0);
  cur_task_time.SetAll(0.0);
  cur_trial_fitnesses.Resize(0);
  cur_trial_bonuses.Resize(0); 
  cur_trial_times_used.Resize(0); 
  m_lifetime.trial_time_used = 0;
  m_lifetime.trial_cpu_cycles_used = 0;
  m_tolerance_immigrants.Clear();        
  m_tolerance_offspring_own.Clear();     
  m_tolerance_offspring_others.Clear();  
  m_intolerances.SetAll(make_pair(-1, -1));  
  m_lifetime.cur_child_germline_propensity = m_world->GetConfig().DEMES_DEFAULT_GERMLINE_PROPENSITY.Get();
  m_lifetime.mating_type = MATING_TYPE_JUVENILE; // @CHC
  m_lifetime.mate_preference = MATE_PREFERENCE_RANDOM; //@CHC
  
  // New organism has no parent and so cannot use its last values; initialize as needed
  m_lifetime.last_merit_base= m_core.genome_length;
  m_lifetime.last_bonus     = 1;
  m_lifetime.last_cpu_cycles_used = 0;
  m_lifetime.last_num_errors = 0;
  m_lifetime.last_num_donates = 0;
  last_task_count.SetAll(0);
  last_host_tasks.SetAll(0);
  last_para_tasks.SetAll(0);
  last_internal_task_count.SetAll(0);
  last_task_quality.SetAll(0);
  last_task_value.SetAll(0);
  last_internal_task_quality.SetAll(0);
  last_rbins_total.SetAll(0);
  last_rbins_avail.SetAll(0);
  last_collect_spec_counts.SetAll(0);
  last_reaction_count.SetAll(0);
  last_reaction_add_reward.SetAll(0);
  last_inst_count.SetAll(0);
  last_from_sensor_count.SetAll(0);
  last_from_message_count.SetAll(0);
  for (int r = 0; r < last_group_attack_count.GetSize(); r++) {
    last_group_attack_count[r].SetAll(0);
    last_top_pred_group_attack_count[r].SetAll(0);
  }
  last_killed_targets.SetAll(0);
  last_attacks = 0;
  last_kills = 0;
  last_sense_count.SetAll(0);
  m_lifetime.last_child_germline_propensity = m_world->GetConfig().DEMES_DEFAULT_GERMLINE_PROPENSITY.Get();
  
  // Setup other miscellaneous values...
  m_lifetime.num_divides     = 0;
  m_lifetime.num_divides_failed = 0;
  m_lifetime.generation      = 0;
  m_lifetime.cpu_cycles_used = 0;
  m_lifetime.time_used       = 0;
  m_lifetime.num_execs       = 0;
  m_lifetime.age             = 0;
  fault_desc      = "";
  m_lifetime.neutral_metric  = 0;
  m_lifetime.life_fitness    = 0;
  m_lifetime.exec_time_born  = 0;
  m_lifetime.birth_update     = m_world->GetStats().GetUpdate();
  
  // Reset all flags to defaults, then set inject-specific values.
  m_flags = avd_pheno_flags_default();
  m_flags.is_injected   = 1;
  m_flags.is_fertile    = 1;
  m_flags.parent_true   = 1;
  m_flags.child_fertile = 1;
  m_flags.last_child_fertile = 1;
  m_flags.mate_select_id = -1;
  is_donor_locus.SetAll(false);
  is_donor_locus_last.SetAll(false);

  total_energy_donated = 0.0;
  total_energy_received = 0.0;
  total_energy_applied = 0.0;
  
  m_lifetime.permanent_germline_propensity = m_world->GetConfig().DEMES_DEFAULT_GERMLINE_PROPENSITY.Get();
  
  initialized = true;
}


void cPhenotype::ResetMerit()
{
  //LZ This was an int!
  double cur_merit_base = CalcSizeMerit();
  //LZ
  const double merit_default_bonus = m_world->GetConfig().MERIT_DEFAULT_BONUS.Get();
  if (merit_default_bonus) {
    m_core.cur_bonus = merit_default_bonus;
  }
  avd_merit_set(&m_core.merit, cur_merit_base * m_core.cur_bonus);

  if (m_world->GetConfig().INHERIT_MERIT.Get() == 0) {
    avd_merit_set(&m_core.merit, cur_merit_base);
  }
}


/**
 * This function is run whenever an organism executes a successful divide.
 **/
void cPhenotype::DivideReset(const InstructionSequence& _genome)
{
  assert(m_lifetime.time_used >= 0);
  assert(initialized == true);
  
  // Update these values as needed...
  //LZ This was an int!
  double cur_merit_base = CalcSizeMerit();
  
  // If we are resetting the current merit, do it here
  // and it will also be propagated to the child
  //LZ
  const double merit_default_bonus = m_world->GetConfig().MERIT_DEFAULT_BONUS.Get();
  if (merit_default_bonus) {
    m_core.cur_bonus = merit_default_bonus;
  }
  avd_merit_set(&m_core.merit, cur_merit_base * m_core.cur_bonus);

  if(m_world->GetConfig().INHERIT_MERIT.Get() == 0)
    avd_merit_set(&m_core.merit, cur_merit_base);

  SetEnergy(m_core.energy_store + m_core.cur_energy_bonus);
  m_world->GetStats().SumEnergyTestamentAcceptedByOrganisms().Add(energy_testament);
  energy_testament = 0.0;
  energy_received_buffer = 0.0;  // If donated energy not applied, it's lost here

  m_core.genome_length   = _genome.GetSize();
  (void) m_core.copied_size;          // Unchanged
  (void) m_core.executed_size;        // Unchanged
  m_core.gestation_time  = m_lifetime.time_used - m_core.gestation_start;
  m_core.gestation_start = m_lifetime.time_used;
  m_core.fitness = CalcFitness( cur_merit_base, m_core.cur_bonus, m_core.gestation_time, m_lifetime.cpu_cycles_used);

  // Lock in cur values as last values.
  m_lifetime.last_merit_base          = cur_merit_base;
  m_lifetime.last_bonus               = m_core.cur_bonus;
  m_lifetime.last_cpu_cycles_used      = m_lifetime.cpu_cycles_used;
  //TODO?  last_energy         = m_core.cur_energy_bonus;
  m_lifetime.last_num_errors           = m_lifetime.cur_num_errors;
  m_lifetime.last_num_donates          = m_lifetime.cur_num_donates;
  last_task_count           = cur_task_count;
  last_host_tasks           = cur_host_tasks;
  last_para_tasks           = cur_para_tasks;
  last_internal_task_count  = cur_internal_task_count;
  last_task_quality         = cur_task_quality;
  last_task_value           = cur_task_value;
  last_internal_task_quality= cur_internal_task_quality;
  last_rbins_total          = cur_rbins_total;
  last_rbins_avail          = cur_rbins_avail;
  last_collect_spec_counts  = cur_collect_spec_counts;
  last_reaction_count       = cur_reaction_count;
  last_reaction_add_reward  = cur_reaction_add_reward;
  last_inst_count           = cur_inst_count;
  last_from_sensor_count    = cur_from_sensor_count;
  last_from_message_count    = cur_from_message_count;
  last_group_attack_count   = cur_group_attack_count;
  last_killed_targets       = cur_killed_targets;
  last_attacks              = cur_attacks;
  last_kills                = cur_kills;
  last_top_pred_group_attack_count    = cur_top_pred_group_attack_count;
  last_sense_count          = cur_sense_count;
  m_lifetime.last_child_germline_propensity = m_lifetime.cur_child_germline_propensity;
  
  m_lifetime.last_mating_display_a = m_lifetime.cur_mating_display_a; //@CHC
  m_lifetime.last_mating_display_b = m_lifetime.cur_mating_display_b;
  
  // Reset cur values.
  m_core.cur_bonus       = m_world->GetConfig().DEFAULT_BONUS.Get();
  m_lifetime.cpu_cycles_used = 0;
  m_core.cur_energy_bonus = 0.0;
  m_lifetime.cur_num_errors  = 0;
  m_lifetime.cur_num_donates  = 0;
  cur_task_count.SetAll(0);
  cur_host_tasks.SetAll(0);

  m_lifetime.cur_mating_display_a = 0; //@CHC
  m_lifetime.cur_mating_display_b = 0;

  // @LZ: figure out when and where to reset cur_para_tasks, depending on the divide method, and
  //      resonable assumptions
  if (avd_cpop_is_divide_method_split(m_world->GetConfig().DIVIDE_METHOD.Get())) {
    last_para_tasks = cur_para_tasks;
    cur_para_tasks.SetAll(0);
  }
  cur_internal_task_count.SetAll(0);
  eff_task_count.SetAll(0);
  cur_task_quality.SetAll(0);
  cur_task_value.SetAll(0);
  cur_internal_task_quality.SetAll(0);
  if (m_world->GetConfig().SPLIT_ON_DIVIDE.Get()) {
    // resources available are split in half -- the offspring gets the other half
    for (int i = 0; i < cur_rbins_avail.GetSize(); i++) {cur_rbins_avail[i] /= 2.0;}
  } else if (m_world->GetConfig().DIVIDE_METHOD.Get() != 0) {
    cur_rbins_avail.SetAll(0);
    cur_rbins_total.SetAll(0);  // total resources collected in lifetime
    
    if (m_world->GetConfig().RESOURCE_GIVEN_AT_BIRTH.Get() > 0.0) {
      const int resource = m_world->GetConfig().COLLECT_SPECIFIC_RESOURCE.Get();
      cur_rbins_avail[resource] += m_world->GetConfig().RESOURCE_GIVEN_AT_BIRTH.Get();
    }
  }
  cur_collect_spec_counts.SetAll(0);
  cur_reaction_count.SetAll(0);
  first_reaction_cycles.SetAll(-1);
  first_reaction_execs.SetAll(-1);
  cur_stolen_reaction_count.SetAll(0);
  cur_reaction_add_reward.SetAll(0);
  cur_inst_count.SetAll(0);
  cur_from_sensor_count.SetAll(0);
  cur_from_message_count.SetAll(0);
  for (int r = 0; r < cur_group_attack_count.GetSize(); r++) {
    cur_group_attack_count[r].SetAll(0);
    cur_top_pred_group_attack_count[r].SetAll(0);
  }
  cur_killed_targets.SetAll(0);
  cur_attacks = 0;
  cur_kills = 0;
  cur_sense_count.SetAll(0);
  cur_task_time.SetAll(0.0);
  m_lifetime.cur_child_germline_propensity = m_world->GetConfig().DEMES_DEFAULT_GERMLINE_PROPENSITY.Get();
  
  // Setup other miscellaneous values...
  m_lifetime.num_divides++;
  (void) m_lifetime.generation;
  (void) m_lifetime.time_used;
  m_lifetime.num_execs       = 0;
  m_lifetime.age             = 0;
  fault_desc      = "";
  (void) m_lifetime.neutral_metric;
  m_lifetime.life_fitness = m_core.fitness;
  m_lifetime.exec_time_born += m_core.gestation_time;  //@MRR Treating organism as sibling
  m_lifetime.birth_update = m_world->GetStats().GetUpdate();
  m_lifetime.num_new_unique_reactions = 0;
  m_lifetime.last_task_id             = -1;
  m_lifetime.res_consumed             = 0;
  m_lifetime.last_task_time           = 0;

  m_flags.num_thresh_gb_donations_last = m_flags.num_thresh_gb_donations;
  m_flags.num_thresh_gb_donations = 0;
  m_flags.num_quanta_thresh_gb_donations_last = m_flags.num_quanta_thresh_gb_donations;
  m_flags.num_quanta_thresh_gb_donations = 0;
  m_flags.num_shaded_gb_donations_last = m_flags.num_shaded_gb_donations;
  m_flags.num_shaded_gb_donations = 0;
  m_flags.num_donations_locus_last = m_flags.num_donations_locus;
  m_flags.num_donations_locus = 0;

  // Leave flags alone...
  (void) m_flags.is_injected;
  m_flags.is_clone = 0; // has legitimately reproduced
  m_flags.is_donor_last = m_flags.is_donor_cur;
  m_flags.is_donor_cur = 0;
  m_flags.is_donor_rand_last = m_flags.is_donor_rand;
  m_flags.is_donor_rand = 0;
  m_flags.is_donor_null_last = m_flags.is_donor_null;
  m_flags.is_donor_null = 0;
  m_flags.is_donor_kin_last = m_flags.is_donor_kin;
  m_flags.is_donor_kin = 0;
  m_flags.is_donor_edit_last = m_flags.is_donor_edit;
  m_flags.is_donor_edit = 0;
  m_flags.is_donor_gbg_last = m_flags.is_donor_gbg;
  m_flags.is_donor_gbg = 0;
  m_flags.is_donor_truegb_last = m_flags.is_donor_truegb;
  m_flags.is_donor_truegb = 0;
  m_flags.is_donor_threshgb_last = m_flags.is_donor_threshgb;
  m_flags.is_donor_threshgb = 0;
  m_flags.is_donor_quanta_threshgb_last = m_flags.is_donor_quanta_threshgb;
  m_flags.is_donor_quanta_threshgb = 0;
  m_flags.is_donor_shadedgb_last = m_flags.is_donor_shadedgb;
  m_flags.is_donor_shadedgb = 0;
  is_donor_locus_last = is_donor_locus;
  is_donor_locus.SetAll(false);

  m_flags.is_receiver_last = m_flags.is_receiver;
  m_flags.is_receiver = 0;
  m_flags.is_receiver_rand = 0;
  m_flags.is_receiver_kin_last = m_flags.is_receiver_kin;
  m_flags.is_receiver_kin = 0;
  m_flags.is_receiver_edit_last = m_flags.is_receiver_edit;
  m_flags.is_receiver_edit = 0;
  m_flags.is_receiver_gbg = 0;
  m_flags.is_receiver_truegb_last = m_flags.is_receiver_truegb;
  m_flags.is_receiver_truegb = 0;
  m_flags.is_receiver_threshgb_last = m_flags.is_receiver_threshgb;
  m_flags.is_receiver_threshgb = 0;
  m_flags.is_receiver_quanta_threshgb_last = m_flags.is_receiver_quanta_threshgb;
  m_flags.is_receiver_quanta_threshgb = 0;
  m_flags.is_receiver_shadedgb_last = m_flags.is_receiver_shadedgb;
  m_flags.is_receiver_shadedgb = 0;
  m_flags.is_receiver_gb_same_locus_last = m_flags.is_receiver_gb_same_locus;
  m_flags.is_receiver_gb_same_locus = 0;

  (void) m_flags.is_modifier;
  (void) m_flags.is_modified;
  (void) m_flags.is_fertile;
  (void) m_flags.is_mutated;
  (void) m_flags.is_multi_thread;
  (void) m_flags.parent_true;
  (void) m_flags.parent_sex;
  (void) m_flags.parent_cross_num;
  (void) m_flags.kaboom_executed;
  (void) m_flags.kaboom_executed2;

  // Reset child info...
  (void) m_flags.copy_true;
  (void) m_flags.divide_sex;
  (void) m_flags.mate_select_id;
  (void) m_flags.cross_num;
  m_flags.last_child_fertile = m_flags.child_fertile;
  m_flags.child_fertile     = 1;
  (void) m_flags.child_copied_size;
  
  // A few final changes if the parent was supposed to be be considered
  // a second child on the divide.
  if (avd_cpop_is_divide_method_split_or_birth(m_world->GetConfig().DIVIDE_METHOD.Get())) {
    m_core.gestation_start = 0;
    m_lifetime.cpu_cycles_used = 0;
    m_lifetime.time_used = 0;
    m_lifetime.neutral_metric += m_world->GetRandom().GetRandNormal();
  }

  if (avd_cpop_is_divide_method_split(m_world->GetConfig().DIVIDE_METHOD.Get())) {
    m_tolerance_immigrants.Clear();
    m_tolerance_offspring_own.Clear();
    m_tolerance_offspring_others.Clear();
    m_intolerances.SetAll(make_pair(-1, -1));
  }

  if (avd_cpop_is_generation_inc_both(m_world->GetConfig().GENERATION_INC_METHOD.Get())) m_lifetime.generation++;

  // Reset Task States
  for (Apto::Map<void*, cTaskState*>::ValueIterator it = m_task_states.Values(); it.Next();) delete *it.Get();
  m_task_states.Clear();
}

/**
 * This function runs whenever a *test* CPU divides. It processes much of
 * the information for that CPU in order to actively reflect its executed
 * and copied size in its merit.
 **/
void cPhenotype::TestDivideReset(const InstructionSequence& _genome)
{
  assert(m_lifetime.time_used > 0);
  assert(initialized == true);
  
  // Update these values as needed...
  //LZ This was an int!
  double cur_merit_base = CalcSizeMerit();
  //LZ
  const double merit_default_bonus = m_world->GetConfig().MERIT_DEFAULT_BONUS.Get();
  if (merit_default_bonus) {
    m_core.cur_bonus = merit_default_bonus;
  }
  avd_merit_set(&m_core.merit, cur_merit_base * m_core.cur_bonus);

  if (m_world->GetConfig().INHERIT_MERIT.Get() == 0) {
    avd_merit_set(&m_core.merit, cur_merit_base);
  }

  m_core.genome_length   = _genome.GetSize();
  (void) m_core.copied_size;                            // Unchanged
  (void) m_core.executed_size;                          // Unchanged
  m_core.gestation_time  = m_lifetime.time_used - m_core.gestation_start;
  m_core.gestation_start = m_lifetime.time_used;
  m_core.fitness         = CalcFitness(cur_merit_base, m_core.cur_bonus, m_core.gestation_time, m_lifetime.cpu_cycles_used);
  (void) m_core.div_type; 				// Unchanged

  // Lock in cur values as last values.
  m_lifetime.last_merit_base          = cur_merit_base;
  m_lifetime.last_bonus               = m_core.cur_bonus;
  m_lifetime.last_cpu_cycles_used      = m_lifetime.cpu_cycles_used;
  m_lifetime.last_num_errors           = m_lifetime.cur_num_errors;
  m_lifetime.last_num_donates          = m_lifetime.cur_num_donates;
  last_task_count           = cur_task_count;
  last_host_tasks           = cur_host_tasks;
  last_para_tasks           = cur_para_tasks;
  last_internal_task_count  = cur_internal_task_count;
  last_task_quality         = cur_task_quality;
  last_task_value			= cur_task_value;
  last_internal_task_quality= cur_internal_task_quality;
  last_rbins_total          = cur_rbins_total;
  last_rbins_avail          = cur_rbins_avail;
  last_collect_spec_counts  = cur_collect_spec_counts;
  last_reaction_count       = cur_reaction_count;
  last_reaction_add_reward  = cur_reaction_add_reward;
  last_inst_count           = cur_inst_count;
  last_from_sensor_count    = cur_from_sensor_count;
  last_from_message_count    = cur_from_message_count;
  last_group_attack_count   = cur_group_attack_count;
  last_killed_targets       = cur_killed_targets;
  last_attacks              = cur_attacks;
  last_kills                = cur_kills;
  last_top_pred_group_attack_count    = cur_top_pred_group_attack_count;
  last_sense_count          = cur_sense_count;
  m_lifetime.last_child_germline_propensity = m_lifetime.cur_child_germline_propensity;
  
  // Reset cur values.
  m_core.cur_bonus       = m_world->GetConfig().DEFAULT_BONUS.Get();
  m_lifetime.cpu_cycles_used = 0;
  m_lifetime.cur_num_errors  = 0;
  m_lifetime.cur_num_donates  = 0;
  cur_task_count.SetAll(0);
  cur_host_tasks.SetAll(0);
  // @LZ: figure out when and where to reset cur_para_tasks, depending on the divide method, and
  //      resonable assumptions
  if (avd_cpop_is_divide_method_split(m_world->GetConfig().DIVIDE_METHOD.Get())) {
    last_para_tasks = cur_para_tasks;
    cur_para_tasks.SetAll(0);
  }
  cur_internal_task_count.SetAll(0);
  eff_task_count.SetAll(0);
  cur_task_quality.SetAll(0);
  cur_task_value.SetAll(0);
  cur_internal_task_quality.SetAll(0);
  cur_rbins_total.SetAll(0);  // total resources collected in lifetime
  if (m_world->GetConfig().RESOURCE_GIVEN_ON_INJECT.Get() > 0.0) {   
    const int resource = m_world->GetConfig().COLLECT_SPECIFIC_RESOURCE.Get();
    cur_rbins_avail[resource] = m_world->GetConfig().RESOURCE_GIVEN_ON_INJECT.Get();
  }
  else cur_rbins_avail.SetAll(0);
  cur_collect_spec_counts.SetAll(0);
  cur_reaction_count.SetAll(0);
  first_reaction_cycles.SetAll(-1);
  first_reaction_execs.SetAll(-1);
  cur_stolen_reaction_count.SetAll(0);
  cur_reaction_add_reward.SetAll(0);
  cur_inst_count.SetAll(0);
  cur_from_sensor_count.SetAll(0);
  cur_from_message_count.SetAll(0);
  for (int r = 0; r < cur_group_attack_count.GetSize(); r++) {
    cur_group_attack_count[r].SetAll(0);
    cur_top_pred_group_attack_count[r].SetAll(0);
  }
  cur_killed_targets.SetAll(0);
  cur_attacks = 0;
  cur_kills = 0;
  cur_sense_count.SetAll(0);
  cur_task_time.SetAll(0.0);
  sensed_resources.SetAll(-1.0);
  cur_trial_fitnesses.Resize(0); 
  cur_trial_bonuses.Resize(0); 
  cur_trial_times_used.Resize(0); 
  m_lifetime.trial_time_used = 0;
  m_lifetime.trial_cpu_cycles_used = 0;
  m_tolerance_immigrants.Clear();
  m_tolerance_offspring_own.Clear();
  m_tolerance_offspring_others.Clear();
  m_intolerances.SetAll(make_pair(-1, -1));  
  m_lifetime.cur_child_germline_propensity = m_world->GetConfig().DEMES_DEFAULT_GERMLINE_PROPENSITY.Get();
  
  // Setup other miscellaneous values...
  m_lifetime.num_divides++;
  m_lifetime.generation++;
  (void) m_lifetime.time_used;
  (void) m_lifetime.num_execs;
  (void) m_lifetime.age;
  (void) fault_desc;
  (void) m_lifetime.neutral_metric;
  m_lifetime.life_fitness = m_core.fitness;
  m_lifetime.exec_time_born += m_core.gestation_time;  //@MRR See DivideReset
  m_lifetime.birth_update  = m_world->GetStats().GetUpdate();
  m_lifetime.num_new_unique_reactions = 0;
  m_lifetime.last_task_id             = -1;
  m_lifetime.res_consumed             = 0;
  m_lifetime.last_task_time           = 0;

  
  m_flags.num_thresh_gb_donations_last = m_flags.num_thresh_gb_donations;
  m_flags.num_thresh_gb_donations = 0;
  m_flags.num_quanta_thresh_gb_donations_last = m_flags.num_quanta_thresh_gb_donations;
  m_flags.num_quanta_thresh_gb_donations = 0;
  m_flags.num_shaded_gb_donations_last = m_flags.num_shaded_gb_donations;
  m_flags.num_shaded_gb_donations = 0;
  m_flags.num_donations_locus_last = m_flags.num_donations_locus;
  m_flags.num_donations_locus = 0;

  // Leave flags alone...
  (void) m_flags.is_injected;
  m_flags.is_clone = 0; // has legitimately reproduced
  m_flags.is_donor_last = m_flags.is_donor_cur;
  m_flags.is_donor_cur = 0;
  m_flags.is_donor_rand_last = m_flags.is_donor_rand;
  m_flags.is_donor_rand = 0;
  m_flags.is_donor_null_last = m_flags.is_donor_null;
  m_flags.is_donor_null = 0;
  m_flags.is_donor_kin_last = m_flags.is_donor_kin;
  m_flags.is_donor_kin = 0;
  m_flags.is_donor_edit_last = m_flags.is_donor_edit;
  m_flags.is_donor_edit = 0;
  m_flags.is_donor_gbg_last = m_flags.is_donor_gbg;
  m_flags.is_donor_gbg = 0;
  m_flags.is_donor_truegb_last = m_flags.is_donor_truegb;
  m_flags.is_donor_truegb = 0;
  m_flags.is_donor_threshgb_last = m_flags.is_donor_threshgb;
  m_flags.is_donor_threshgb = 0;
  m_flags.is_donor_quanta_threshgb_last = m_flags.is_donor_quanta_threshgb;
  m_flags.is_donor_quanta_threshgb = 0;
  m_flags.is_donor_shadedgb_last = m_flags.is_donor_shadedgb;
  m_flags.is_donor_shadedgb = 0;
  is_donor_locus_last = is_donor_locus;
  is_donor_locus.SetAll(false);

  m_flags.is_receiver_last = m_flags.is_receiver;
  m_flags.is_receiver = 0;
  m_flags.is_receiver_rand = 0;
  m_flags.is_receiver_kin_last = m_flags.is_receiver_kin;
  m_flags.is_receiver_kin = 0;
  m_flags.is_receiver_edit_last = m_flags.is_receiver_edit;
  m_flags.is_receiver_edit = 0;
  m_flags.is_receiver_gbg = 0;
  m_flags.is_receiver_truegb_last = m_flags.is_receiver_truegb;
  m_flags.is_receiver_truegb = 0;
  m_flags.is_receiver_threshgb_last = m_flags.is_receiver_threshgb;
  m_flags.is_receiver_threshgb = 0;
  m_flags.is_receiver_quanta_threshgb_last = m_flags.is_receiver_quanta_threshgb;
  m_flags.is_receiver_quanta_threshgb = 0;
  m_flags.is_receiver_shadedgb_last = m_flags.is_receiver_shadedgb;
  m_flags.is_receiver_shadedgb = 0;
  m_flags.is_receiver_gb_same_locus_last = m_flags.is_receiver_gb_same_locus;
  m_flags.is_receiver_gb_same_locus = 0;

  (void) m_flags.is_modifier;
  (void) m_flags.is_modified;
  (void) m_flags.is_fertile;
  (void) m_flags.is_mutated;
  (void) m_flags.is_multi_thread;
  (void) m_flags.parent_true;
  (void) m_flags.parent_sex;
  (void) m_flags.parent_cross_num;
  (void) m_flags.kaboom_executed;
  (void) m_flags.kaboom_executed2;

  // Reset child info...
  (void) m_flags.copy_true;
  (void) m_flags.divide_sex;
  (void) m_flags.mate_select_id;
  (void) m_flags.cross_num;
  (void) m_flags.child_fertile;
  (void) m_flags.last_child_fertile;
  (void) m_flags.child_copied_size;
}


/**
 * This function is run when an organism is being forced to replicate, but
 * not at the end of its replication cycle.
 *
 * Assumptions:
 *   - new organism is an exact clone of the parent, with *same* last info.
 *   - this is the first method run on an otherwise freshly built phenotype.
 **/

void cPhenotype::SetupClone(const cPhenotype& clone_phenotype)
{
  // Copy divide values from parent, which should already be setup.
  m_core.merit    = clone_phenotype.m_core.merit;

  m_core.energy_store    = clone_phenotype.m_core.energy_store;
  energy_tobe_applied = 0.0;
  energy_testament = 0.0;
  energy_received_buffer = 0.0;

  if (m_world->GetConfig().INHERIT_EXE_RATE.Get() == 0) m_core.execution_ratio = 1.0;
  else m_core.execution_ratio = clone_phenotype.m_core.execution_ratio;

  m_core.genome_length   = clone_phenotype.m_core.genome_length;
  m_core.copied_size     = clone_phenotype.m_core.copied_size;
  // m_core.copied_size     = clone_phenotype.child_copied_size;
  m_core.executed_size   = clone_phenotype.m_core.executed_size;
  m_core.gestation_time  = clone_phenotype.m_core.gestation_time;
  m_core.gestation_start = 0;
  m_core.fitness         = clone_phenotype.m_core.fitness;
  m_core.div_type        = clone_phenotype.m_core.div_type;

  assert(m_core.genome_length > 0);
  assert(m_core.copied_size > 0);
  assert(m_core.gestation_time >= 0); //@JEB 0 valid for some fitness methods
  assert(m_core.div_type > 0);

  // Initialize current values, as neeeded.
  m_core.cur_bonus       = m_world->GetConfig().DEFAULT_BONUS.Get();
  m_lifetime.cpu_cycles_used = 0;
  m_lifetime.cur_num_errors  = 0;
  m_lifetime.cur_num_donates  = 0;
  cur_task_count.SetAll(0);
  cur_host_tasks.SetAll(0);
  cur_para_tasks.SetAll(0);
  cur_internal_task_count.SetAll(0);
  eff_task_count.SetAll(0);
  cur_rbins_total.SetAll(0);
  cur_rbins_avail.SetAll(0);
  cur_collect_spec_counts.SetAll(0);
  cur_reaction_count.SetAll(0);
  first_reaction_cycles.SetAll(-1);
  first_reaction_execs.SetAll(-1);
  cur_stolen_reaction_count.SetAll(0);
  cur_reaction_add_reward.SetAll(0);
  cur_inst_count.SetAll(0);
  cur_from_sensor_count.SetAll(0);
  cur_from_message_count.SetAll(0);
  for (int r = 0; r < cur_group_attack_count.GetSize(); r++) {
    cur_group_attack_count[r].SetAll(0);
    cur_top_pred_group_attack_count[r].SetAll(0);
  }
  cur_killed_targets.SetAll(0);
  cur_attacks = 0;
  cur_kills = 0;
  cur_sense_count.SetAll(0);
  cur_task_time.SetAll(0.0);
  for (int j = 0; j < sensed_resources.GetSize(); j++) {
    sensed_resources[j] = clone_phenotype.sensed_resources[j];
  }
  cur_trial_fitnesses.Resize(0); 
  cur_trial_bonuses.Resize(0); 
  cur_trial_times_used.Resize(0); 
  m_lifetime.trial_time_used = 0;
  m_lifetime.trial_cpu_cycles_used = 0;
  m_tolerance_immigrants.Clear();        
  m_tolerance_offspring_own.Clear();     
  m_tolerance_offspring_others.Clear();  
  m_intolerances.SetAll(make_pair(-1, -1));  
  m_lifetime.cur_child_germline_propensity = m_world->GetConfig().DEMES_DEFAULT_GERMLINE_PROPENSITY.Get();
  m_lifetime.mating_type = MATING_TYPE_JUVENILE; // @CHC
  m_lifetime.mate_preference = MATE_PREFERENCE_RANDOM; //@CHC
  
  // Copy last values from parent
  m_lifetime.last_merit_base         = clone_phenotype.m_lifetime.last_merit_base;
  m_lifetime.last_bonus              = clone_phenotype.m_lifetime.last_bonus;
  m_lifetime.last_cpu_cycles_used     = clone_phenotype.m_lifetime.last_cpu_cycles_used;
  m_lifetime.last_num_errors          = clone_phenotype.m_lifetime.last_num_errors;
  m_lifetime.last_num_donates         = clone_phenotype.m_lifetime.last_num_donates;
  last_task_count          = clone_phenotype.last_task_count;
  last_host_tasks          = clone_phenotype.last_host_tasks;
  last_para_tasks          = clone_phenotype.last_para_tasks;
  last_internal_task_count = clone_phenotype.last_internal_task_count;
  last_rbins_total         = clone_phenotype.last_rbins_total;
  last_rbins_avail         = clone_phenotype.last_rbins_avail;
  last_collect_spec_counts = clone_phenotype.last_collect_spec_counts;
  last_reaction_count      = clone_phenotype.last_reaction_count;
  last_reaction_add_reward = clone_phenotype.last_reaction_add_reward;
  last_inst_count          = clone_phenotype.last_inst_count;
  last_from_sensor_count   = clone_phenotype.last_from_sensor_count;
  last_from_message_count   = clone_phenotype.last_from_message_count;
  last_group_attack_count   = clone_phenotype.last_group_attack_count;
  last_top_pred_group_attack_count   = clone_phenotype.last_top_pred_group_attack_count;
  last_killed_targets      = clone_phenotype.last_killed_targets;
  last_attacks             = clone_phenotype.last_attacks;
  last_kills                = clone_phenotype.last_kills;
  last_sense_count         = clone_phenotype.last_sense_count;
  m_lifetime.last_fitness             = CalcFitness(m_lifetime.last_merit_base, m_lifetime.last_bonus, m_core.gestation_time, m_lifetime.last_cpu_cycles_used);
  m_lifetime.last_child_germline_propensity = clone_phenotype.m_lifetime.last_child_germline_propensity;

  // Setup other miscellaneous values...
  m_lifetime.num_divides     = 0;
  m_lifetime.num_divides_failed = 0;
  m_lifetime.generation      = clone_phenotype.m_lifetime.generation;
  if (!avd_cpop_is_generation_inc_both(m_world->GetConfig().GENERATION_INC_METHOD.Get())) m_lifetime.generation++;
  m_lifetime.cpu_cycles_used = 0;
  m_lifetime.time_used       = 0;
  m_lifetime.num_execs       = 0;
  m_lifetime.age             = 0;
  fault_desc      = "";
  m_lifetime.neutral_metric  = clone_phenotype.m_lifetime.neutral_metric + m_world->GetRandom().GetRandNormal();
  m_lifetime.life_fitness    = m_core.fitness;
  m_lifetime.exec_time_born  = 0;
  m_lifetime.birth_update    = m_world->GetStats().GetUpdate();
  m_lifetime.num_new_unique_reactions = clone_phenotype.m_lifetime.num_new_unique_reactions;
  m_lifetime.last_task_id             = clone_phenotype.m_lifetime.last_task_id;
  m_lifetime.res_consumed             = clone_phenotype.m_lifetime.res_consumed;
  m_lifetime.is_germ_cell             = clone_phenotype.m_lifetime.is_germ_cell;
  m_lifetime.last_task_time           = clone_phenotype.m_lifetime.last_task_time;

  
  // Copy most flags from clone, then override clone-specific values.
  m_flags = clone_phenotype.m_flags;
  is_donor_locus_last = clone_phenotype.is_donor_locus_last;
  is_donor_locus = clone_phenotype.is_donor_locus;

  // Override clone-specific flags
  m_flags.is_injected   = 0;
  m_flags.is_clone   = 1;
  m_flags.is_modifier   = 0;
  m_flags.is_modified   = 0;
  m_flags.is_fertile    = clone_phenotype.m_flags.last_child_fertile;
  m_flags.is_mutated    = 0;
  m_flags.parent_true   = clone_phenotype.m_flags.copy_true;
  m_flags.parent_sex    = clone_phenotype.m_flags.divide_sex;
  m_flags.parent_cross_num    = clone_phenotype.m_flags.cross_num;
  m_flags.make_random_resource = 0;
  m_flags.to_die = 0;
  m_flags.to_delete = 0;
  m_flags.is_energy_requestor = 0;
  m_flags.is_energy_donor = 0;
  m_flags.is_energy_receiver = 0;
  m_flags.has_used_donated_energy = 0;
  m_flags.has_open_energy_request = 0;
  m_flags.kaboom_executed = 0;
  m_flags.kaboom_executed2 = 0;

  // Setup child info...
  m_flags.copy_true          = 0;
  m_flags.divide_sex         = 0;
  m_flags.mate_select_id     = 0;
  m_flags.cross_num          = 0;
  m_flags.last_child_fertile = m_flags.is_fertile;
  m_flags.child_fertile      = 1;
  m_flags.child_copied_size  = 0;
  m_lifetime.permanent_germline_propensity = clone_phenotype.m_lifetime.permanent_germline_propensity;
  
  initialized = true;
}




bool cPhenotype::TestInput(tBuffer<int>&, tBuffer<int>&)
{
  assert(initialized == true);
  // For the moment, lets not worry about inputs...
  return false; // Nothing happened...
}

bool cPhenotype::TestOutput(cAvidaContext& ctx, cTaskContext& taskctx,
                            const AvidaArray<double>& res_in, const AvidaArray<double>& rbins_in,
                            AvidaArray<double>& res_change, AvidaArray<cString>& insts_triggered,
                            bool is_parasite, cContextPhenotype* context_phenotype)
{
  assert(initialized == true);
  taskctx.SetTaskStates(&m_task_states);
  
  const cEnvironment& env = m_world->GetEnvironment();
  const int num_resources = env.GetResourceLib().GetSize();
  const int num_tasks = env.GetNumTasks();
  const int num_reactions = env.GetReactionLib().GetSize();
  
  // For refractory period @WRE 03-20-07
  const int cur_update_time = m_world->GetStats().GetUpdate();
  const double task_refractory_period = m_world->GetConfig().TASK_REFRACTORY_PERIOD.Get();
  double refract_factor;
  
  if (!m_reaction_result) m_reaction_result = new cReactionResult(num_resources, num_tasks, num_reactions);
  cReactionResult& result = *m_reaction_result;
  
  // Run everything through the environment.
  bool found = env.TestOutput(ctx, result, taskctx, eff_task_count, cur_reaction_count, res_in, rbins_in,
                              is_parasite, context_phenotype);
  
  // If nothing was found, stop here.
  if (found == false) {
    result.Invalidate();
    res_change.SetAll(0.0);
    return false;  // Nothing happened.
  }
  
  // Update the phenotype with the results...
  // Start with updating task and reaction counters
  for (int i = 0; i < num_tasks; i++) {
    // Calculate refractory period factor @WRE
    // Modify TaskQuality amount based on refractory period
    // Logistic equation using refractory period
    // in update units from configuration file.  @WRE 03-20-07, 04-17-07

    if (task_refractory_period == 0.0) {
      refract_factor = 1.0;
    } else {
      refract_factor = 1.0 - (1.0 / (1.0 + exp((cur_update_time - cur_task_time[i]) - task_refractory_period * 0.5)));
    }

    if (result.TaskDone(i) == true) {
      cur_task_count[i]++;
      eff_task_count[i]++;
      
      // Update parasite/host task tracking appropriately
      if (is_parasite) {
        cur_para_tasks[i]++;
      }
      else {
        cur_host_tasks[i]++;
      }
      
      if (context_phenotype != 0) {
        context_phenotype->GetTaskCounts()[i]++;
      }
      if (result.UsedEnvResource() == false) { cur_internal_task_count[i]++; }
      
      // if we want to generate an m_lifetime.age-task histogram
      if (m_world->GetConfig().AGE_POLY_TRACKING.Get()) {
        m_world->GetStats().AgeTaskEvent(taskctx.GetOrganism()->GetID(), i, m_lifetime.time_used);
      }
    }
    
    if (result.TaskQuality(i) > 0) {
      cur_task_quality[i] += result.TaskQuality(i) * refract_factor;
      if (result.UsedEnvResource() == false) {
        cur_internal_task_quality[i] += result.TaskQuality(i) * refract_factor;
      }
    }

    cur_task_value[i] = result.TaskValue(i);
    cur_task_time[i] = cur_update_time; // Find out time from context
  }

  for (int i = 0; i < num_tasks; i++) {
    if (result.TaskDone(i) && !last_task_count[i]) {
      m_world->GetStats().AddNewTaskCount(i);
      int prev_num_tasks = 0;
      int cur_num_tasks = 0;
      for (int j=0; j< num_tasks; j++) {
        if (last_task_count[j]>0) prev_num_tasks++;
        if (cur_task_count[j]>0) cur_num_tasks++;
      }
      m_world->GetStats().AddOtherTaskCounts(i, prev_num_tasks, cur_num_tasks);
    }
  }
  
  for (int i = 0; i < num_reactions; i++) {
    cur_reaction_add_reward[i] += result.GetReactionAddBonus(i);
    if (result.ReactionTriggered(i) && last_reaction_count[i]==0) {
      m_world->GetStats().AddNewReactionCount(i);
    }
    if (result.ReactionTriggered(i) == true) {
      if (context_phenotype != 0) {
        context_phenotype->GetReactionCounts()[i]++;
      }
      // If the organism has not performed this task,
      // then consider it to be a task switch.
      // If applicable, add in the penalty.
      switch (m_world->GetConfig().TASK_SWITCH_PENALTY_TYPE.Get()) {
        case 0: { // no penalty
          break;
        }
        case 1: { // "learning" cost
          int n_react = cur_reaction_count[i] -1;
          if (n_react < m_world->GetConfig().LEARNING_COUNT.Get()) {
            m_lifetime.num_new_unique_reactions += ( m_world->GetConfig().LEARNING_COUNT.Get() - n_react);
          }
          break;
        }
        case 2: { // "retooling" cost
          if (m_lifetime.last_task_id == -1) {
            m_lifetime.last_task_id = i;
            m_lifetime.last_task_time = m_lifetime.time_used;
          }	else {
            // track time used if applicable
            int cur_time_used = m_lifetime.time_used - m_lifetime.last_task_time; 
            m_lifetime.last_task_time = m_lifetime.time_used;
            m_world->GetStats().AddTaskSwitchTime(m_lifetime.last_task_id, i, cur_time_used);
            if (m_lifetime.last_task_id != i) {
              m_lifetime.num_new_unique_reactions++;
              m_lifetime.last_task_id = i;
            } 

          }
          break;
        }
        case 3: { // centrifuge
          // task switching cost is calculated based on 
          // the distance between the two tasks.
          
          int distance = abs(i - m_lifetime.last_task_id);
          m_lifetime.num_new_unique_reactions += distance;
          m_lifetime.last_task_id = i;
          
          break;
        }
        default: {
          assert(false);
          break;
        }
      }
    }		
  }
  
  // Update the merit bonus
  m_core.cur_bonus *= result.GetMultBonus();
  m_core.cur_bonus += result.GetAddBonus();
  
  // update the germline propensity
  m_lifetime.cur_child_germline_propensity += result.GetAddGermline();
  m_lifetime.cur_child_germline_propensity *= result.GetMultGermline();
  
  // Update deme merit (guard against running in the test CPU, where there is
  // no deme object.  Don't touch deme merit if there is no deme frac component.
  cDeme* deme = taskctx.GetOrganism()->GetDeme();
  if (deme) {
    if (result.GetActiveDeme()) {
      double deme_bonus = deme->GetHeritableDemeMerit().GetDouble();
      deme_bonus *= result.GetMultDemeBonus();
      deme_bonus += result.GetAddDemeBonus();
      deme->UpdateHeritableDemeMerit(deme_bonus);
    }
    
    //also count tasks/reactions
    for (int i = 0; i < num_tasks; i++) {
      if (result.TaskDone(i) == true) deme->AddCurTask(i);
    }
    for (int i = 0; i < num_reactions; i++) {
      if (result.ReactionTriggered(i) == true) deme->AddCurReaction(i);
    }
  }
  
  // Update the energy bonus
  m_core.cur_energy_bonus += result.GetAddEnergy();
  
  // Denote consumed resources...
  for (int i = 0; i < res_in.GetSize(); i++) {
    res_change[i] = result.GetProduced(i) - result.GetConsumed(i);
    m_lifetime.res_consumed += result.GetConsumed(i);
  }
  
  // Update rbins as necessary
  if (result.UsedEnvResource() == false) {
    double rbin_diff;
    for (int i = 0; i < num_resources; i++) {
      rbin_diff = result.GetInternalConsumed(i) - result.GetInternalProduced(i); ;
      cur_rbins_avail[i] -= rbin_diff;
      if(rbin_diff != 0) { cur_rbins_total[i] += rbin_diff; }
    }
  }
  
  // Save the instructions that should be triggered...
  insts_triggered = result.GetInstArray();
  
  //Put in detected resources
  for (int j = 0; j < res_in.GetSize(); j++) {
    if(result.GetDetected(j) != -1.0) {
      sensed_resources[j] = result.GetDetected(j);
    }
  }
  
  //Note if the resource should be placed in a random cell instead of this cell
  if (result.GetIsRandomResource())
  {
    m_flags.make_random_resource = 1;
  }

  //Kill any cells that did lethal reactions
  if (result.GetLethal())
  {
    m_flags.to_die = 1;
  }

  // Sterilize organisms that have performed a sterilizing task.
  if (result.GetSterilize()) {
    m_flags.is_fertile = 0;
  }
  
  result.Invalidate();
  return true;
}


void cPhenotype::Sterilize()
{
  m_flags.is_fertile = 0;
}


void cPhenotype::PrintStatus(ostream& fp) const
{
  fp << "  MeritBase:"
  << CalcSizeMerit()
  << " Bonus:" << m_core.cur_bonus
  << " Errors:" << m_lifetime.cur_num_errors
  << " Donates:" << m_lifetime.cur_num_donates
  << '\n';
  
  fp << "  Task Count (Quality):";
  for (int i = 0; i < cur_task_count.GetSize(); i++) {
    fp << " " << cur_task_count[i] << " (" << cur_task_quality[i] << ")";
  }
  fp << '\n';
  
  // if using resoruce bins, print the relevant stats
  if (m_world->GetConfig().USE_RESOURCE_BINS.Get()) {
    fp << "  Used-Internal-Resources Task Count (Quality):";
    for (int i = 0; i < cur_internal_task_count.GetSize(); i++) {
      fp << " " << cur_internal_task_count[i] << " (" << cur_internal_task_quality[i] << ")";
    }
    fp << endl;
 		
    fp << "  Available Internal Resource Bin Contents (Total Ever Collected):";
    for(int i = 0; i < cur_rbins_avail.GetSize(); i++) {
      fp << " " << cur_rbins_avail[i] << " (" << cur_rbins_total[i] << ")";
    }
    fp << endl;
  }
}

int cPhenotype::CalcSizeMerit() const
{
  assert(m_core.genome_length > 0);
  assert(m_core.copied_size > 0);

  return avd_pheno_calc_size_merit(
    &m_core,
    m_world->GetConfig().BASE_MERIT_METHOD.Get(),
    m_world->GetConfig().BASE_CONST_MERIT.Get(),
    m_lifetime.cpu_cycles_used,
    m_world->GetConfig().FITNESS_VALLEY.Get(),
    m_world->GetConfig().FITNESS_VALLEY_START.Get(),
    m_world->GetConfig().FITNESS_VALLEY_STOP.Get(),
    m_world->GetConfig().MERIT_BONUS_EFFECT.Get()
  );
}

double cPhenotype::CalcCurrentMerit() const
{
  //LZ this was int
  double merit_base = CalcSizeMerit();

  return merit_base * m_core.cur_bonus;
}


double cPhenotype::CalcFitness(double _merit_base, double _bonus, int _gestation_time, int) const
{
  double out_fitness = 0;
  switch (m_world->GetConfig().FITNESS_METHOD.Get()) {
    case 0: // Normal
      assert(_gestation_time > 0);
      out_fitness = _merit_base * _bonus / _gestation_time;
      break;
      
    case 1: // Sigmoidal returns (should be used with an additive reward)
    {
      assert(_gestation_time > 0);
      out_fitness = 0;
      //Note: this operates on accumulated bonus and ignores the default bonus.
      double converted_bonus = (_bonus - m_world->GetConfig().DEFAULT_BONUS.Get()) * m_world->GetConfig().FITNESS_COEFF_2.Get() / (1 + _bonus * m_world->GetConfig().FITNESS_COEFF_2.Get() ) ;
      out_fitness = _merit_base * exp(converted_bonus * log(m_world->GetConfig().FITNESS_COEFF_1.Get())) / _gestation_time;
    }
      break;
      
    case 2: //Activity of one enzyme in pathway altered (with diminishing returns and a cost for each executed instruction)
    {
      out_fitness = 0;
      double net_bonus = _bonus +  - m_world->GetConfig().DEFAULT_BONUS.Get();
      out_fitness = net_bonus / (net_bonus + 1)* exp (_gestation_time * log(1 - m_world->GetConfig().FITNESS_COEFF_1.Get())); 
    }
      break;
      
    default:
      cout << "Unknown FITNESS_METHOD!" << endl;
      exit(1);
  }
  
  return out_fitness;
}

/* Returns the organism's total tolerance towards immigrants by counting
 the total number of dec-tolerance entries within the update window that have been executed. 
 */
int cPhenotype::CalcToleranceImmigrants()
{
  const int cur_update = m_world->GetStats().GetUpdate();
  const int tolerance_max = m_world->GetConfig().MAX_TOLERANCE.Get();

  // Check if cached value is up-to-date, return
  if (m_intolerances[0].first == cur_update) return tolerance_max - m_intolerances[0].second;
  
  const int update_window = m_world->GetConfig().TOLERANCE_WINDOW.Get();
  // Update the tolerance list by getting rid of outdated records
  while (m_tolerance_immigrants.GetSize() && *m_tolerance_immigrants.GetLast() < cur_update - update_window)
    delete m_tolerance_immigrants.PopRear();
  
  // And prune the list down to MAX_TOLERANCE entries.
  while (m_tolerance_immigrants.GetSize() > tolerance_max)
    delete m_tolerance_immigrants.PopRear();

  const int tolerance = tolerance_max - m_tolerance_immigrants.GetSize();

  // Update cached values
  m_intolerances[0].first = cur_update;
  m_intolerances[0].second = m_tolerance_immigrants.GetSize();
  return tolerance;
}

/* Returns the organism's total tolerance towards the organism's own offspring by counting
 the total number of dec-tolerance entries within the update window that have been executed. 
 */
int cPhenotype::CalcToleranceOffspringOwn()
{
  const int cur_update = m_world->GetStats().GetUpdate();
  const int tolerance_max = m_world->GetConfig().MAX_TOLERANCE.Get();
  
  // If offspring tolerances off, skip calculations returning max
  if (m_world->GetConfig().TOLERANCE_VARIATIONS.Get() > 0) return tolerance_max;

  // Check if cached value is up-to-date, return
  if (m_intolerances[1].first == cur_update) return tolerance_max - m_intolerances[1].second;

  const int update_window = m_world->GetConfig().TOLERANCE_WINDOW.Get();
  
  // Update the tolerance list by getting rid of outdated records
  while (m_tolerance_offspring_own.GetSize() && *m_tolerance_offspring_own.GetLast() < cur_update - update_window)
    delete m_tolerance_offspring_own.PopRear();
  
  // And prune the list down to MAX_TOLERANCE entries.
  while (m_tolerance_offspring_own.GetSize() > tolerance_max)
    delete m_tolerance_offspring_own.PopRear();
  
  const int tolerance = tolerance_max - m_tolerance_offspring_own.GetSize();

  // Update cached values
  m_intolerances[1].first = cur_update;
  m_intolerances[1].second = m_tolerance_offspring_own.GetSize();
  return tolerance;
}

/* Returns the organism's total tolerance towards the offspring of others in the group by counting
 the total number of dec-tolerance entries within the update window that have been executed. 
 */
int cPhenotype::CalcToleranceOffspringOthers()
{
  const int cur_update = m_world->GetStats().GetUpdate();
  const int tolerance_max = m_world->GetConfig().MAX_TOLERANCE.Get();

  // If offspring tolerances off, skip calculations returning max
  if (m_world->GetConfig().TOLERANCE_VARIATIONS.Get() > 0) return tolerance_max;

  // Check if cached value is up-to-date, return
  if (m_intolerances[2].first == cur_update) return tolerance_max - m_intolerances[2].second;

  const int update_window = m_world->GetConfig().TOLERANCE_WINDOW.Get();  
  
  // Update the tolerance list by getting rid of outdated records
  while (m_tolerance_offspring_others.GetSize() && *m_tolerance_offspring_others.GetLast() < cur_update - update_window) 
    delete m_tolerance_offspring_others.PopRear();
  
  // And prune the list down to MAX_TOLERANCE entries.
  while (m_tolerance_offspring_others.GetSize() > tolerance_max)
    delete m_tolerance_offspring_others.PopRear();

  const int tolerance = tolerance_max - m_tolerance_offspring_others.GetSize();

  // Update cached values
  m_intolerances[2].first = cur_update;
  m_intolerances[2].second = m_tolerance_offspring_others.GetSize();
  return tolerance;
}

void cPhenotype::IncAttackedPreyFTData(int target_ft) {
  AvidaArray<int> target_list = m_world->GetEnvironment().GetAttackPreyFTList();
  if (!cur_killed_targets.GetSize()) {
    cur_killed_targets.Resize(target_list.GetSize());
    cur_killed_targets.SetAll(0);
  }
  if (target_ft < -3) target_ft = -3;
  int this_index = target_ft;
  for (int i = 0; i < target_list.GetSize(); i++) {
    if (target_list[i] == target_ft) {
      this_index = i;
      break;
    }
  }
  assert(this_index >= 0);
  assert(cur_killed_targets.GetSize() == target_list.GetSize());
  cur_killed_targets[this_index]++;
}

void cPhenotype::ReduceEnergy(const double cost) {
  SetEnergy(m_core.energy_store - cost);
}

void cPhenotype::SetEnergy(const double value) {
  m_core.energy_store = max(0.0, min(value, (double)m_world->GetConfig().ENERGY_CAP.Get()));
}

void cPhenotype::DoubleEnergyUsage() {
  m_core.execution_ratio *= 2.0;
}

void cPhenotype::HalveEnergyUsage() {
  m_core.execution_ratio *= 0.5;
}

void cPhenotype::DefaultEnergyUsage() {
  m_core.execution_ratio = 1.0;
}

void cPhenotype::DivideFailed() {
  m_lifetime.num_divides_failed++;
}


/**
 Credit organism with energy reward, but only update energy store if APPLY_ENERGY_METHOD = "on task completion" (1)
 */
void cPhenotype::RefreshEnergy() {
  if(m_core.cur_energy_bonus > 0) {
    if(m_world->GetConfig().APPLY_ENERGY_METHOD.Get() == 0 || // on divide
       m_world->GetConfig().APPLY_ENERGY_METHOD.Get() == 2) {  // on sleep
      energy_tobe_applied += m_core.cur_energy_bonus;
    } else if(m_world->GetConfig().APPLY_ENERGY_METHOD.Get() == 1) {
      SetEnergy(m_core.energy_store + m_core.cur_energy_bonus);
      m_world->GetStats().SumEnergyTestamentAcceptedByOrganisms().Add(energy_testament);
      energy_testament = 0.0;
    } else {
      cerr<< "Unknown APPLY_ENERGY_METHOD value " << m_world->GetConfig().APPLY_ENERGY_METHOD.Get();
      exit(-1);
    }
    m_core.cur_energy_bonus = 0;
  }
}

void cPhenotype::ApplyToEnergyStore() {
  SetEnergy(m_core.energy_store + energy_tobe_applied);
  m_world->GetStats().SumEnergyTestamentAcceptedByOrganisms().Add(energy_testament);
  energy_testament = 0.0;
  energy_tobe_applied = 0.0;
  energy_testament = 0.0;
}

void cPhenotype::EnergyTestament(const double value) {
  assert(value > 0.0);
  energy_tobe_applied += value;
  energy_testament += value;
} //! external energy given to organism


void cPhenotype::ApplyDonatedEnergy() {
  double energy_cap = m_world->GetConfig().ENERGY_CAP.Get();
  
  if((m_core.energy_store + energy_received_buffer) >= energy_cap) {
    IncreaseEnergyApplied(energy_cap - m_core.energy_store);
    SetEnergy(m_core.energy_store + (energy_cap - energy_received_buffer));
  } else {
    IncreaseEnergyApplied(energy_received_buffer);
    SetEnergy(m_core.energy_store + energy_received_buffer);
  }
  
  IncreaseNumEnergyApplications();
  SetHasUsedDonatedEnergy();
  
  energy_received_buffer = 0.0;
  
} //End AppplyDonatedEnergy()


void cPhenotype::ReceiveDonatedEnergy(const double donation) {
  assert(donation >= 0.0);  
  energy_received_buffer += donation;
  IncreaseEnergyReceived(donation);
  SetIsEnergyReceiver();
  IncreaseNumEnergyReceptions();
} //End ReceiveDonatedEnergy()


double cPhenotype::ExtractParentEnergy() {
  assert(m_world->GetConfig().ENERGY_ENABLED.Get() > 0);
  // energy model config variables
  double energy_given_at_birth = m_world->GetConfig().ENERGY_GIVEN_AT_BIRTH.Get();
  double frac_parent_energy_given_at_birth = m_world->GetConfig().FRAC_PARENT_ENERGY_GIVEN_TO_ORG_AT_BIRTH.Get();
  double frac_energy_decay_at_birth = m_world->GetConfig().FRAC_ENERGY_DECAY_AT_ORG_BIRTH.Get();
  double energy_cap = m_world->GetConfig().ENERGY_CAP.Get();
  
  // apply energy if APPLY_ENERGY_METHOD is set to "on divide" (0)
  if(m_world->GetConfig().APPLY_ENERGY_METHOD.Get() == 0) {
    RefreshEnergy();
    ApplyToEnergyStore();
  }
  
  // decay of energy in parent
  ReduceEnergy(GetStoredEnergy() * frac_energy_decay_at_birth);
  
  // calculate energy to be given to child
  double child_energy = max(0.0, min(GetStoredEnergy() * frac_parent_energy_given_at_birth + energy_given_at_birth, energy_cap));
  assert(GetStoredEnergy()>0.0);
  // adjust energy in parent
  ReduceEnergy(child_energy - 2*energy_given_at_birth); // 2*energy_given_at_birth: 1 in child_energy & 1 for parent
  
  //TODO: add energy_given_at_birth to Stored_energy
  cMerit parentMerit(ConvertEnergyToMerit(GetStoredEnergy() * GetEnergyUsageRatio()));
  if (parentMerit.GetDouble() > 0.0) SetMerit(parentMerit);
  else SetToDie();
	
  return child_energy;
}

// Save the current fitness and reset relevant parts of the phenotype
void cPhenotype::NewTrial()
{ 
  //Return if a complete trial has not occurred.
  //(This will happen if CompeteOrganisms was called before in the same update
  if (m_lifetime.trial_cpu_cycles_used == 0) return;
  
  //Record the merit of this trial
  m_core.fitness = CalcFitness( GetCurMeritBase(), GetCurBonus() , m_lifetime.trial_time_used, m_lifetime.trial_cpu_cycles_used); // This is a per-trial fitness @JEB
  cur_trial_fitnesses.Push(m_core.fitness);
  cur_trial_bonuses.Push(GetCurBonus());
  cur_trial_times_used.Push(m_lifetime.trial_time_used);

  //The rest of the function, resets the phenotype like DivideReset(), but without
  //incrementing the m_lifetime.generation or child statistics.

  //Most importantly, this does (below):
  // m_lifetime.trial_time_used = 0;
  // m_lifetime.trial_cpu_cycles_used = 0;
  // SetCurBonus(m_world->GetConfig().DEFAULT_BONUS.Get());

  // Update these values as needed...
  //LZ This was an int!

  double cur_merit_base = CalcSizeMerit();

  // If we are resetting the current merit, do it here
  // and it will also be propagated to the child
  //LZ
  double merit_default_bonus = m_world->GetConfig().MERIT_DEFAULT_BONUS.Get();
  if (merit_default_bonus) {
    m_core.cur_bonus = merit_default_bonus;
  }
  avd_merit_set(&m_core.merit, cur_merit_base * m_core.cur_bonus);

  // update energy store
  m_core.energy_store += m_core.cur_energy_bonus;
  m_core.energy_store = m_world->GetConfig().ENERGY_GIVEN_AT_BIRTH.Get(); // We reset to what they had at birth
  m_core.cur_energy_bonus = 0;
  // to be perfectly accurate, this should be from a last_energy value??


  // m_core.genome_length   = _genome.GetSize();  //No child! @JEB
  (void) m_core.copied_size;          // Unchanged
  (void) m_core.executed_size;        // Unchanged
  m_core.gestation_time  = m_lifetime.time_used - m_core.gestation_start;  //Keep gestation referring to actual replication time! @JEB
  m_core.gestation_start = m_lifetime.time_used;                    //Keep gestation referring to actual replication time! @JEB
  // m_core.fitness         = m_core.merit.value / m_core.gestation_time; //Use fitness measure that is per-trial @JEB

  // Lock in cur values as last values.
  m_lifetime.last_merit_base          = cur_merit_base;
  m_lifetime.last_bonus               = m_core.cur_bonus;
  m_lifetime.last_cpu_cycles_used      = m_lifetime.cpu_cycles_used;
  //TODO?  last_energy         = m_core.cur_energy_bonus;
  m_lifetime.last_num_errors           = m_lifetime.cur_num_errors;
  m_lifetime.last_num_donates          = m_lifetime.cur_num_donates;
  last_task_count           = cur_task_count;
  last_host_tasks           = cur_host_tasks;
  last_para_tasks           = cur_para_tasks;
  last_internal_task_count  = cur_internal_task_count;
  last_task_quality         = cur_task_quality;
  last_internal_task_quality= cur_internal_task_quality;
  last_task_value			      = cur_task_value;
  last_rbins_total          = cur_rbins_total;
  last_rbins_avail          = cur_rbins_avail;
  last_collect_spec_counts  = cur_collect_spec_counts;
  last_reaction_count       = cur_reaction_count;
  last_reaction_add_reward  = cur_reaction_add_reward;
  last_inst_count           = cur_inst_count;
  last_from_sensor_count    = cur_from_sensor_count;
  last_from_message_count    = cur_from_message_count;
  last_group_attack_count   = cur_group_attack_count;
  last_killed_targets       = cur_killed_targets;
  last_attacks              = cur_attacks;
  last_kills                = cur_kills;
  last_top_pred_group_attack_count    = cur_top_pred_group_attack_count;
  last_sense_count          = cur_sense_count;
  
  // Reset cur values.
  m_core.cur_bonus       = m_world->GetConfig().DEFAULT_BONUS.Get();
  m_lifetime.cpu_cycles_used = 0;
  m_core.cur_energy_bonus = 0.0;
  m_lifetime.cur_num_errors  = 0;
  m_lifetime.cur_num_donates  = 0;
  cur_task_count.SetAll(0);
  cur_host_tasks.SetAll(0);
  cur_para_tasks.SetAll(0);
  cur_internal_task_count.SetAll(0);
  eff_task_count.SetAll(0);
  cur_task_quality.SetAll(0);
  cur_internal_task_quality.SetAll(0);
  cur_task_value.SetAll(0);
  cur_rbins_total.SetAll(0);
  cur_rbins_avail.SetAll(0);
  cur_collect_spec_counts.SetAll(0);
  cur_reaction_count.SetAll(0);
  first_reaction_cycles.SetAll(-1);
  first_reaction_execs.SetAll(-1);
  cur_stolen_reaction_count.SetAll(0);
  cur_reaction_add_reward.SetAll(0);
  cur_inst_count.SetAll(0);
  cur_from_sensor_count.SetAll(0);
  cur_from_message_count.SetAll(0);
  for (int r = 0; r < cur_group_attack_count.GetSize(); r++) {
    cur_group_attack_count[r].SetAll(0);
    cur_top_pred_group_attack_count[r].SetAll(0);
  }
  cur_killed_targets.SetAll(0);
  cur_attacks = 0;
  cur_kills = 0;
  cur_sense_count.SetAll(0);
  //cur_trial_fitnesses.Resize(0); Don't throw out the trial fitnesses! @JEB
  m_lifetime.trial_time_used = 0;
  m_lifetime.trial_cpu_cycles_used = 0;
  m_tolerance_immigrants.Clear();        
  m_tolerance_offspring_own.Clear();     
  m_tolerance_offspring_others.Clear();  
  m_intolerances.SetAll(make_pair(-1, -1));  
  
  // Setup other miscellaneous values...
  m_lifetime.num_divides++;
  (void) m_lifetime.generation;
  (void) m_lifetime.time_used;
  m_lifetime.num_execs       = 0;
  m_lifetime.age             = 0;
  fault_desc      = "";
  (void) m_lifetime.neutral_metric;
  m_lifetime.life_fitness = m_core.fitness;


  m_flags.num_thresh_gb_donations_last = m_flags.num_thresh_gb_donations;
  m_flags.num_thresh_gb_donations = 0;
  m_flags.num_quanta_thresh_gb_donations_last = m_flags.num_quanta_thresh_gb_donations;
  m_flags.num_quanta_thresh_gb_donations = 0;
  m_flags.num_shaded_gb_donations_last = m_flags.num_shaded_gb_donations;
  m_flags.num_shaded_gb_donations = 0;
  m_flags.num_donations_locus_last = m_flags.num_donations_locus;
  m_flags.num_donations_locus = 0;

  // Leave flags alone...
  (void) m_flags.is_injected;
  (void) m_flags.is_clone;
  m_flags.is_donor_last = m_flags.is_donor_cur;
  m_flags.is_donor_cur = 0;
  m_flags.is_donor_rand_last = m_flags.is_donor_rand;
  m_flags.is_donor_rand = 0;
  m_flags.is_donor_null_last = m_flags.is_donor_null;
  m_flags.is_donor_null = 0;
  m_flags.is_donor_kin_last = m_flags.is_donor_kin;
  m_flags.is_donor_kin = 0;
  m_flags.is_donor_edit_last = m_flags.is_donor_edit;
  m_flags.is_donor_edit = 0;
  m_flags.is_donor_gbg_last = m_flags.is_donor_gbg;
  m_flags.is_donor_gbg = 0;
  m_flags.is_donor_truegb_last = m_flags.is_donor_truegb;
  m_flags.is_donor_truegb = 0;
  m_flags.is_donor_threshgb_last = m_flags.is_donor_threshgb;
  m_flags.is_donor_threshgb = 0;
  m_flags.is_donor_quanta_threshgb_last = m_flags.is_donor_quanta_threshgb;
  m_flags.is_donor_quanta_threshgb = 0;
  m_flags.is_donor_shadedgb_last = m_flags.is_donor_shadedgb;
  m_flags.is_donor_shadedgb = 0;
  is_donor_locus_last = is_donor_locus;
  is_donor_locus.SetAll(false);

  m_flags.is_receiver_last = m_flags.is_receiver;
  m_flags.is_receiver = 0;
  m_flags.is_receiver_rand = 0;
  m_flags.is_receiver_kin_last = m_flags.is_receiver_kin;
  m_flags.is_receiver_kin = 0;
  m_flags.is_receiver_edit_last = m_flags.is_receiver_edit;
  m_flags.is_receiver_edit = 0;
  m_flags.is_receiver_gbg = 0;
  m_flags.is_receiver_truegb_last = m_flags.is_receiver_truegb;
  m_flags.is_receiver_truegb = 0;
  m_flags.is_receiver_threshgb_last = m_flags.is_receiver_threshgb;
  m_flags.is_receiver_threshgb = 0;
  m_flags.is_receiver_quanta_threshgb_last = m_flags.is_receiver_quanta_threshgb;
  m_flags.is_receiver_quanta_threshgb = 0;
  m_flags.is_receiver_shadedgb_last = m_flags.is_receiver_shadedgb;
  m_flags.is_receiver_shadedgb = 0;
  m_flags.is_receiver_gb_same_locus_last = m_flags.is_receiver_gb_same_locus;
  m_flags.is_receiver_gb_same_locus = 0;

  m_flags.is_energy_requestor = 0;
  m_flags.is_energy_donor = 0;
  m_flags.is_energy_receiver = 0;
  (void) m_flags.is_modifier;
  (void) m_flags.is_modified;
  (void) m_flags.is_fertile;
  (void) m_flags.is_mutated;
  (void) m_flags.is_multi_thread;
  (void) m_flags.parent_true;
  (void) m_flags.parent_sex;
  (void) m_flags.parent_cross_num;
  (void) m_flags.kaboom_executed;
  (void) m_flags.kaboom_executed2;
}

/**
 * This function is run to reset an organism whose task counts (etc) have already been moved from cur to last
 * by another call (like NewTrial). It is a subset of DivideReset @JEB
 **/
void cPhenotype::TrialDivideReset(const InstructionSequence& _genome)
{
  //LZ This was an int!
  double cur_merit_base = CalcSizeMerit();
  
  // If we are resetting the current merit, do it here
  // and it will also be propagated to the child
  //LZ
  const double merit_default_bonus = m_world->GetConfig().MERIT_DEFAULT_BONUS.Get();
  if (merit_default_bonus) {
    m_core.cur_bonus = merit_default_bonus;
  }
  avd_merit_set(&m_core.merit, cur_merit_base * m_core.cur_bonus);

  SetEnergy(m_core.energy_store + m_core.cur_energy_bonus);
  m_world->GetStats().SumEnergyTestamentAcceptedByOrganisms().Add(energy_testament);
  energy_testament = 0.0;

  m_core.genome_length   = _genome.GetSize();
  m_core.gestation_start = m_lifetime.time_used;
  cur_trial_fitnesses.Resize(0); 
  cur_trial_bonuses.Resize(0); 
  cur_trial_times_used.Resize(0); 
  
  // Reset child info...
  (void) m_flags.copy_true;
  (void) m_flags.divide_sex;
  (void) m_flags.mate_select_id;
  (void) m_flags.cross_num;
  m_flags.last_child_fertile = m_flags.child_fertile;
  m_flags.child_fertile     = 1;
  (void) m_flags.child_copied_size;
  
  // A few final changes if the parent was supposed to be be considered
  // a second child on the divide.
  if (avd_cpop_is_divide_method_split_or_birth(m_world->GetConfig().DIVIDE_METHOD.Get())) {    
    m_core.gestation_start = 0;
    m_lifetime.cpu_cycles_used = 0;
    m_lifetime.time_used = 0;
    m_lifetime.num_execs = 0;
    m_lifetime.neutral_metric += m_world->GetRandom().GetRandNormal();
  }
  
  if (avd_cpop_is_divide_method_split(m_world->GetConfig().DIVIDE_METHOD.Get())) {
    m_tolerance_immigrants.Clear();        
    m_tolerance_offspring_own.Clear();     
    m_tolerance_offspring_others.Clear();  
    m_intolerances.SetAll(make_pair(-1,-1));  
  }

  if (avd_cpop_is_generation_inc_both(m_world->GetConfig().GENERATION_INC_METHOD.Get())) m_lifetime.generation++;
}

// Arbitrary (but consistant) ordering.
// Return -1 if lhs is "less", +1 is it is "greater", and 0 otherwise.
int cPhenotype::Compare(const cPhenotype* lhs, const cPhenotype* rhs) {
  // Compare first based on merit...
  if ( lhs->GetMerit() < rhs->GetMerit() ) return -1;
  else if ( lhs->GetMerit() > rhs->GetMerit() ) return 1;
  
  // If merits are equal, compare gestation time...
  if ( lhs->GetGestationTime() < rhs->GetGestationTime() ) return -1;
  else if ( lhs->GetGestationTime() > rhs->GetGestationTime() ) return 1;
  
  // If gestation times are also equal, compare each task
  AvidaArray<int> lhsTasks = lhs->GetLastTaskCount();
  AvidaArray<int> rhsTasks = rhs->GetLastTaskCount();
  for (int k = 0; k < lhsTasks.GetSize(); k++) {
    if (lhsTasks[k] < rhsTasks[k]) return -1;
    else if (lhsTasks[k] > rhsTasks[k]) return 1;
  }
  
  // Assume they are identical.
  return 0;
}

bool cPhenotype::PhenotypeCompare::operator()(const cPhenotype* lhs, const cPhenotype* rhs) const {
  return cPhenotype::Compare(lhs, rhs) < 0;
}


  
// Return an integer classifying the organism's energy level as -1=error,0=low,1=med,2=high
int cPhenotype::GetDiscreteEnergyLevel() const {
  double max_energy = m_world->GetConfig().ENERGY_CAP.Get();
  double high_pct = m_world->GetConfig().ENERGY_THRESH_HIGH.Get();
  double low_pct = m_world->GetConfig().ENERGY_THRESH_LOW.Get();
	
  assert(max_energy >= 0);
  assert(high_pct <= 1);
  assert(high_pct >= 0);
  assert(low_pct <= 1);
  assert(low_pct >= 0);
  assert(low_pct <= high_pct);
	
  if (m_core.energy_store < (low_pct * max_energy)) {
    return ENERGY_LEVEL_LOW;
  } else if ( (m_core.energy_store >= (low_pct * max_energy)) && (m_core.energy_store <= (high_pct * max_energy)) ) {
    return ENERGY_LEVEL_MEDIUM;
  } else if (m_core.energy_store > (high_pct * max_energy)) {
    return ENERGY_LEVEL_HIGH;
  } else {
    return -1;
  }			 
	
} //End GetDiscreteEnergyLevel()


double cPhenotype::ConvertEnergyToMerit(double energy) const
{
  assert(m_world->GetConfig().ENERGY_ENABLED.Get() == 1);
  
	double FIX_METABOLIC_RATE = m_world->GetConfig().FIX_METABOLIC_RATE.Get();
	if (FIX_METABOLIC_RATE > 0.0) return 100 * FIX_METABOLIC_RATE;
  
  return 100 * energy / m_world->GetConfig().NUM_CYCLES_EXC_BEFORE_0_ENERGY.Get();
}



double cPhenotype::GetResourcesConsumed() 
{
	double r = m_lifetime.res_consumed; 
	m_lifetime.res_consumed =0; 
	return r; 
}

//Deep copy parasite task count
void cPhenotype::SetLastParasiteTaskCount(AvidaArray<int> oldParaPhenotype)
{
  assert(initialized == true);
  
  for(int i=0;i<oldParaPhenotype.GetSize();i++)
  {
    last_para_tasks[i] = oldParaPhenotype[i];
  }
}

/* Return the cumulative reaction count if we aren't resetting on divide. */
AvidaArray<int> cPhenotype::GetCumulativeReactionCount()
{
  if (m_world->GetConfig().DIVIDE_METHOD.Get() == 0) {
    AvidaArray<int> cum_react;
    for (int i=0; i<cur_reaction_count.GetSize(); ++i) 
    {
      cum_react.Push(cur_reaction_count[i] + last_reaction_count[i]);
    }
//    return (cur_reaction_count + last_reaction_count); 
    return cum_react;
  } else {
    return cur_reaction_count;
  }
}
