/*
 *  cDoubleSum.h
 *  Avida
 *
 *  Called "double_sum.hh" prior to 12/7/05.
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

#ifndef cDoubleSum_h
#define cDoubleSum_h

#include <cmath>
#include <climits>
#include <limits>
#include <cassert>
#include "rust/running_stats_ffi.h"

class cDoubleSum {
private:
  AvidaDoubleSumHandle* m_handle;

public:
  cDoubleSum()
    : m_handle(avd_ds_new())
  {
    assert(m_handle != 0);
  }

  cDoubleSum(const cDoubleSum& rhs)
    : m_handle(avd_ds_clone(rhs.m_handle))
  {
    assert(m_handle != 0);
  }

  cDoubleSum& operator=(const cDoubleSum& rhs)
  {
    if (this != &rhs) {
      AvidaDoubleSumHandle* new_handle = avd_ds_clone(rhs.m_handle);
      assert(new_handle != 0);
      avd_ds_free(m_handle);
      m_handle = new_handle;
    }
    return *this;
  }

  ~cDoubleSum()
  {
    avd_ds_free(m_handle);
    m_handle = 0;
  }

  void Clear()
  {
    avd_ds_clear(m_handle);
  }

  double Count() const
  {
    return avd_ds_count(m_handle);
  }

  double N() const { return Count(); }

  double Sum() const
  {
    return avd_ds_sum(m_handle);
  }

  double Max() const
  {
    return avd_ds_max(m_handle);
  }

  double Average() const
  {
    return avd_ds_average(m_handle);
  }

  double Variance() const
  {
    return avd_ds_variance(m_handle);
  }

  double StdDeviation() const
  {
    return avd_ds_std_deviation(m_handle);
  }

  double StdError() const
  {
    return avd_ds_std_error(m_handle);
  }
  
  // Notation Shortcuts
  double Ave() const { return Average(); }
  double Var() const { return Variance(); }

  void Add(double value, double weight = 1.0)
  {
    avd_ds_add(m_handle, value, weight);
  }

  void Subtract(double value, double weight = 1.0)
  {
    avd_ds_subtract(m_handle, value, weight);
  }
};

#endif
