/*
 *  cHistogram.cc
 *  Avida
 *
 *  Called "histogram.cc" prior to 12/7/05.
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

#include "cHistogram.h"

#include <cstdio>
#include <iostream>

using namespace std;


cHistogram::cHistogram(int in_max, int in_min)
{
  m_handle = avd_hist_new(in_max, in_min);
  assert(m_handle != 0);
}

void cHistogram::Resize(int new_max, int new_min)
{
  avd_hist_resize(m_handle, new_max, new_min);
}

void cHistogram::Print()
{
  FILE * fp = fopen("test.dat", "w");
  fprintf(fp, "Min = %d, Max = %d, Count = %d, Total = %d, Ave = %f\n",
      GetMinBin(), GetMaxBin(), GetCount(), GetTotal(), GetAverage());
  for (int i = GetMinBin(); i <= GetMaxBin(); i++) {
    fprintf(fp, "%d : %d\n", i, GetCount(i));
  }
  fflush(fp);
  fclose(fp);
}
