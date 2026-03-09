/*
 *  cWeighedIndex.cc
 *  Avida
 *
 *  Called "weighted_index.cc" prior to 12/7/05.
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

#include "cOrderedWeightedIndex.h"
#include <iostream>

using namespace std;

cOrderedWeightedIndex::cOrderedWeightedIndex()
  : m_handle(avd_owi_new())
{
  assert(m_handle != 0);
}

cOrderedWeightedIndex::cOrderedWeightedIndex(const cOrderedWeightedIndex& in)
  : m_handle(avd_owi_clone(in.m_handle))
{
  assert(m_handle != 0);
}

cOrderedWeightedIndex& cOrderedWeightedIndex::operator=(const cOrderedWeightedIndex& in)
{
  if (this != &in) {
    AvidaOrderedWeightedIndexHandle* new_handle = avd_owi_clone(in.m_handle);
    assert(new_handle != 0);
    avd_owi_free(m_handle);
    m_handle = new_handle;
  }
  return *this;
}


cOrderedWeightedIndex::~cOrderedWeightedIndex()
{
  avd_owi_free(m_handle);
  m_handle = 0;
}



 
void cOrderedWeightedIndex::SetWeight(int value, double in_weight)
{
  avd_owi_set_weight(m_handle, value, in_weight);
 }

int cOrderedWeightedIndex::FindPosition(double position){
  return avd_owi_find_position(m_handle, position);
}



