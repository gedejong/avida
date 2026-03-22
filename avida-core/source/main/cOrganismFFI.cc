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
#include "cStats.h"
#include "cWorld.h"
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

// ---- World accessors ----

cPopulation* avd_world_get_population(cWorld* world) {
  if (!world) return nullptr;
  return &world->GetPopulation();
}

int avd_world_get_update(cWorld* world) {
  if (!world) return 0;
  return world->GetStats().GetUpdate();
}

} // extern "C"
