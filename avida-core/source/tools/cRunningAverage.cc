/*
 *  cRunningAverage.cc
 *  Avida
 *
 *  Called "running_average.cc" prior to 12/7/05.
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

#include "cRunningAverage.h"

#include <cassert>


cRunningAverage::cRunningAverage( int window_size ) : 
  m_handle(0), m_window_size(window_size)
{
  assert( m_window_size > 1 );
  m_handle = avd_ra_new(m_window_size);
  assert(m_handle != 0);
}


cRunningAverage::~cRunningAverage() {
  avd_ra_free(m_handle);
  m_handle = 0;
}


void cRunningAverage::Add( double value ) {
  avd_ra_add(m_handle, value);
}


void cRunningAverage::Clear() {
  avd_ra_clear(m_handle);
}
