/*
 *  cMutationRates.cc
 *  Avida
 *
 *  Called "mutation_rates.cc" prior to 12/5/05.
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

#include "cMutationRates.h"

#include "cWorld.h"
#include "cAvidaConfig.h"


void cMutationRates::Setup(cWorld* world)
{
  SetCopyMutProb(world->GetConfig().COPY_MUT_PROB.Get());
  SetCopyInsProb(world->GetConfig().COPY_INS_PROB.Get());
  SetCopyDelProb(world->GetConfig().COPY_DEL_PROB.Get());
  SetCopyUniformProb(world->GetConfig().COPY_UNIFORM_PROB.Get());
  SetCopySlipProb(world->GetConfig().COPY_SLIP_PROB.Get());

  SetDivInsProb(world->GetConfig().DIV_INS_PROB.Get());
  SetDivDelProb(world->GetConfig().DIV_DEL_PROB.Get());
  SetDivMutProb(world->GetConfig().DIV_MUT_PROB.Get());
  SetDivUniformProb(world->GetConfig().DIV_UNIFORM_PROB.Get());
  SetDivSlipProb(world->GetConfig().DIV_SLIP_PROB.Get());
  SetDivTransProb(world->GetConfig().DIV_TRANS_PROB.Get());
  SetDivLGTProb(world->GetConfig().DIV_LGT_PROB.Get());

  SetDivideMutProb(world->GetConfig().DIVIDE_MUT_PROB.Get());
  SetDivideInsProb(world->GetConfig().DIVIDE_INS_PROB.Get());
  SetDivideDelProb(world->GetConfig().DIVIDE_DEL_PROB.Get());
  SetDivideUniformProb(world->GetConfig().DIVIDE_UNIFORM_PROB.Get());
  SetDivideSlipProb(world->GetConfig().DIVIDE_SLIP_PROB.Get());
  SetDivideTransProb(world->GetConfig().DIVIDE_TRANS_PROB.Get());
  SetDivideLGTProb(world->GetConfig().DIVIDE_LGT_PROB.Get());

  m_rates.divide.divide_poisson_mut_mean = world->GetConfig().DIVIDE_POISSON_MUT_MEAN.Get();
  m_rates.divide.divide_poisson_ins_mean = world->GetConfig().DIVIDE_POISSON_INS_MEAN.Get();
  m_rates.divide.divide_poisson_del_mean = world->GetConfig().DIVIDE_POISSON_DEL_MEAN.Get();
  m_rates.divide.divide_poisson_slip_mean = world->GetConfig().DIVIDE_POISSON_SLIP_MEAN.Get();
  m_rates.divide.divide_poisson_trans_mean = world->GetConfig().DIVIDE_POISSON_TRANS_MEAN.Get();
  m_rates.divide.divide_poisson_lgt_mean = world->GetConfig().DIVIDE_POISSON_LGT_MEAN.Get();

  SetParentMutProb(world->GetConfig().PARENT_MUT_PROB.Get());
  SetParentInsProb(world->GetConfig().PARENT_INS_PROB.Get());
  SetParentDelProb(world->GetConfig().PARENT_DEL_PROB.Get());

  SetPointInsProb(world->GetConfig().POINT_INS_PROB.Get());
  SetPointDelProb(world->GetConfig().POINT_DEL_PROB.Get());
  SetPointMutProb(world->GetConfig().POINT_MUT_PROB.Get());

  SetInjectInsProb(world->GetConfig().INJECT_INS_PROB.Get());
  SetInjectDelProb(world->GetConfig().INJECT_DEL_PROB.Get());
  SetInjectMutProb(world->GetConfig().INJECT_MUT_PROB.Get());

  SetMetaCopyMutProb(world->GetConfig().META_COPY_MUT.Get());
  SetMetaStandardDev(world->GetConfig().META_STD_DEV.Get());

  SetDeathProb(world->GetConfig().DEATH_PROB.Get());
}
