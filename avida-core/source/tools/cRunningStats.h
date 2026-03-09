/*
 *  cRunningStats.h
 *  Avida
 *
 *  Created by David on 10/21/09.
 *  Copyright 2009-2011 Michigan State University. All rights reserved.
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

#ifndef cRunningStats_h
#define cRunningStats_h

#include <cmath>
#include <cassert>
#include "rust/running_stats_ffi.h"


class cRunningStats
{
private:
  AvidaRunningStatsHandle* m_handle;
  
public:
  inline cRunningStats()
    : m_handle(avd_rs_new())
  {
    assert(m_handle != 0);
  }

  inline cRunningStats(const cRunningStats& rhs)
    : m_handle(avd_rs_clone(rhs.m_handle))
  {
    assert(m_handle != 0);
  }

  inline cRunningStats& operator=(const cRunningStats& rhs)
  {
    if (this != &rhs) {
      AvidaRunningStatsHandle* new_handle = avd_rs_clone(rhs.m_handle);
      assert(new_handle != 0);
      avd_rs_free(m_handle);
      m_handle = new_handle;
    }
    return *this;
  }

  inline ~cRunningStats()
  {
    avd_rs_free(m_handle);
    m_handle = 0;
  }

  inline void Clear()
  {
    avd_rs_clear(m_handle);
  }
  
  inline void Push(double x);

  inline double N() const
  {
    return avd_rs_n(m_handle);
  }

  inline double Mean() const
  {
    return avd_rs_mean(m_handle);
  }

  inline double StdDeviation() const
  {
    return avd_rs_std_deviation(m_handle);
  }

  inline double StdError() const
  {
    return avd_rs_std_error(m_handle);
  }

  inline double Variance() const
  {
    return avd_rs_variance(m_handle);
  }

  inline double Skewness() const
  {
    return avd_rs_skewness(m_handle);
  }

  inline double Kurtosis() const
  {
    return avd_rs_kurtosis(m_handle);
  }
};


inline void cRunningStats::Push(double x)
{
  avd_rs_push(m_handle, x);
}

#endif
