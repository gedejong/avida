/*
 *  main.cc
 *  Avida
 *
 *  Created by David on 5/3/07
 *  Copyright 2007-2011 Michigan State University. All rights reserved.
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

#include <iostream>
#include <iomanip>
#include <cmath>
#include <limits>

using namespace std;


class cUnitTest
{
private:
  int m_total;
  int m_failed;

protected:
  virtual void RunTests() = 0;
  
  void ReportTestResult(const char* test_name, bool successful);

public:
  cUnitTest() : m_total(0), m_failed(0) { ; }
  virtual ~cUnitTest() { ; }

  virtual const char* GetUnitName() = 0;
  
  void Execute();
  
  inline int GetNumTests() { return m_total; }
  inline int GetSuccessful() { return m_total - m_failed; }
  inline int GetFailed() { return m_failed; }
};



#include "cBitArray.h"
#include "apto/core/Map.h"
#include "avida/data/Package.h"
#include "avida/data/Provider.h"
#include "avida/data/TimeSeriesRecorder.h"
#include "rust/running_stats_ffi.h"
#include "cDoubleSum.h"
#include "cHistogram.h"
#include "cOrderedWeightedIndex.h"
#include "cRunningAverage.h"
#include "cRunningStats.h"
#include "cWeightedIndex.h"
class cRawBitArrayTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cRawBitArray"; }
protected:
  void RunTests()
  {
    int result = true;
    
    cRawBitArray bit_array1;
    bit_array1.ResizeClear(10);
    for (int i = 0; i < 10; i++) {
      if (bit_array1.GetBit(i) != false) {
        result = false;
        break;
      }
    }
    ReportTestResult("Default Constructor - ResizeClear", result);
    

    result = true;
    
    bit_array1.SetBit(1, true);
    bit_array1.SetBit(3, true);
    bit_array1.SetBit(5, true);
    bit_array1.SetBit(7, true);
    bit_array1.SetBit(9, true);
    
    for (int i = 0; i < 10; i++) {
      bool bit_value = !(2*(i/2) == i);
      if (bit_array1.GetBit(i) != bit_value) {
        result = false;
        break;
      }
    }
    ReportTestResult("SetBit (round 1)", result);

    
    result = true;
    
    bit_array1.SetBit(0, true);
    bit_array1.SetBit(1, false);
    bit_array1.SetBit(5, false);
    bit_array1.SetBit(6, true);
    bit_array1.SetBit(7, false);
    
    for (int i = 0; i < 10; i++) {
      bool bit_value = (3*(i/3) == i);
      if (bit_array1.GetBit(i) != bit_value) {
        result = false;
        break;
      }
    }
    ReportTestResult("SetBit (round 2)", result);
    
    // Test constructor with initial size < 32.
    cRawBitArray bit_array2(26);
    for (int i = 0; i < 26; i++) {
      if (bit_array2.GetBit(i) != false) {
        result = false;
        break;
      }
    }
    ReportTestResult("Constructor (size < 32)", result);
    
    
    result = true;
    
    bit_array2.SetBit(8, true);
    bit_array2.Copy(bit_array1, 10);
    
    for (int i = 0; i < 10; i++) {
      bool bit_value = (3*(i/3) == i);
      if (bit_array2.GetBit(i) != bit_value) {
        result = false;
        break;
      }
    }
    ReportTestResult("Copy", result);
    
    
    result = true;
    
    // Test constructor with initial size > 32.
    const int high_bit_count = 1000;
    cRawBitArray bit_array3(high_bit_count);
    int bit_pos = 2;
    while (bit_pos < high_bit_count) {
      bit_array3.SetBit(bit_pos, true);
      bit_pos = bit_pos * 3 / 2;
    }
    
    
    // Test faux copy constructor.
    cRawBitArray bit_array4(bit_array3, high_bit_count);
    bit_array4.SetBit(22, true);
    bit_array4.SetBit(24, true);
    int count1 =  bit_array3.CountBits(high_bit_count);
    int count2 =  bit_array3.CountBits2(high_bit_count);
    int count3 =  bit_array4.CountBits(high_bit_count);
    int count4 =  bit_array4.CountBits2(high_bit_count);
    ReportTestResult("CountBits vs. CountBits2", (count1 == count2 && count3 == count4));
    ReportTestResult("CountBits - Post Copy", (count1 == (count3 - 2)));
    
    
    int diff_count = 0;
    for (int i = 0; i < high_bit_count; i++) {
      if (bit_array3.GetBit(i) != bit_array4.GetBit(i)) diff_count++;
    }
    ReportTestResult("Copy Constructor", (diff_count == 2));
    
    
    // LOGICAL OPERATORS
    
    bit_array4.Resize(1000, 70);
    int count5 = bit_array4.CountBits(70);
    bit_array4.NOT(70);
    int count6 = bit_array4.CountBits(70);
    bit_array4.NOT(70);
    ReportTestResult("NOT Operation", (count5 + count6 == 70));
    
    
    cRawBitArray bit_array5(70);
    int pos = 1;
    int step = 1;
    while (pos <= 70) {
      bit_array5.SetBit(70 - pos, true);
      pos += step++;
    }
    
    cRawBitArray bit_array6(70);
    bit_array6.AND(bit_array4, bit_array5, 70);
    int count_and = bit_array6.CountBits(70);
    ReportTestResult("AND Operation", (count_and == 3));
    
    
    bit_array6.OR(bit_array4, bit_array5, 70);
    int count_or = bit_array6.CountBits(70);
    ReportTestResult("OR Operation", (count_or == 21));
    
    
    bit_array6.NAND(bit_array4, bit_array5, 70);
    int count_nand = bit_array6.CountBits(70);
    ReportTestResult("NAND Operation", (count_nand == 67));
    
    
    bit_array6.NOR(bit_array4, bit_array5, 70);
    int count_nor = bit_array6.CountBits(70);
    ReportTestResult("NOR Operation", (count_nor == 49));
    
    
    bit_array6.XOR(bit_array4, bit_array5, 70);
    int count_xor = bit_array6.CountBits(70);
    ReportTestResult("XOR Operation", (count_xor == 18));
    
    
    bit_array6.EQU(bit_array4, bit_array5, 70);
    int count_equ = bit_array6.CountBits(70);
    ReportTestResult("EQU Operation", (count_equ == 52));
    
    
    // LEFT AND RIGHT SHIFT
    
    cRawBitArray bit_array7(32);
    bit_array7.SetBit(0, true);
    
    bit_array7.SHIFT(32, 0);
    ReportTestResult("Shift 0", (bit_array7.GetBit(0) && bit_array7.CountBits(32) == 1));
    
    
    bit_array7.SHIFT(32, 31);
    ReportTestResult("Shift Left", (bit_array7.GetBit(31) && bit_array7.CountBits(32) == 1));
    
    
    bit_array7.SHIFT(32, -31);
    ReportTestResult("Shift Right (sign bit)", (bit_array7.GetBit(0) || bit_array7.CountBits(32) == 1));
    
    
    bit_array7.SHIFT(32, 30);
    bit_array7.SHIFT(32, -30);
    ReportTestResult("Shift Right (no sign bit)", (bit_array7.GetBit(0) && bit_array7.CountBits(32) == 1));
    
    
    bit_array7.SHIFT(32, 32);
    ReportTestResult("Shift Left Overflow", (bit_array7.CountBits(32) == 0));
    
    
    bit_array7.SetBit(31, true);
    bit_array7.SHIFT(32, -32);
    ReportTestResult("Shift Right Overflow", (bit_array7.CountBits(32) == 0));
    
    
    cRawBitArray bit_array8(34);
    bit_array8.SetBit(0, true);
    
    bit_array8.SHIFT(34, 33);
    ReportTestResult("Shift Left (across bit fields)", (bit_array8.GetBit(33) && bit_array8.CountBits(34) == 1));
    
    
    bit_array8.SHIFT(34, -33);
    ReportTestResult("Shift Right (across bit fields)", (bit_array8.GetBit(0) && bit_array8.CountBits(34) == 1));
    
    
    cRawBitArray bit_array9(66);
    bit_array9.SetBit(0, true);
    bit_array9.SetBit(32, true);
    
    bit_array9.SHIFT(66, 65);
    ReportTestResult("Shift Left (multiple bit fields)", (bit_array9.GetBit(65) && bit_array9.CountBits(66) == 1));
    
    
    bit_array9.SHIFT(66, -65);
    ReportTestResult("Shift Right (multiple bit fields)", (bit_array9.GetBit(0) && bit_array9.CountBits(66) == 1));
    
    // INCREMENT
    
    cRawBitArray bit_array10(1);
    
    bit_array10.INCREMENT(1);
    ReportTestResult("Increment", (bit_array10.GetBit(0) && bit_array10.CountBits(1) == 1));
    
    bit_array10.INCREMENT(1);
    ReportTestResult("Increment Overflow", (bit_array10.GetBit(0) == false && bit_array10.CountBits(1) == 0));
    
    cRawBitArray bit_array11(33);
    for (int i = 0; i < 32; i++) { bit_array11.SetBit(i, true); }
    bit_array11.INCREMENT(33);
    ReportTestResult("Increment (multiple bit fields)", (bit_array11.GetBit(32) == 1 && bit_array11.CountBits(33) == 1));
    
  }
};

class cBitArrayTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cBitArray"; }
protected:
  void RunTests()
  {
    cBitArray ba(74);
    for (int i = 0; i < 74; i++) if (i % 5 == 3) ba[i] = true;
    
    cBitArray ba2(74);
    for (int i = 0; i < 74; i++) {
      if ((i % 2 == 0 || i % 3 == 0) && i % 6 != 0) ba2[i] = true;
    }
    
    ReportTestResult("operator&", ((ba & ba2).CountBits() == 8));
    ReportTestResult("operator|", ((ba | ba2).CountBits() == 43));
    ReportTestResult("operator^", ((ba ^ ba2).CountBits() == 35));
    ReportTestResult("operator~", ((~ba).CountBits() == 59));
    ReportTestResult("operator<<", ((ba << 65).CountBits() == 2));
    ReportTestResult("operator>>", ((ba >> 65).CountBits() == 2));
    ReportTestResult("Chained Bitwise Operations", ((~ba & ~ba2).CountBits() == 31));
    ReportTestResult("++operator", ((++(~ba & ~ba2)).CountBits() == 30));
    ReportTestResult("operator++", (((~ba & ~ba2)++).CountBits() == 31));

    cBitArray find_test(16);
    find_test.Clear();
    find_test.Set(5, true);
    find_test.Set(9, true);
    ReportTestResult("FindBit1", (find_test.FindBit1(0) == 5 && find_test.FindBit1(6) == 9 && find_test.FindBit1(10) == -1));

    Apto::Array<int> ones = find_test.GetOnes();
    ReportTestResult("GetOnes", (ones.GetSize() == 2 && ones[0] == 5 && ones[1] == 9));

    cBitArray resize_test(34);
    resize_test.SetAll();
    ReportTestResult("SetAll NonWordAligned", (resize_test.CountBits() == 34));
    resize_test.Resize(33);
    ReportTestResult("Resize Shrink Masks Tail", (resize_test.CountBits() == 33));
    resize_test.Clear();
    ReportTestResult("Clear NonWordAligned", (resize_test.CountBits() == 0));

    cBitArray hash_a(16);
    hash_a.Set(1, true);
    hash_a.Set(3, true);
    cBitArray hash_b(hash_a);
    int hash_a_val = Apto::HashKey<cBitArray, 101>::Hash(hash_a);
    int hash_b_val = Apto::HashKey<cBitArray, 101>::Hash(hash_b);
    hash_b.Set(4, true);
    int hash_c_val = Apto::HashKey<cBitArray, 101>::Hash(hash_b);
    ReportTestResult("MapKey Equality/Hash Compatibility", (hash_a == cBitArray(hash_a) && hash_a_val == hash_b_val && hash_c_val != hash_a_val));

    Apto::Map<cBitArray, int> phenotype_table;
    cBitArray phen_a(8);
    phen_a.Clear();
    phen_a.Set(0, true); // viability bit
    phen_a.Set(3, true); // one task bit

    cBitArray phen_a_same(8);
    phen_a_same.Clear();
    phen_a_same.Set(0, true);
    phen_a_same.Set(3, true);

    int cpu_count = 0;
    ReportTestResult("MapKey New Phenotype Missing", phenotype_table.Get(phen_a, cpu_count) == false);
    phenotype_table.Set(phen_a, 1);
    ReportTestResult("MapKey Grouping Reuses Equal Key", phenotype_table.Get(phen_a_same, cpu_count) && cpu_count == 1);

    // Mutating the external key object must not mutate the key stored in the map.
    phen_a.Resize(9);
    phen_a.Set(8, true);
    ReportTestResult("MapKey Stored Entry Survives External Key Mutation",
      phenotype_table.Get(phen_a_same, cpu_count) && cpu_count == 1);

    cBitArray phen_b(8);
    phen_b.Clear();
    phen_b.Set(0, true);
    phen_b.Set(4, true); // different phenotype from phen_a_same
    ReportTestResult("MapKey Distinguishes Different Phenotypes", phenotype_table.Get(phen_b, cpu_count) == false);
    phenotype_table.Set(phen_b, 3);
    ReportTestResult("MapKey Independent Buckets Preserve Counts",
      phenotype_table.Get(phen_a_same, cpu_count) && cpu_count == 1 && phenotype_table.Get(phen_b, cpu_count) && cpu_count == 3);
  }
};

class cRunningStatsTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cRunningStats"; }
protected:
  void RunTests()
  {
    cRunningStats stats;

    stats.Push(1.0);
    stats.Push(2.0);
    stats.Push(3.0);
    stats.Push(4.0);

    ReportTestResult("Count", (fabs(stats.N() - 4.0) < 1e-12));
    ReportTestResult("Mean", (fabs(stats.Mean() - 2.5) < 1e-12));
    ReportTestResult("Variance", (fabs(stats.Variance() - 1.6666666666666667) < 1e-12));
    ReportTestResult("StdDeviation", (fabs(stats.StdDeviation() - 1.2909944487358056) < 1e-12));
    ReportTestResult("StdError", (fabs(stats.StdError() - 0.6454972243679028) < 1e-12));

    cRunningStats copied(stats);
    ReportTestResult("Copy Constructor", (fabs(copied.Mean() - stats.Mean()) < 1e-12));

    cRunningStats assigned;
    assigned = stats;
    ReportTestResult("Assignment", (fabs(assigned.Variance() - stats.Variance()) < 1e-12));

    stats.Clear();
    ReportTestResult("Clear", (fabs(stats.N()) < 1e-12 && fabs(stats.Mean()) < 1e-12));
  }
};

class cRunningAverageTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cRunningAverage"; }
protected:
  void RunTests()
  {
    cRunningAverage avg(3);

    avg.Add(1.0);
    avg.Add(2.0);
    ReportTestResult("Warm-up Average", (fabs(avg.Average()) < 1e-12));
    ReportTestResult("Warm-up Variance", (fabs(avg.Variance()) < 1e-12));
    ReportTestResult("Warm-up StdError", (fabs(avg.StdError()) < 1e-12));

    avg.Add(3.0);
    ReportTestResult("Steady Average", (fabs(avg.Average() - 2.0) < 1e-12));
    ReportTestResult("Steady Variance", (fabs(avg.Variance() - 1.0) < 1e-12));
    ReportTestResult("Steady StdError", (fabs(avg.StdError() - 3.4641016151377544) < 1e-12));

    avg.Add(10.0);
    ReportTestResult("Wrap Average", (fabs(avg.Average() - 5.0) < 1e-12));
    ReportTestResult("Wrap Variance", (fabs(avg.Variance() - 19.0) < 1e-12));

    avg.Clear();
    ReportTestResult("Clear Resets Average", (fabs(avg.Average()) < 1e-12));
    ReportTestResult("Clear Resets Variance", (fabs(avg.Variance()) < 1e-12));
  }
};

class cDoubleSumTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cDoubleSum"; }
protected:
  void RunTests()
  {
    cDoubleSum sum;
    ReportTestResult("Initial Count", (fabs(sum.Count()) < 1e-12));
    ReportTestResult("Initial Sum", (fabs(sum.Sum()) < 1e-12));
    ReportTestResult("Initial Average", (fabs(sum.Average()) < 1e-12));

    sum.Add(2.0);
    sum.Add(4.0, 2.0);
    ReportTestResult("Weighted Count", (fabs(sum.Count() - 3.0) < 1e-12));
    ReportTestResult("Weighted Sum", (fabs(sum.Sum() - 10.0) < 1e-12));
    ReportTestResult("Weighted Average", (fabs(sum.Average() - (10.0 / 3.0)) < 1e-12));
    ReportTestResult("Tracked Max", (fabs(sum.Max() - 4.0) < 1e-12));

    cDoubleSum copied(sum);
    ReportTestResult("Copy Constructor", (fabs(copied.Sum() - sum.Sum()) < 1e-12));
    copied.Subtract(4.0, 2.0);
    ReportTestResult("Subtract", (fabs(copied.Count() - 1.0) < 1e-12 && fabs(copied.Sum() - 2.0) < 1e-12));

    cDoubleSum assigned;
    assigned = sum;
    ReportTestResult("Assignment", (fabs(assigned.Variance() - sum.Variance()) < 1e-12));

    sum.Clear();
    ReportTestResult("Clear", (fabs(sum.Count()) < 1e-12 && fabs(sum.Sum()) < 1e-12));
  }
};

class cWeightedIndexTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cWeightedIndex"; }
protected:
  void RunTests()
  {
    cWeightedIndex wi(7);
    wi.SetWeight(0, 1.0);
    wi.SetWeight(1, 2.0);
    wi.SetWeight(2, 3.0);
    wi.SetWeight(3, 4.0);
    wi.SetWeight(4, 5.0);
    wi.SetWeight(5, 6.0);
    wi.SetWeight(6, 7.0);

    ReportTestResult("TotalWeight", (fabs(wi.GetTotalWeight() - 28.0) < 1e-12));
    ReportTestResult("Find Root Bucket", (wi.FindPosition(0.5) == 0));
    ReportTestResult("Find Left Branch Bucket", (wi.FindPosition(3.5) == 3));
    ReportTestResult("Find Right Branch Bucket", (wi.FindPosition(27.5) == 6));

    cWeightedIndex copied(wi);
    ReportTestResult("Copy Constructor", (fabs(copied.GetTotalWeight() - wi.GetTotalWeight()) < 1e-12));

    cWeightedIndex assigned(7);
    assigned = wi;
    ReportTestResult("Assignment", (fabs(assigned.GetWeight(4) - 5.0) < 1e-12));
  }
};

class cOrderedWeightedIndexTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cOrderedWeightedIndex"; }
protected:
  void RunTests()
  {
    cOrderedWeightedIndex owi;
    owi.SetWeight(10, 1.0);
    owi.SetWeight(20, 2.0);
    owi.SetWeight(30, 3.0);
    owi.SetWeight(40, 4.0);

    ReportTestResult("TotalWeight", (fabs(owi.GetTotalWeight() - 10.0) < 1e-12));
    ReportTestResult("Find First Bucket", (owi.FindPosition(0.5) == 10));
    ReportTestResult("Find Mid Bucket", (owi.FindPosition(3.5) == 30));
    ReportTestResult("Find Last Bucket", (owi.FindPosition(8.5) == 40));

    cOrderedWeightedIndex copied(owi);
    ReportTestResult("Copy Constructor", (fabs(copied.GetTotalWeight() - owi.GetTotalWeight()) < 1e-12));

    cOrderedWeightedIndex assigned;
    assigned = owi;
    ReportTestResult("Assignment", (assigned.GetValue(2) == 30));
  }
};

class cHistogramTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cHistogram"; }
protected:
  void RunTests()
  {
    cHistogram hist(5, 1);
    hist.Insert(1, 1);
    hist.Insert(3, 2);
    hist.Insert(5, 1);

    ReportTestResult("Count/Total", (hist.GetCount() == 4 && hist.GetTotal() == 12));
    ReportTestResult("Mode", (hist.GetMode() == 3));
    ReportTestResult("Average", (fabs(hist.GetAverage() - 3.0) < 1e-12));
    ReportTestResult("Variance", (fabs(hist.GetVariance() - (8.0 / 3.0)) < 1e-12));

    hist.Remove(3);
    ReportTestResult("Remove", (hist.GetCount() == 3 && hist.GetTotal() == 9));

    hist.RemoveBin(5);
    ReportTestResult("RemoveBin", (hist.GetCount() == 2 && hist.GetTotal() == 4));

    hist.Resize(4, 2);
    ReportTestResult("Resize", (hist.GetMinBin() == 2 && hist.GetMaxBin() == 4 && hist.GetCount() == 1 && hist.GetTotal() == 3));
  }
};

class cPackageTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "Data::Package"; }
protected:
  void RunTests()
  {
    Avida::Data::Wrap<Apto::String> truthy("true");
    Avida::Data::Wrap<Apto::String> truthy_short("T");
    Avida::Data::Wrap<Apto::String> falsey("false");
    Avida::Data::Wrap<Apto::String> hex_value("0x10");
    Avida::Data::Wrap<Apto::String> double_value("2.5");
    Avida::Data::Wrap<Apto::String> malformed("not_a_number");

    ReportTestResult("Wrap<String> BoolValue true", truthy.BoolValue());
    ReportTestResult("Wrap<String> BoolValue short true", truthy_short.BoolValue());
    ReportTestResult("Wrap<String> BoolValue false", !falsey.BoolValue());
    ReportTestResult("Wrap<String> IntValue hex parse", hex_value.IntValue() == 16);
    ReportTestResult("Wrap<String> DoubleValue parse", fabs(double_value.DoubleValue() - 2.5) < 1e-12);
    ReportTestResult("Wrap<String> malformed numeric fallback",
      malformed.IntValue() == 0 && fabs(malformed.DoubleValue()) < 1e-12);

    Avida::Data::Wrap<bool> wrapped_bool(true);
    Avida::Data::Wrap<int> wrapped_int(-42);
    Avida::Data::Wrap<double> wrapped_double(1234567.0);
    Avida::Data::Wrap<double> wrapped_nan(std::numeric_limits<double>::quiet_NaN());
    ReportTestResult("Wrap<bool> StringValue parity", wrapped_bool.StringValue() == Apto::AsStr(true));
    ReportTestResult("Wrap<int> StringValue parity", wrapped_int.StringValue() == Apto::AsStr(-42));
    ReportTestResult("Wrap<double> StringValue parity", wrapped_double.StringValue() == Apto::AsStr(1234567.0));
    ReportTestResult("Wrap<double> NaN StringValue parity", wrapped_nan.StringValue() == Apto::AsStr(std::numeric_limits<double>::quiet_NaN()));

    Avida::Data::ArrayPackage arr;
    ReportTestResult("ArrayPackage empty conversions",
      arr.BoolValue() == false &&
      arr.IntValue() == 0 &&
      arr.StringValue() == "" &&
      Apto::String(arr.GetAggregateDescriptor()) == "array(0)" &&
      std::isnan(arr.DoubleValue()));

    arr.AddComponent(Avida::Data::PackagePtr(new Avida::Data::Wrap<Apto::String>("a")));
    arr.AddComponent(Avida::Data::PackagePtr(new Avida::Data::Wrap<Apto::String>("b")));
    ReportTestResult("ArrayPackage deterministic formatting",
      arr.NumComponents() == 2 &&
      Apto::String(arr.GetAggregateDescriptor()) == "array(2)" &&
      arr.StringValue() == "'a','b'");

    const bool bool_matrix[] = {false, true};
    bool bool_matrix_ok = true;
    for (size_t i = 0; i < sizeof(bool_matrix) / sizeof(bool_matrix[0]); ++i) {
      Avida::Data::Wrap<bool> wrapped(bool_matrix[i]);
      if (wrapped.StringValue() != Apto::String(Apto::AsStr(bool_matrix[i]))) bool_matrix_ok = false;
    }
    ReportTestResult("Wrap<bool> parity matrix", bool_matrix_ok);

    const int int_matrix[] = {
      0, 1, -1, std::numeric_limits<int>::max(), std::numeric_limits<int>::min()
    };
    bool int_matrix_ok = true;
    for (size_t i = 0; i < sizeof(int_matrix) / sizeof(int_matrix[0]); ++i) {
      Avida::Data::Wrap<int> wrapped(int_matrix[i]);
      if (wrapped.StringValue() != Apto::String(Apto::AsStr(int_matrix[i]))) int_matrix_ok = false;
    }
    ReportTestResult("Wrap<int> parity matrix", int_matrix_ok);

    const double double_matrix[] = {
      0.0,
      -0.0,
      std::numeric_limits<double>::min(),
      -std::numeric_limits<double>::min(),
      std::numeric_limits<double>::denorm_min(),
      -std::numeric_limits<double>::denorm_min(),
      9.99999e-5,
      1.0e-4,
      9.99999e5,
      1.0e6,
      1.23456789,
      -1.23456789,
      std::numeric_limits<double>::infinity(),
      -std::numeric_limits<double>::infinity(),
      std::numeric_limits<double>::quiet_NaN()
    };
    bool double_matrix_ok = true;
    for (size_t i = 0; i < sizeof(double_matrix) / sizeof(double_matrix[0]); ++i) {
      Avida::Data::Wrap<double> wrapped(double_matrix[i]);
      if (wrapped.StringValue() != Apto::String(Apto::AsStr(double_matrix[i]))) {
        double_matrix_ok = false;
      }
    }
    ReportTestResult("Wrap<double> parity matrix", double_matrix_ok);
  }
};

class cTimeSeriesRecorderDouble : public Avida::Data::TimeSeriesRecorder<double>
{
public:
  cTimeSeriesRecorderDouble(const Avida::Data::DataID& data_id)
    : Avida::Data::TimeSeriesRecorder<double>(data_id) { ; }
  cTimeSeriesRecorderDouble(const Avida::Data::DataID& data_id, Apto::String str)
    : Avida::Data::TimeSeriesRecorder<double>(data_id, str) { ; }
protected:
  bool shouldRecordValue(Avida::Update) { return true; }
};

class cTimeSeriesRecorderTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "TimeSeriesRecorder"; }
protected:
  void RunTests()
  {
    Avida::Data::DataID data_id("demo.value");
    cTimeSeriesRecorderDouble rec(data_id);
    Avida::Data::DataRetrievalFunctor retrieve = [] (const Avida::Data::DataID&) {
      return Avida::Data::PackagePtr(new Avida::Data::Wrap<double>(1.25));
    };
    rec.NotifyData(10, retrieve);
    rec.NotifyData(12, retrieve);
    ReportTestResult("NotifyData + AsString", (rec.AsString() == "10:1.250000,12:1.250000"));
    ReportTestResult("NotifyData count from Rust authority", (rec.NumPoints() == 2));
    ReportTestResult("NotifyData typed retrieval parity",
      (fabs(rec.DataPoint(0) - 1.25) < 1e-12) &&
      (fabs(rec.DataPoint(1) - 1.25) < 1e-12) &&
      (rec.DataTime(0) == 10) &&
      (rec.DataTime(1) == 12));

    cTimeSeriesRecorderDouble loaded(data_id, "3:2.500000,5:4.000000");
    ReportTestResult("String Constructor Parses Count", (loaded.NumPoints() == 2));
    ReportTestResult("String Constructor Parses Data", (fabs(loaded.DataPoint(0) - 2.5) < 1e-12 && fabs(loaded.DataPoint(1) - 4.0) < 1e-12));
    ReportTestResult("String Constructor Parses Time", (loaded.DataTime(0) == 3 && loaded.DataTime(1) == 5));
    ReportTestResult("String Constructor Roundtrip", (loaded.AsString() == "3:2.500000,5:4.000000"));

    cTimeSeriesRecorderDouble malformed(data_id, "7:bad,8:3.000000");
    ReportTestResult("Malformed numeric entry defaults safely", (fabs(malformed.DataPoint(0) - 0.0) < 1e-12));
    ReportTestResult("Malformed constructor keeps timeline", (malformed.NumPoints() == 2 && malformed.DataTime(0) == 7 && malformed.DataTime(1) == 8));

    AvidaTimeSeriesHandle* matrix = avd_tsr_from_string("1:1,2:T,3:true,4: true,5:2,6:0x10,7:7x,8:1e2,9:nan");
    int out_i = -99;
    int out_b = -99;
    double out_d = -99.0;

    ReportTestResult("TSR bool exact true literal",
      avd_tsr_value_as_bool(matrix, 2, &out_b) == 1 && out_b == 1);
    ReportTestResult("TSR bool leading-space false",
      avd_tsr_value_as_bool(matrix, 3, &out_b) == 1 && out_b == 0);
    ReportTestResult("TSR bool non-canonical nonzero false",
      avd_tsr_value_as_bool(matrix, 4, &out_b) == 1 && out_b == 0);
    ReportTestResult("TSR int hex coercion parity",
      avd_tsr_value_as_int(matrix, 5, &out_i) == 1 && out_i == 16);
    ReportTestResult("TSR int partial parse parity",
      avd_tsr_value_as_int(matrix, 6, &out_i) == 1 && out_i == 7);
    ReportTestResult("TSR double exponent parse parity",
      avd_tsr_value_as_double(matrix, 7, &out_d) == 1 && fabs(out_d - 100.0) < 1e-12);
    ReportTestResult("TSR double nan parse parity",
      avd_tsr_value_as_double(matrix, 8, &out_d) == 1 && std::isnan(out_d));
    avd_tsr_free(matrix);
  }
};

class cMockArgumentedProvider : public Avida::Data::ArgumentedProvider
{
public:
  mutable Avida::Data::DataID last_id;
  mutable Avida::Data::Argument last_arg;

  Avida::Data::ConstDataSetPtr Provides() const { return Avida::Data::ConstDataSetPtr(); }
  void UpdateProvidedValues(Avida::Update) { ; }
  Apto::String DescribeProvidedValue(const Avida::Data::DataID& data_id) const { return data_id; }
  void SetActiveArguments(const Avida::Data::DataID&, Avida::Data::ConstArgumentSetPtr) { ; }
  Avida::Data::ConstArgumentSetPtr GetValidArguments(const Avida::Data::DataID&) const { return Avida::Data::ConstArgumentSetPtr(); }
  bool IsValidArgument(const Avida::Data::DataID&, Avida::Data::Argument) const { return true; }

  Avida::Data::PackagePtr GetProvidedValueForArgument(const Avida::Data::DataID& data_id, const Avida::Data::Argument& arg) const {
    last_id = data_id;
    last_arg = arg;
    return Avida::Data::PackagePtr(new Avida::Data::Wrap<int>(arg.GetSize()));
  }
};

class cProviderTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "Data::Provider"; }
protected:
  void RunTests()
  {
    const int kInvalid = 0;
    const int kStandard = 1;
    const int kArgumented = 2;

    struct ProviderCase {
      const char* data_id;
      int expected_kind;
      const char* expected_raw;
      const char* expected_arg;
      bool expect_package;
      const char* expected_provider_id;
      const char* expected_provider_arg;
    };

    const ProviderCase cases[] = {
      {"core.demo", kStandard, NULL, NULL, true, "core.demo", ""},
      {"demo[]", kArgumented, "demo[]", "", true, "demo[]", ""},
      {"demo[value]", kArgumented, "demo[]", "value", true, "demo[]", "value"},
      {"demo[[x]]", kArgumented, "demo[]", "[x]", true, "demo[]", "[x]"},
      {"demo[x][y]", kArgumented, "demo[]", "x][y", true, "demo[]", "x][y"},
      {"[x]", kArgumented, "[]", "x", true, "[]", "x"},
      {"demo[", kStandard, NULL, NULL, true, "demo[", ""},
      {"demo]", kInvalid, NULL, NULL, false, NULL, NULL}
    };

    cMockArgumentedProvider provider;
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); ++i) {
      const ProviderCase& test_case = cases[i];
      char* raw_id = NULL;
      char* arg = NULL;
      int kind = avd_provider_classify_id(test_case.data_id, &raw_id, &arg);
      bool classify_ok = kind == test_case.expected_kind;
      if (test_case.expected_raw) {
        classify_ok = classify_ok && raw_id && Apto::String(raw_id) == test_case.expected_raw;
      } else {
        classify_ok = classify_ok && raw_id == NULL;
      }
      if (test_case.expected_arg) {
        classify_ok = classify_ok && arg && Apto::String(arg) == test_case.expected_arg;
      } else {
        classify_ok = classify_ok && arg == NULL;
      }
      ReportTestResult("Provider classify matrix case", classify_ok);
      avd_provider_string_free(raw_id);
      avd_provider_string_free(arg);

      Avida::Data::PackagePtr pkg = provider.GetProvidedValue(test_case.data_id);
      bool dispatch_ok = (pkg != NULL) == test_case.expect_package;
      if (test_case.expect_package) {
        dispatch_ok = dispatch_ok &&
          provider.last_id == test_case.expected_provider_id &&
          provider.last_arg == test_case.expected_provider_arg &&
          pkg->IntValue() == (int)Apto::String(test_case.expected_provider_arg).GetSize();
      }
      ReportTestResult("Provider dispatch matrix case", dispatch_ok);
    }
  }
};

class cManagerDataIdHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "Data::Manager ID Helpers"; }
protected:
  void RunTests()
  {
    const int kInvalid = 0;
    const int kStandard = 1;
    const int kArgumented = 2;

    char* raw_id = NULL;
    char* arg = NULL;

    int ok = avd_provider_split_argumented_id("demo[value]", &raw_id, &arg);
    bool split_ok = (ok != 0) && raw_id && arg &&
      Apto::String(raw_id) == "demo[]" &&
      Apto::String(arg) == "value";
    ReportTestResult("Split valid argumented ID", split_ok);
    avd_provider_string_free(raw_id);
    avd_provider_string_free(arg);

    raw_id = NULL;
    arg = NULL;
    ok = avd_provider_split_argumented_id("demo]", &raw_id, &arg);
    ReportTestResult("Split malformed argumented ID fails", ok == 0 && raw_id == NULL && arg == NULL);

    ReportTestResult("Standard ID classification", avd_provider_is_standard_id("core.demo") == 1);
    ReportTestResult("Argumented ID classification", avd_provider_is_argumented_id("core.demo[]") == 1);
    ReportTestResult("Malformed trailing bracket classification", avd_provider_is_argumented_id("x]") == 0);

    raw_id = NULL;
    arg = NULL;
    int kind = avd_provider_classify_id("core.demo", &raw_id, &arg);
    ReportTestResult("Classify standard ID", kind == kStandard && raw_id == NULL && arg == NULL);

    kind = avd_provider_classify_id("demo[value]", &raw_id, &arg);
    bool classify_argumented_ok = (kind == kArgumented) && raw_id && arg &&
      Apto::String(raw_id) == "demo[]" &&
      Apto::String(arg) == "value";
    ReportTestResult("Classify argumented ID", classify_argumented_ok);
    avd_provider_string_free(raw_id);
    avd_provider_string_free(arg);

    raw_id = NULL;
    arg = NULL;
    kind = avd_provider_classify_id("demo]", &raw_id, &arg);
    ReportTestResult("Classify malformed ID", kind == kInvalid && raw_id == NULL && arg == NULL);

    struct MatrixCase {
      const char* id;
      int expected_kind;
      const char* expected_raw;
      const char* expected_arg;
    };
    const MatrixCase matrix_cases[] = {
      {"", kInvalid, NULL, NULL},
      {"demo", kStandard, NULL, NULL},
      {"demo[]", kArgumented, "demo[]", ""},
      {"demo[value]", kArgumented, "demo[]", "value"},
      {"demo[[x]]", kArgumented, "demo[]", "[x]"},
      {"demo[x][y]", kArgumented, "demo[]", "x][y"},
      {"[x]", kArgumented, "[]", "x"},
      {"demo[", kStandard, NULL, NULL},
      {"demo]", kInvalid, NULL, NULL}
    };
    bool matrix_ok = true;
    for (size_t i = 0; i < sizeof(matrix_cases) / sizeof(matrix_cases[0]); ++i) {
      const MatrixCase& c = matrix_cases[i];
      raw_id = NULL;
      arg = NULL;
      kind = avd_provider_classify_id(c.id, &raw_id, &arg);
      bool case_ok = kind == c.expected_kind;
      if (c.expected_raw) case_ok = case_ok && raw_id && Apto::String(raw_id) == c.expected_raw;
      else case_ok = case_ok && raw_id == NULL;
      if (c.expected_arg) case_ok = case_ok && arg && Apto::String(arg) == c.expected_arg;
      else case_ok = case_ok && arg == NULL;
      avd_provider_string_free(raw_id);
      avd_provider_string_free(arg);
      if (!case_ok) matrix_ok = false;
    }
    ReportTestResult("Classify edge-shape matrix", matrix_ok);
  }
};

class cResourceCountLookupHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cResourceCount Lookup Helpers"; }
protected:
  void RunTests()
  {
    const char* names[] = {"resA", "resB", "resA"};
    ReportTestResult(
      "Lookup first match",
      avd_rc_lookup_resource_index(names, 3, "resA") == 0
    );
    ReportTestResult(
      "Lookup later match",
      avd_rc_lookup_resource_index(names, 3, "resB") == 1
    );
    ReportTestResult(
      "Lookup missing name",
      avd_rc_lookup_resource_index(names, 3, "resX") == -1
    );
    ReportTestResult(
      "Lookup invalid input",
      avd_rc_lookup_resource_index(NULL, 3, "resA") == -1
    );
    ReportTestResult(
      "Lookup null query",
      avd_rc_lookup_resource_index(names, 3, NULL) == -1
    );
  }
};

class cResourceCountPrecalcHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cResourceCount Precalc Helpers"; }
protected:
  void RunTests()
  {
    const double update_step = 1.0 / 10000.0;
    const double decay_rate = 0.91;
    const double inflow = 1.75;
    const int rounds = 64;

    double step_decay_ref = pow(decay_rate, update_step);
    double step_inflow_ref = inflow * update_step;
    double step_decay_rust = avd_rc_step_decay(decay_rate, update_step);
    double step_inflow_rust = avd_rc_step_inflow(inflow, update_step);

    ReportTestResult("Step decay parity", fabs(step_decay_ref - step_decay_rust) < 1e-15);
    ReportTestResult("Step inflow parity", fabs(step_inflow_ref - step_inflow_rust) < 1e-15);

    double inflow_ref = 0.0;
    double inflow_rust = 0.0;
    double decay_ref = 1.0;
    double decay_rust = 1.0;
    for (int i = 0; i < rounds; ++i) {
      inflow_ref = inflow_ref * step_decay_ref + step_inflow_ref;
      inflow_rust = avd_rc_inflow_precalc_next(inflow_rust, step_decay_rust, step_inflow_rust);
      decay_ref = decay_ref * step_decay_ref;
      decay_rust = avd_rc_decay_precalc_next(decay_rust, step_decay_rust);
    }

    ReportTestResult("Inflow recurrence parity", fabs(inflow_ref - inflow_rust) < 1e-12);
    ReportTestResult("Decay recurrence parity", fabs(decay_ref - decay_rust) < 1e-12);

    double decay_table[13];
    double inflow_table[13];
    avd_rc_fill_precalc_tables(decay_rate, inflow, update_step, 12, decay_table, inflow_table);
    bool table_parity_ok = true;
    for (int i = 0; i <= 12; ++i) {
      if (fabs(decay_table[i] - pow(decay_rate, update_step * i)) > 1e-12) {
        table_parity_ok = false;
      }
      double inflow_ref_i = 0.0;
      for (int j = 0; j < i; ++j) {
        inflow_ref_i = inflow_ref_i * step_decay_ref + step_inflow_ref;
      }
      if (fabs(inflow_table[i] - inflow_ref_i) > 1e-12) {
        table_parity_ok = false;
      }
    }
    ReportTestResult("Precalc table fill helper parity", table_parity_ok);

    double unchanged_inflow[2] = {9.0, 9.0};
    avd_rc_fill_precalc_tables(decay_rate, inflow, update_step, 1, NULL, unchanged_inflow);
    ReportTestResult("Precalc table fill null decay output no-op", unchanged_inflow[0] == 9.0 && unchanged_inflow[1] == 9.0);
  }
};

class cResourceCountSchedulingHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cResourceCount Scheduling Helpers"; }
protected:
  void RunTests()
  {
    const double step = 1.0 / 10000.0;
    struct Case {
      double update_time;
      int expected_steps;
    };
    const Case cases[] = {
      {0.0, 0},
      {0.5 * step, 0},
      {-0.5 * step, 0},
      {step, 1},
      {2.9 * step, 2},
      {-2.9 * step, -2},
      {25.0 * step, 25}
    };

    ReportTestResult(
      "Scheduling accumulate parity",
      fabs(avd_rc_accumulate_update_time(0.25, 0.125) - (0.25 + 0.125)) < 1e-15
    );

    bool steps_ok = true;
    bool remainder_ok = true;
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); ++i) {
      int got = avd_rc_num_steps(cases[i].update_time, step);
      if (got != cases[i].expected_steps) steps_ok = false;
      double rem = avd_rc_remainder_update_time(cases[i].update_time, step, got);
      double expected_rem = cases[i].update_time - cases[i].expected_steps * step;
      if (fabs(rem - expected_rem) > 1e-15) remainder_ok = false;
    }

    ReportTestResult("Scheduling num_steps parity", steps_ok);
    ReportTestResult("Scheduling remainder parity", remainder_ok);
    ReportTestResult("Scheduling zero-step guard", avd_rc_num_steps(1.0, 0.0) == 0);
    ReportTestResult("Scheduling NaN update_time guard", avd_rc_num_steps(nan(""), step) == 0);
    ReportTestResult(
      "Scheduling positive saturation",
      avd_rc_num_steps(INFINITY, step) == std::numeric_limits<int>::max()
    );
    ReportTestResult(
      "Scheduling negative saturation",
      avd_rc_num_steps(-INFINITY, step) == std::numeric_limits<int>::min()
    );
    ReportTestResult(
      "Spatial scheduling positive delta",
      avd_rc_num_spatial_updates(10, 4) == 6
    );
    ReportTestResult(
      "Spatial scheduling negative delta",
      avd_rc_num_spatial_updates(4, 10) == -6
    );
    ReportTestResult(
      "Spatial scheduling positive saturation",
      avd_rc_num_spatial_updates(std::numeric_limits<int>::max(), -1) == std::numeric_limits<int>::max()
    );
    ReportTestResult(
      "Spatial scheduling negative saturation",
      avd_rc_num_spatial_updates(std::numeric_limits<int>::min(), 1) == std::numeric_limits<int>::min()
    );

    const int precalc_distance = 4;
    const double decay[] = {1.0, 0.9, 0.81, 0.729, 0.6561};
    const double inflow[] = {0.0, 0.1, 0.19, 0.271, 0.3439};
    const double initial = 10.0;
    int remaining = 9;
    double expected = initial;
    while (remaining > precalc_distance) {
      expected = expected * decay[precalc_distance] + inflow[precalc_distance];
      remaining -= precalc_distance;
    }
    expected = expected * decay[remaining] + inflow[remaining];
    const double got = avd_rc_apply_nonspatial_steps(initial, decay, inflow, precalc_distance, 9);
    ReportTestResult("Non-spatial step apply parity", fabs(got - expected) < 1e-12);
    ReportTestResult(
      "Non-spatial step apply null guard",
      avd_rc_apply_nonspatial_steps(initial, NULL, inflow, precalc_distance, 9) == initial
    );
  }
};

class cResourceHistoryHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cResourceHistory Helpers"; }
protected:
  void RunTests()
  {
    const int updates[] = {10, 20, 30};
    const int update_count = sizeof(updates) / sizeof(updates[0]);

    ReportTestResult(
      "Select exact hit",
      avd_rh_select_entry_index(updates, update_count, 20, 1) == 1
    );
    ReportTestResult(
      "Select exact miss",
      avd_rh_select_entry_index(updates, update_count, 25, 1) == -1
    );
    ReportTestResult(
      "Select nearest below first",
      avd_rh_select_entry_index(updates, update_count, 5, 0) == 0
    );
    ReportTestResult(
      "Select nearest interior",
      avd_rh_select_entry_index(updates, update_count, 25, 0) == 1
    );
    ReportTestResult(
      "Select nearest above last",
      avd_rh_select_entry_index(updates, update_count, 40, 0) == 2
    );
    ReportTestResult(
      "Select empty input",
      avd_rh_select_entry_index(NULL, 0, 25, 0) == -1
    );
    ReportTestResult(
      "Select negative count input",
      avd_rh_select_entry_index(updates, -1, 25, 0) == -1
    );

    const double values[] = {1.25, 2.5};
    const int value_count = sizeof(values) / sizeof(values[0]);
    ReportTestResult(
      "Value lookup in range",
      fabs(avd_rh_value_at_or_zero(values, value_count, 1) - 2.5) < 1e-15
    );
    ReportTestResult(
      "Value lookup negative index defaults zero",
      avd_rh_value_at_or_zero(values, value_count, -1) == 0.0
    );
    ReportTestResult(
      "Value lookup overflow index defaults zero",
      avd_rh_value_at_or_zero(values, value_count, value_count) == 0.0
    );
    ReportTestResult(
      "Value lookup null input defaults zero",
      avd_rh_value_at_or_zero(NULL, value_count, 0) == 0.0
    );
    ReportTestResult(
      "Value lookup negative count defaults zero",
      avd_rh_value_at_or_zero(values, -1, 0) == 0.0
    );
  }
};

class cSpatialResCountHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cSpatialResCount Helpers"; }
protected:
  void RunTests()
  {
    int start = -99;
    int end = -99;
    ReportTestResult(
      "Normalize span clamps to bounds",
      avd_src_normalize_span(-5, 42, 10, &start, &end) == 1 && start == 0 && end == 10
    );
    ReportTestResult(
      "Normalize span wraps crossing ranges",
      avd_src_normalize_span(8, 3, 10, &start, &end) == 1 && start == 8 && end == 13
    );
    ReportTestResult(
      "Normalize span null output guard",
      avd_src_normalize_span(1, 2, 10, NULL, &end) == 0
    );

    const double dist = sqrt(2.0);
    const double elem1_amount = 10.0;
    const double elem2_amount = 4.0;
    const double inxdiffuse = 1.0;
    const double inydiffuse = 1.0;
    const double inxgravity = 0.5;
    const double inygravity = -0.25;
    const int xdist = 1;
    const int ydist = -1;
    const double diff = elem1_amount - elem2_amount;
    const double xgravity = elem1_amount * fabs(inxgravity) / 3.0;
    const double ygravity = elem1_amount * fabs(inygravity) / 3.0;
    const double expected =
      ((inxdiffuse * diff / 16.0 + inydiffuse * diff / 16.0 + xgravity + ygravity) /
       (fabs(xdist * 1.0) + fabs(ydist * 1.0))) / dist;
    const double got = avd_src_compute_flow_scalar(
      elem1_amount, elem2_amount, inxdiffuse, inydiffuse, inxgravity, inygravity, xdist, ydist, dist
    );
    ReportTestResult("Flow scalar parity", fabs(got - expected) < 1e-12);
    ReportTestResult(
      "Flow scalar legacy zero guard",
      avd_src_compute_flow_scalar(0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 1, 0, -1.0) == 0.0
    );
    ReportTestResult(
      "Source per-cell scalar parity",
      fabs(avd_src_source_per_cell(12.0, 0, 1, 0, 2) - (12.0 / 6.0)) < 1e-15
    );
    ReportTestResult(
      "Source per-cell single-cell passthrough",
      fabs(avd_src_source_per_cell(5.0, 2, 2, 3, 3) - 5.0) < 1e-15
    );
    ReportTestResult(
      "Sink delta parity",
      fabs(avd_src_sink_delta(10.0, 0.2) - 8.0) < 1e-15
    );
    ReportTestResult(
      "Sink delta clamps at zero",
      avd_src_sink_delta(10.0, 1.5) == 0.0
    );
    ReportTestResult(
      "Cell outflow delta parity",
      fabs(avd_src_cell_outflow_delta(10.0, 0.2) - 2.0) < 1e-15
    );
    ReportTestResult(
      "Cell outflow delta clamps at zero",
      avd_src_cell_outflow_delta(10.0, -0.2) == 0.0
    );

    struct WrappedIndexCase {
      int x;
      int y;
      int world_x;
      int world_y;
    };
    const WrappedIndexCase wrapped_cases[] = {
      {2, 1, 5, 4},
      {-1, 0, 5, 4},
      {6, -1, 5, 4},
      {-13, 9, 5, 4}
    };
    const auto wrap = [](int value, int bound) {
      int rem = value % bound;
      return (rem < 0) ? (rem + bound) : rem;
    };
    bool wrapped_index_parity_ok = true;
    for (size_t i = 0; i < sizeof(wrapped_cases) / sizeof(wrapped_cases[0]); ++i) {
      const WrappedIndexCase& c = wrapped_cases[i];
      const int expected = (wrap(c.y, c.world_y) * c.world_x) + wrap(c.x, c.world_x);
      const int got = avd_src_wrapped_elem_index(c.x, c.y, c.world_x, c.world_y);
      if (got != expected) wrapped_index_parity_ok = false;
    }
    ReportTestResult("Wrapped elem index parity matrix", wrapped_index_parity_ok);
    ReportTestResult(
      "Wrapped elem index invalid world_x guard",
      avd_src_wrapped_elem_index(1, 2, 0, 4) == -1
    );
    ReportTestResult(
      "Wrapped elem index invalid world_y guard",
      avd_src_wrapped_elem_index(1, 2, 4, 0) == -1
    );
  }
};

class cEventListParsingHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cEventList Parsing Helpers"; }
protected:
  void RunTests()
  {
    ReportTestResult(
      "Trigger parse immediate alias",
      avd_event_parse_trigger("immediate") == 2
    );
    ReportTestResult(
      "Trigger parse update alias",
      avd_event_parse_trigger("u") == 0
    );
    ReportTestResult(
      "Trigger parse generation alias",
      avd_event_parse_trigger("generation") == 1
    );
    ReportTestResult(
      "Trigger parse births alias",
      avd_event_parse_trigger("births") == 3
    );
    ReportTestResult(
      "Trigger parse org_id alias",
      avd_event_parse_trigger("org_id") == 5
    );
    ReportTestResult(
      "Trigger parse invalid token",
      avd_event_parse_trigger("unknown") == -1
    );
    ReportTestResult(
      "Trigger parse null token",
      avd_event_parse_trigger(NULL) == -1
    );

    double start = 0.0;
    double interval = 0.0;
    double stop = 0.0;
    ReportTestResult(
      "Timing parse begin defaults",
      avd_event_parse_timing("begin", &start, &interval, &stop) == 1 &&
        start == std::numeric_limits<double>::min() &&
        interval == std::numeric_limits<double>::max() &&
        stop == std::numeric_limits<double>::max()
    );
    ReportTestResult(
      "Timing parse start all",
      avd_event_parse_timing("10:all", &start, &interval, &stop) == 1 &&
        fabs(start - 10.0) < 1e-15 &&
        interval == 0.0 &&
        stop == std::numeric_limits<double>::max()
    );
    ReportTestResult(
      "Timing parse full tuple",
      avd_event_parse_timing("10:2:20", &start, &interval, &stop) == 1 &&
        fabs(start - 10.0) < 1e-15 &&
        fabs(interval - 2.0) < 1e-15 &&
        fabs(stop - 20.0) < 1e-15
    );
    ReportTestResult(
      "Timing parse invalid first token",
      avd_event_parse_timing("bad:1:2", &start, &interval, &stop) == 0
    );
    ReportTestResult(
      "Timing parse null output pointer guard",
      avd_event_parse_timing("10:all", NULL, &interval, &stop) == 0
    );
    ReportTestResult(
      "Timing parse legacy fallback on invalid interval",
      avd_event_parse_timing("10:notnum:end", &start, &interval, &stop) == 1 &&
        fabs(interval - 0.0) < 1e-15 &&
        stop == std::numeric_limits<double>::max()
    );
  }
};




#define TEST(CLASS) \
tester = new CLASS ## Tests(); \
tester->Execute(); \
total += tester->GetNumTests(); \
failed += tester->GetFailed(); \
delete tester;

int main(int argc, const char* argv[])
{
  int total = 0;
  int failed = 0;
  
  cUnitTest* tester = NULL;
  
  cout << "Avida Tools Unit Tests" << endl;
  cout << endl;
  
  TEST(cRawBitArray);
  TEST(cBitArray);
  TEST(cRunningStats);
  TEST(cRunningAverage);
  TEST(cDoubleSum);
  TEST(cWeightedIndex);
  TEST(cOrderedWeightedIndex);
  TEST(cHistogram);
  TEST(cPackage);
  TEST(cTimeSeriesRecorder);
  TEST(cProvider);
  TEST(cManagerDataIdHelper);
  TEST(cResourceCountLookupHelper);
  TEST(cResourceCountPrecalcHelper);
  TEST(cResourceCountSchedulingHelper);
  TEST(cResourceHistoryHelper);
  TEST(cSpatialResCountHelper);
  TEST(cEventListParsingHelper);
  
  if (failed == 0)
    cout << "All unit tests passed." << endl;
  else
    cout << failed << " of " << total << " unit tests failed." << endl;

  return failed;
}


void cUnitTest::Execute()
{
  cout << "Testing: " << GetUnitName() << endl;
  cout << "--------------------------------------------------------------------------------" << endl;
  RunTests();
  cout << "--------------------------------------------------------------------------------" << endl;
  if (GetFailed() == 0)
    cout << "All " << GetUnitName() << " tests passed." << endl;
  else
    cout << GetFailed() << " of " << GetNumTests() << " tests failed." << endl;
  
  cout << endl;
}

void cUnitTest::ReportTestResult(const char* test_name, bool successful)
{
  m_total++;
  
  size_t l = strlen(test_name);
  char* str = new char[l + 3];
  str = strncpy(str, test_name, l + 1);
  str = strncat(str, ": ", 2);
  
  cout << setw(74) << left << str;
  if (successful) {
    cout << "passed";
  } else {
    cout << "failed";
    m_failed++;
  }
  cout << endl;
  
  delete[] str;
}
