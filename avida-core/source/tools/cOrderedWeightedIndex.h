/*
 *  cOrderedWeightedIndex.h
 *  Avida
 *
 *  Called "weighted_index.hh" prior to 12/7/05.
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

#ifndef cOrderedWeightedIndex_h
#define cOrderedWeightedIndex_h

#include "avida/core/Types.h"
#include <cassert>
#include "rust/running_stats_ffi.h"

#ifndef NULL
#define NULL 0
#endif

/**
 * This class allows indecies to be assigned a "weight" and then indexed by
 * that weight.
 **/

#include <fstream>

using namespace std;


class cOrderedWeightedIndex
{
protected:
  AvidaOrderedWeightedIndexHandle* m_handle;

public:
  cOrderedWeightedIndex();
  cOrderedWeightedIndex(const cOrderedWeightedIndex& in);
  cOrderedWeightedIndex& operator=(const cOrderedWeightedIndex& in);
  ~cOrderedWeightedIndex();

  void   SetWeight(int value, double weight);
  
  double GetWeight(int id)
  {
    return avd_owi_get_weight(m_handle, id);
  }
  int GetValue(int id)
  {
    return avd_owi_get_value(m_handle, id);
  }

  double GetTotalWeight()
  {
    return avd_owi_get_total_weight(m_handle);
  }
  int GetSize() const
  {
    return avd_owi_get_size(m_handle);
  }
  
  int FindPosition(double position);

};
#endif

