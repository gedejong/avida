/*
 *  cRunningAverage.h
 *  Avida
 *
 *  Called "running_average.hh" prior to 12/7/05.
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

#ifndef cRunningAverage_h
#define cRunningAverage_h

#include <cmath>
#include <cassert>
#include "rust/running_stats_ffi.h"

class cRunningAverage
{
private:
  AvidaRunningAverageHandle* m_handle;
  int m_window_size;
  
  
  cRunningAverage(); // @not_implemented
  cRunningAverage(const cRunningAverage&); // @not_implemented
  cRunningAverage& operator=(const cRunningAverage&); // @not_implemented
  
public:
  cRunningAverage(int window_size);
  ~cRunningAverage();
  
  
  //manipulators
  void Add(double value);
  void Clear();
  
  
  //accessors
  double Sum() const
  {
    return avd_ra_sum(m_handle);
  }

  double S1() const { return Sum(); }

  double SumOfSquares() const
  {
    return avd_ra_sum_of_squares(m_handle);
  }

  double S2() const { return SumOfSquares(); }
  
  double Average() const
  {
    return avd_ra_average(m_handle);
  }

  double Variance() const
  {
    return avd_ra_variance(m_handle);
  }
    
  double StdDeviation() const
  {
    return avd_ra_std_deviation(m_handle);
  }

  double StdError()  const
  {
    return avd_ra_std_error(m_handle);
  }

  // Notation Shortcuts
  double Ave() const { return Average(); }
  double Var() const { return Variance(); }
};

#endif
