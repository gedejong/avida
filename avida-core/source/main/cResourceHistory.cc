/*
 *  cResourceHistory.cpp
 *  Avida
 *
 *  Created by David Bryson on 10/27/08.
 *  Copyright 2008-2011 Michigan State University. All rights reserved.
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

#include "cResourceHistory.h"

#include "cInitFile.h"
#include "cResourceCount.h"
#include "cStringList.h"
#include "rust/running_stats_ffi.h"


int cResourceHistory::getEntryForUpdate(int update, bool exact) const
{
  Apto::Array<int> updates;
  updates.Resize(m_entries.GetSize());
  for (int i = 0; i < m_entries.GetSize(); ++i) updates[i] = m_entries[i].update;
  const int* updates_ptr = (updates.GetSize() > 0) ? &updates[0] : NULL;
  return avd_rh_select_entry_index(updates_ptr, updates.GetSize(), update, exact ? 1 : 0);
}

bool cResourceHistory::GetResourceCountForUpdate(cAvidaContext& ctx, int update, cResourceCount& rc, bool exact) const
{
  int entry = getEntryForUpdate(update, exact);
  if (entry < 0 || entry >= m_entries.GetSize()) return false;

  const int value_count = m_entries[entry].values.GetSize();
  const double* values_ptr = (value_count > 0) ? &m_entries[entry].values[0] : NULL;
  for (int i = 0; i < rc.GetSize(); i++) {
    rc.Set(ctx, i, avd_rh_value_at_or_zero(values_ptr, value_count, i));
  }
  
  return true;
}

bool cResourceHistory::GetResourceLevelsForUpdate(int update, Apto::Array<double>& levels, bool exact) const
{
  int entry = getEntryForUpdate(update, exact);
  if (entry < 0 || entry >= m_entries.GetSize()) return false;
  
  levels.Resize(m_entries[entry].values.GetSize());
  const int value_count = m_entries[entry].values.GetSize();
  const double* values_ptr = (value_count > 0) ? &m_entries[entry].values[0] : NULL;
  for (int i = 0; i < levels.GetSize(); i++) {
    levels[i] = avd_rh_value_at_or_zero(values_ptr, value_count, i);
  }
  
  return true;
}

void cResourceHistory::AddEntry(int update, const Apto::Array<double>& values)
{
  // Note that this method does not currently validate that 'update' does not already exist as an entry
  // If this happens, incorrect resource levels may be returned upon retreival

  int new_entry = m_entries.GetSize();
  m_entries.Resize(new_entry + 1);
  m_entries[new_entry].update = update;
  m_entries[new_entry].values = values;
}

bool cResourceHistory::LoadFile(const cString& filename, const cString& working_dir)
{
  cInitFile file(filename, working_dir);
  
  if (!file.WasOpened()) {
//    tConstListIterator<cString> err_it(file.GetErrors());
//    const cString* errstr = NULL;
//    while ((errstr = err_it.Next())) m_world->GetDriver().RaiseException(*errstr);
//    m_world->GetDriver().RaiseFatalException(1, cString("Could not open instruction set '") + filename + "'.");
    return false;
  }
  
  m_entries.Resize(file.GetNumLines());
  for (int line = 0; line < file.GetNumLines(); line++) {
    cStringList cur_line(file.GetLine(line));
    assert(cur_line.GetSize());
    
    m_entries[line].update = cur_line.Pop().AsInt();
    
    int num_values = cur_line.GetSize();
    m_entries[line].values.Resize(num_values);
    for (int i = 0; i < num_values; i++) m_entries[line].values[i] = cur_line.Pop().AsDouble();
  }
  
  return true;
}

