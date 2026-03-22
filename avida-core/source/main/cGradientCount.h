/*
 *  cGradientCount.h
 *  Avida
 *
 *  Copyright 2010-2011 Michigan State University. All rights reserved.
 *  http://avida.devosoft.org/
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
 *  Authors: Aaron P Wagner <apwagner@msu.edu>
 *
 */

#ifndef cGradientCount_h
#define cGradientCount_h

#include "cSpatialResCount.h"

#include "AvidaArray.h"
#include "rust/running_stats_ffi.h"

class cWorld;

class cGradientCount : public cSpatialResCount
{
private:
  cWorld* m_world;

  // Configuration Arguments (Rust #[repr(C)] struct)
  GradientConfig m_cfg;

  // Internal Values (Rust #[repr(C)] struct)
  GradientState m_st;

  // Arrays not migrated (contain heap allocations)
  AvidaArray<double> m_plateau_array;
  AvidaArray<int> m_plateau_cell_IDs;
  AvidaArray<int> m_wall_cells;
  AvidaArray<int> m_prob_res_cells;

public:
  cGradientCount(cWorld* world, int peakx, int peaky, int height, int spread, double plateau, int decay,
                 int max_x, int max_y, int min_x, int min_y, double move_a_scaler, int updatestep,
                 int worldx, int worldy, int geometry,int halo, int halo_inner_radius, int halo_width,
                 int halo_anchor_x, int halo_anchor_y, int move_speed, int move_resistance, double plateau_inflow, double plateau_outflow,
                 double cone_inflow, double cone_outflow, double gradient_inflow, int is_plateau_common,
                 double floor, int habitat, int min_size, int max_size, int config, int count,
                 double init_plat, double threshold, double damage, double death_odds, int path, int hammer);
  ~cGradientCount();

  void UpdateCount(cAvidaContext& ctx);
  void StateAll();

  void SetGradInitialPlat(double plat_val) { m_cfg.initial_plat = plat_val; m_st.initial = 1; }
  void SetGradPeakX(int peakx) { m_cfg.peakx = peakx; }
  void SetGradPeakY(int peaky) { m_cfg.peaky = peaky; }
  void SetGradHeight(int height) { m_cfg.height = height; }
  void SetGradSpread(int spread) { m_cfg.spread = spread; }
  void SetGradPlateau(double plateau) { m_cfg.plateau = plateau; }
  void SetGradDecay(int decay) { m_cfg.decay = decay; }
  void SetGradMaxX(int max_x) { m_cfg.max_x = max_x; }
  void SetGradMaxY(int max_y) { m_cfg.max_y = max_y; }
  void SetGradMinX(int min_x) { m_cfg.min_x = min_x; }
  void SetGradMinY(int min_y) { m_cfg.min_y = min_y; }
  void SetGradMoveScaler(double move_a_scaler) { m_cfg.move_a_scaler = move_a_scaler; }
  void SetGradUpdateStep(int updatestep) { m_cfg.updatestep = updatestep; }
  void SetGradIsHalo(bool halo) { m_cfg.halo = halo; }
  void SetGradHaloInnerRad(int halo_inner_radius) { m_cfg.halo_inner_radius = halo_inner_radius; }
  void SetGradHaloWidth(int halo_width) { m_cfg.halo_width = halo_width; }
  void SetGradHaloX(int halo_anchor_x) { m_cfg.halo_anchor_x = halo_anchor_x; }
  void SetGradHaloY(int halo_anchor_y) { m_cfg.halo_anchor_y = halo_anchor_y; }
  void SetGradMoveSpeed(int move_speed) { m_cfg.move_speed = move_speed; }
  void SetGradMoveResistance(int move_resistance) { m_cfg.move_resistance = move_resistance; }
  void SetGradPlatInflow(double plateau_inflow) { m_cfg.plateau_inflow = plateau_inflow; }
  void SetGradPlatOutflow(double plateau_outflow) { m_cfg.plateau_outflow = plateau_outflow; }
  void SetGradConeInflow(double cone_inflow) { m_cfg.cone_inflow = cone_inflow; }
  void SetGradConeOutflow(double cone_outflow) { m_cfg.cone_outflow = cone_outflow; }
  void SetGradientInflow(double gradient_inflow) { m_cfg.gradient_inflow = gradient_inflow; }
  void SetGradPlatIsCommon(bool is_plateau_common) { m_cfg.is_plateau_common = is_plateau_common; }
  void SetGradFloor(double floor) { m_cfg.floor = floor; }
  void SetGradHabitat(int habitat) { m_cfg.habitat = habitat; }
  void SetGradMinSize(int min_size) { m_cfg.min_size = min_size; }
  void SetGradMaxSize(int max_size) { m_cfg.max_size = max_size; }
  void SetGradConfig(int config) { m_cfg.config = config; }
  void SetGradCount(int count) { m_cfg.count = count; }

  void SetGradPlatVarInflow(cAvidaContext& ctx, double mean, double variance, int type);

  void SetPredatoryResource(double odds, int juvsper);
  void UpdatePredatoryRes(cAvidaContext& ctx);

  void UpdateDamagingRes(cAvidaContext& ctx);
  void SetDeadlyRes(double odds) { m_st.death_odds = odds; m_st.deadly = (m_st.death_odds != 0); }
  void SetIsPath(bool path) { m_st.path = path; }
  void UpdateDeadlyRes(cAvidaContext& ctx);

  void SetProbabilisticResource(cAvidaContext& ctx, double initial, double inflow, double outflow, double lambda, double theta, int x, int y, int num_cells);
  void BuildProbabilisticRes(cAvidaContext& ctx, double lambda, double theta, int x, int y, int num_cells);
  void UpdateProbabilisticRes();

  void ResetGradRes(cAvidaContext& ctx, int worldx, int worldy);

  AvidaArray<int>* GetWallCells() { return &m_wall_cells; }
  int GetMinUsedX() { return m_st.min_usedx; }
  int GetMinUsedY() { return m_st.min_usedy; }
  int GetMaxUsedX() { return m_st.max_usedx; }
  int GetMaxUsedY() { return m_st.max_usedy; }

private:
  void fillinResourceValues();
  void updatePeakRes(cAvidaContext& ctx);
  void moveRes(cAvidaContext& ctx);
  int setHaloOrbit(cAvidaContext& ctx, int current_orbit);
  void setPeakMoveMovement(cAvidaContext& ctx);
  void moveHaloPeak(int current_orbit);
  void confirmHaloPeak();
  void movePeak();
  void generatePeak(cAvidaContext& ctx);
  void getCurrentPlatValues();
  void generateBarrier(cAvidaContext& ctx);
  void generateHills(cAvidaContext& ctx);
  void updateBounds(int x, int y);
  void resetUsedBounds();
  void clearExistingProbRes();

  inline void setHaloDirection(cAvidaContext& ctx);
};

#endif
