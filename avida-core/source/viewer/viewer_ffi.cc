/*
 *  viewer_ffi.cc — C FFI wrapper around Avida::Viewer::Driver.
 *
 *  Allows the Rust egui viewer to create, control, and read state from
 *  a running Avida simulation.
 */

#include "avida/viewer/Driver.h"
#include "avida/viewer/Map.h"

#include "cWorld.h"
#include "cPopulation.h"
#include "cStats.h"

using namespace Avida::Viewer;

extern "C" {

// ---- Lifecycle ----

Driver* avd_viewer_create(const char* config_dir) {
  if (!config_dir) return nullptr;
  return Driver::InitWithDirectory(Apto::String(config_dir));
}

void avd_viewer_start(Driver* driver) {
  if (!driver) return;
  driver->Start();  // Starts the simulation thread
}

void avd_viewer_pause(Driver* driver) {
  if (!driver) return;
  driver->Pause();
}

void avd_viewer_resume(Driver* driver) {
  if (!driver) return;
  driver->Resume();
}

void avd_viewer_finish(Driver* driver) {
  if (!driver) return;
  driver->Finish();
}

int avd_viewer_is_paused(Driver* driver) {
  if (!driver) return 0;
  return driver->IsPaused() ? 1 : 0;
}

int avd_viewer_has_finished(Driver* driver) {
  if (!driver) return 1;
  return driver->HasFinished() ? 1 : 0;
}

// ---- Stats ----

int avd_viewer_get_update(Driver* driver) {
  if (!driver) return 0;
  return driver->CurrentUpdate();
}

int avd_viewer_get_num_organisms(Driver* driver) {
  if (!driver) return 0;
  return driver->NumOrganisms();
}

int avd_viewer_get_world_x(Driver* driver) {
  if (!driver) return 0;
  return driver->WorldX();
}

int avd_viewer_get_world_y(Driver* driver) {
  if (!driver) return 0;
  return driver->WorldY();
}

// ---- Stats from cWorld ----

double avd_viewer_get_avg_fitness(Driver* driver) {
  if (!driver || !driver->GetOldWorld()) return 0.0;
  return driver->GetOldWorld()->GetStats().GetAveFitness();
}

double avd_viewer_get_avg_merit(Driver* driver) {
  if (!driver || !driver->GetOldWorld()) return 0.0;
  return driver->GetOldWorld()->GetStats().SumMerit().Average();
}

double avd_viewer_get_avg_gestation(Driver* driver) {
  if (!driver || !driver->GetOldWorld()) return 0.0;
  return driver->GetOldWorld()->GetStats().SumGestation().Average();
}

double avd_viewer_get_avg_genome_length(Driver* driver) {
  if (!driver || !driver->GetOldWorld()) return 0.0;
  return driver->GetOldWorld()->GetStats().GetAveMemSize();
}

// ---- Cleanup ----

void avd_viewer_free(Driver* driver) {
  if (driver) delete driver;
}

} // extern "C"
