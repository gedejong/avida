/*
 *  cPhenotype.h
 *  Avida
 *
 *  Called "phenotype.hh" prior to 12/5/05.
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

#ifndef cPhenotype_h
#define cPhenotype_h

#include "avida/core/InstructionSequence.h"

#include <fstream>

#include "cMerit.h"
#include "cString.h"
#include "cCodeLabel.h"
#include "cWorld.h"
#include "rust/running_stats_ffi.h"


/*************************************************************************
 *
 * The cPhenotype object contains a general description of all the
 * phenotypic characteristics an organism has displayed.  That is, it
 * monitors all of the organism's behaviors.
 *
 * After a phenotype is created in organism and organism within a population,
 * it must have either SetupOffspring() or SetupInject() run on it to prime
 * it for that population.  After that, it must have DivideReset() run on it
 * whenever it produces an offspring.
 *
 * If the phenotype is part of an organism in a test cpu, no initial priming
 * is required, and SetupTestDivide() needs to be run once it finally has
 * produced an offspring in order to properly lock in any final values.
 *
 * In addition to a reference to the relevent environment, the data
 * contained within this class comes in six flavors:
 *   1. Calculations made at the previous divide.
 *   2. Stats which are accumulated over each gestation cycle.
 *   3. The final result of accumulations over the previous gestation cycle.
 *   4. Accumulations over the entire life of the genome.
 *   5. A collection of flags to indicate the presence of characteristics.
 *   6. Information about the child being constructed.
 *
 *************************************************************************/

class cAvidaContext;
class cContextPhenotype;
class cEnvironment;
template <class T> class tBuffer;
template <class T> class tList;
class cTaskContext;
class cTaskState;
class cPhenPlastSummary;
class cReactionResult;

using namespace Avida;


class cPhenotype
{
  friend class cOrganism;
private:
  cWorld* m_world;
  bool initialized;

  // 1+2. Core scalar metrics — stored in Rust-owned #[repr(C)] struct.
  // Fields: merit, execution_ratio (was executionRatio), energy_store,
  //   genome_length, bonus_instruction_count, copied_size, executed_size,
  //   gestation_time, gestation_start, fitness, div_type, cur_bonus, cur_energy_bonus
  PhenotypeCoreMetrics m_core;
  double energy_tobe_applied;                 // Energy that has not yet been added to energy store.
  double energy_testament;
  double energy_received_buffer;              // Energy received through donation, but not yet applied to energy store
  double total_energy_donated;                // Tota amount of energy that has been donated
  double total_energy_received;               // Total amount of energy received through donations
  double total_energy_applied;                // Total amount of received energy applied to energy store
  int num_energy_requests;                    // Number of times organism has requested energy
  int num_energy_donations;                   // Number of times energy has been donated
  int num_energy_receptions;                  // Number of times organism has received energy donations
  int num_energy_applications;                // Number of times organism has applied donated energy to its energy store
  int cur_num_errors;                         // Total instructions executed illeagally.
  int cur_num_donates;                        // Number of donations so far

  AvidaArray<int> cur_task_count;                 // Total times each task was performed
  AvidaArray<int> cur_para_tasks;                 // Total times each task was performed by the parasite @LZ
  AvidaArray<int> cur_host_tasks;                 // Total times each task was done by JUST the host @LZ
  AvidaArray<int> cur_internal_task_count;        // Total times each task was performed using internal resources
  AvidaArray<int> eff_task_count;                 // Total times each task was performed (resetable during the life of the organism)
  AvidaArray<double> cur_task_quality;            // Average (total?) quality with which each task was performed
  AvidaArray<double> cur_task_value;              // Value with which this phenotype performs task
  AvidaArray<double> cur_internal_task_quality;   // Average (total?) quaility with which each task using internal resources was performed
  AvidaArray<double> cur_rbins_total;             // Total amount of resources collected over the organism's life
  AvidaArray<double> cur_rbins_avail;             // Amount of internal resources available
  AvidaArray<int> cur_collect_spec_counts;        // How many times each nop-specification was used in a collect-type instruction
  AvidaArray<int> cur_reaction_count;             // Total times each reaction was triggered.
  AvidaArray<int> first_reaction_cycles;          // CPU cycles of first time reaction was triggered.
  AvidaArray<int> first_reaction_execs;            // Execution count at first time reaction was triggered (will be > cycles in parallel exec multithreaded orgs).
  AvidaArray<int> cur_stolen_reaction_count;      // Total counts of reactions stolen by predators.
  AvidaArray<double> cur_reaction_add_reward;     // Bonus change from triggering each reaction.
  AvidaArray<int> cur_inst_count;                 // Instruction exection counter
  AvidaArray<int> cur_from_sensor_count;           // Use of inputs that originated from sensory data were used in execution of this instruction.
  AvidaArray<Apto::Array<int> > cur_group_attack_count;
  AvidaArray<Apto::Array<int> > cur_top_pred_group_attack_count;
  AvidaArray<int> cur_killed_targets;
  int cur_attacks;
  int cur_kills;
  
  AvidaArray<int> cur_sense_count;                // Total times resource combinations have been sensed; @JEB
  AvidaArray<double> sensed_resources;            // Resources which the organism has sensed; @JEB
  AvidaArray<double> cur_task_time;               // Time at which each task was last performed; WRE 03-18-07
  Apto::Map<void*, cTaskState*> m_task_states;
  AvidaArray<double> cur_trial_fitnesses;         // Fitnesses of various trials.; @JEB
  AvidaArray<double> cur_trial_bonuses;           // Bonuses of various trials.; @JEB
  AvidaArray<int> cur_trial_times_used;           // Time used in of various trials.; @JEB
  AvidaArray<int> cur_from_message_count;           // Use of inputs that originated from messages were used in execution of this instruction.

  int trial_time_used;                        // like time_used, but reset every trial; @JEB
  int trial_cpu_cycles_used;                  // like cpu_cycles_used, but reset every trial; @JEB
  tList<int> m_tolerance_immigrants;           // record of previous updates tolerance has been decreased towards immigrants 
  tList<int> m_tolerance_offspring_own;        // record of previous updates tolerance has been decreased towards org's own offspring 
  tList<int> m_tolerance_offspring_others;     // record of previous updates tolerance has been decreased towards other offspring in group 
  AvidaArray<pair<int,int> > m_intolerances;        // caches temporary values of the intolerance and the update
  double last_child_germline_propensity;   // chance of child being a germline cell; @JEB

  int mating_type;                            // Organism's phenotypic sex @CHC
  int mate_preference;                        // Organism's mating preference @CHC
  
  int cur_mating_display_a;                   // value of organism's current mating display A trait
  int cur_mating_display_b;                   // value of organism's current mating display B trait

  cReactionResult* m_reaction_result;
  


  // 3. These mark the status of "in progress" variables at the last divide.
  double last_merit_base;         // Either constant or based on genome length.
  double last_bonus;
  double last_energy_bonus;
  int last_num_errors;
  int last_num_donates;

  AvidaArray<int> last_task_count;
  AvidaArray<int> last_para_tasks;
  AvidaArray<int> last_host_tasks;                // Last task counts from hosts only, before last divide @LZ
  AvidaArray<int> last_internal_task_count;
  AvidaArray<double> last_task_quality;
  AvidaArray<double> last_task_value;
  AvidaArray<double> last_internal_task_quality;
  AvidaArray<double> last_rbins_total;
  AvidaArray<double> last_rbins_avail;
  AvidaArray<int> last_collect_spec_counts;
  AvidaArray<int> last_reaction_count;
  AvidaArray<double> last_reaction_add_reward;
  AvidaArray<int> last_inst_count;	  // Instruction exection counter
  AvidaArray<int> last_from_sensor_count;
  AvidaArray<int> last_sense_count;   // Total times resource combinations have been sensed; @JEB
  AvidaArray<Apto::Array<int> > last_group_attack_count;
  AvidaArray<Apto::Array<int> > last_top_pred_group_attack_count;
  AvidaArray<int> last_killed_targets;
  int last_attacks;
  int last_kills;

  AvidaArray<int> last_from_message_count;

  double last_fitness;            // Used to determine sterilization.
  int last_cpu_cycles_used;
  double cur_child_germline_propensity;   // chance of child being a germline cell; @JEB
  
  int last_mating_display_a;                   // value of organism's last mating display A trait
  int last_mating_display_b;                   // value of organism's last mating display B trait
  

  // 4. Records from this organism's life...
  int num_divides_failed; //Number of failed divide events @LZ
  int num_divides;       // Total successful divides organism has produced.
  int generation;        // Number of birth events to original ancestor.
  int cpu_cycles_used;   // Total CPU cycles consumed. @JEB
  int time_used;         // Total CPU cycles consumed, including additional time costs of some instructions.
  int num_execs;         // Total number of instructions executions attempted...accounts for parallel executions in multi-threaded orgs & corrects for cpu-cost 'pauses'
  int age;               // Number of updates organism has survived for.
  cString fault_desc;    // A description of the most recent error.
  double neutral_metric; // Undergoes drift (gausian 0,1) per generation
  double life_fitness; 	 // Organism fitness during its lifetime, 
		         // calculated based on merit just before the divide
  int exec_time_born;    // @MRR number of instructions since seed ancestor start
  double gmu_exec_time_born; //@MRR mutation-rate and gestation time scaled time of birth
  int birth_update;      // @MRR update *organism* born
  int birth_cell_id;
  int av_birth_cell_id;
  int birth_group_id;
  int birth_forager_type;
  AvidaArray<int> testCPU_inst_count;	  // Instruction exection counter as calculated by Test CPU
  int last_task_id; // id of the previous task
  int num_new_unique_reactions; // count the number of new unique reactions this organism has performed.
  double res_consumed; // amount of resources consumed since the organism last turned them over to the deme.
  bool is_germ_cell; // records whether or not the organism is part of the germline.
  int last_task_time; // time at which the previous task was performed
  
  

  
  // 5+6. Status flags + child info — stored in Rust-owned #[repr(C)] struct.
  // Bool fields are stored as int (0/1) for FFI safety.
  PhenotypeStatusFlags m_flags;

  // Dynamic arrays stay outside the flags struct.
  AvidaArray<bool> is_donor_locus; // Did this org target a donation at a specific locus.
  AvidaArray<bool> is_donor_locus_last; // Did this org's parent target a donation at a specific locus.

  // 7. Information that is set once (when organism was born)
  double permanent_germline_propensity;
  

  inline void SetInstSetSize(int inst_set_size);
  inline void SetGroupAttackInstSetSize(int num_group_attack_inst);
  
public:
  cPhenotype() : m_world(NULL), m_reaction_result(NULL) { ; } // Will not construct a valid cPhenotype! Only exists to support incorrect cDeme AvidaArray usage.
  cPhenotype(cWorld* world, int parent_generation, int num_nops);


  cPhenotype(const cPhenotype&); 
  cPhenotype& operator=(const cPhenotype&); 
  ~cPhenotype();
  
  enum energy_levels {ENERGY_LEVEL_LOW = 0, ENERGY_LEVEL_MEDIUM, ENERGY_LEVEL_HIGH};
	
  void ResetMerit();
  void Sterilize();
  // Run when being setup *as* and offspring.
  void SetupOffspring(const cPhenotype & parent_phenotype, const InstructionSequence & _genome);

  // Run when being setup as an injected organism.
  void SetupInject(const InstructionSequence & _genome);

  // Run when this organism successfully executes a divide.
  void DivideReset(const InstructionSequence & _genome);
  
  // Same as DivideReset(), but only run in test CPUs.
  void TestDivideReset(const InstructionSequence & _genome);

  // Run when an organism is being forced to replicate, but not at the end
  // of its replication cycle.  Assume exact clone with no mutations.
  void SetupClone(const cPhenotype & clone_phenotype);

  // Input and Output Reaction Tests
  bool TestInput(tBuffer<int>& inputs, tBuffer<int>& outputs);
  bool TestOutput(cAvidaContext& ctx, cTaskContext& taskctx,
                  const AvidaArray<double>& res_in, const AvidaArray<double>& rbins_in, AvidaArray<double>& res_change,
                  AvidaArray<cString>& insts_triggered, bool is_parasite=false, cContextPhenotype* context_phenotype = 0);

  // State saving and loading, and printing...
  void PrintStatus(std::ostream& fp) const;

  // Some useful methods...
  int CalcSizeMerit() const;
  double CalcCurrentMerit() const;
  double CalcFitness(double _merit_base, double _bonus, int _gestation_time, int _cpu_cycles) const;

  double CalcFitnessRatio() {
    
    //LZ this was int!
    const double merit_base = CalcSizeMerit();
    const double cur_fitness = merit_base * m_core.cur_bonus / time_used;
    return cur_fitness / last_fitness;
  }
  int CalcID() const {
    int phen_id = 0;
    for (int i = 0; i < last_task_count.GetSize(); i++) {
      if (last_task_count[i] > 0) phen_id += (1 << i);
    }
    return phen_id;
  }

  /////////////////////  Accessors -- Retrieving  ////////////////////
  // cMerit and AvidaMerit share identical memory layout.
  const cMerit & GetMerit() const { assert(initialized == true); return reinterpret_cast<const cMerit&>(m_core.merit); }
  double GetEnergyUsageRatio() const { assert(initialized == true); return m_core.execution_ratio; }
  int GetGenomeLength() const { assert(initialized == true); return m_core.genome_length; }
  int GetCopiedSize() const { assert(initialized == true); return m_core.copied_size; }
  int GetExecutedSize() const { assert(initialized == true); return m_core.executed_size; }
  int GetGestationTime() const { assert(initialized == true); return m_core.gestation_time; }
  int GetGestationStart() const { assert(initialized == true); return m_core.gestation_start; }
  double GetFitness() const { assert(initialized == true); return m_core.fitness; }
  double GetDivType() const { assert(initialized == true); return m_core.div_type; }

  double GetCurBonus() const { assert(initialized == true); return m_core.cur_bonus; }
  int    GetCurBonusInstCount() const { assert(m_core.bonus_instruction_count >= 0); return m_core.bonus_instruction_count; }

  double GetCurMeritBase() const { assert(initialized == true); return CalcSizeMerit(); }
  double GetStoredEnergy() const { return m_core.energy_store; }
  double GetEnergyBonus() const { assert(initialized == true); return m_core.cur_energy_bonus; }
  int GetDiscreteEnergyLevel() const;
  double GetEnergyInBufferAmount() const { return energy_received_buffer; }
  
  double ConvertEnergyToMerit(double energy) const;
  
  //@MRR Organism-specific birth tracking
  double GetGMuExecTimeBorn() const {return gmu_exec_time_born;}
  int GetExecTimeBorn() const {return exec_time_born;}
  int GetUpdateBorn() const {return birth_update;}
  
  int GetBirthCell() const { return birth_cell_id; }
  int GetAVBirthCell() const { return av_birth_cell_id; }
  int GetBirthGroupID() const { return birth_group_id; }
  int GetBirthForagerType() const { return birth_forager_type; }
  inline void SetBirthCellID(int birth_cell);
  inline void SetAVBirthCellID(int av_birth_cell);
  inline void SetBirthGroupID(int group_id);
  inline void SetBirthForagerType(int forager_type);

  int GetMatingType() const { return mating_type; } //@CHC
  int GetMatePreference() const { return mate_preference; } //@CHC

  int GetCurMatingDisplayA() const { return cur_mating_display_a; } //@CHC
  int GetCurMatingDisplayB() const { return cur_mating_display_b; } //@CHC
  int GetLastMatingDisplayA() const { return last_mating_display_a; } //@CHC
  int GetLastMatingDisplayB() const { return last_mating_display_b; } //@CHC

  bool GetMakeRandomResource() const {assert(initialized == true); return m_flags.make_random_resource != 0;}
  bool GetToDie() const { assert(initialized == true); return m_flags.to_die != 0; }
  bool GetToDelete() const { assert(initialized == true); return m_flags.to_delete != 0; }
  int GetCurNumErrors() const { assert(initialized == true); return cur_num_errors; }
  int GetCurNumDonates() const { assert(initialized == true); return cur_num_donates; }
  int GetCurCountForTask(int idx) const { assert(initialized == true); return cur_task_count[idx]; }
  const AvidaArray<int>& GetCurTaskCount() const { assert(initialized == true); return cur_task_count; }
  const AvidaArray<int>& GetCurHostTaskCount() const { assert(initialized == true); return cur_host_tasks; }
  const AvidaArray<int>& GetCurParasiteTaskCount() const { assert(initialized == true); return cur_para_tasks; }
  const AvidaArray<int>& GetCurInternalTaskCount() const { assert(initialized == true); return cur_internal_task_count; }
  void ClearEffTaskCount() { assert(initialized == true); eff_task_count.SetAll(0); }
  const AvidaArray<double> & GetCurTaskQuality() const { assert(initialized == true); return cur_task_quality; }
  const AvidaArray<double> & GetCurTaskValue() const { assert(initialized == true); return cur_task_value; }
  const AvidaArray<double> & GetCurInternalTaskQuality() const { assert(initialized == true); return cur_internal_task_quality; }
  const AvidaArray<double>& GetCurRBinsTotal() const { assert(initialized == true); return cur_rbins_total; }
  double GetCurRBinTotal(int index) const { assert(initialized == true); return cur_rbins_total[index]; }
  const AvidaArray<double>& GetCurRBinsAvail() const { assert(initialized == true); return cur_rbins_avail; }
  double GetCurRBinAvail(int index) const { assert(initialized == true); return cur_rbins_avail[index]; }

  const AvidaArray<int>& GetCurReactionCount() const { assert(initialized == true); return cur_reaction_count;}
  const AvidaArray<int>& GetFirstReactionCycles() const { assert(initialized == true); return first_reaction_cycles;}
  void SetFirstReactionCycle(int idx) { if (first_reaction_cycles[idx] < 0) first_reaction_cycles[idx] = time_used; }
  const AvidaArray<int>& GetFirstReactionExecs() const { assert(initialized == true); return first_reaction_execs;}
  void SetFirstReactionExec(int idx) { if (first_reaction_execs[idx] < 0) first_reaction_execs[idx] = num_execs; }

  const AvidaArray<int>& GetStolenReactionCount() const { assert(initialized == true); return cur_stolen_reaction_count;}
  const AvidaArray<double>& GetCurReactionAddReward() const { assert(initialized == true); return cur_reaction_add_reward;}
  const AvidaArray<int>& GetCurInstCount() const { assert(initialized == true); return cur_inst_count; }
  const AvidaArray<int>& GetCurSenseCount() const { assert(initialized == true); return cur_sense_count; }

  double GetSensedResource(int _in) { assert(initialized == true); return sensed_resources[_in]; }
  const AvidaArray<int>& GetCurCollectSpecCounts() const { assert(initialized == true); return cur_collect_spec_counts; }
  int GetCurCollectSpecCount(int spec_id) const { assert(initialized == true); return cur_collect_spec_counts[spec_id]; }
  const AvidaArray<int>& GetTestCPUInstCount() const { assert(initialized == true); return testCPU_inst_count; }

  void  NewTrial(); //Save the current fitness, and reset the bonus. @JEB
  void  TrialDivideReset(const InstructionSequence & _genome); //Subset of resets specific to division not done by NewTrial. @JEB
  const AvidaArray<double>& GetTrialFitnesses() { return cur_trial_fitnesses; }; //Return list of trial fitnesses. @JEB
  const AvidaArray<double>& GetTrialBonuses() { return cur_trial_bonuses; }; //Return list of trial bonuses. @JEB
  const AvidaArray<int>& GetTrialTimesUsed() { return cur_trial_times_used; }; //Return list of trial times used. @JEB

  tList<int>& GetToleranceImmigrants() { assert(initialized == true); return m_tolerance_immigrants; }
  tList<int>& GetToleranceOffspringOwn() { assert(initialized == true); return m_tolerance_offspring_own; }
  tList<int>& GetToleranceOffspringOthers() { assert(initialized == true); return m_tolerance_offspring_others; }
  AvidaArray<pair<int,int> >& GetIntolerances() { assert(initialized == true); return m_intolerances; }
  int CalcToleranceImmigrants();
  int CalcToleranceOffspringOwn();
  int CalcToleranceOffspringOthers();

  double GetLastMeritBase() const { assert(initialized == true); return last_merit_base; }
  double GetLastBonus() const { assert(initialized == true); return last_bonus; }

  double GetLastMerit() const { assert(initialized == true); return last_merit_base*last_bonus; }
  int GetLastNumErrors() const { assert(initialized == true); return last_num_errors; }
  int GetLastNumDonates() const { assert(initialized == true); return last_num_donates; }

  int GetLastCountForTask(int idx) const { assert(initialized == true); return last_task_count[idx]; }
  const AvidaArray<int>& GetLastTaskCount() const { assert(initialized == true); return last_task_count; }
  void SetLastTaskCount(AvidaArray<int> tasks) { assert(initialized == true); last_task_count = tasks; }
  const AvidaArray<int>& GetLastHostTaskCount() const { assert(initialized == true); return last_host_tasks; }
  const AvidaArray<int>& GetLastParasiteTaskCount() const { assert(initialized == true); return last_para_tasks; }
  void  SetLastParasiteTaskCount(AvidaArray<int>  oldParaPhenotype);
  const AvidaArray<int>& GetLastInternalTaskCount() const { assert(initialized == true); return last_internal_task_count; }
  const AvidaArray<double>& GetLastTaskQuality() const { assert(initialized == true); return last_task_quality; }
  const AvidaArray<double>& GetLastTaskValue() const { assert(initialized == true); return last_task_value; }
  const AvidaArray<double>& GetLastInternalTaskQuality() const { assert(initialized == true); return last_internal_task_quality; }
  const AvidaArray<double>& GetLastRBinsTotal() const { assert(initialized == true); return last_rbins_total; }
  const AvidaArray<double>& GetLastRBinsAvail() const { assert(initialized == true); return last_rbins_avail; }
  const AvidaArray<int>& GetLastReactionCount() const { assert(initialized == true); return last_reaction_count; }
  const AvidaArray<double>& GetLastReactionAddReward() const { assert(initialized == true); return last_reaction_add_reward; }
  const AvidaArray<int>& GetLastInstCount() const { assert(initialized == true); return last_inst_count; }
  const AvidaArray<int>& GetLastFromSensorInstCount() const { assert(initialized == true); return last_from_sensor_count; }
  const AvidaArray<int>& GetLastSenseCount() const { assert(initialized == true); return last_sense_count; }
  const AvidaArray<Apto::Array<int> >& GetLastGroupAttackInstCount() const { assert(initialized == true); return last_group_attack_count; }
  const AvidaArray<Apto::Array<int> >& GetLastTopPredGroupAttackInstCount() const { assert(initialized == true); return last_top_pred_group_attack_count; }

  const AvidaArray<int>& GetLastFromMessageInstCount() const { assert(initialized == true); return last_from_message_count; }

  double GetLastFitness() const { assert(initialized == true); return last_fitness; }
  double GetPermanentGermlinePropensity() const { assert(initialized == true); return permanent_germline_propensity; }
  const AvidaArray<int>& GetLastCollectSpecCounts() const { assert(initialized == true); return last_collect_spec_counts; }
  int GetLastCollectSpecCount(int spec_id) const { assert(initialized == true); return last_collect_spec_counts[spec_id]; }

  int GetNumDivides() const { assert(initialized == true); return num_divides;}
  int GetNumDivideFailed() const { assert(initialized == true); return num_divides_failed;}

  int GetGeneration() const { return generation; }
  int GetCPUCyclesUsed() const { assert(initialized == true); return cpu_cycles_used; }
  int GetTimeUsed()   const { assert(initialized == true); return time_used; }
  int GetNumExecs() const { assert(initialized == true); return num_execs; }
  int GetTrialTimeUsed()   const { assert(initialized == true); return trial_time_used; }
  int GetAge()        const { assert(initialized == true); return age; }
  const cString& GetFault() const { assert(initialized == true); return fault_desc; }
  double GetNeutralMetric() const { assert(initialized == true); return neutral_metric; }
  double GetLifeFitness() const { assert(initialized == true); return life_fitness; }
  int  GetNumThreshGbDonations() const { assert(initialized == true); return m_flags.num_thresh_gb_donations; }
  int  GetNumThreshGbDonationsLast() const { assert(initialized == true); return m_flags.num_thresh_gb_donations_last; }
  int  GetNumQuantaThreshGbDonations() const { assert(initialized == true); return m_flags.num_quanta_thresh_gb_donations; }
  int  GetNumQuantaThreshGbDonationsLast() const { assert(initialized == true); return m_flags.num_quanta_thresh_gb_donations_last; }
  int  GetNumShadedGbDonations() const { assert(initialized == true); return m_flags.num_shaded_gb_donations; }
  int  GetNumShadedGbDonationsLast() const { assert(initialized == true); return m_flags.num_shaded_gb_donations_last; }
  int GetNumDonationsLocus() const { assert(initialized == true); return m_flags.num_donations_locus; }
  int GetNumDonationsLocusLast() const { assert(initialized == true); return m_flags.num_donations_locus_last; }

  bool IsInjected() const { assert(initialized == true); return m_flags.is_injected != 0; }
  bool IsClone() const { assert(initialized == true); return m_flags.is_clone != 0; }
  bool IsDonorCur() const { assert(initialized == true); return m_flags.is_donor_cur != 0; }
  bool IsDonorLast() const { assert(initialized == true); return m_flags.is_donor_last != 0; }
  bool IsDonorRand() const { assert(initialized == true); return m_flags.is_donor_rand != 0; }
  bool IsDonorRandLast() const { assert(initialized == true); return m_flags.is_donor_rand_last != 0; }
  bool IsDonorKin() const { assert(initialized == true); return m_flags.is_donor_kin != 0; }
  bool IsDonorKinLast() const { assert(initialized == true); return m_flags.is_donor_kin_last != 0; }
  bool IsDonorEdit() const { assert(initialized == true); return m_flags.is_donor_edit != 0; }
  bool IsDonorEditLast() const { assert(initialized == true); return m_flags.is_donor_edit_last != 0; }
  bool IsDonorGbg() const { assert(initialized == true); return m_flags.is_donor_gbg != 0; }
  bool IsDonorGbgLast() const { assert(initialized == true); return m_flags.is_donor_gbg_last != 0; }
  bool IsDonorTrueGb() const { assert(initialized == true); return m_flags.is_donor_truegb != 0; }
  bool IsDonorTrueGbLast() const { assert(initialized == true); return m_flags.is_donor_truegb_last != 0; }
  bool IsDonorThreshGb() const { assert(initialized == true); return m_flags.is_donor_threshgb != 0; }
  bool IsDonorThreshGbLast() const { assert(initialized == true); return m_flags.is_donor_threshgb_last != 0; }
  bool IsDonorQuantaThreshGb() const { assert(initialized == true); return m_flags.is_donor_quanta_threshgb != 0; }
  bool IsDonorQuantaThreshGbLast() const { assert(initialized == true); return m_flags.is_donor_quanta_threshgb_last != 0; }
  bool IsDonorShadedGb() const { assert(initialized == true); return m_flags.is_donor_shadedgb != 0; }
  bool IsDonorShadedGbLast() const { assert(initialized == true); return m_flags.is_donor_shadedgb_last != 0; }
  bool IsDonorPosition(int pos) const {assert(initialized == true); return is_donor_locus.GetSize() > pos ? is_donor_locus[pos] : 0; }
  bool IsDonorPositionLast(int pos) const {assert(initialized == true); return is_donor_locus_last.GetSize() > pos ? is_donor_locus_last[pos] : 0; }

  bool IsEnergyRequestor() const { assert(initialized == true); return m_flags.is_energy_requestor != 0; }
  bool IsEnergyDonor() const { assert(initialized == true); return m_flags.is_energy_donor != 0; }
  bool IsEnergyReceiver() const { assert(initialized == true); return m_flags.is_energy_receiver != 0; }
  bool HasUsedEnergyDonation() const { assert(initialized == true); return m_flags.has_used_donated_energy != 0; }
  bool HasOpenEnergyRequest() const { assert(initialized == true); return m_flags.has_open_energy_request != 0; }
  bool IsReceiver() const { assert(initialized == true); return m_flags.is_receiver != 0; }
  bool IsReceiverLast() const { assert(initialized == true); return m_flags.is_receiver_last != 0; }
  bool IsReceiverRand() const { assert(initialized == true); return m_flags.is_receiver_rand != 0; }
  bool IsReceiverKin() const { assert(initialized == true); return m_flags.is_receiver_kin != 0; }
  bool IsReceiverKinLast() const { assert(initialized == true); return m_flags.is_receiver_kin_last != 0; }
  bool IsReceiverEdit() const { assert(initialized == true); return m_flags.is_receiver_edit != 0; }
  bool IsReceiverEditLast() const { assert(initialized == true); return m_flags.is_receiver_edit_last != 0; }
  bool IsReceiverGbg() const { assert(initialized == true); return m_flags.is_receiver_gbg != 0; }
  bool IsReceiverTrueGb() const { assert(initialized == true); return m_flags.is_receiver_truegb != 0; }
  bool IsReceiverTrueGbLast() const { assert(initialized == true); return m_flags.is_receiver_truegb_last != 0; }
  bool IsReceiverThreshGb() const { assert(initialized == true); return m_flags.is_receiver_threshgb != 0; }
  bool IsReceiverThreshGbLast() const { assert(initialized == true); return m_flags.is_receiver_threshgb_last != 0; }
  bool IsReceiverQuantaThreshGb() const { assert(initialized == true); return m_flags.is_receiver_quanta_threshgb != 0; }
  bool IsReceiverQuantaThreshGbLast() const { assert(initialized == true); return m_flags.is_receiver_quanta_threshgb_last != 0; }
  bool IsReceiverShadedGb() const { assert(initialized == true); return m_flags.is_receiver_shadedgb != 0; }
  bool IsReceiverShadedGbLast() const { assert(initialized == true); return m_flags.is_receiver_shadedgb_last != 0; }
  bool IsReceiverGBSameLocus() const { assert(initialized == true); return m_flags.is_receiver_gb_same_locus != 0; }
  bool IsReceiverGBSameLocusLast() const { assert(initialized == true); return m_flags.is_receiver_gb_same_locus_last != 0; }
  bool IsModifier() const { assert(initialized == true); return m_flags.is_modifier != 0; }
  bool IsModified() const { assert(initialized == true); return m_flags.is_modified != 0; }
  bool IsFertile() const  { assert(initialized == true); return m_flags.is_fertile != 0; }
  bool IsMutated() const  { assert(initialized == true); return m_flags.is_mutated != 0; }
  bool IsMultiThread() const { assert(initialized == true); return m_flags.is_multi_thread != 0; }
  bool ParentTrue() const { assert(initialized == true); return m_flags.parent_true != 0; }
  bool ParentSex() const  { assert(initialized == true); return m_flags.parent_sex != 0; }
  int  ParentCrossNum() const  { assert(initialized == true); return m_flags.parent_cross_num; }
  bool BornParentGroup() const { assert(initialized == true); return m_flags.born_parent_group != 0; }

  bool CopyTrue() const   { assert(initialized == true); return m_flags.copy_true != 0; }
  bool DivideSex() const  { assert(initialized == true); return m_flags.divide_sex != 0; }
  int MateSelectID() const { assert(initialized == true); return m_flags.mate_select_id; }
  int CrossNum() const  { assert(initialized == true); return m_flags.cross_num; }
  bool ChildFertile() const { assert(initialized == true); return m_flags.child_fertile != 0;}
  int GetChildCopiedSize() const { assert(initialized == true); return m_flags.child_copied_size; }
  


  ////////////////////  Accessors -- Modifying  ///////////////////
  void SetMerit(const cMerit& in_merit) { reinterpret_cast<cMerit&>(m_core.merit) = in_merit; }
  void SetFitness(const double in_fit) { m_core.fitness = in_fit; }
  void ReduceEnergy(const double cost);
  void SetEnergy(const double value);
  void SetGestationTime(int in_time) { m_core.gestation_time = in_time; }
  void SetTimeUsed(int in_time) { time_used = in_time; }
  void SetTrialTimeUsed(int in_time) { trial_time_used = in_time; }
  void SetGeneration(int in_generation) { generation = in_generation; }
  void SetPermanentGermlinePropensity(double _in) { permanent_germline_propensity = _in; }
  void SetFault(const cString& in_fault) { fault_desc = in_fault; }
  void SetNeutralMetric(double _in){ neutral_metric = _in; }
  void SetLifeFitness(double _in){ life_fitness = _in; }
  void SetLinesExecuted(int _exe_size) { m_core.executed_size = _exe_size; }
  void SetLinesCopied(int _copied_size) { m_flags.child_copied_size = _copied_size; }
  void SetDivType(double _div_type) { m_core.div_type = _div_type; }
  void SetDivideSex(bool _divide_sex) { m_flags.divide_sex = _divide_sex ? 1 : 0; }
  void SetMateSelectID(int _select_id) { m_flags.mate_select_id = _select_id; }
  void SetCrossNum(int _cross_num) { m_flags.cross_num = _cross_num; }
  void SetToDie() { m_flags.to_die = 1; }
  void SetToDelete() { m_flags.to_delete = 1; }
  void SetTestCPUInstCount(const AvidaArray<int>& in_counts) { testCPU_inst_count = in_counts; }
  void IncreaseEnergyDonated(double amount) { assert(amount >=0); total_energy_donated += amount; }
  void IncreaseEnergyReceived(double amount) { assert(amount >=0); total_energy_received += amount; }
  void IncreaseEnergyApplied(double amount) { assert(amount >=0); total_energy_applied += amount; }
  void IncreaseNumEnergyRequests() { num_energy_requests++; }
  void IncreaseNumEnergyDonations() { num_energy_donations++; }
  void IncreaseNumEnergyApplications() { num_energy_applications++; }
  void IncreaseNumEnergyReceptions() { num_energy_receptions++; }
  double GetAmountEnergyDonated() { return total_energy_donated; }
  double GetAmountEnergyReceived() { return total_energy_received; }
  double GetAmountEnergyApplied() { return total_energy_applied; }
  int GetNumEnergyDonations() { return num_energy_donations; }
  int GetNumEnergyReceptions() { return num_energy_receptions; }
  int GetNumEnergyApplications() { return num_energy_applications; }
  
  void SetReactionCount(int index, int val) { cur_reaction_count[index] = val; }
  void SetStolenReactionCount(int index, int val) { cur_stolen_reaction_count[index] = val; }
  
  bool GetKaboomExecuted() {return m_flags.kaboom_executed != 0;} //@AEJ
  void SetKaboomExecuted(bool value) {m_flags.kaboom_executed = value ? 1 : 0;} //@AEJ
  bool GetKaboomExecuted2() {return m_flags.kaboom_executed2 != 0;} //@AEJ
  void SetKaboomExecuted2(bool value) {m_flags.kaboom_executed2 = value ? 1 : 0;} //@AEJ
  void ClearKaboomExecuted() {m_flags.kaboom_executed = 0;} //@AEJ


  void SetCurRBinsAvail(const AvidaArray<double>& in_avail) { cur_rbins_avail = in_avail; }
  void SetCurRbinsTotal(const AvidaArray<double>& in_total) { cur_rbins_total = in_total; }
  void SetCurRBinAvail(int index, double val) { cur_rbins_avail[index] = val; }
  void SetCurRBinTotal(int index, double val) { cur_rbins_total[index] = val; }
  void AddToCurRBinAvail(int index, double val) { cur_rbins_avail[index] += val; }
  void AddToCurRBinTotal(int index, double val) { cur_rbins_total[index] += val; }
  void SetCurCollectSpecCount(int spec_id, int val) { cur_collect_spec_counts[spec_id] = val; }

  void SetMatingType(int _mating_type) { mating_type = _mating_type; } //@CHC
  void SetMatePreference(int _mate_preference) { mate_preference = _mate_preference; } //@CHC

  void SetIsMultiThread() { m_flags.is_multi_thread = 1; }
  void SetIsDonorCur() { m_flags.is_donor_cur = 1; }
  void SetIsDonorRand() { SetIsDonorCur(); m_flags.is_donor_rand = 1; }
  void SetIsDonorKin() { SetIsDonorCur(); m_flags.is_donor_kin = 1; }
  void SetIsDonorNull() { SetIsDonorCur(); m_flags.is_donor_null = 1; }
  void SetIsDonorEdit() { SetIsDonorCur(); m_flags.is_donor_edit = 1; }
  void SetIsDonorGbg() { SetIsDonorCur(); m_flags.is_donor_gbg = 1; }
  void SetIsDonorTrueGb() { SetIsDonorCur(); m_flags.is_donor_truegb = 1; }
  void SetIsDonorThreshGb() { SetIsDonorCur(); m_flags.is_donor_threshgb = 1; }
  void SetIsDonorQuantaThreshGb() { SetIsDonorCur(); m_flags.is_donor_quanta_threshgb = 1; }
  void SetIsDonorShadedGb() { SetIsDonorCur(); m_flags.is_donor_shadedgb = 1; }
  void SetIsDonorPosition(int pos) { SetIsDonorCur(); if (is_donor_locus.GetSize() <= pos) is_donor_locus.Resize(pos+1, false); is_donor_locus[pos] = true; }
  void SetIsReceiver() { m_flags.is_receiver = 1; }
  void SetIsReceiverRand() { SetIsReceiver(); m_flags.is_receiver_rand = 1; }
  void SetIsReceiverKin() { SetIsReceiver(); m_flags.is_receiver_kin = 1; }
  void SetIsReceiverEdit() { SetIsReceiver(); m_flags.is_receiver_edit = 1; }
  void SetIsReceiverGbg() { SetIsReceiver(); m_flags.is_receiver_gbg = 1; }
  void SetIsReceiverTrueGb() { SetIsReceiver(); m_flags.is_receiver_truegb = 1; }
  void SetIsReceiverThreshGb() { SetIsReceiver(); m_flags.is_receiver_threshgb = 1; }
  void SetIsReceiverQuantaThreshGb() { SetIsReceiver(); m_flags.is_receiver_quanta_threshgb = 1; }
  void SetIsReceiverShadedGb() { SetIsReceiver(); m_flags.is_receiver_shadedgb = 1; }
  void SetIsReceiverGBSameLocus() { SetIsReceiver(); m_flags.is_receiver_gb_same_locus = 1; }
  void SetIsEnergyRequestor() { m_flags.is_energy_requestor = 1; }
  void SetIsEnergyDonor() { m_flags.is_energy_donor = 1; }
  void SetIsEnergyReceiver() { m_flags.is_energy_receiver = 1; }
  int& SetBornParentGroup() { return m_flags.born_parent_group; }
  void SetHasUsedDonatedEnergy() { m_flags.has_used_donated_energy = 1; }
  void SetHasOpenEnergyRequest() { m_flags.has_open_energy_request = 1; }
  void ClearHasOpenEnergyRequest() { m_flags.has_open_energy_request = 0; }
  void ClearIsMultiThread() { m_flags.is_multi_thread = 0; }
  
  void SetCurBonus(double _bonus) { m_core.cur_bonus = _bonus; }
  void SetCurBonusInstCount(int _num_bonus_inst) { m_core.bonus_instruction_count = _num_bonus_inst; }

  void IncCurInstCount(int _inst_num)  { assert(initialized == true); cur_inst_count[_inst_num]++; } 
  void DecCurInstCount(int _inst_num)  { assert(initialized == true); cur_inst_count[_inst_num]--; }
  void IncCurFromSensorInstCount(int _inst_num)  { assert(initialized == true); cur_from_sensor_count[_inst_num]++; }
  void IncCurGroupAttackInstCount(int _inst_num, int pack_size_idx)  { assert(initialized == true); cur_group_attack_count[_inst_num][pack_size_idx]++; }
  void IncCurTopPredGroupAttackInstCount(int _inst_num, int pack_size_idx)  { assert(initialized == true); cur_top_pred_group_attack_count[_inst_num][pack_size_idx]++; }
  void IncAttackedPreyFTData(int target_ft);
  AvidaArray<int> GetKilledPreyFTData() { return cur_killed_targets; }
  void IncAttacks() { cur_attacks++; }
  void IncKills() { cur_kills++; }
  int GetLastAttacks() const { return last_attacks; }
  int GetLastKills() const { return last_kills; }
  
  void IncNumThreshGbDonations() { assert(initialized == true); m_flags.num_thresh_gb_donations++; }
  void IncNumQuantaThreshGbDonations() { assert(initialized == true); m_flags.num_quanta_thresh_gb_donations++; }
  void IncNumShadedGbDonations() { assert(initialized == true); m_flags.num_shaded_gb_donations++; }
  void IncNumGreenBeardSameLocus() { assert(initialized == true); m_flags.num_donations_locus++; }	
  void IncAge()      { assert(initialized == true); age++; }
  void IncCPUCyclesUsed() { assert(initialized == true); cpu_cycles_used++; trial_cpu_cycles_used++; }
  void DecCPUCyclesUsed() { assert(initialized == true); cpu_cycles_used--; trial_cpu_cycles_used--; }
  void IncTimeUsed(int i=1) { assert(initialized == true); time_used+=i; trial_time_used+=i; }
  void IncNumExecs() { assert(initialized == true); num_execs++; }
  void IncErrors()   { assert(initialized == true); cur_num_errors++; }
  void IncDonates()   { assert(initialized == true); cur_num_donates++; }
  void IncSenseCount(const int) { /*assert(initialized == true); cur_sense_count[i]++;*/ }  
  
  void SetCurMatingDisplayA(int _cur_mating_display_a) { cur_mating_display_a = _cur_mating_display_a; } //@CHC
  void SetCurMatingDisplayB(int _cur_mating_display_b) { cur_mating_display_b = _cur_mating_display_b; } //@CHC
  void SetLastMatingDisplayA(int _last_mating_display_a) { last_mating_display_a = _last_mating_display_a; } //@CHC
  void SetLastMatingDisplayB(int _last_mating_display_b) { last_mating_display_b = _last_mating_display_b; } //@CHC
  
  int& IsInjected() { assert(initialized == true); return m_flags.is_injected; }
  int& IsClone() { assert(initialized == true); return m_flags.is_clone; }
  int& IsModifier() { assert(initialized == true); return m_flags.is_modifier; }
  int& IsModified() { assert(initialized == true); return m_flags.is_modified; }
  int& IsFertile()  { assert(initialized == true); return m_flags.is_fertile; }
  int& IsMutated()  { assert(initialized == true); return m_flags.is_mutated; }
  int& ParentTrue() { assert(initialized == true); return m_flags.parent_true; }
  int& ParentSex()  { assert(initialized == true); return m_flags.parent_sex; }
  int& ParentCrossNum()  { assert(initialized == true); return m_flags.parent_cross_num; }
  int& CopyTrue()   { assert(initialized == true); return m_flags.copy_true; }
  int& DivideSex()  { assert(initialized == true); return m_flags.divide_sex; }
  int& MateSelectID() { assert(initialized == true); return m_flags.mate_select_id; }
  int& CrossNum()     { assert(initialized == true); return m_flags.cross_num; }
  int& ChildFertile() { assert(initialized == true); return m_flags.child_fertile; }
  int& IsMultiThread() { assert(initialized == true); return m_flags.is_multi_thread; }
  
  void DoubleEnergyUsage();
  void HalveEnergyUsage();
  void DefaultEnergyUsage();
	
  // --- Support for Division of Labor --- //
  int GetLastTaskID() const { return last_task_id; }
  int  GetNumNewUniqueReactions() const {assert(initialized == true);  return num_new_unique_reactions; }
  void  ResetNumNewUniqueReactions()  {num_new_unique_reactions =0; }
  double GetResourcesConsumed(); 
  AvidaArray<int> GetCumulativeReactionCount();
  void IncCurFromMessageInstCount(int _inst_num)  { assert(initialized == true); cur_from_message_count[_inst_num]++; }
 

  // @LZ - Parasite Etc. Helpers
  void DivideFailed();
  void UpdateParasiteTasks() { last_para_tasks = cur_para_tasks; cur_para_tasks.SetAll(0); return; }
  

  void RefreshEnergy();
  void ApplyToEnergyStore();
  void EnergyTestament(const double value); //! external energy given to organism
  void ApplyDonatedEnergy();
  void ReceiveDonatedEnergy(const double value);
  double ExtractParentEnergy();
  
  // Compare two phenotypes and determine an ordering (arbitrary, but consistant among phenotypes).
  static int Compare(const cPhenotype* lhs, const cPhenotype* rhs);

  // This pseudo-function is used to help sort phenotypes
  struct PhenotypeCompare {
    bool operator()(const cPhenotype* lhs, const cPhenotype* rhs) const;
  };
};


inline void cPhenotype::SetInstSetSize(int inst_set_size)
{
  cur_inst_count.Resize(inst_set_size, 0);
  cur_from_sensor_count.Resize(inst_set_size, 0);
  cur_from_message_count.Resize(inst_set_size, 0);
  last_inst_count.Resize(inst_set_size, 0);
  last_from_sensor_count.Resize(inst_set_size, 0);
  last_from_message_count.Resize(inst_set_size, 0);
}

inline void cPhenotype::SetGroupAttackInstSetSize(int num_group_attack_inst)
{
  last_group_attack_count.Resize(num_group_attack_inst);
  last_top_pred_group_attack_count.Resize(num_group_attack_inst);
  cur_group_attack_count.Resize(num_group_attack_inst);
  cur_top_pred_group_attack_count.Resize(num_group_attack_inst);
  for (int i = 0; i < last_group_attack_count.GetSize(); i++) {
    last_group_attack_count[i].Resize(20, 0);
    last_top_pred_group_attack_count[i].Resize(20, 0);
    cur_group_attack_count[i].Resize(20, 0);
    cur_top_pred_group_attack_count[i].Resize(20, 0);
  }
}

inline void cPhenotype::SetBirthCellID(int birth_cell) { birth_cell_id = birth_cell; }
inline void cPhenotype::SetAVBirthCellID(int av_birth_cell) { av_birth_cell_id = av_birth_cell; }
inline void cPhenotype::SetBirthGroupID(int group_id) { birth_group_id = group_id; }
inline void cPhenotype::SetBirthForagerType(int forager_type) { birth_forager_type = forager_type; }

#endif
