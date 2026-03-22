/*
 *  cBirthEntry.h
 *  Avida
 *
 *  Created by David Bryson on 4/1/09.
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

#ifndef cBirthEntry_h
#define cBirthEntry_h

#include "avida/core/Genome.h"
#include "avida/systematics/Types.h"

#include "AvidaArray.h"
#include "cMerit.h"
#include "cString.h"

#include "rust/running_stats_ffi.h"

class cBioGroup;
class cOrganism;

using namespace Avida;

class cBirthEntry
{
private:
  AvidaArray<int> m_parent_task_count;
public:
  BirthEntryScalars m_scalars;
  Genome genome;
  Systematics::GroupMembershipPtr groups;

  cBirthEntry();
  cBirthEntry(const cBirthEntry& _birth_entry);
  cBirthEntry(const Genome& _offspring, cOrganism* _parent, int _timestamp);
  ~cBirthEntry();

  //Accessor functions
  int GetMatingType() const { return m_scalars.mating_type; }
  int GetParentTaskCount(int which_task) { return m_parent_task_count[which_task]; }
  const AvidaArray<int>& GetParentTaskCount() const { return m_parent_task_count; }
  int GetMatingDisplayA() const { return m_scalars.mating_display_a; }
  int GetMatingDisplayB() const { return m_scalars.mating_display_b; }
  int GetMatePreference() const { return m_scalars.mate_preference; }
  int GetGroupID() const { return m_scalars.group_id; }

  void SetMatingType(int _mating_type) { m_scalars.mating_type = _mating_type; } //@CHC
  void SetParentTaskCount(AvidaArray<int> _parent_task_count) { m_parent_task_count = _parent_task_count; } //@CHC
  void SetMatingDisplayA(int _mating_display_a) { m_scalars.mating_display_a = _mating_display_a; } //@CHC
  void SetMatingDisplayB(int _mating_display_b) { m_scalars.mating_display_b = _mating_display_b; } //@CHC
  void SetMatePreference(int _mate_preference) { m_scalars.mate_preference = _mate_preference; }
  void SetGroupID(int _group_id) { m_scalars.group_id = _group_id; }

  //Other functions
  cString GetPhenotypeString();
  static cString GetPhenotypeStringFormat();

  //Operators
  cBirthEntry& operator=(const cBirthEntry& _birth_entry);

};

#endif
