/*
 *  cResourceCount.cc
 *  Avida
 *
 *  Called "resource_count.cc" prior to 12/5/05.
 *  Copyright 1999-2011 Michigan State University. All rights reserved.
 *  Copyright 1993-2001 California Institute of Technology.
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

#include "cResourceCount.h"
#include "cResource.h"
#include "cGradientCount.h"
#include "cWorld.h"
#include "cStats.h"

#include "nGeometry.h"
#include "rust/running_stats_ffi.h"

#include <cmath>
#include <vector>

using namespace std;

namespace {
int LookupResourceIndex(const Apto::Array<cString>& resource_names, const cString& query)
{
  const int count = resource_names.GetSize();
  if (count <= 0) return -1;

  std::vector<const char*> names;
  names.reserve(count);
  for (int i = 0; i < count; ++i) {
    names.push_back((const char*) resource_names[i]);
  }

  return avd_rc_lookup_resource_index(names.data(), count, (const char*) query);
}

void ApplyGradientScalarSetter(cSpatialResCount* spatial_res, int opcode, double value)
{
  switch (opcode) {
    case AVD_RC_GRAD_SCALAR_SETTER_PLATEAU_INFLOW: spatial_res->SetGradPlatInflow(value); break;
    case AVD_RC_GRAD_SCALAR_SETTER_PLATEAU_OUTFLOW: spatial_res->SetGradPlatOutflow(value); break;
    case AVD_RC_GRAD_SCALAR_SETTER_CONE_INFLOW: spatial_res->SetGradConeInflow(value); break;
    case AVD_RC_GRAD_SCALAR_SETTER_CONE_OUTFLOW: spatial_res->SetGradConeOutflow(value); break;
    case AVD_RC_GRAD_SCALAR_SETTER_GRADIENT_INFLOW: spatial_res->SetGradientInflow(value); break;
    default:
      assert(opcode == AVD_RC_GRAD_SCALAR_SETTER_INVALID && "Unexpected gradient scalar setter opcode");
      break;
  }
}

void ApplyGradientVarInflowSetter(
  cSpatialResCount* spatial_res,
  int opcode,
  cAvidaContext& ctx,
  double mean,
  double variance,
  int type
)
{
  switch (opcode) {
    case AVD_RC_GRAD_VAR_INFLOW_SETTER_PLAT_VAR_INFLOW:
      spatial_res->SetGradPlatVarInflow(ctx, mean, variance, type);
      break;
    default:
      assert(opcode == AVD_RC_GRAD_VAR_INFLOW_SETTER_INVALID && "Unexpected gradient var-inflow setter opcode");
      break;
  }
}

void ApplyPredatorySetter(
  cSpatialResCount* spatial_res,
  int opcode,
  double odds,
  int juvsper
)
{
  switch (opcode) {
    case AVD_RC_PREDATORY_SETTER_SET_PREDATORY_RESOURCE:
      spatial_res->SetPredatoryResource(odds, juvsper);
      break;
    default:
      assert(opcode == AVD_RC_PREDATORY_SETTER_INVALID && "Unexpected predatory setter opcode");
      break;
  }
}

void ApplyProbabilisticSetter(
  cSpatialResCount* spatial_res,
  int opcode,
  cAvidaContext& ctx,
  double initial,
  double inflow,
  double outflow,
  double lambda,
  double theta,
  int x,
  int y,
  int count
)
{
  switch (opcode) {
    case AVD_RC_PROBABILISTIC_SETTER_SET_PROBABILISTIC_RESOURCE:
      spatial_res->SetProbabilisticResource(ctx, initial, inflow, outflow, lambda, theta, x, y, count);
      break;
    default:
      assert(opcode == AVD_RC_PROBABILISTIC_SETTER_INVALID && "Unexpected probabilistic setter opcode");
      break;
  }
}

int ApplyPeakGetter(cSpatialResCount* spatial_res, int opcode)
{
  switch (opcode) {
    case AVD_RC_PEAK_GETTER_CURR_X:
    case AVD_RC_PEAK_GETTER_FROZEN_X:
      return spatial_res->GetCurrPeakX();
    case AVD_RC_PEAK_GETTER_CURR_Y:
    case AVD_RC_PEAK_GETTER_FROZEN_Y:
      return spatial_res->GetCurrPeakY();
    default:
      assert(opcode == AVD_RC_PEAK_GETTER_INVALID && "Unexpected peak getter opcode");
      return 0;
  }
}
}

const double cResourceCount::UPDATE_STEP(1.0 / 10000.0);
const double cResourceCount::EPSILON (1.0e-15);
const int cResourceCount::PRECALC_DISTANCE(100);


cResourceCount::cResourceCount(int num_resources)
  : update_time(0.0)
  , m_last_updated(0)
  , m_spatial_update(0)
{
  if(num_resources > 0) {
    SetSize(num_resources);
  }

  return;
}

cResourceCount::cResourceCount(const cResourceCount &rc) {
  *this = rc;

  return;
}

const cResourceCount &cResourceCount::operator=(const cResourceCount &rc) {
  SetSize(rc.GetSize());
  resource_name = rc.resource_name;
  resource_initial = rc.resource_initial;  
  resource_count = rc.resource_count;
  decay_rate = rc.decay_rate;
  inflow_rate = rc.inflow_rate;
  decay_precalc = rc.decay_precalc;
  inflow_precalc = rc.inflow_precalc;
  geometry = rc.geometry;
  
  for (int i = 0; i < rc.spatial_resource_count.GetSize(); i++) { 
    *(spatial_resource_count[i]) = *(rc.spatial_resource_count[i]);
  }
  
  curr_grid_res_cnt = rc.curr_grid_res_cnt;
  curr_spatial_res_cnt = rc.curr_spatial_res_cnt;
  update_time = rc.update_time;
  cell_lists = rc.cell_lists;

  return *this;
}

void cResourceCount::SetSize(int num_resources)
{
  resource_name.ResizeClear(num_resources);
  resource_initial.ResizeClear(num_resources);
  resource_count.ResizeClear(num_resources);
  decay_rate.ResizeClear(num_resources);
  inflow_rate.ResizeClear(num_resources);
  if(num_resources > 0) {
    decay_precalc.ResizeClear(num_resources, PRECALC_DISTANCE+1);
    inflow_precalc.ResizeClear(num_resources, PRECALC_DISTANCE+1);
  }
  geometry.ResizeClear(num_resources);
  
  for (int i = 0; i < spatial_resource_count.GetSize(); i++) {
    delete spatial_resource_count[i]; 
  }
  
  spatial_resource_count.ResizeClear(num_resources);
  
  for(int i = 0; i < num_resources; i++){
    spatial_resource_count[i] = new cSpatialResCount(); 
  }
    
  curr_grid_res_cnt.ResizeClear(num_resources);
  curr_spatial_res_cnt.ResizeClear(num_resources);
  cell_lists.ResizeClear(num_resources);
  resource_name.SetAll("");
  resource_initial.SetAll(0.0);
  resource_count.SetAll(0.0);
  decay_rate.SetAll(0.0);
  inflow_rate.SetAll(0.0);
  decay_precalc.SetAll(1.0); // This is 1-inflow, so there should be no inflow by default, JEB
  inflow_precalc.SetAll(0.0);
  geometry.SetAll(nGeometry::GLOBAL);
  curr_grid_res_cnt.SetAll(0.0);
  //DO spacial resources need to be set to zero?
}

cResourceCount::~cResourceCount()
{
  for (int i = 0; i < spatial_resource_count.GetSize(); i++) {
    delete spatial_resource_count[i]; 
  }
}

void cResourceCount::SetCellResources(int cell_id, const Apto::Array<double> & res)
{
  assert(resource_count.GetSize() == res.GetSize());

  for (int i = 0; i < resource_count.GetSize(); i++) {
    if (avd_rc_setcell_write_path_kind(geometry[i]) == AVD_RC_SETCELL_SPATIAL_WRITE) {
      spatial_resource_count[i]->SetCellAmount(cell_id, res[i]);

      /* Ideally the state of the cell's resource should not be set till
         the end of the update so that all processes (inflow, outflow, 
         diffision, gravity and organism demand) have the same weight.  However
         waiting can cause problems with negative resources so we allow
         the organism demand to work immediately on the state of the resource */ 
    } else {
      // Global/partial resources intentionally no-op here.
    }
  }
}

void cResourceCount::Setup(cWorld* world, const int& res_index, const cString& name, const double& initial, const double& inflow, const double& decay,                  
                           const int& in_geometry, const double& in_xdiffuse, const double& in_xgravity, 
                           const double& in_ydiffuse, const double& in_ygravity,
                           const int& in_inflowX1, const int& in_inflowX2, const int& in_inflowY1, const int& in_inflowY2,
                           const int& in_outflowX1, const int& in_outflowX2, const int& in_outflowY1, 
                           const int& in_outflowY2, Apto::Array<cCellResource> *in_cell_list_ptr,
                           Apto::Array<int> *in_cell_id_list_ptr, const int& verbosity_level,
                           const int&,
                           const double&, const double&, const double&,
                           const double&, const double&,
                           const double&, const double&,
                           const double&, const double&,
                           const double&, const double&,
                           const double&, const double&,
                           const double&, const double&,
                           const int&, const int& in_peakx, const int& in_peaky,
                           const int& in_height, const int& in_spread, const double& in_plateau, const int& in_decay,
                           const int& in_max_x, const int& in_min_x, const int& in_max_y, const int& in_min_y, const double& in_move_a_scaler,
                           const int& in_updatestep, const int& in_halo, const int& in_halo_inner_radius, const int& in_halo_width,
                           const int& in_halo_anchor_x, const int& in_halo_anchor_y, const int& in_move_speed, const int& in_move_resistance,
                           const double& in_plateau_inflow, const double& in_plateau_outflow, const double& in_cone_inflow, const double& in_cone_outflow,
                           const double& in_gradient_inflow, const int& in_is_plateau_common, const double& in_floor, const int& in_habitat, 
                           const int& in_min_size, const int& in_max_size, const int& in_config, const int& in_count, const double& in_resistance,
                           const double& in_damage, const double& in_death_odds, const int& in_path, const int& in_hammer, const double& in_init_plat, const double& in_threshold,
                           const int& in_refuge, const bool& isgradient)
{
  (void)in_threshold;
  (void)in_refuge;
  (void)isgradient;
  
  assert(res_index >= 0 && res_index < resource_count.GetSize());
  assert(initial >= 0.0);
  assert(decay >= 0.0);
  assert(inflow >= 0.0);
  assert(spatial_resource_count[res_index]->GetSize() > 0);
  int tempx = spatial_resource_count[res_index]->GetX();
  int tempy = spatial_resource_count[res_index]->GetY();

  const int setup_path_kind = avd_rc_setup_path_kind(in_geometry);
  cString geo_name;
  if (setup_path_kind == AVD_RC_SETUP_PATH_GLOBAL) {
    geo_name = "GLOBAL";
  } else if (in_geometry == nGeometry::GRID) {
    geo_name = "GRID";
  } else if (in_geometry == nGeometry::TORUS) {
    geo_name = "TORUS";
  } else if (setup_path_kind == AVD_RC_SETUP_PATH_PARTIAL) {
    geo_name = "PARTIAL";
  }
  else {
    cerr << "[cResourceCount::Setup] Unknown resource geometry " << in_geometry << ".  Exiting.";
    exit(2);
  }


  /* If the verbose flag is set print out information about resources */
  verbosity = verbosity_level;
  if (verbosity > VERBOSE_NORMAL) {
    cout << "Setting up resource " << name
         << "(" << geo_name 
         << ") with initial quantity=" << initial
         << ", decay=" << decay
         << ", inflow=" << inflow
         << endl;
    if (avd_rc_should_log_spatial_rectangles(in_geometry) != 0) {
      cout << "  Inflow rectangle (" << in_inflowX1 
           << "," << in_inflowY1 
           << ") to (" << in_inflowX2 
           << "," << in_inflowY2 
           << ")" << endl; 
      cout << "  Outflow rectangle (" << in_outflowX1 
           << "," << in_outflowY1 
           << ") to (" << in_outflowX2 
           << "," << in_outflowY2 
           << ")" << endl;
      cout << "  xdiffuse=" << in_xdiffuse
           << ", xgravity=" << in_xgravity
           << ", ydiffuse=" << in_ydiffuse
           << ", ygravity=" << in_ygravity
           << endl;
    }   
  }
  

  /* recource_count gets only the values for global resources */

  resource_name[res_index] = name;
  resource_initial[res_index] = initial;
  if (setup_path_kind == AVD_RC_SETUP_PATH_GLOBAL) {
    resource_count[res_index] = initial;
    spatial_resource_count[res_index]->RateAll(0);
  } 
  else if (setup_path_kind == AVD_RC_SETUP_PATH_PARTIAL) {
    resource_count[res_index]=initial;
    
    spatial_resource_count[res_index]->RateAll(0);
    // want to set list of cell ids here
    cell_lists[res_index].Resize(in_cell_id_list_ptr->GetSize());
    for (int i = 0; i < in_cell_id_list_ptr->GetSize(); i++) {
      cell_lists[res_index][i] = (*in_cell_id_list_ptr)[i]; 
    }
  }
  else {
    resource_count[res_index] = 0; 
    if (isgradient) {
      delete spatial_resource_count[res_index];
      spatial_resource_count[res_index] = new cGradientCount(world, in_peakx, in_peaky, in_height, in_spread, in_plateau, in_decay,                                
                                                             in_max_x, in_max_y, in_min_x, in_min_y, in_move_a_scaler, in_updatestep, 
                                                             tempx, tempy, in_geometry, in_halo, in_halo_inner_radius, 
                                                             in_halo_width, in_halo_anchor_x, in_halo_anchor_y, in_move_speed, in_move_resistance,
                                                             in_plateau_inflow, in_plateau_outflow, in_cone_inflow, in_cone_outflow,
                                                             in_gradient_inflow, in_is_plateau_common, in_floor, in_habitat, 
                                                             in_min_size, in_max_size, in_config, in_count, in_init_plat, in_threshold,
                                                             in_damage, in_death_odds, in_path, in_hammer);
      spatial_resource_count[res_index]->RateAll(0);
    }
    
    else{
      spatial_resource_count[res_index]->SetInitial(initial / spatial_resource_count[res_index]->GetSize());
      spatial_resource_count[res_index]->RateAll(spatial_resource_count[res_index]->GetInitial());
    }
  }
  spatial_resource_count[res_index]->StateAll();  
  decay_rate[res_index] = decay;
  inflow_rate[res_index] = inflow;
  geometry[res_index] = in_geometry;
  spatial_resource_count[res_index]->SetGeometry(in_geometry);
  spatial_resource_count[res_index]->SetPointers();
  spatial_resource_count[res_index]->SetCellList(in_cell_list_ptr);

  avd_rc_fill_precalc_tables(decay, inflow, UPDATE_STEP, PRECALC_DISTANCE,
                             &decay_precalc(res_index, 0),
                             &inflow_precalc(res_index, 0));
  spatial_resource_count[res_index]->SetXdiffuse(in_xdiffuse);
  spatial_resource_count[res_index]->SetXgravity(in_xgravity);
  spatial_resource_count[res_index]->SetYdiffuse(in_ydiffuse);
  spatial_resource_count[res_index]->SetYgravity(in_ygravity);
  spatial_resource_count[res_index]->SetInflowX1(in_inflowX1);
  spatial_resource_count[res_index]->SetInflowX2(in_inflowX2);
  spatial_resource_count[res_index]->SetInflowY1(in_inflowY1);
  spatial_resource_count[res_index]->SetInflowY2(in_inflowY2);
  spatial_resource_count[res_index]->SetOutflowX1(in_outflowX1);
  spatial_resource_count[res_index]->SetOutflowX2(in_outflowX2);
  spatial_resource_count[res_index]->SetOutflowY1(in_outflowY1);
  spatial_resource_count[res_index]->SetOutflowY2(in_outflowY2);
}

void cResourceCount::SetGradientCount(cAvidaContext& ctx, cWorld* world, const int& res_id, const int& peakx, const int& peaky,
                      const int& height, const int& spread, const double& plateau, const int& decay, 
                      const int& max_x, const int& min_x, const int& max_y, const int& min_y, const double& move_a_scaler,
                      const int& updatestep, const int& halo, const int& halo_inner_radius, const int& halo_width,
                      const int& halo_anchor_x, const int& halo_anchor_y, const int& move_speed, const int& move_resistance,
                      const double& plateau_inflow, const double& plateau_outflow, const double& cone_inflow, const double& cone_outflow,
                      const double& gradient_inflow, const int& is_plateau_common, const double& floor, const int& habitat, 
                      const int& min_size, const int& max_size, const int& config, const int& count, const double& resistance, 
                      const double& damage,const double& death_odds, const int& path, const int& hammer,
                      const double& plat_val, const double& threshold, const int& refuge)
{
  (void)world;
  
  assert(res_id >= 0 && res_id < resource_count.GetSize());
  assert(spatial_resource_count[res_id]->GetSize() > 0);
  cGradientCount* const gradient = static_cast<cGradientCount*>(spatial_resource_count[res_id]);
  const int worldx = gradient->GetX();
  const int worldy = gradient->GetY();

  const int setter_count = avd_rc_gradient_setter_count();
  for (int setter_idx = 0; setter_idx < setter_count; ++setter_idx) {
    const int setter = avd_rc_gradient_setter_opcode(setter_idx);
    switch (setter) {
      case AVD_RC_GRAD_SETTER_PEAK_X: gradient->SetGradPeakX(peakx); break;
      case AVD_RC_GRAD_SETTER_PEAK_Y: gradient->SetGradPeakY(peaky); break;
      case AVD_RC_GRAD_SETTER_HEIGHT: gradient->SetGradHeight(height); break;
      case AVD_RC_GRAD_SETTER_SPREAD: gradient->SetGradSpread(spread); break;
      case AVD_RC_GRAD_SETTER_PLATEAU: gradient->SetGradPlateau(plateau); break;
      case AVD_RC_GRAD_SETTER_INITIAL_PLAT: gradient->SetGradInitialPlat(plat_val); break;
      case AVD_RC_GRAD_SETTER_DECAY: gradient->SetGradDecay(decay); break;
      case AVD_RC_GRAD_SETTER_MAX_X: gradient->SetGradMaxX(max_x); break;
      case AVD_RC_GRAD_SETTER_MAX_Y: gradient->SetGradMaxY(max_y); break;
      case AVD_RC_GRAD_SETTER_MIN_X: gradient->SetGradMinX(min_x); break;
      case AVD_RC_GRAD_SETTER_MIN_Y: gradient->SetGradMinY(min_y); break;
      case AVD_RC_GRAD_SETTER_MOVE_SCALER: gradient->SetGradMoveScaler(move_a_scaler); break;
      case AVD_RC_GRAD_SETTER_UPDATE_STEP: gradient->SetGradUpdateStep(updatestep); break;
      case AVD_RC_GRAD_SETTER_IS_HALO: gradient->SetGradIsHalo(halo); break;
      case AVD_RC_GRAD_SETTER_HALO_INNER_RADIUS: gradient->SetGradHaloInnerRad(halo_inner_radius); break;
      case AVD_RC_GRAD_SETTER_HALO_WIDTH: gradient->SetGradHaloWidth(halo_width); break;
      case AVD_RC_GRAD_SETTER_HALO_ANCHOR_X: gradient->SetGradHaloX(halo_anchor_x); break;
      case AVD_RC_GRAD_SETTER_HALO_ANCHOR_Y: gradient->SetGradHaloY(halo_anchor_y); break;
      case AVD_RC_GRAD_SETTER_MOVE_SPEED: gradient->SetGradMoveSpeed(move_speed); break;
      case AVD_RC_GRAD_SETTER_MOVE_RESISTANCE: gradient->SetGradMoveResistance(move_resistance); break;
      case AVD_RC_GRAD_SETTER_PLATEAU_INFLOW: gradient->SetGradPlatInflow(plateau_inflow); break;
      case AVD_RC_GRAD_SETTER_PLATEAU_OUTFLOW: gradient->SetGradPlatOutflow(plateau_outflow); break;
      case AVD_RC_GRAD_SETTER_CONE_INFLOW: gradient->SetGradConeInflow(cone_inflow); break;
      case AVD_RC_GRAD_SETTER_CONE_OUTFLOW: gradient->SetGradConeOutflow(cone_outflow); break;
      case AVD_RC_GRAD_SETTER_GRADIENT_INFLOW: gradient->SetGradientInflow(gradient_inflow); break;
      case AVD_RC_GRAD_SETTER_PLATEAU_COMMON: gradient->SetGradPlatIsCommon(is_plateau_common); break;
      case AVD_RC_GRAD_SETTER_FLOOR: gradient->SetGradFloor(floor); break;
      case AVD_RC_GRAD_SETTER_HABITAT: gradient->SetGradHabitat(habitat); break;
      case AVD_RC_GRAD_SETTER_MIN_SIZE: gradient->SetGradMinSize(min_size); break;
      case AVD_RC_GRAD_SETTER_MAX_SIZE: gradient->SetGradMaxSize(max_size); break;
      case AVD_RC_GRAD_SETTER_CONFIG: gradient->SetGradConfig(config); break;
      case AVD_RC_GRAD_SETTER_COUNT: gradient->SetGradCount(count); break;
      case AVD_RC_GRAD_SETTER_RESISTANCE: gradient->SetGradResistance(resistance); break;
      case AVD_RC_GRAD_SETTER_DAMAGE: gradient->SetGradDamage(damage); break;
      case AVD_RC_GRAD_SETTER_THRESHOLD: gradient->SetGradThreshold(threshold); break;
      case AVD_RC_GRAD_SETTER_REFUGE: gradient->SetGradRefuge(refuge); break;
      case AVD_RC_GRAD_SETTER_DEATH_ODDS: gradient->SetGradDeathOdds(death_odds); break;
      default:
        assert(setter == AVD_RC_GRAD_SETTER_INVALID && "Unexpected gradient setter opcode");
        break;
    }
  }

  gradient->ResetGradRes(ctx, worldx, worldy);
}

void cResourceCount::SetGradientPlatInflow(const int& res_id, const double& inflow) 
{
  assert(res_id >= 0 && res_id < resource_count.GetSize());
  assert(spatial_resource_count[res_id]->GetSize() > 0);
  const int opcode = avd_rc_gradient_scalar_setter_opcode(0);
  ApplyGradientScalarSetter(spatial_resource_count[res_id], opcode, inflow);
}

void cResourceCount::SetGradientPlatOutflow(const int& res_id, const double& outflow) 
{
  assert(res_id >= 0 && res_id < resource_count.GetSize());
  assert(spatial_resource_count[res_id]->GetSize() > 0);
  const int opcode = avd_rc_gradient_scalar_setter_opcode(1);
  ApplyGradientScalarSetter(spatial_resource_count[res_id], opcode, outflow);
}

void cResourceCount::SetGradientConeInflow(const int& res_id, const double& inflow) 
{
  assert(res_id >= 0 && res_id < resource_count.GetSize());
  assert(spatial_resource_count[res_id]->GetSize() > 0);
  const int opcode = avd_rc_gradient_scalar_setter_opcode(2);
  ApplyGradientScalarSetter(spatial_resource_count[res_id], opcode, inflow);
}

void cResourceCount::SetGradientConeOutflow(const int& res_id, const double& outflow) 
{
  assert(res_id >= 0 && res_id < resource_count.GetSize());
  assert(spatial_resource_count[res_id]->GetSize() > 0);
  const int opcode = avd_rc_gradient_scalar_setter_opcode(3);
  ApplyGradientScalarSetter(spatial_resource_count[res_id], opcode, outflow);
}

void cResourceCount::SetGradientInflow(const int& res_id, const double& inflow) 
{
  assert(res_id >= 0 && res_id < resource_count.GetSize());
  assert(spatial_resource_count[res_id]->GetSize() > 0);
  const int opcode = avd_rc_gradient_scalar_setter_opcode(4);
  ApplyGradientScalarSetter(spatial_resource_count[res_id], opcode, inflow);
}

void cResourceCount::SetGradPlatVarInflow(cAvidaContext& ctx, const int& res_id, const double& mean, const double& variance, const int& type) 
{
  assert(res_id >= 0 && res_id < resource_count.GetSize());
  assert(spatial_resource_count[res_id]->GetSize() > 0);
  const int opcode = avd_rc_gradient_var_inflow_setter_opcode(0);
  ApplyGradientVarInflowSetter(spatial_resource_count[res_id], opcode, ctx, mean, variance, type);
}

void cResourceCount::SetPredatoryResource(const int& res_id, const double& odds, const int& juvsper) 
{
  assert(res_id >= 0 && res_id < resource_count.GetSize());
  assert(spatial_resource_count[res_id]->GetSize() > 0);
  const int opcode = avd_rc_predatory_setter_opcode(0);
  ApplyPredatorySetter(spatial_resource_count[res_id], opcode, odds, juvsper);
}

void cResourceCount::SetProbabilisticResource(cAvidaContext& ctx, const int& res_id, const double& initial, const double& inflow,
                                              const double& outflow, const double& lambda, const double& theta, const int& x, const int& y, const int& count) 
{
  assert(res_id >= 0 && res_id < resource_count.GetSize());
  assert(spatial_resource_count[res_id]->GetSize() > 0);
  const int opcode = avd_rc_probabilistic_setter_opcode(0);
  ApplyProbabilisticSetter(spatial_resource_count[res_id], opcode, ctx, initial, inflow, outflow, lambda, theta, x, y, count);
}

int cResourceCount::GetResourceCountID(const cString& res_name)
{
  int result = GetResourceByName(res_name);
  if (result == -1) {
    cerr << "Error: Unknown resource '" << res_name << "'." << endl;
  }
  return result;
}

double cResourceCount::GetInflow(const cString& name)
{
  int id = GetResourceCountID(name);
  if (id == -1) return -1;
  
  return inflow_rate[id];
}

void cResourceCount::SetInflow(const cString& name, const double _inflow)
{
  int id = GetResourceCountID(name);
  if (id == -1) return;

  inflow_rate[id] = _inflow;
  avd_rc_fill_inflow_precalc_table(
    decay_rate[id],
    _inflow,
    UPDATE_STEP,
    PRECALC_DISTANCE,
    &inflow_precalc(id, 0)
  );
}

double cResourceCount::GetDecay(const cString& name)
{
  int id = GetResourceCountID(name);
  if (id == -1) return -1;
  
  return decay_rate[id];
}

void cResourceCount::SetDecay(const cString& name, const double _decay)
{
  int id = GetResourceCountID(name);
  if (id == -1) return;

  decay_rate[id] = _decay;
  avd_rc_fill_decay_precalc_table(
    _decay,
    UPDATE_STEP,
    PRECALC_DISTANCE,
    &decay_precalc(id, 0)
  );
}

void cResourceCount::Update(double in_time) 
{ 
  update_time = avd_rc_accumulate_update_time(update_time, avd_rc_update_time_delta(in_time));
 }

void cResourceCount::UpdateGlobalResources(cAvidaContext& ctx)
{
  DoUpdates(ctx, avd_rc_wrapper_global_only_flag(AVD_RC_WRAPPER_GLOBAL_ONLY) != 0);
}

void cResourceCount::UpdateRandomResources(cAvidaContext& ctx)
{
  DoUpdates(ctx, avd_rc_wrapper_global_only_flag(AVD_RC_WRAPPER_RANDOM) != 0);
}

void cResourceCount::UpdateResources(cAvidaContext& ctx)
{
  DoUpdates(ctx, avd_rc_wrapper_global_only_flag(AVD_RC_WRAPPER_FULL) != 0);
}

double cResourceCount::ReadCellResourceValue(int cell_id, int res_id) const
{
  if (avd_rc_read_path_kind(geometry[res_id]) == AVD_RC_READ_PATH_GLOBAL) {
    return resource_count[res_id];
  }
  return spatial_resource_count[res_id]->GetAmount(cell_id);
}

 
const Apto::Array<double> & cResourceCount::GetResources(cAvidaContext& ctx) const
{
  DoUpdates(ctx); 
  return resource_count;
}
 
const Apto::Array<double> & cResourceCount::GetCellResources(int cell_id, cAvidaContext& ctx) const
  // Get amount of the resource for a given cell in the grid.  If it is a
  // global resource pass out the entire content of that resource.
{
  int num_resources = resource_count.GetSize();

  DoUpdates(ctx);
              
  for (int i = 0; i < num_resources; i++) {
    curr_grid_res_cnt[i] = ReadCellResourceValue(cell_id, i);
  }
  return curr_grid_res_cnt;

}

const Apto::Array<double> & cResourceCount::GetFrozenResources(cAvidaContext&, int cell_id) const
// Get amount of the resource for a given cell in the grid.  If it is a
// global resource pass out the entire content of that resource.
// This differs from GetCellResources by leaving out DoUpdates which is
// useful inside methods that repeatedly call this before cells can change.
{
  int num_resources = resource_count.GetSize();
  
  for (int i = 0; i < num_resources; i++) {
    curr_grid_res_cnt[i] = ReadCellResourceValue(cell_id, i);
  }
  return curr_grid_res_cnt;
}

double cResourceCount::GetFrozenCellResVal(cAvidaContext& ctx, int cell_id, int res_id) const
// This differs from GetFrozenCellResources by only pulling for res of interest.
{
  (void)ctx;
  return ReadCellResourceValue(cell_id, res_id);
}

double cResourceCount::GetCellResVal(cAvidaContext& ctx, int cell_id, int res_id) const
// This differs from GetCellResources by only pulling for res of interest.
{
  DoUpdates(ctx);
  return ReadCellResourceValue(cell_id, res_id);
}

const Apto::Array<int> & cResourceCount::GetResourcesGeometry() const
{
  return geometry;
}

const Apto::Array< Apto::Array<double> > &  cResourceCount::GetSpatialRes(cAvidaContext& ctx)
{
  const int num_spatial_resources = spatial_resource_count.GetSize();
  if (num_spatial_resources > 0) {
    const int num_cells = spatial_resource_count[0]->GetSize();
    DoUpdates(ctx);
    for (int i = 0; i < num_spatial_resources; i++) {
      for (int j = 0; j < num_cells; j++) {
        curr_spatial_res_cnt[i][j] = spatial_resource_count[i]->GetAmount(j);
      }
    }
  }
  return curr_spatial_res_cnt;
}

void cResourceCount::Modify(cAvidaContext& ctx, const Apto::Array<double> & res_change)
{
  assert(resource_count.GetSize() == res_change.GetSize());

  DoUpdates(ctx);
  for (int i = 0; i < resource_count.GetSize(); i++) {
    resource_count[i] += res_change[i];
    assert(resource_count[i] >= 0.0);
  }
}


void cResourceCount::Modify(cAvidaContext& ctx, int res_index, double change)
{
  assert(res_index < resource_count.GetSize());

  DoUpdates(ctx);
  resource_count[res_index] += change;
  assert(resource_count[res_index] >= 0.0);
}

void cResourceCount::ModifyCell(cAvidaContext& ctx, const Apto::Array<double> & res_change, int cell_id)
{
  assert(resource_count.GetSize() == res_change.GetSize());

  DoUpdates(ctx);
  for (int i = 0; i < resource_count.GetSize(); i++) {
    if (avd_rc_is_spatial_geometry(geometry[i]) == 0) {
        resource_count[i] += res_change[i];
      assert(resource_count[i] >= 0.0);
    } else {
      double temp = spatial_resource_count[i]->Element(cell_id).GetAmount();
      spatial_resource_count[i]->Rate(cell_id, res_change[i]);
      /* Ideally the state of the cell's resource should not be set till
         the end of the update so that all processes (inflow, outflow, 
         diffision, gravity and organism demand) have the same weight.  However
         waiting can cause problems with negative resources so we allow
         the organism demand to work immediately on the state of the resource */ 
    
      spatial_resource_count[i]->State(cell_id);
      if(spatial_resource_count[i]->Element(cell_id).GetAmount() != temp){
        spatial_resource_count[i]->SetModified(true);
      }
      assert(spatial_resource_count[i]->Element(cell_id).GetAmount() >= 0.0);
    }
  }
}

double cResourceCount::Get(cAvidaContext& ctx, int res_id) const
{
  assert(res_id < resource_count.GetSize());
  DoUpdates(ctx);
  if (avd_rc_read_path_kind(geometry[res_id]) == AVD_RC_READ_PATH_GLOBAL) {
      return resource_count[res_id];
  } //else return spacial resource sum
  return spatial_resource_count[res_id]->SumAll();
}

void cResourceCount::Set(cAvidaContext& ctx, int res_id, double new_level)
{
  assert(res_id < resource_count.GetSize());
  DoUpdates(ctx);
  if (avd_rc_is_spatial_geometry(geometry[res_id]) == 0) {
     resource_count[res_id] = new_level;
  } else {
    for(int i = 0; i < spatial_resource_count[res_id]->GetSize(); i++) {
      spatial_resource_count[res_id]->SetCellAmount(i, new_level/spatial_resource_count[res_id]->GetSize());
    }
  }
}

void cResourceCount::ResizeSpatialGrids(int in_x, int in_y)
{
  for (int i = 0; i < resource_count.GetSize(); i++) {
    spatial_resource_count[i]->ResizeClear(in_x, in_y, geometry[i]);
    curr_spatial_res_cnt[i].Resize(avd_rc_resize_cell_count(in_x, in_y));
  }
}

int cResourceCount::GetCurrPeakX(cAvidaContext& ctx, int res_id) const
{ 
  const int opcode = avd_rc_peak_getter_opcode(0);
  if (avd_rc_peak_getter_requires_update(opcode) != 0) DoUpdates(ctx);
  return ApplyPeakGetter(spatial_resource_count[res_id], opcode);
}

int cResourceCount::GetCurrPeakY(cAvidaContext& ctx, int res_id) const
{ 
  const int opcode = avd_rc_peak_getter_opcode(1);
  if (avd_rc_peak_getter_requires_update(opcode) != 0) DoUpdates(ctx);
  return ApplyPeakGetter(spatial_resource_count[res_id], opcode);
}

int cResourceCount::GetFrozenPeakX(cAvidaContext&, int res_id) const
{ 
  const int opcode = avd_rc_peak_getter_opcode(2);
  return ApplyPeakGetter(spatial_resource_count[res_id], opcode);
}

int cResourceCount::GetFrozenPeakY(cAvidaContext&, int res_id) const
{ 
  const int opcode = avd_rc_peak_getter_opcode(3);
  return ApplyPeakGetter(spatial_resource_count[res_id], opcode);
}

Apto::Array<int>* cResourceCount::GetWallCells(int res_id)
{
  return spatial_resource_count[res_id]->GetWallCells();
}

int cResourceCount::GetMinUsedX(int res_id)
{
  return spatial_resource_count[res_id]->GetMinUsedX();
}

int cResourceCount::GetMinUsedY(int res_id)
{
  return spatial_resource_count[res_id]->GetMinUsedY();
}

int cResourceCount::GetMaxUsedX(int res_id)
{
  return spatial_resource_count[res_id]->GetMaxUsedX();
}

int cResourceCount::GetMaxUsedY(int res_id)
{
  return spatial_resource_count[res_id]->GetMaxUsedY();
}



void cResourceCount::DoUpdates(cAvidaContext& ctx, bool global_only) const
{ 

  
  // GLOBAL AND PARTIAL CALCULATION VALUES ======================================
  /*
     UPDATE_STEP is the fraction of an update per calculation step
     PRECALC_DISTANCE is how many steps to precalculate
     EPSILON is the tolerance for roundoff errors
     
     update_time is the portion of an update remaining.  It's remainder will
     get used in the next round of updating.
     
     Keep in mind that regardless of the type of resource, all resources
     will have an inflow, outflow, initial, geometry, resource_count,
     and spatial_resource_count (the latter even for global or partial resources,
     it just is not used.)
   */
  
  // Make sure that our fraction of an update remaining is greater than twice
  // the roundoff error.
  assert(update_time >= -EPSILON);
  // UPDATE_STEP is a fixed positive scheduling interval.
  assert(UPDATE_STEP > 0.0);
  // Determine how many resource steps we wish to process
  const int num_steps = avd_rc_num_steps(update_time, UPDATE_STEP);
  // Given the tolerated negative remainder bound above, scheduling never
  // runs a negative number of non-spatial update steps.
  assert(num_steps >= 0);
  
  // Preserve remainder of update_time for use on the next DoUpdates as
  // we may not get around to calculating them this round
  update_time = avd_rc_remainder_update_time(update_time, UPDATE_STEP, num_steps);
  
  
  // SPATIAL CALCULATION VALUES ==================================
  /*
    For some reason we can calculate spatial resources over different update
    ranges.  For the time being, everything seems to be set upon initialization
    or in cPopulation's ProcessPreUpdate, which just passes the current update
    
    m_spatial_update is the current update we wish to calculate to
    m_last_updated is the last update we calculated
    num_spatial_updates is how many updates we wish to calculate
  */
  const int num_spatial_updates = avd_rc_num_spatial_updates(m_spatial_update, m_last_updated);
  
  
  // DO UPDATE FOR EACH RESOURCE ================================================
  for (int res_id = 0; res_id < resource_count.GetSize(); res_id++) {
    const int is_spatial = avd_rc_is_spatial_geometry(geometry[res_id]);
    const int action = avd_rc_dispatch_action(is_spatial, global_only ? 1 : 0);
    if (action == AVD_RC_DISPATCH_NONSPATIAL) {
      DoNonSpatialUpdates(ctx, res_id, num_steps);
    } else if (action == AVD_RC_DISPATCH_SPATIAL) {
      DoSpatialUpdates(ctx, res_id, num_spatial_updates);
    }
  }
  
  if (avd_rc_should_advance_last_updated(global_only ? 1 : 0) != 0) {
    m_last_updated = m_spatial_update;
  }
}

void cResourceCount::DoNonSpatialUpdates(cAvidaContext& ctx, const int res_id, int num_steps) const
{
  (void)ctx;
  resource_count[res_id] = avd_rc_apply_nonspatial_steps(
    resource_count[res_id],
    &decay_precalc(res_id, 0),
    &inflow_precalc(res_id, 0),
    PRECALC_DISTANCE,
    num_steps
  );
}



void cResourceCount::DoSpatialUpdates(cAvidaContext& ctx, const int res_id, int num_updates) const
{
  const int iterations = avd_rc_spatial_step_iterations(num_updates);
  const int use_cell_branch = avd_rc_use_cell_list_branch(spatial_resource_count[res_id]->GetCellListSize());
  for (int kk = 0; kk < iterations; kk++) {
    spatial_resource_count[res_id]->UpdateCount(ctx);  //Only for Gradient Resources
    spatial_resource_count[res_id]->Source(inflow_rate[res_id]);  
    spatial_resource_count[res_id]->Sink(decay_rate[res_id]);   
    if (use_cell_branch != 0) {  // Only for CELL resources?
      spatial_resource_count[res_id]->CellInflow();
      spatial_resource_count[res_id]->CellOutflow();
    }
    spatial_resource_count[res_id]->FlowAll();
    spatial_resource_count[res_id]->StateAll();
    // BDB: resource_count[res_ndx] = spatial_resource_count[i]->SumAll();
  }
}



void cResourceCount::ReinitializeResources(cAvidaContext& ctx, double additional_resource)
{
  for(int i = 0; i < resource_name.GetSize(); i++) {
    Set(ctx, i, resource_initial[i] + additional_resource); //will cause problem if more than one resource is used. -- why?  each resource is stored separately (BDC)

    // Additionally, set any initial values given by the CELL command
    spatial_resource_count[i]->ResetResourceCounts();
    if (additional_resource != 0.0) {
      spatial_resource_count[i]->RateAll(additional_resource);
      spatial_resource_count[i]->StateAll();
    }

  } //End going through the resources
}

int cResourceCount::GetResourceByName(const cString& name) const
{
  return LookupResourceIndex(resource_name, name);
}


