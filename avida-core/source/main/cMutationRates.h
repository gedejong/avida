/*
 *  cMutationRates.h
 *  Avida
 *
 *  Called "mutation_rates.hh" prior to 12/5/05.
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

#ifndef cMutationRates_h
#define cMutationRates_h

#include "cAvidaContext.h"
#include "rust/running_stats_ffi.h"

class cWorld;

class cMutationRates
{
private:
  AvidaMutationRates m_rates;

public:
  cMutationRates() { m_rates = avd_mutation_rates_default(); }
  cMutationRates(const cMutationRates& in_muts) : m_rates(in_muts.m_rates) { }
  cMutationRates& operator=(const cMutationRates& in_muts) { m_rates = in_muts.m_rates; return *this; }
  ~cMutationRates() { ; }

  void Setup(cWorld* world);
  void Clear() { avd_mutation_rates_clear(&m_rates); }
  void Copy(const cMutationRates& in_muts) { m_rates = in_muts.m_rates; }

  // Copy muts should always check if they are 0.0 before consulting the random number generator for performance
  bool TestCopyMut(cAvidaContext& ctx) const { return (m_rates.copy.mut_prob == 0.0) ? false : ctx.GetRandom().P(m_rates.copy.mut_prob); }
  bool TestCopyIns(cAvidaContext& ctx) const { return (m_rates.copy.ins_prob == 0.0) ? false : ctx.GetRandom().P(m_rates.copy.ins_prob); }
  bool TestCopyDel(cAvidaContext& ctx) const { return (m_rates.copy.del_prob == 0.0) ? false : ctx.GetRandom().P(m_rates.copy.del_prob); }
  bool TestCopySlip(cAvidaContext& ctx) const { return (m_rates.copy.slip_prob == 0.0) ? false : ctx.GetRandom().P(m_rates.copy.slip_prob); }
  bool TestCopyUniform(cAvidaContext& ctx) const
  {
    return (m_rates.copy.uniform_prob == 0.0) ? false : ctx.GetRandom().P(m_rates.copy.uniform_prob);
  }

  bool TestDivideMut(cAvidaContext& ctx) const { return ctx.GetRandom().P(m_rates.divide.divide_mut_prob); }
  bool TestDivideIns(cAvidaContext& ctx) const { return ctx.GetRandom().P(m_rates.divide.divide_ins_prob); }
  bool TestDivideDel(cAvidaContext& ctx) const { return ctx.GetRandom().P(m_rates.divide.divide_del_prob); }
  bool TestDivideUniform(cAvidaContext& ctx) const
  {
    return (m_rates.divide.divide_uniform_prob == 0.0) ? false : ctx.GetRandom().P(m_rates.divide.divide_uniform_prob);
  }
  bool TestDivideSlip(cAvidaContext& ctx) const { return ctx.GetRandom().P(m_rates.divide.divide_slip_prob); }
  bool TestDivideTrans(cAvidaContext& ctx) const
  {
    return (m_rates.divide.divide_trans_prob == 0.0) ? false : ctx.GetRandom().P(m_rates.divide.divide_trans_prob);
  }
  bool TestDivideLGT(cAvidaContext& ctx) const
  {
    return (m_rates.divide.divide_lgt_prob == 0.0) ? false : ctx.GetRandom().P(m_rates.divide.divide_lgt_prob);
  }


  unsigned int NumDividePoissonMut(cAvidaContext& ctx) const
    { return (m_rates.divide.divide_poisson_mut_mean == 0.0) ? 0 : ctx.GetRandom().GetRandPoisson(m_rates.divide.divide_poisson_mut_mean); }
  unsigned int NumDividePoissonIns(cAvidaContext& ctx) const
    { return (m_rates.divide.divide_poisson_ins_mean == 0.0) ? 0 : ctx.GetRandom().GetRandPoisson(m_rates.divide.divide_poisson_ins_mean); }
  unsigned int NumDividePoissonDel(cAvidaContext& ctx) const
    { return (m_rates.divide.divide_poisson_del_mean == 0.0) ? 0 : ctx.GetRandom().GetRandPoisson(m_rates.divide.divide_poisson_del_mean); }
  unsigned int NumDividePoissonSlip(cAvidaContext& ctx) const
    { return (m_rates.divide.divide_poisson_slip_mean == 0.0) ? 0 : ctx.GetRandom().GetRandPoisson(m_rates.divide.divide_poisson_slip_mean); }
  unsigned int NumDividePoissonTrans(cAvidaContext& ctx) const
    { return (m_rates.divide.divide_poisson_trans_mean == 0.0) ? 0 : ctx.GetRandom().GetRandPoisson(m_rates.divide.divide_poisson_trans_mean); }
  unsigned int NumDividePoissonLGT(cAvidaContext& ctx) const
    { return (m_rates.divide.divide_poisson_lgt_mean == 0.0) ? 0 : ctx.GetRandom().GetRandPoisson(m_rates.divide.divide_poisson_lgt_mean); }


  double DoMetaCopyMut(cAvidaContext& ctx) {
    if (m_rates.meta.copy_mut_prob == 0.0 || !ctx.GetRandom().P(m_rates.meta.copy_mut_prob)) return 1.0;
    const double exp = ctx.GetRandom().GetRandNormal() * m_rates.meta.standard_dev;
    const double change = pow(2.0, exp);
    m_rates.copy.mut_prob *= change;
    return change;
  }

  bool TestDeath(cAvidaContext& ctx) const { return (m_rates.update.death_prob == 0.0) ? false : ctx.GetRandom().P(m_rates.update.death_prob); }


  double GetCopyMutProb() const       { return m_rates.copy.mut_prob; }
  double GetCopyInsProb() const       { return m_rates.copy.ins_prob; }
  double GetCopyDelProb() const       { return m_rates.copy.del_prob; }
  double GetCopyUniformProb() const   { return m_rates.copy.uniform_prob; }
  double GetCopySlipProb() const      { return m_rates.copy.slip_prob; }

  double GetDivInsProb() const        { return m_rates.divide.ins_prob; }
  double GetDivDelProb() const        { return m_rates.divide.del_prob; }
  double GetDivMutProb() const        { return m_rates.divide.mut_prob; }
  double GetDivUniformProb() const    { return m_rates.divide.uniform_prob; }
  double GetDivSlipProb() const       { return m_rates.divide.slip_prob; }
  double GetDivTransProb() const      { return m_rates.divide.trans_prob; }
  double GetDivLGTProb() const        { return m_rates.divide.lgt_prob; }

  double GetDivideMutProb() const     { return m_rates.divide.divide_mut_prob; }
  double GetDivideInsProb() const     { return m_rates.divide.divide_ins_prob; }
  double GetDivideDelProb() const     { return m_rates.divide.divide_del_prob; }
  double GetDivideUniformProb() const { return m_rates.divide.divide_uniform_prob; }
  double GetDivideSlipProb() const    { return m_rates.divide.divide_slip_prob; }
  double GetDivideTransProb() const   { return m_rates.divide.divide_trans_prob; }
  double GetDivideLGTProb() const     { return m_rates.divide.divide_lgt_prob; }

  double GetPointInsProb() const      { return m_rates.point.ins_prob; }
  double GetPointDelProb() const      { return m_rates.point.del_prob; }
  double GetPointMutProb() const      { return m_rates.point.mut_prob; }

  double GetParentMutProb() const     { return m_rates.divide.parent_mut_prob; }
  double GetParentInsProb() const     { return m_rates.divide.parent_ins_prob; }
  double GetParentDelProb() const     { return m_rates.divide.parent_del_prob; }

  double GetInjectInsProb() const     { return m_rates.inject.ins_prob; }
  double GetInjectDelProb() const     { return m_rates.inject.del_prob; }
  double GetInjectMutProb() const     { return m_rates.inject.mut_prob; }

  double GetMetaCopyMutProb() const   { return m_rates.meta.copy_mut_prob; }
  double GetMetaStandardDev() const   { return m_rates.meta.standard_dev; }

  double GetDeathProb() const         { return m_rates.update.death_prob; }


  void SetCopyMutProb(double in_prob)       { m_rates.copy.mut_prob = in_prob; }
  void SetCopyInsProb(double in_prob)       { m_rates.copy.ins_prob = in_prob; }
  void SetCopyDelProb(double in_prob)       { m_rates.copy.del_prob = in_prob; }
  void SetCopyUniformProb(double in_prob)   { m_rates.copy.uniform_prob = in_prob; }
  void SetCopySlipProb(double in_prob)      { m_rates.copy.slip_prob = in_prob; }

  void SetDivMutProb(double in_prob)        { m_rates.divide.mut_prob = in_prob; }
  void SetDivInsProb(double in_prob)        { m_rates.divide.ins_prob = in_prob; }
  void SetDivDelProb(double in_prob)        { m_rates.divide.del_prob = in_prob; }
  void SetDivUniformProb(double in_prob)    { m_rates.divide.uniform_prob = in_prob; }
  void SetDivSlipProb(double in_prob)       { m_rates.divide.slip_prob = in_prob; }
  void SetDivTransProb(double in_prob)      { m_rates.divide.trans_prob = in_prob; }
  void SetDivLGTProb(double in_prob)        { m_rates.divide.lgt_prob = in_prob; }

  void SetDivideMutProb(double in_prob)     { m_rates.divide.divide_mut_prob = in_prob; }
  void SetDivideInsProb(double in_prob)     { m_rates.divide.divide_ins_prob = in_prob; }
  void SetDivideDelProb(double in_prob)     { m_rates.divide.divide_del_prob = in_prob; }
  void SetDivideUniformProb(double in_prob) { m_rates.divide.divide_uniform_prob = in_prob; }
  void SetDivideSlipProb(double in_prob)    { m_rates.divide.divide_slip_prob = in_prob; }
  void SetDivideTransProb(double in_prob)   { m_rates.divide.divide_trans_prob = in_prob; }
  void SetDivideLGTProb(double in_prob)     { m_rates.divide.divide_lgt_prob = in_prob; }

  void SetPointInsProb(double in_prob)      { m_rates.point.ins_prob        = in_prob; }
  void SetPointDelProb(double in_prob)      { m_rates.point.del_prob        = in_prob; }
  void SetPointMutProb(double in_prob)      { m_rates.point.mut_prob        = in_prob; }

  void SetParentMutProb(double in_prob)     { m_rates.divide.parent_mut_prob = in_prob; }
  void SetParentInsProb(double in_prob)     { m_rates.divide.parent_ins_prob = in_prob; }
  void SetParentDelProb(double in_prob)     { m_rates.divide.parent_del_prob = in_prob; }

  void SetInjectInsProb(double in_prob)     { m_rates.inject.ins_prob        = in_prob; }
  void SetInjectDelProb(double in_prob)     { m_rates.inject.del_prob        = in_prob; }
  void SetInjectMutProb(double in_prob)     { m_rates.inject.mut_prob        = in_prob; }

  void SetMetaCopyMutProb(double in_prob)   { m_rates.meta.copy_mut_prob   = in_prob; }
  void SetMetaStandardDev(double in_dev)    { m_rates.meta.standard_dev     = in_dev; }

  void SetDeathProb(double in_prob)         { m_rates.update.death_prob      = in_prob; }
};

#endif
