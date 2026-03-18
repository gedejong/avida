/*
 *  cCPUStack.h
 *  Avida
 *
 *  Called "cpu_stack.hh" prior to 11/17/05.
 *  Copyright 1999-2011 Michigan State University. All rights reserved.
 *  Copyright 1999-2001 California Institute of Technology.
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

#ifndef cCPUStack_h
#define cCPUStack_h

#include <iostream>

#ifndef nHardware_h
#include "nHardware.h"
#endif

#include "rust/running_stats_ffi.h"

// cCPUStack now delegates to AvidaCpuStack (Rust-native CpuStack type).
// Memory layout is identical: { int stack[10], unsigned char stack_pointer }.

class cCPUStack
{
private:
  // Layout matches AvidaCpuStack exactly
  int stack[nHardware::STACK_SIZE];
  unsigned char stack_pointer;

  AvidaCpuStack* as_rust() { return reinterpret_cast<AvidaCpuStack*>(this); }
  const AvidaCpuStack* as_rust() const { return reinterpret_cast<const AvidaCpuStack*>(this); }

public:
  cCPUStack() { Clear(); }
  cCPUStack(const cCPUStack& in_stack);
  ~cCPUStack() { ; }

  void operator=(const cCPUStack& in_stack);

  inline void Push(int value) { avd_cpu_stack_push(as_rust(), value); }
  inline int Pop() { return avd_cpu_stack_pop(as_rust()); }
  inline int& Peek() { return stack[stack_pointer]; }
  inline int Peek() const { return avd_cpu_stack_peek(as_rust()); }
  inline int Get(int depth=0) const { return avd_cpu_stack_get(as_rust(), depth); }
  inline void Clear() { avd_cpu_stack_clear(as_rust()); }
  inline int Top() { return avd_cpu_stack_top(as_rust()); }
  void Flip() { avd_cpu_stack_flip(as_rust()); }

  void SaveState(std::ostream& fp);
  void LoadState(std::istream & fp);
};

#endif
