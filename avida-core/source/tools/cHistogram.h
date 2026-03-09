/*
 *  cHistogram.h
 *  Avida
 *
 *  Called "histogram.hh" prior to 12/7/05.
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

#ifndef cHistogram_h
#define cHistogram_h

#include <cmath>
#include <iostream>
#include <cassert>
#include "rust/running_stats_ffi.h"


class cHistogram {
private:
  AvidaHistogramHandle* m_handle;
public:
  cHistogram(int in_max=1, int in_min=0);
  inline ~cHistogram()
  {
    avd_hist_free(m_handle);
    m_handle = 0;
  }

  void Resize(int new_max, int new_min=0);
  void Print();
  inline void Clear();
  inline void Insert(int value, int count=1);
  inline void Remove(int value);
  inline void RemoveBin(int value);

  inline double GetAverage()
  {
    return avd_hist_get_average(m_handle);
  }
  inline double GetCountAverage()
  {
    return avd_hist_get_count_average(m_handle);
  }
  inline int GetMode();
  inline double GetVariance();
  inline double GetCountVariance();
  inline double GetStdDev();
  inline double GetCountStdDev();
  inline double GetEntropy();
  inline double GetNormEntropy();

  inline int GetCount()
  {
    return avd_hist_get_count(m_handle);
  }
  inline int GetCount(int value)
  {
    return avd_hist_get_count_for_value(m_handle, value);
  }
  inline int GetTotal()
  {
    return avd_hist_get_total(m_handle);
  }
  inline int GetMinBin()
  {
    return avd_hist_get_min_bin(m_handle);
  }
  inline int GetMaxBin()
  {
    return avd_hist_get_max_bin(m_handle);
  }
  inline int GetNumBins()
  {
    return avd_hist_get_num_bins(m_handle);
  }
};


inline void cHistogram::Clear()
{
  avd_hist_clear(m_handle);
}


inline void cHistogram::Insert(int value, int count)
{
  avd_hist_insert(m_handle, value, count);
}

inline void cHistogram::Remove(int value)
{
  avd_hist_remove(m_handle, value);
}

inline void cHistogram::RemoveBin(int value)
{
  avd_hist_remove_bin(m_handle, value);
}

inline int cHistogram::GetMode()
{
  return avd_hist_get_mode(m_handle);
}

inline double cHistogram::GetVariance()
{
  return avd_hist_get_variance(m_handle);
}

inline double cHistogram::GetCountVariance()
{
  return avd_hist_get_count_variance(m_handle);
}

inline double cHistogram::GetStdDev()
{
  return avd_hist_get_std_dev(m_handle);
}

inline double cHistogram::GetCountStdDev()
{
  return avd_hist_get_count_std_dev(m_handle);
}

inline double cHistogram::GetEntropy()
{
  return avd_hist_get_entropy(m_handle);
}

inline double cHistogram::GetNormEntropy()
{
  return avd_hist_get_norm_entropy(m_handle);
}

#endif
