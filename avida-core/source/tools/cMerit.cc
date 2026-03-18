/*
 *  cMerit.cc
 *  Avida
 *
 *  Called "merit.cc" prior to 12/7/05.
 *  Copyright 1999-2011 Michigan State University. All rights reserved.
 *  Copyright 1993-2001 California Institute of Technology
 *
 */

#include "cMerit.h"

using namespace std;


ostream& cMerit::BinaryPrint(ostream& os) const
{
  for (int i = GetNumBits() - 1; i >= 0; --i) os << GetBit(i);
  return os;
}


bool cMerit::OK() const
{
  double test_value = static_cast<double>(base) * pow(2.0, offset);
  int test_bits = static_cast<int>(log(value) / log(2.0)) + 1;
  if (base == 0) test_bits = 0;

  assert(test_bits == bits &&
         (test_value <= value * (1 + 1 / UINT_MAX) ||
          test_value >= value / (1 + 1 / UINT_MAX)));

  return (test_bits == bits &&
          (test_value <= value * (1 + 1 / UINT_MAX) ||
           test_value >= value / (1 + 1 / UINT_MAX)));
}

ostream& operator<<(ostream& os, const cMerit& merit)
{
  os << merit.GetDouble();
  return os;
}
