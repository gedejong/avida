/*
 *  cOrganismFFI.cc — Thin read-only FFI layer for organism/cell/population access.
 *
 *  Exposes organism properties to Rust via opaque pointers.
 *  C++ retains ownership; Rust reads through these accessors.
 *  Uses non-const pointers because many C++ methods lack const qualification.
 */

#include "cOrganism.h"
#include "cPhenotype.h"
#include "cPopulation.h"
#include "cPopulationCell.h"
#include "cDeme.h"
#include "cDemeCellEvent.h"
#include "cStats.h"
#include "cWorld.h"
#include "cEnvironment.h"
#include "cOrgMessage.h"
#include "rust/running_stats_ffi.h"

extern "C" {

// ---- Organism accessors ----

double avd_org_get_fitness(cOrganism* org) {
  if (!org) return 0.0;
  return org->GetPhenotype().GetFitness();
}

double avd_org_get_merit(cOrganism* org) {
  if (!org) return 0.0;
  return org->GetPhenotype().GetMerit().GetDouble();
}

int avd_org_get_gestation_time(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().GetGestationTime();
}

int avd_org_get_genome_length(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().GetGenomeLength();
}

int avd_org_get_forage_target(cOrganism* org) {
  if (!org) return -1;
  return org->GetForageTarget();
}

int avd_org_get_cell_id(cOrganism* org) {
  if (!org) return -1;
  return org->GetCellID();
}

double avd_org_get_cur_bonus(cOrganism* org) {
  if (!org) return 0.0;
  return org->GetPhenotype().GetCurBonus();
}

int avd_org_get_generation(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().GetGeneration();
}

int avd_org_get_time_used(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().GetTimeUsed();
}

// ---- Cell accessors ----

int avd_cell_is_occupied(cPopulationCell* cell) {
  if (!cell) return 0;
  return cell->IsOccupied() ? 1 : 0;
}

cOrganism* avd_cell_get_organism(cPopulationCell* cell) {
  if (!cell || !cell->IsOccupied()) return nullptr;
  return cell->GetOrganism();
}

int avd_cell_get_id(cPopulationCell* cell) {
  if (!cell) return -1;
  return cell->GetID();
}

int avd_cell_get_deme_id(cPopulationCell* cell) {
  if (!cell) return -1;
  return cell->GetDemeID();
}

// ---- Population accessors ----

cPopulationCell* avd_pop_get_cell(cPopulation* pop, int cell_id) {
  if (!pop || cell_id < 0 || cell_id >= pop->GetSize()) return nullptr;
  return &pop->GetCell(cell_id);
}

int avd_pop_get_size(cPopulation* pop) {
  if (!pop) return 0;
  return pop->GetSize();
}

int avd_pop_get_num_organisms(cPopulation* pop) {
  if (!pop) return 0;
  return pop->GetNumOrganisms();
}

// ---- Organism accessors (extended) ----

int avd_org_get_av_cell_id(cOrganism* org) {
  if (!org) return -1;
  return org->GetAVCellID();
}

int avd_org_get_prev_seen_cell_id(cOrganism* org) {
  if (!org) return -1;
  return org->GetPrevSeenCellID();
}

void avd_org_set_prev_seen_cell_id(cOrganism* org, int id) {
  if (!org) return;
  org->SetPrevSeenCellID(id);
}

int avd_org_has_opinion(cOrganism* org) {
  if (!org) return 0;
  return org->HasOpinion() ? 1 : 0;
}

int avd_org_get_opinion_value(cOrganism* org) {
  if (!org || !org->HasOpinion()) return -1;
  return org->GetOpinion().first;
}

cDeme* avd_org_get_deme(cOrganism* org) {
  if (!org) return nullptr;
  return org->GetDeme();
}

// ---- Cell accessors (extended) ----

int avd_cell_get_data(cPopulationCell* cell) {
  if (!cell) return 0;
  return cell->GetCellData();
}

int avd_cell_get_x(cPopulationCell* cell) {
  if (!cell) return -1;
  int x, y;
  cell->GetPosition(x, y);
  return x;
}

int avd_cell_get_y(cPopulationCell* cell) {
  if (!cell) return -1;
  int x, y;
  cell->GetPosition(x, y);
  return y;
}

// ---- Population accessors (extended) ----

int avd_pop_get_num_demes(cPopulation* pop) {
  if (!pop) return 0;
  return pop->GetNumDemes();
}

cDeme* avd_pop_get_deme(cPopulation* pop, int deme_id) {
  if (!pop || deme_id < 0 || deme_id >= pop->GetNumDemes()) return nullptr;
  return &pop->GetDeme(deme_id);
}

int avd_pop_get_num_in_group(cPopulation* pop, int group_id) {
  if (!pop) return 0;
  return pop->NumberOfOrganismsInGroup(group_id);
}

// ---- Deme accessors ----

int avd_deme_get_id(cDeme* deme) {
  if (!deme) return -1;
  return deme->GetID();
}

int avd_deme_get_size(cDeme* deme) {
  if (!deme) return 0;
  return deme->GetSize();
}

int avd_deme_get_width(cDeme* deme) {
  if (!deme) return 0;
  return deme->GetWidth();
}

int avd_deme_get_height(cDeme* deme) {
  if (!deme) return 0;
  return deme->GetHeight();
}

int avd_deme_get_cell_position_x(cDeme* deme, int cell_id) {
  if (!deme) return -1;
  return deme->GetCellPosition(cell_id).first;
}

int avd_deme_get_cell_position_y(cDeme* deme, int cell_id) {
  if (!deme) return -1;
  return deme->GetCellPosition(cell_id).second;
}

int avd_deme_get_relative_cell_id(cDeme* deme, int absolute_cell_id) {
  if (!deme) return -1;
  return deme->GetRelativeCellID(absolute_cell_id);
}

int avd_deme_get_num_events(cDeme* deme) {
  if (!deme) return 0;
  return deme->GetNumEvents();
}

int avd_deme_get_cell_event_id(cDeme* deme, int event_idx) {
  if (!deme) return -1;
  cDemeCellEvent* ev = deme->GetCellEvent(event_idx);
  if (!ev) return -1;
  return ev->GetEventID();
}

int avd_deme_get_org_count(cDeme* deme) {
  if (!deme) return 0;
  return deme->GetOrgCount();
}

// ---- Organism accessors (Phase 1: read-only phenotype/identity) ----

int avd_org_get_id(cOrganism* org) {
  if (!org) return -1;
  return org->GetID();
}

int avd_org_get_lyse_display(cOrganism* org) {
  if (!org) return 0;
  return org->GetLyseDisplay() ? 1 : 0;
}

int avd_org_get_cell_data_org(cOrganism* org) {
  if (!org) return -1;
  return org->GetCellData();
}

int avd_org_get_input_at(cOrganism* org, int index) {
  if (!org) return 0;
  return org->GetInputAt(index);
}

double avd_org_get_stored_energy(cOrganism* org) {
  if (!org) return 0.0;
  return org->GetPhenotype().GetStoredEnergy();
}

int avd_org_is_fertile(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().IsFertile() ? 1 : 0;
}

int avd_org_is_germline(cOrganism* org) {
  if (!org) return 0;
  return org->IsGermline() ? 1 : 0;
}

int avd_org_get_num_divides(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().GetNumDivides();
}

int avd_org_get_kaboom_executed(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().GetKaboomExecuted() ? 1 : 0;
}

// ---- Organism accessors (Phase 1b: more reads for CPU handlers) ----

int avd_org_get_reputation(cOrganism* org) {
  if (!org) return 0;
  return org->GetReputation();
}

int avd_org_get_faced_dir(cOrganism* org) {
  if (!org) return 0;
  return org->GetFacedDir();
}

int avd_org_get_northerly(cOrganism* org) {
  if (!org) return 0;
  return org->GetNortherly();
}

int avd_org_get_easterly(cOrganism* org) {
  if (!org) return 0;
  return org->GetEasterly();
}

int avd_org_get_neighbor_cell_contents(cOrganism* org) {
  if (!org) return 0;
  return org->GetNeighborCellContents();
}

int avd_org_get_faced_cell_data_org_id(cOrganism* org) {
  if (!org) return -1;
  return org->GetFacedCellDataOrgID();
}

int avd_org_get_number_strings_on_hand(cOrganism* org, int type) {
  if (!org) return 0;
  return org->GetNumberStringsOnHand(type);
}

int avd_org_get_mating_type(cOrganism* org) {
  if (!org) return -1;
  return org->GetPhenotype().GetMatingType();
}

double avd_org_get_energy_in_buffer(cOrganism* org) {
  if (!org) return 0.0;
  return org->GetPhenotype().GetEnergyInBufferAmount();
}

int avd_org_get_cell_y_position(cOrganism* org) {
  if (!org) return -1;
  return org->GetOrgInterface().GetCellYPosition();
}

// ---- Organism accessors (Phase 1c: neighbor + energy level) ----

cOrganism* avd_org_get_neighbor(cOrganism* org) {
  if (!org) return nullptr;
  return org->GetNeighbor();
}

int avd_org_is_neighbor_cell_occupied(cOrganism* org) {
  if (!org) return 0;
  return org->IsNeighborCellOccupied() ? 1 : 0;
}

int avd_org_is_dead(cOrganism* org) {
  if (!org) return 1;
  return org->IsDead() ? 1 : 0;
}

double avd_org_get_vitality(cOrganism* org) {
  if (!org) return 0.0;
  return org->GetVitality();
}

int avd_org_get_discrete_energy_level(cOrganism* org) {
  if (!org) return -1;
  return org->GetPhenotype().GetDiscreteEnergyLevel();
}

int avd_org_get_cell_position_x(cOrganism* org) {
  if (!org) return -1;
  return org->GetOrgInterface().GetCellXPosition();
}

int avd_org_get_opinion_only(cOrganism* org) {
  if (!org) return 0;
  if (!org->HasOpinion()) return 0;
  return org->GetOpinion().first;
}

// ---- Organism WRITE accessors (Phase 2: mutable state) ----

// Energy writes
void avd_org_reduce_energy(cOrganism* org, double amount) {
  if (!org) return;
  org->GetPhenotype().ReduceEnergy(amount);
}

void avd_org_increase_energy_donated(cOrganism* org, double amount) {
  if (!org) return;
  org->GetPhenotype().IncreaseEnergyDonated(amount);
}

void avd_org_receive_donated_energy(cOrganism* org, double amount) {
  if (!org) return;
  org->GetPhenotype().ReceiveDonatedEnergy(amount);
}

void avd_org_apply_donated_energy(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().ApplyDonatedEnergy();
}

void avd_org_set_frac_energy_donating(cOrganism* org, double frac) {
  if (!org) return;
  org->SetFracEnergyDonating(frac);
}

// Flag writes
void avd_org_set_fertile(cOrganism* org, int value) {
  if (!org) return;
  org->GetPhenotype().IsFertile() = value;
}

void avd_org_set_kaboom_executed(cOrganism* org, int value) {
  if (!org) return;
  org->GetPhenotype().SetKaboomExecuted(value != 0);
}

void avd_org_set_is_energy_donor(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().SetIsEnergyDonor();
}

void avd_org_set_is_energy_receiver(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().SetIsEnergyReceiver();
}

// Bonus/merit writes
void avd_org_set_cur_bonus(cOrganism* org, double value) {
  if (!org) return;
  org->GetPhenotype().SetCurBonus(value);
}

// Pheromone
void avd_org_set_pheromone(cOrganism* org, int value) {
  if (!org) return;
  org->SetPheromone(value != 0);
}

// Energy request flags
void avd_org_set_is_energy_requestor(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().SetIsEnergyRequestor();
}

void avd_org_increase_num_energy_requests(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().IncreaseNumEnergyRequests();
}

void avd_org_set_has_open_energy_request(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().SetHasOpenEnergyRequest();
}

void avd_org_clear_has_open_energy_request(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().ClearHasOpenEnergyRequest();
}

double avd_org_get_frac_energy_donating(cOrganism* org) {
  if (!org) return 0.0;
  return org->GetFracEnergyDonating();
}

// Germline/repair
void avd_org_join_germline(cOrganism* org) {
  if (!org) return;
  org->JoinGermline();
}

void avd_org_exit_germline(cOrganism* org) {
  if (!org) return;
  org->ExitGermline();
}

void avd_org_toggle_pheromone(cOrganism* org) {
  if (!org) return;
  org->TogglePheromone();
}

void avd_org_repair_point_mut_on(cOrganism* org) {
  if (!org) return;
  org->RepairPointMutOn();
}

void avd_org_repair_point_mut_off(cOrganism* org) {
  if (!org) return;
  org->RepairPointMutOff();
}

// Simple state writes (Phase 2 batch)
void avd_org_clear_easterly(cOrganism* org) {
  if (!org) return;
  org->ClearEasterly();
}

void avd_org_clear_northerly(cOrganism* org) {
  if (!org) return;
  org->ClearNortherly();
}

void avd_org_set_lyse_display(cOrganism* org) {
  if (!org) return;
  org->SetLyseDisplay();
}

void avd_org_set_mate_preference(cOrganism* org, int pref) {
  if (!org) return;
  org->GetPhenotype().SetMatePreference(pref);
}

int avd_org_get_cur_mating_display_a(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().GetCurMatingDisplayA();
}

int avd_org_get_cur_mating_display_b(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().GetCurMatingDisplayB();
}

void avd_org_set_cur_mating_display_a(cOrganism* org, int val) {
  if (!org) return;
  org->GetPhenotype().SetCurMatingDisplayA(val);
}

void avd_org_set_cur_mating_display_b(cOrganism* org, int val) {
  if (!org) return;
  org->GetPhenotype().SetCurMatingDisplayB(val);
}

void avd_org_set_cell_data(cOrganism* org, int data) {
  if (!org) return;
  org->SetCellData(data);
}

double avd_org_get_copy_mut_prob(cOrganism* org) {
  if (!org) return 0.0;
  return org->GetCopyMutProb();
}

void avd_org_set_copy_mut_prob(cOrganism* org, double prob) {
  if (!org) return;
  org->SetCopyMutProb(prob);
}

// Output buffer
void avd_org_add_output(cOrganism* org, int val) {
  if (!org) return;
  org->AddOutput(val);
}

// Resource bin access
double avd_org_get_rbin(cOrganism* org, int index) {
  if (!org) return 0.0;
  return org->GetRBin(index);
}

int avd_org_get_num_rbins(cOrganism* org) {
  if (!org) return 0;
  return org->GetRBins().GetSize();
}

void avd_org_add_to_rbin(cOrganism* org, int index, double amount) {
  if (!org) return;
  org->AddToRBin(index, amount);
}

void avd_org_set_rbin(cOrganism* org, int index, double value) {
  if (!org) return;
  org->SetRBin(index, value);
}

// ---- Organism I/O delegation (Phase 3) ----

// DoOutput triggers task evaluation — the core organism↔environment interaction.
// Returns void; the task-checking side effects happen inside DoOutput.
void avd_org_do_output(cOrganism* org, cAvidaContext* ctx, int value) {
  if (!org || !ctx) return;
  org->DoOutput(*ctx, value);
}

int avd_org_get_next_input(cOrganism* org) {
  if (!org) return 0;
  return org->GetNextInput();
}

void avd_org_do_input(cOrganism* org, int value) {
  if (!org) return;
  org->DoInput(value);
}

// ---- World accessors ----

cPopulation* avd_world_get_population(cWorld* world) {
  if (!world) return nullptr;
  return &world->GetPopulation();
}

int avd_world_get_update(cWorld* world) {
  if (!world) return 0;
  return world->GetStats().GetUpdate();
}

// ---- Organism accessors (Phase 2b: neighbor energy + request status) ----

int avd_org_is_energy_requestor(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().IsEnergyRequestor() ? 1 : 0;
}

int avd_org_has_open_energy_request_read(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().HasOpenEnergyRequest() ? 1 : 0;
}

void avd_org_set_reputation(cOrganism* org, int rep) {
  if (!org) return;
  org->SetReputation(rep);
}

int avd_org_get_faced_cell_data(cOrganism* org) {
  if (!org) return 0;
  return org->GetFacedCellData();
}

int avd_org_get_faced_cell_data_update(cOrganism* org) {
  if (!org) return 0;
  return org->GetFacedCellDataUpdate();
}

int avd_org_is_donor(cOrganism* org, int neighbor_id) {
  if (!org) return 0;
  return org->IsDonor(neighbor_id) ? 1 : 0;
}

// ---- Opinion interface (Phase 4: opinion via OrgInterface) ----

void avd_org_iface_set_opinion(cOrganism* org, int value) {
  if (!org) return;
  org->GetOrgInterface().SetOpinion(value, org);
}

void avd_org_iface_clear_opinion(cOrganism* org) {
  if (!org) return;
  org->GetOrgInterface().ClearOpinion(org);
}

int avd_org_iface_has_opinion(cOrganism* org) {
  if (!org) return 0;
  return org->GetOrgInterface().HasOpinion(org) ? 1 : 0;
}

int avd_org_get_opinion_first(cOrganism* org) {
  if (!org || !org->HasOpinion()) return 0;
  return org->GetOpinion().first;
}

int avd_org_get_opinion_second(cOrganism* org) {
  if (!org || !org->HasOpinion()) return 0;
  return org->GetOpinion().second;
}

// ---- Messaging (Phase 4) ----

void avd_org_send_value(cOrganism* org, int value) {
  if (!org) return;
  org->SendValue(value);
}

// ---- Donation support (Phase 4) ----

int avd_org_get_cur_num_donates(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().GetCurNumDonates();
}

void avd_org_inc_donates(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().IncDonates();
}

void avd_org_set_is_donor_rand(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().SetIsDonorRand();
}

void avd_org_set_is_donor_null(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().SetIsDonorNull();
}

void avd_org_set_is_receiver_flag(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().SetIsReceiver();
}

double avd_org_get_merit_double(cOrganism* org) {
  if (!org) return 0.0;
  return org->GetPhenotype().GetMerit().GetDouble();
}

void avd_org_update_merit(cOrganism* org, cAvidaContext* ctx, double new_merit) {
  if (!org || !ctx) return;
  org->UpdateMerit(*ctx, new_merit);
}

double avd_org_get_energy_usage_ratio(cOrganism* org) {
  if (!org) return 0.0;
  return org->GetPhenotype().GetEnergyUsageRatio();
}

double avd_org_convert_energy_to_merit(cOrganism* org, double energy) {
  if (!org) return 0.0;
  return org->GetPhenotype().ConvertEnergyToMerit(energy);
}

void avd_org_increase_energy_received(cOrganism* org, double amount) {
  if (!org) return;
  org->GetPhenotype().IncreaseEnergyReceived(amount);
}

void avd_org_increase_num_energy_donations(cOrganism* org) {
  if (!org) return;
  org->GetPhenotype().IncreaseNumEnergyDonations();
}

void avd_org_deme_increase_energy_donated(cOrganism* org, double amount) {
  if (!org) return;
  cDeme* deme = org->GetDeme();
  if (deme) deme->IncreaseEnergyDonated(amount);
}

void avd_org_deme_increase_energy_received(cOrganism* org, double amount) {
  if (!org) return;
  cDeme* deme = org->GetDeme();
  if (deme) deme->IncreaseEnergyReceived(amount);
}

int avd_org_has_open_energy_request(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().HasOpenEnergyRequest() ? 1 : 0;
}

// ---- Organism state transitions (Phase 4: I/O reset, death) ----

void avd_org_reset_inputs(cOrganism* org, cAvidaContext* ctx) {
  if (!org || !ctx) return;
  org->GetOrgInterface().ResetInputs(*ctx);
}

void avd_org_clear_input(cOrganism* org) {
  if (!org) return;
  org->ClearInput();
}

void avd_org_die(cOrganism* org, cAvidaContext* ctx) {
  if (!org || !ctx) return;
  org->Die(*ctx);
}

int avd_org_get_cpu_cycles_used(cOrganism* org) {
  if (!org) return 0;
  return org->GetPhenotype().GetCPUCyclesUsed();
}

// ---- OrgInterface delegation (Phase 5: group/population queries) ----

int avd_org_iface_number_of_orgs_in_group(cOrganism* org, int group_id) {
  if (!org) return 0;
  return org->GetOrgInterface().NumberOfOrganismsInGroup(group_id);
}

int avd_org_receive_value(cOrganism* org) {
  if (!org) return 0;
  return org->ReceiveValue();
}

void avd_org_donate_res_consumed_to_deme(cOrganism* org) {
  if (!org) return;
  org->DonateResConsumedToDeme();
}

// ---- Movement FFI (Phase 5: movement delegation) ----

int avd_org_move(cOrganism* org, cAvidaContext* ctx) {
  if (!org || !ctx) return 0;
  return org->Move(*ctx) ? 1 : 0;
}

void avd_org_rotate(cOrganism* org, cAvidaContext* ctx, int direction) {
  if (!org || !ctx) return;
  org->Rotate(*ctx, direction);
}

int avd_org_get_neighborhood_size(cOrganism* org) {
  if (!org) return 0;
  return org->GetNeighborhoodSize();
}

int avd_org_get_facing(cOrganism* org) {
  if (!org) return 0;
  return org->GetFacing();
}

// ---- Kaboom/Stats FFI (Phase 5: kazi/lyse handlers) ----

void avd_org_kaboom(cOrganism* org, cAvidaContext* ctx, int distance) {
  if (!org || !ctx) return;
  org->GetOrgInterface().Kaboom(distance, *ctx);
}

void avd_org_kaboom_with_effect(cOrganism* org, cAvidaContext* ctx, int distance, double effect) {
  if (!org || !ctx) return;
  org->Kaboom(distance, *ctx, effect);
}

void avd_stats_inc_kaboom(cWorld* world) {
  if (!world) return;
  world->GetStats().IncKaboom();
}

void avd_stats_inc_dont_explode(cWorld* world) {
  if (!world) return;
  world->GetStats().IncDontExplode();
}

void avd_stats_inc_perc_lyse(cWorld* world, double perc) {
  if (!world) return;
  world->GetStats().IncPercLyse(perc);
}

void avd_stats_inc_sum_cpus(cWorld* world, int cpu_cycles) {
  if (!world) return;
  world->GetStats().IncSumCPUs(cpu_cycles);
}

// ---- Messaging FFI (Phase 5: send/retrieve message) ----

int avd_org_send_message_regs(cOrganism* org, cAvidaContext* ctx, int label, int data, int msg_type) {
  if (!org || !ctx) return 0;
  cOrgMessage msg(org);
  msg.SetData(data);
  msg.SetLabel(label);
  (void)msg_type; // msg_type not used in base API
  return org->SendMessage(*ctx, msg) ? 1 : 0;
}

int avd_org_retrieve_message(cOrganism* org, int* out_label, int* out_data, int log_enabled, cWorld* world) {
  if (!org) return 0;
  (void)log_enabled;
  (void)world;
  std::pair<bool, cOrgMessage> result = org->RetrieveMessage();
  if (!result.first) return 0;
  if (out_label) *out_label = result.second.GetLabel();
  if (out_data) *out_data = result.second.GetData();
  return 1;
}

// ---- Organism methods: Flash / Neighborhood ----

void avd_org_send_flash(cOrganism* org, cAvidaContext* ctx) {
  if (!org || !ctx) return;
  org->SendFlash(*ctx);
}

void avd_org_load_neighborhood(cOrganism* org, cAvidaContext* ctx) {
  if (!org || !ctx) return;
  org->LoadNeighborhood(*ctx);
}

int avd_org_has_neighborhood_changed(cOrganism* org, cAvidaContext* ctx) {
  if (!org || !ctx) return 0;
  return org->HasNeighborhoodChanged(*ctx) ? 1 : 0;
}

// ---- Organism methods: BroadcastMessage ----

int avd_org_broadcast_message(cOrganism* org, cAvidaContext* ctx, int label, int data, int depth) {
  if (!org || !ctx) return 0;
  cOrgMessage msg(org);
  msg.SetLabel(label);
  msg.SetData(data);
  return org->BroadcastMessage(*ctx, msg, depth) ? 1 : 0;
}

// ---- Organism methods: BcastAlarmMSG ----

int avd_org_bcast_alarm_msg(cOrganism* org, cAvidaContext* ctx, int jump_label, int bcast_range) {
  if (!org || !ctx) return 0;
  return org->BcastAlarmMSG(*ctx, jump_label, bcast_range) ? 1 : 0;
}

// ---- Resource sensing FFI ----

int avd_org_sense_resource_x(cOrganism* org, cAvidaContext* ctx, int cell_id, int res_id) {
  if (!org || !ctx || res_id < 0) return -1;
  const AvidaArray<double> res_count = org->GetOrgInterface().GetResources(*ctx) +
    org->GetOrgInterface().GetDemeResources(org->GetOrgInterface().GetDemeID(), *ctx);
  if (res_id >= res_count.GetSize()) return -1;
  return (int)res_count[res_id];
}

int avd_org_get_faced_cell_id(cOrganism* org) {
  if (!org) return -1;
  return org->GetOrgInterface().GetFacedCellID();
}

// Note: avd_hw_get_last_cell_data_valid/value are in cHardwareFFI.cc (need cHardwareCPU.h)

} // extern "C"
