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

    double inflow_only_table[13];
    avd_rc_fill_inflow_precalc_table(decay_rate, inflow, update_step, 12, inflow_only_table);
    bool inflow_only_ok = true;
    for (int i = 0; i <= 12; ++i) {
      if (fabs(inflow_only_table[i] - inflow_table[i]) > 1e-12) inflow_only_ok = false;
    }
    ReportTestResult("Inflow-only precalc fill helper parity", inflow_only_ok);

    double decay_only_table[13];
    avd_rc_fill_decay_precalc_table(decay_rate, update_step, 12, decay_only_table);
    bool decay_only_ok = true;
    for (int i = 0; i <= 12; ++i) {
      if (fabs(decay_only_table[i] - decay_table[i]) > 1e-12) decay_only_ok = false;
    }
    ReportTestResult("Decay-only precalc fill helper parity", decay_only_ok);

    double unchanged_decay[2] = {7.0, 7.0};
    avd_rc_fill_decay_precalc_table(decay_rate, update_step, 1, NULL);
    avd_rc_fill_inflow_precalc_table(decay_rate, inflow, update_step, -1, unchanged_inflow);
    avd_rc_fill_decay_precalc_table(decay_rate, update_step, -1, unchanged_decay);
    ReportTestResult("Inflow-only precalc fill invalid distance no-op", unchanged_inflow[0] == 9.0 && unchanged_inflow[1] == 9.0);
    ReportTestResult("Decay-only precalc fill invalid distance no-op", unchanged_decay[0] == 7.0 && unchanged_decay[1] == 7.0);
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
    ReportTestResult(
      "Scheduling update-time delta passthrough",
      fabs(avd_rc_update_time_delta(0.125) - 0.125) < 1e-15
    );
    ReportTestResult(
      "Scheduling update-time delta NaN passthrough",
      isnan(avd_rc_update_time_delta(nan("")))
    );
    ReportTestResult(
      "Scheduling wrapper global-only flag",
      avd_rc_wrapper_global_only_flag(0) == 1
    );
    ReportTestResult(
      "Scheduling wrapper random flag",
      avd_rc_wrapper_global_only_flag(1) == 0
    );
    ReportTestResult(
      "Scheduling wrapper full flag",
      avd_rc_wrapper_global_only_flag(2) == 0
    );
    ReportTestResult(
      "Scheduling wrapper unknown defaults full",
      avd_rc_wrapper_global_only_flag(99) == 0
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
    ReportTestResult(
      "Spatial step iteration positive passthrough",
      avd_rc_spatial_step_iterations(6) == 6
    );
    ReportTestResult(
      "Spatial step iteration zero passthrough",
      avd_rc_spatial_step_iterations(0) == 0
    );
    ReportTestResult(
      "Spatial step iteration negative clamps to zero",
      avd_rc_spatial_step_iterations(-6) == 0
    );
    ReportTestResult(
      "Spatial cell-list branch enabled",
      avd_rc_use_cell_list_branch(4) == 1
    );
    ReportTestResult(
      "Spatial cell-list branch disabled for zero",
      avd_rc_use_cell_list_branch(0) == 0
    );
    ReportTestResult(
      "Spatial cell-list branch disabled for negative",
      avd_rc_use_cell_list_branch(-2) == 0
    );
    ReportTestResult(
      "Dispatch geometry global classified non-spatial",
      avd_rc_is_spatial_geometry(0) == 0
    );
    ReportTestResult(
      "Dispatch geometry partial classified non-spatial",
      avd_rc_is_spatial_geometry(5) == 0
    );
    ReportTestResult(
      "Dispatch geometry grid classified spatial",
      avd_rc_is_spatial_geometry(1) == 1
    );
    ReportTestResult(
      "Dispatch geometry torus classified spatial",
      avd_rc_is_spatial_geometry(2) == 1
    );
    ReportTestResult(
      "Read path global geometry selects global",
      avd_rc_read_path_kind(0) == 0
    );
    ReportTestResult(
      "Read path partial geometry selects global",
      avd_rc_read_path_kind(5) == 0
    );
    ReportTestResult(
      "Read path spatial geometry selects spatial",
      avd_rc_read_path_kind(1) == 1 && avd_rc_read_path_kind(2) == 1
    );
    ReportTestResult(
      "Read path unknown geometry defaults spatial",
      avd_rc_read_path_kind(42) == 1
    );
    const double global_read_payload = 17.25;
    const double spatial_read_payload = 3.5;
    const auto read_path_select_payload = [&](int geometry) {
      return (avd_rc_read_path_kind(geometry) == 1) ? spatial_read_payload : global_read_payload;
    };
    ReportTestResult(
      "Read payload policy global geometry",
      fabs(read_path_select_payload(0) - global_read_payload) < 1e-15
    );
    ReportTestResult(
      "Read payload policy partial geometry",
      fabs(read_path_select_payload(5) - global_read_payload) < 1e-15
    );
    ReportTestResult(
      "Read payload policy spatial geometry",
      fabs(read_path_select_payload(1) - spatial_read_payload) < 1e-15
    );
    ReportTestResult(
      "Write path global geometry selects non-spatial",
      avd_rc_is_spatial_geometry(0) == 0
    );
    ReportTestResult(
      "Write path partial geometry selects non-spatial",
      avd_rc_is_spatial_geometry(5) == 0
    );
    ReportTestResult(
      "Write path grid geometry selects spatial",
      avd_rc_is_spatial_geometry(1) == 1
    );
    ReportTestResult(
      "Write path torus geometry selects spatial",
      avd_rc_is_spatial_geometry(2) == 1
    );
    ReportTestResult(
      "Write path unknown geometry defaults spatial",
      avd_rc_is_spatial_geometry(42) == 1
    );
    ReportTestResult(
      "SetCell write path global geometry no-op",
      avd_rc_setcell_write_path_kind(0) == 0
    );
    ReportTestResult(
      "SetCell write path partial geometry no-op",
      avd_rc_setcell_write_path_kind(5) == 0
    );
    ReportTestResult(
      "SetCell write path spatial geometry write",
      avd_rc_setcell_write_path_kind(1) == 1 && avd_rc_setcell_write_path_kind(2) == 1
    );
    ReportTestResult(
      "SetCell write path unknown geometry defaults write",
      avd_rc_setcell_write_path_kind(42) == 1
    );
    ReportTestResult(
      "Setup path global geometry",
      avd_rc_setup_path_kind(0) == 0
    );
    ReportTestResult(
      "Setup path partial geometry",
      avd_rc_setup_path_kind(5) == 1
    );
    ReportTestResult(
      "Setup path spatial geometry",
      avd_rc_setup_path_kind(1) == 2 && avd_rc_setup_path_kind(2) == 2
    );
    ReportTestResult(
      "Setup path unknown geometry defaults spatial",
      avd_rc_setup_path_kind(42) == 2
    );
    ReportTestResult(
      "Setup logging rectangles only for grid and torus",
      avd_rc_should_log_spatial_rectangles(1) == 1 &&
      avd_rc_should_log_spatial_rectangles(2) == 1 &&
      avd_rc_should_log_spatial_rectangles(0) == 0 &&
      avd_rc_should_log_spatial_rectangles(5) == 0
    );
    ReportTestResult(
      "Resize cell count multiplication policy",
      avd_rc_resize_cell_count(40, 30) == 1200 &&
      avd_rc_resize_cell_count(0, 30) == 0 &&
      avd_rc_resize_cell_count(-2, 7) == -14
    );
    const double global_write_payload = -2.0;
    const double spatial_write_payload = 6.0;
    const auto write_path_select_payload = [&](int geometry) {
      return (avd_rc_setcell_write_path_kind(geometry) == 0) ? global_write_payload : spatial_write_payload;
    };
    ReportTestResult(
      "Write payload policy global geometry",
      fabs(write_path_select_payload(0) - global_write_payload) < 1e-15
    );
    ReportTestResult(
      "Write payload policy partial geometry",
      fabs(write_path_select_payload(5) - global_write_payload) < 1e-15
    );
    ReportTestResult(
      "Write payload policy spatial geometry",
      fabs(write_path_select_payload(1) - spatial_write_payload) < 1e-15
    );
    ReportTestResult(
      "Write payload policy unknown geometry",
      fabs(write_path_select_payload(42) - spatial_write_payload) < 1e-15
    );
    const int expected_gradient_sequence[] = {
      AVD_RC_GRAD_SETTER_PEAK_X,
      AVD_RC_GRAD_SETTER_PEAK_Y,
      AVD_RC_GRAD_SETTER_HEIGHT,
      AVD_RC_GRAD_SETTER_SPREAD,
      AVD_RC_GRAD_SETTER_PLATEAU,
      AVD_RC_GRAD_SETTER_INITIAL_PLAT,
      AVD_RC_GRAD_SETTER_DECAY,
      AVD_RC_GRAD_SETTER_MAX_X,
      AVD_RC_GRAD_SETTER_MAX_Y,
      AVD_RC_GRAD_SETTER_MIN_X,
      AVD_RC_GRAD_SETTER_MIN_Y,
      AVD_RC_GRAD_SETTER_MOVE_SCALER,
      AVD_RC_GRAD_SETTER_UPDATE_STEP,
      AVD_RC_GRAD_SETTER_IS_HALO,
      AVD_RC_GRAD_SETTER_HALO_INNER_RADIUS,
      AVD_RC_GRAD_SETTER_HALO_WIDTH,
      AVD_RC_GRAD_SETTER_HALO_ANCHOR_X,
      AVD_RC_GRAD_SETTER_HALO_ANCHOR_Y,
      AVD_RC_GRAD_SETTER_MOVE_SPEED,
      AVD_RC_GRAD_SETTER_MOVE_RESISTANCE,
      AVD_RC_GRAD_SETTER_PLATEAU_INFLOW,
      AVD_RC_GRAD_SETTER_PLATEAU_OUTFLOW,
      AVD_RC_GRAD_SETTER_CONE_INFLOW,
      AVD_RC_GRAD_SETTER_CONE_OUTFLOW,
      AVD_RC_GRAD_SETTER_GRADIENT_INFLOW,
      AVD_RC_GRAD_SETTER_PLATEAU_COMMON,
      AVD_RC_GRAD_SETTER_FLOOR,
      AVD_RC_GRAD_SETTER_HABITAT,
      AVD_RC_GRAD_SETTER_MIN_SIZE,
      AVD_RC_GRAD_SETTER_MAX_SIZE,
      AVD_RC_GRAD_SETTER_CONFIG,
      AVD_RC_GRAD_SETTER_COUNT,
      AVD_RC_GRAD_SETTER_RESISTANCE,
      AVD_RC_GRAD_SETTER_DAMAGE,
      AVD_RC_GRAD_SETTER_THRESHOLD,
      AVD_RC_GRAD_SETTER_REFUGE,
      AVD_RC_GRAD_SETTER_DEATH_ODDS
    };
    const int expected_gradient_count = sizeof(expected_gradient_sequence) / sizeof(expected_gradient_sequence[0]);
    ReportTestResult(
      "Gradient setter sequence count policy",
      avd_rc_gradient_setter_count() == expected_gradient_count
    );
    bool gradient_sequence_ok = (avd_rc_gradient_setter_count() == expected_gradient_count);
    for (int i = 0; i < expected_gradient_count; ++i) {
      if (avd_rc_gradient_setter_opcode(i) != expected_gradient_sequence[i]) {
        gradient_sequence_ok = false;
        break;
      }
    }
    ReportTestResult("Gradient setter sequence order policy", gradient_sequence_ok);
    ReportTestResult(
      "Gradient setter sequence negative index guard",
      avd_rc_gradient_setter_opcode(-1) == AVD_RC_GRAD_SETTER_INVALID
    );
    ReportTestResult(
      "Gradient setter sequence out-of-range index guard",
      avd_rc_gradient_setter_opcode(avd_rc_gradient_setter_count()) == AVD_RC_GRAD_SETTER_INVALID
    );
    const int expected_gradient_scalar_sequence[] = {
      AVD_RC_GRAD_SCALAR_SETTER_PLATEAU_INFLOW,
      AVD_RC_GRAD_SCALAR_SETTER_PLATEAU_OUTFLOW,
      AVD_RC_GRAD_SCALAR_SETTER_CONE_INFLOW,
      AVD_RC_GRAD_SCALAR_SETTER_CONE_OUTFLOW,
      AVD_RC_GRAD_SCALAR_SETTER_GRADIENT_INFLOW
    };
    const int expected_gradient_scalar_count = sizeof(expected_gradient_scalar_sequence) / sizeof(expected_gradient_scalar_sequence[0]);
    ReportTestResult(
      "Gradient scalar setter sequence count policy",
      avd_rc_gradient_scalar_setter_count() == expected_gradient_scalar_count
    );
    bool gradient_scalar_sequence_ok = (avd_rc_gradient_scalar_setter_count() == expected_gradient_scalar_count);
    for (int i = 0; i < expected_gradient_scalar_count; ++i) {
      if (avd_rc_gradient_scalar_setter_opcode(i) != expected_gradient_scalar_sequence[i]) {
        gradient_scalar_sequence_ok = false;
        break;
      }
    }
    ReportTestResult("Gradient scalar setter sequence order policy", gradient_scalar_sequence_ok);
    ReportTestResult(
      "Gradient scalar setter negative index guard",
      avd_rc_gradient_scalar_setter_opcode(-1) == AVD_RC_GRAD_SCALAR_SETTER_INVALID
    );
    ReportTestResult(
      "Gradient scalar setter out-of-range index guard",
      avd_rc_gradient_scalar_setter_opcode(avd_rc_gradient_scalar_setter_count()) == AVD_RC_GRAD_SCALAR_SETTER_INVALID
    );
    const double scalar_payload[] = {0.11, 0.22, 0.33, 0.44, 0.55};
    double scalar_payload_sum = 0.0;
    for (int i = 0; i < avd_rc_gradient_scalar_setter_count(); ++i) {
      const int opcode = avd_rc_gradient_scalar_setter_opcode(i);
      if (opcode >= 0 && opcode < avd_rc_gradient_scalar_setter_count()) {
        scalar_payload_sum += scalar_payload[opcode];
      }
    }
    ReportTestResult(
      "Gradient scalar setter payload selection parity",
      fabs(scalar_payload_sum - (0.11 + 0.22 + 0.33 + 0.44 + 0.55)) < 1e-15
    );
    const int expected_gradient_var_inflow_sequence[] = {
      AVD_RC_GRAD_VAR_INFLOW_SETTER_PLAT_VAR_INFLOW
    };
    const int expected_gradient_var_inflow_count =
      sizeof(expected_gradient_var_inflow_sequence) / sizeof(expected_gradient_var_inflow_sequence[0]);
    ReportTestResult(
      "Gradient var-inflow setter sequence count policy",
      avd_rc_gradient_var_inflow_setter_count() == expected_gradient_var_inflow_count
    );
    bool gradient_var_inflow_sequence_ok =
      (avd_rc_gradient_var_inflow_setter_count() == expected_gradient_var_inflow_count);
    for (int i = 0; i < expected_gradient_var_inflow_count; ++i) {
      if (avd_rc_gradient_var_inflow_setter_opcode(i) != expected_gradient_var_inflow_sequence[i]) {
        gradient_var_inflow_sequence_ok = false;
        break;
      }
    }
    ReportTestResult(
      "Gradient var-inflow setter sequence order policy",
      gradient_var_inflow_sequence_ok
    );
    ReportTestResult(
      "Gradient var-inflow setter negative index guard",
      avd_rc_gradient_var_inflow_setter_opcode(-1) == AVD_RC_GRAD_VAR_INFLOW_SETTER_INVALID
    );
    ReportTestResult(
      "Gradient var-inflow setter out-of-range index guard",
      avd_rc_gradient_var_inflow_setter_opcode(avd_rc_gradient_var_inflow_setter_count()) ==
        AVD_RC_GRAD_VAR_INFLOW_SETTER_INVALID
    );
    const double var_inflow_payload[] = {0.77};
    double var_inflow_payload_sum = 0.0;
    for (int i = 0; i < avd_rc_gradient_var_inflow_setter_count(); ++i) {
      const int opcode = avd_rc_gradient_var_inflow_setter_opcode(i);
      if (opcode >= 0 && opcode < avd_rc_gradient_var_inflow_setter_count()) {
        var_inflow_payload_sum += var_inflow_payload[opcode];
      }
    }
    ReportTestResult(
      "Gradient var-inflow setter payload selection parity",
      fabs(var_inflow_payload_sum - 0.77) < 1e-15
    );
    const int expected_predatory_sequence[] = {
      AVD_RC_PREDATORY_SETTER_SET_PREDATORY_RESOURCE
    };
    const int expected_predatory_count =
      sizeof(expected_predatory_sequence) / sizeof(expected_predatory_sequence[0]);
    ReportTestResult(
      "Predatory setter sequence count policy",
      avd_rc_predatory_setter_count() == expected_predatory_count
    );
    bool predatory_sequence_ok =
      (avd_rc_predatory_setter_count() == expected_predatory_count);
    for (int i = 0; i < expected_predatory_count; ++i) {
      if (avd_rc_predatory_setter_opcode(i) != expected_predatory_sequence[i]) {
        predatory_sequence_ok = false;
        break;
      }
    }
    ReportTestResult(
      "Predatory setter sequence order policy",
      predatory_sequence_ok
    );
    ReportTestResult(
      "Predatory setter negative index guard",
      avd_rc_predatory_setter_opcode(-1) == AVD_RC_PREDATORY_SETTER_INVALID
    );
    ReportTestResult(
      "Predatory setter out-of-range index guard",
      avd_rc_predatory_setter_opcode(avd_rc_predatory_setter_count()) ==
        AVD_RC_PREDATORY_SETTER_INVALID
    );
    const double predatory_payload[] = {0.91};
    double predatory_payload_sum = 0.0;
    for (int i = 0; i < avd_rc_predatory_setter_count(); ++i) {
      const int opcode = avd_rc_predatory_setter_opcode(i);
      if (opcode >= 0 && opcode < avd_rc_predatory_setter_count()) {
        predatory_payload_sum += predatory_payload[opcode];
      }
    }
    ReportTestResult(
      "Predatory setter payload selection parity",
      fabs(predatory_payload_sum - 0.91) < 1e-15
    );
    const int expected_probabilistic_sequence[] = {
      AVD_RC_PROBABILISTIC_SETTER_SET_PROBABILISTIC_RESOURCE
    };
    const int expected_probabilistic_count =
      sizeof(expected_probabilistic_sequence) / sizeof(expected_probabilistic_sequence[0]);
    ReportTestResult(
      "Probabilistic setter sequence count policy",
      avd_rc_probabilistic_setter_count() == expected_probabilistic_count
    );
    bool probabilistic_sequence_ok =
      (avd_rc_probabilistic_setter_count() == expected_probabilistic_count);
    for (int i = 0; i < expected_probabilistic_count; ++i) {
      if (avd_rc_probabilistic_setter_opcode(i) != expected_probabilistic_sequence[i]) {
        probabilistic_sequence_ok = false;
        break;
      }
    }
    ReportTestResult(
      "Probabilistic setter sequence order policy",
      probabilistic_sequence_ok
    );
    ReportTestResult(
      "Probabilistic setter negative index guard",
      avd_rc_probabilistic_setter_opcode(-1) == AVD_RC_PROBABILISTIC_SETTER_INVALID
    );
    ReportTestResult(
      "Probabilistic setter out-of-range index guard",
      avd_rc_probabilistic_setter_opcode(avd_rc_probabilistic_setter_count()) ==
        AVD_RC_PROBABILISTIC_SETTER_INVALID
    );
    const double probabilistic_payload[] = {1.23};
    double probabilistic_payload_sum = 0.0;
    for (int i = 0; i < avd_rc_probabilistic_setter_count(); ++i) {
      const int opcode = avd_rc_probabilistic_setter_opcode(i);
      if (opcode >= 0 && opcode < avd_rc_probabilistic_setter_count()) {
        probabilistic_payload_sum += probabilistic_payload[opcode];
      }
    }
    ReportTestResult(
      "Probabilistic setter payload selection parity",
      fabs(probabilistic_payload_sum - 1.23) < 1e-15
    );
    const int expected_peak_getter_sequence[] = {
      AVD_RC_PEAK_GETTER_CURR_X,
      AVD_RC_PEAK_GETTER_CURR_Y,
      AVD_RC_PEAK_GETTER_FROZEN_X,
      AVD_RC_PEAK_GETTER_FROZEN_Y
    };
    const int expected_peak_getter_count =
      sizeof(expected_peak_getter_sequence) / sizeof(expected_peak_getter_sequence[0]);
    ReportTestResult(
      "Peak getter sequence count policy",
      avd_rc_peak_getter_count() == expected_peak_getter_count
    );
    bool peak_getter_sequence_ok =
      (avd_rc_peak_getter_count() == expected_peak_getter_count);
    for (int i = 0; i < expected_peak_getter_count; ++i) {
      if (avd_rc_peak_getter_opcode(i) != expected_peak_getter_sequence[i]) {
        peak_getter_sequence_ok = false;
        break;
      }
    }
    ReportTestResult(
      "Peak getter sequence order policy",
      peak_getter_sequence_ok
    );
    ReportTestResult(
      "Peak getter negative index guard",
      avd_rc_peak_getter_opcode(-1) == AVD_RC_PEAK_GETTER_INVALID
    );
    ReportTestResult(
      "Peak getter out-of-range index guard",
      avd_rc_peak_getter_opcode(avd_rc_peak_getter_count()) ==
        AVD_RC_PEAK_GETTER_INVALID
    );
    ReportTestResult(
      "Peak getter update policy current-x",
      avd_rc_peak_getter_requires_update(AVD_RC_PEAK_GETTER_CURR_X) == 1
    );
    ReportTestResult(
      "Peak getter update policy current-y",
      avd_rc_peak_getter_requires_update(AVD_RC_PEAK_GETTER_CURR_Y) == 1
    );
    ReportTestResult(
      "Peak getter update policy frozen-x",
      avd_rc_peak_getter_requires_update(AVD_RC_PEAK_GETTER_FROZEN_X) == 0
    );
    ReportTestResult(
      "Peak getter update policy frozen-y",
      avd_rc_peak_getter_requires_update(AVD_RC_PEAK_GETTER_FROZEN_Y) == 0
    );
    ReportTestResult(
      "Peak getter update policy invalid opcode",
      avd_rc_peak_getter_requires_update(AVD_RC_PEAK_GETTER_INVALID) == 0
    );
    const int peak_payload[] = {101, 202, 303, 404};
    int peak_payload_sum = 0;
    for (int i = 0; i < avd_rc_peak_getter_count(); ++i) {
      const int opcode = avd_rc_peak_getter_opcode(i);
      if (opcode >= 0 && opcode < avd_rc_peak_getter_count()) {
        peak_payload_sum += peak_payload[opcode];
      }
    }
    ReportTestResult(
      "Peak getter payload selection parity",
      peak_payload_sum == (101 + 202 + 303 + 404)
    );
    int (*policy_count_fns[])() = {
      avd_rc_gradient_setter_count,
      avd_rc_gradient_scalar_setter_count,
      avd_rc_gradient_var_inflow_setter_count,
      avd_rc_predatory_setter_count,
      avd_rc_probabilistic_setter_count,
      avd_rc_peak_getter_count
    };
    int (*policy_opcode_fns[])(int) = {
      avd_rc_gradient_setter_opcode,
      avd_rc_gradient_scalar_setter_opcode,
      avd_rc_gradient_var_inflow_setter_opcode,
      avd_rc_predatory_setter_opcode,
      avd_rc_probabilistic_setter_opcode,
      avd_rc_peak_getter_opcode
    };
    const int* policy_expected_sequences[] = {
      expected_gradient_sequence,
      expected_gradient_scalar_sequence,
      expected_gradient_var_inflow_sequence,
      expected_predatory_sequence,
      expected_probabilistic_sequence,
      expected_peak_getter_sequence
    };
    const int policy_expected_counts[] = {
      expected_gradient_count,
      expected_gradient_scalar_count,
      expected_gradient_var_inflow_count,
      expected_predatory_count,
      expected_probabilistic_count,
      expected_peak_getter_count
    };
    const int policy_invalid_opcodes[] = {
      AVD_RC_GRAD_SETTER_INVALID,
      AVD_RC_GRAD_SCALAR_SETTER_INVALID,
      AVD_RC_GRAD_VAR_INFLOW_SETTER_INVALID,
      AVD_RC_PREDATORY_SETTER_INVALID,
      AVD_RC_PROBABILISTIC_SETTER_INVALID,
      AVD_RC_PEAK_GETTER_INVALID
    };
    const double gradient_payload_full[] = {
      0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0,
      1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0,
      2.1, 2.2, 2.3, 2.4, 2.5, 2.6, 2.7, 2.8, 2.9, 3.0,
      3.1, 3.2, 3.3, 3.4, 3.5, 3.6, 3.7
    };
    const double peak_payload_double[] = {101.0, 202.0, 303.0, 404.0};
    const double* policy_payloads[] = {
      gradient_payload_full,
      scalar_payload,
      var_inflow_payload,
      predatory_payload,
      probabilistic_payload,
      peak_payload_double
    };
    const double policy_expected_payload_sums[] = {
      70.3,
      1.65,
      0.77,
      0.91,
      1.23,
      1010.0
    };
    bool unified_policy_matrix_ok = true;
    const int policy_family_count = sizeof(policy_expected_counts) / sizeof(policy_expected_counts[0]);
    for (int family = 0; family < policy_family_count; ++family) {
      const int expected_count = policy_expected_counts[family];
      const int reported_count = policy_count_fns[family]();
      if (reported_count != expected_count) {
        unified_policy_matrix_ok = false;
      }
      if (policy_opcode_fns[family](-1) != policy_invalid_opcodes[family]) {
        unified_policy_matrix_ok = false;
      }
      if (policy_opcode_fns[family](reported_count) != policy_invalid_opcodes[family]) {
        unified_policy_matrix_ok = false;
      }
      double payload_sum = 0.0;
      for (int i = 0; i < expected_count; ++i) {
        const int opcode = policy_opcode_fns[family](i);
        if (opcode != policy_expected_sequences[family][i]) {
          unified_policy_matrix_ok = false;
        }
        if (opcode >= 0 && opcode < expected_count) {
          payload_sum += policy_payloads[family][opcode];
        }
      }
      if (fabs(payload_sum - policy_expected_payload_sums[family]) > 1e-15) {
        unified_policy_matrix_ok = false;
      }
    }
    ReportTestResult(
      "Unified setter/getter opcode matrix policy",
      unified_policy_matrix_ok
    );
    ReportTestResult(
      "Dispatch action non-spatial ignores global-only",
      avd_rc_dispatch_action(0, 1) == 1
    );
    ReportTestResult(
      "Dispatch action spatial runs for non-global-only",
      avd_rc_dispatch_action(1, 0) == 2
    );
    ReportTestResult(
      "Dispatch action spatial skipped for global-only",
      avd_rc_dispatch_action(1, 1) == 0
    );
    ReportTestResult(
      "Dispatch last-updated advances when not global-only",
      avd_rc_should_advance_last_updated(0) == 1
    );
    ReportTestResult(
      "Dispatch last-updated held when global-only",
      avd_rc_should_advance_last_updated(1) == 0
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
    double out_elem1_delta = 0.0;
    double out_elem2_delta = 0.0;
    const int pair_status = avd_src_compute_flow_pair_deltas(
      elem1_amount, elem2_amount, inxdiffuse, inydiffuse, inxgravity, inygravity, xdist, ydist, dist,
      &out_elem1_delta, &out_elem2_delta
    );
    ReportTestResult("Flow pair helper success status", pair_status == 1);
    ReportTestResult("Flow pair delta parity elem1", fabs(out_elem1_delta + expected) < 1e-12);
    ReportTestResult("Flow pair delta parity elem2", fabs(out_elem2_delta - expected) < 1e-12);
    ReportTestResult("Flow pair delta conservation", fabs(out_elem1_delta + out_elem2_delta) < 1e-12);
    ReportTestResult(
      "Flow pair helper null output guard",
      avd_src_compute_flow_pair_deltas(
        elem1_amount, elem2_amount, inxdiffuse, inydiffuse, inxgravity, inygravity, xdist, ydist, dist,
        NULL, &out_elem2_delta
      ) == 0
    );
    double folded_amount = -1.0;
    double folded_delta = -1.0;
    ReportTestResult(
      "State fold helper parity",
      avd_src_state_fold(7.25, -2.0, &folded_amount, &folded_delta) == 1 &&
        fabs(folded_amount - 5.25) < 1e-15 &&
        folded_delta == 0.0
    );
    ReportTestResult(
      "State fold helper null output guard",
      avd_src_state_fold(1.0, 2.0, NULL, &folded_delta) == 0
    );
    const double sum_values[] = {1.5, -2.0, 4.25};
    ReportTestResult(
      "Sum helper parity",
      fabs(avd_src_sum_amounts(sum_values, 3) - 3.75) < 1e-15
    );
    ReportTestResult(
      "Sum helper null input defaults zero",
      avd_src_sum_amounts(NULL, 3) == 0.0
    );
    ReportTestResult(
      "Sum helper zero count defaults zero",
      avd_src_sum_amounts(sum_values, 0) == 0.0
    );
    double next_delta = 0.0;
    ReportTestResult(
      "Rate-next-delta helper parity",
      avd_src_rate_next_delta(1.25, -0.5, &next_delta) == 1 &&
        fabs(next_delta - 0.75) < 1e-15
    );
    ReportTestResult(
      "Rate-next-delta helper null output guard",
      avd_src_rate_next_delta(1.0, 2.0, NULL) == 0
    );
    double reset_amount = 0.0;
    ReportTestResult(
      "Reset-amount helper parity",
      avd_src_reset_amount(2.5, 1.25, &reset_amount) == 1 &&
        fabs(reset_amount - 3.75) < 1e-15
    );
    ReportTestResult(
      "Reset-amount helper null output guard",
      avd_src_reset_amount(1.0, 2.0, NULL) == 0
    );
    double setcell_amount = 0.0;
    double setcell_delta = -1.0;
    ReportTestResult(
      "SetCell apply-initial helper parity",
      avd_src_setcell_apply_initial(3.0, -0.25, 1.5, &setcell_amount, &setcell_delta) == 1 &&
        fabs(setcell_amount - 4.25) < 1e-15 &&
        setcell_delta == 0.0
    );
    ReportTestResult(
      "SetCell apply-initial helper null output guard",
      avd_src_setcell_apply_initial(1.0, 2.0, 3.0, NULL, &setcell_delta) == 0
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
    ReportTestResult(
      "Cell-id strict rejects lower bound underflow",
      avd_src_cell_id_in_bounds_strict(-1, 5) == 0
    );
    ReportTestResult(
      "Cell-id strict allows interior range",
      avd_src_cell_id_in_bounds_strict(4, 5) == 1
    );
    ReportTestResult(
      "Cell-id strict rejects upper bound",
      avd_src_cell_id_in_bounds_strict(5, 5) == 0
    );
    ReportTestResult(
      "Cell-id strict rejects zero-size grid",
      avd_src_cell_id_in_bounds_strict(0, 0) == 0
    );
    ReportTestResult(
      "Cell-id legacy SetCellList allows equal-to-size",
      avd_src_cell_id_in_bounds_legacy_setcell(5, 5) == 1
    );
    ReportTestResult(
      "Cell-id legacy SetCellList rejects above-size",
      avd_src_cell_id_in_bounds_legacy_setcell(6, 5) == 0
    );
    ReportTestResult(
      "Cell-id legacy SetCellList keeps zero-size quirk",
      avd_src_cell_id_in_bounds_legacy_setcell(0, 0) == 1
    );

    const int kGeometryGrid = 1;
    const int kGeometryTorus = 2;
    const int kResourceNone = -99;
    const auto wrap_coord_ref = [](int value, int bound) {
      int rem = value % bound;
      return (rem < 0) ? (rem + bound) : rem;
    };
    const auto slot_delta_ref = [](int slot, int& xdist_ref, int& ydist_ref, double& dist_ref) {
      switch (slot) {
        case 0: xdist_ref = -1; ydist_ref = -1; dist_ref = sqrt(2.0); break;
        case 1: xdist_ref =  0; ydist_ref = -1; dist_ref = 1.0; break;
        case 2: xdist_ref =  1; ydist_ref = -1; dist_ref = sqrt(2.0); break;
        case 3: xdist_ref =  1; ydist_ref =  0; dist_ref = 1.0; break;
        case 4: xdist_ref =  1; ydist_ref =  1; dist_ref = sqrt(2.0); break;
        case 5: xdist_ref =  0; ydist_ref =  1; dist_ref = 1.0; break;
        case 6: xdist_ref = -1; ydist_ref =  1; dist_ref = sqrt(2.0); break;
        case 7: xdist_ref = -1; ydist_ref =  0; dist_ref = 1.0; break;
        default: xdist_ref = 0; ydist_ref = 0; dist_ref = 0.0; break;
      }
    };
    const auto grid_masked_ref = [](int cell_id, int world_x, int world_y, int slot) {
      if (world_x <= 0 || world_y <= 0) return true;
      const int row = cell_id / world_x;
      const int col = cell_id % world_x;
      const bool top = (row == 0);
      const bool bottom = (row == world_y - 1);
      const bool left = (col == 0);
      const bool right = (col == world_x - 1);
      if (top && (slot == 0 || slot == 1 || slot == 2)) return true;
      if (bottom && (slot == 4 || slot == 5 || slot == 6)) return true;
      if (left && (slot == 0 || slot == 7 || slot == 6)) return true;
      if (right && (slot == 2 || slot == 3 || slot == 4)) return true;
      return false;
    };
    const auto expected_setpointer_entry_ref =
      [&](int cell_id, int world_x, int world_y, int geometry, int slot, int& elem_ref, int& xdist_ref, int& ydist_ref, double& dist_ref) {
        slot_delta_ref(slot, xdist_ref, ydist_ref, dist_ref);
        if (geometry == kGeometryGrid && grid_masked_ref(cell_id, world_x, world_y, slot)) {
          elem_ref = kResourceNone;
          xdist_ref = kResourceNone;
          ydist_ref = kResourceNone;
          dist_ref = kResourceNone;
          return;
        }
        const int x = cell_id % world_x;
        const int y = cell_id / world_x;
        const int nx = wrap_coord_ref(x + xdist_ref, world_x);
        const int ny = wrap_coord_ref(y + ydist_ref, world_y);
        elem_ref = ny * world_x + nx;
      };

    struct SetPointerDims {
      int world_x;
      int world_y;
    };
    const SetPointerDims parity_dims[] = {
      {4, 3},
      {1, 1},
      {1, 3},
      {3, 1}
    };
    bool setpointer_grid_ok = true;
    bool setpointer_torus_ok = true;
    for (size_t di = 0; di < sizeof(parity_dims) / sizeof(parity_dims[0]); ++di) {
      const int world_x = parity_dims[di].world_x;
      const int world_y = parity_dims[di].world_y;
      const int num_cells = world_x * world_y;
      for (int cell_id = 0; cell_id < num_cells; ++cell_id) {
        for (int slot = 0; slot < 8; ++slot) {
          int elem = 0;
          int xdelta = 0;
          int ydelta = 0;
          double d = 0.0;
          int exp_elem = 0;
          int exp_xdelta = 0;
          int exp_ydelta = 0;
          double exp_d = 0.0;

          const int ok = avd_src_setpointer_entry(
            cell_id, world_x, world_y, kGeometryGrid, slot, &elem, &xdelta, &ydelta, &d
          );
          if (ok != 1) {
            setpointer_grid_ok = false;
            continue;
          }
          expected_setpointer_entry_ref(
            cell_id, world_x, world_y, kGeometryGrid, slot, exp_elem, exp_xdelta, exp_ydelta, exp_d
          );
          if (elem != exp_elem || xdelta != exp_xdelta || ydelta != exp_ydelta || fabs(d - exp_d) > 1e-12) {
            setpointer_grid_ok = false;
          }

          const int ok_torus = avd_src_setpointer_entry(
            cell_id, world_x, world_y, kGeometryTorus, slot, &elem, &xdelta, &ydelta, &d
          );
          if (ok_torus != 1) {
            setpointer_torus_ok = false;
            continue;
          }
          expected_setpointer_entry_ref(
            cell_id, world_x, world_y, kGeometryTorus, slot, exp_elem, exp_xdelta, exp_ydelta, exp_d
          );
          if (elem != exp_elem || xdelta != exp_xdelta || ydelta != exp_ydelta || fabs(d - exp_d) > 1e-12) {
            setpointer_torus_ok = false;
          }
        }
      }
    }
    ReportTestResult("SetPointers GRID entry parity matrix", setpointer_grid_ok);
    ReportTestResult("SetPointers TORUS entry parity matrix", setpointer_torus_ok);
    int elem = 0;
    int xdelta = 0;
    int ydelta = 0;
    double d = 0.0;
    ReportTestResult(
      "SetPointers entry guard invalid slot",
      avd_src_setpointer_entry(0, 4, 3, kGeometryGrid, 8, &elem, &xdelta, &ydelta, &d) == 0
    );
    ReportTestResult(
      "SetPointers entry guard invalid dimensions",
      avd_src_setpointer_entry(0, 0, 3, kGeometryGrid, 0, &elem, &xdelta, &ydelta, &d) == 0
    );
    ReportTestResult(
      "SetPointers entry guard null output",
      avd_src_setpointer_entry(0, 4, 3, kGeometryGrid, 0, NULL, &xdelta, &ydelta, &d) == 0
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

class cTaskLibRewardHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cTaskLib Reward Helpers"; }
protected:
  static double ReferenceFractionalReward(unsigned int supplied, unsigned int correct)
  {
    unsigned int diff = supplied ^ correct;
    int bit_diff = 0;
    for (int i = 0; i < 32; ++i) {
      bit_diff += (diff & 1u) ? 1 : 0;
      diff >>= 1;
    }
    return static_cast<double>(32 - bit_diff) / 32.0;
  }

  static double ReferenceThresholdHalflifeQuality(long long diff, int threshold, double halflife_arg)
  {
    if (threshold >= 0 && diff > threshold) return 0.0;
    const double halflife = -1.0 * fabs(halflife_arg);
    return pow(2.0, static_cast<double>(diff) / halflife);
  }

  void RunTests()
  {
    struct Case {
      unsigned int supplied;
      unsigned int correct;
      const char* label;
    };
    const Case cases[] = {
      {0u, 0u, "zero/zero"},
      {0xFFFFFFFFu, 0xFFFFFFFFu, "all-ones identity"},
      {0u, 0xFFFFFFFFu, "all bits differ"},
      {0xAAAAAAAAu, 0x55555555u, "alternating complements"},
      {0xDEADBEEFu, 0xDEADBEEFu, "identity value"},
      {0xDEADBEEFu, 0xDEADBEFFu, "small diff"},
      {0x12345678u, 0x87654321u, "mixed diff"}
    };

    bool matrix_ok = true;
    for (size_t i = 0; i < sizeof(cases) / sizeof(cases[0]); ++i) {
      const double got = avd_tasklib_fractional_reward_bits(cases[i].supplied, cases[i].correct);
      const double expected = ReferenceFractionalReward(cases[i].supplied, cases[i].correct);
      if (fabs(got - expected) > 1e-15) {
        matrix_ok = false;
        break;
      }
    }
    ReportTestResult("TaskLib fractional reward matrix parity", matrix_ok);
    ReportTestResult(
      "TaskLib fractional reward identity",
      fabs(avd_tasklib_fractional_reward_bits(0u, 0u) - 1.0) < 1e-15
    );
    ReportTestResult(
      "TaskLib fractional reward one-bit diff",
      fabs(avd_tasklib_fractional_reward_bits(0u, 1u) - (31.0 / 32.0)) < 1e-15
    );
    ReportTestResult(
      "TaskLib fractional reward full diff",
      fabs(avd_tasklib_fractional_reward_bits(0u, 0xFFFFFFFFu) - 0.0) < 1e-15
    );
    const double ab = avd_tasklib_fractional_reward_bits(0x1234ABCDu, 0xABCD1234u);
    const double ba = avd_tasklib_fractional_reward_bits(0xABCD1234u, 0x1234ABCDu);
    ReportTestResult("TaskLib fractional reward symmetry", fabs(ab - ba) < 1e-15);
    ReportTestResult(
      "TaskLib registration-family classifier logic_3",
      avd_tasklib_is_logic3_or_math1_name("logic_3AA") == 1
    );
    ReportTestResult(
      "TaskLib registration-family classifier math_1",
      avd_tasklib_is_logic3_or_math1_name("math_1AF") == 1
    );
    ReportTestResult(
      "TaskLib registration-family classifier non-family",
      avd_tasklib_is_logic3_or_math1_name("math_2AA") == 0 &&
      avd_tasklib_is_logic3_or_math1_name("echo") == 0
    );
    ReportTestResult(
      "TaskLib registration-family classifier null guard",
      avd_tasklib_is_logic3_or_math1_name(NULL) == 0
    );
    ReportTestResult(
      "TaskLib registration-family classifier math_2",
      avd_tasklib_is_math2_or_math3_name("math_2AA") == 1
    );
    ReportTestResult(
      "TaskLib registration-family classifier math_3",
      avd_tasklib_is_math2_or_math3_name("math_3AF") == 1
    );
    ReportTestResult(
      "TaskLib registration-family classifier non-math2math3",
      avd_tasklib_is_math2_or_math3_name("math_1AF") == 0 &&
      avd_tasklib_is_math2_or_math3_name("logic_3AA") == 0
    );
    ReportTestResult(
      "TaskLib registration-family classifier math2math3 null guard",
      avd_tasklib_is_math2_or_math3_name(NULL) == 0
    );
    ReportTestResult(
      "TaskLib registration-family classifier fibonacci",
      avd_tasklib_is_fibonacci_name("fib_7") == 1 &&
      avd_tasklib_is_fibonacci_name("fibonacci_seq") == 0
    );
    ReportTestResult(
      "TaskLib registration-family classifier fibonacci null guard",
      avd_tasklib_is_fibonacci_name(NULL) == 0
    );
    ReportTestResult(
      "TaskLib registration-family classifier matching-sequence",
      avd_tasklib_is_matching_sequence_name("matchstr") == 1 &&
      avd_tasklib_is_matching_sequence_name("sort_inputs") == 1 &&
      avd_tasklib_is_matching_sequence_name("fibonacci_seq") == 1 &&
      avd_tasklib_is_matching_sequence_name("fib_7") == 0
    );
    ReportTestResult(
      "TaskLib registration-family classifier matching-sequence null guard",
      avd_tasklib_is_matching_sequence_name(NULL) == 0
    );
    ReportTestResult(
      "TaskLib registration-family classifier load-based",
      avd_tasklib_is_load_based_name("mult") == 1 &&
      avd_tasklib_is_load_based_name("optimize") == 1 &&
      avd_tasklib_is_load_based_name("eat-target") == 1 &&
      avd_tasklib_is_load_based_name("move-ft") == 1 &&
      avd_tasklib_is_load_based_name("move_to_event") == 0
    );
    ReportTestResult(
      "TaskLib registration-family classifier load-based null guard",
      avd_tasklib_is_load_based_name(NULL) == 0
    );
    ReportTestResult(
      "TaskLib threshold-halflife quality threshold policy",
      fabs(
        avd_tasklib_threshold_halflife_quality(5, 10, 4.0) -
        ReferenceThresholdHalflifeQuality(5, 10, 4.0)
      ) < 1e-15 &&
      avd_tasklib_threshold_halflife_quality(11, 10, 4.0) == 0.0 &&
      fabs(
        avd_tasklib_threshold_halflife_quality(11, -1, 4.0) -
        ReferenceThresholdHalflifeQuality(11, -1, 4.0)
      ) < 1e-15
    );
    const double quality_pos_halflife = avd_tasklib_threshold_halflife_quality(7, -1, 3.0);
    const double quality_neg_halflife = avd_tasklib_threshold_halflife_quality(7, -1, -3.0);
    ReportTestResult(
      "TaskLib threshold-halflife quality sign policy",
      fabs(quality_pos_halflife - quality_neg_halflife) < 1e-15
    );
    ReportTestResult(
      "TaskLib threshold-halflife quality zero-halflife edge policy",
      std::isnan(avd_tasklib_threshold_halflife_quality(0, -1, 0.0)) &&
      avd_tasklib_threshold_halflife_quality(2, -1, 0.0) == 0.0
    );
    ReportTestResult(
      "TaskLib unary math input diff opcode policy",
      avd_tasklib_unary_math_input_diff(0, 0, AVD_TASKLIB_UNARY_OP_LOG, 100000.0) == 0 &&
      avd_tasklib_unary_math_input_diff(1, 0, AVD_TASKLIB_UNARY_OP_LOG2, 100000.0) == 0 &&
      avd_tasklib_unary_math_input_diff(1, 0, AVD_TASKLIB_UNARY_OP_LOG10, 100000.0) == 0 &&
      avd_tasklib_unary_math_input_diff(-9, 3, AVD_TASKLIB_UNARY_OP_SQRT, 100000.0) == 0 &&
      avd_tasklib_unary_math_input_diff(0, 0, AVD_TASKLIB_UNARY_OP_COSINE, 100000.0) == 100000 &&
      avd_tasklib_unary_math_input_diff(1, 0, AVD_TASKLIB_UNARY_OP_SINE, 100000.0) == 0
    );
    ReportTestResult(
      "TaskLib unary math input diff invalid opcode guard",
      avd_tasklib_unary_math_input_diff(1, 0, AVD_TASKLIB_UNARY_OP_INVALID, 100000.0) == LLONG_MAX
    );
    ReportTestResult(
      "TaskLib binary pair input diff opcode policy",
      avd_tasklib_binary_pair_input_diff(3, 4, 11, AVD_TASKLIB_BINARY_OP_MULT) == 1 &&
      avd_tasklib_binary_pair_input_diff(8, 2, 3, AVD_TASKLIB_BINARY_OP_DIV) == 1
    );
    ReportTestResult(
      "TaskLib binary pair input diff guard policy",
      avd_tasklib_binary_pair_input_diff(8, 0, 3, AVD_TASKLIB_BINARY_OP_DIV) == LLONG_MAX &&
      avd_tasklib_binary_pair_input_diff(8, 2, 3, AVD_TASKLIB_BINARY_OP_INVALID) == LLONG_MAX
    );
    const long long diff_seed = avd_tasklib_diff_scan_init();
    ReportTestResult(
      "TaskLib diff-scan reducer init policy",
      diff_seed == 4294967296LL
    );
    ReportTestResult(
      "TaskLib diff-scan reducer update policy",
      avd_tasklib_diff_scan_update(10, 12) == 10 &&
      avd_tasklib_diff_scan_update(10, 7) == 7 &&
      avd_tasklib_diff_scan_update(diff_seed, LLONG_MAX) == diff_seed
    );
  }
};

class cHardwareCPUDispatchPolicyHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cHardwareCPU Dispatch Policy Helpers"; }
protected:
  void RunTests()
  {
    ReportTestResult(
      "CPU dispatch family precedence policy",
      avd_cpu_dispatch_family(1, 1, 1, 1) == AVD_CPU_DISPATCH_FAMILY_STALL &&
      avd_cpu_dispatch_family(1, 1, 1, 0) == AVD_CPU_DISPATCH_FAMILY_PROMOTER &&
      avd_cpu_dispatch_family(1, 1, 0, 0) == AVD_CPU_DISPATCH_FAMILY_LABEL &&
      avd_cpu_dispatch_family(1, 0, 0, 0) == AVD_CPU_DISPATCH_FAMILY_NOP &&
      avd_cpu_dispatch_family(0, 0, 0, 0) == AVD_CPU_DISPATCH_FAMILY_DEFAULT
    );
    ReportTestResult(
      "CPU dispatch family invalid-bit guard",
      avd_cpu_dispatch_family(2, 0, 0, 0) == AVD_CPU_DISPATCH_FAMILY_INVALID &&
      avd_cpu_dispatch_family(0, -1, 0, 0) == AVD_CPU_DISPATCH_FAMILY_INVALID
    );
    ReportTestResult(
      "CPU dispatch counted-opcode identity policy",
      avd_cpu_dispatch_counted_opcode(77, AVD_CPU_DISPATCH_FAMILY_DEFAULT) == 77 &&
      avd_cpu_dispatch_counted_opcode(11, AVD_CPU_DISPATCH_FAMILY_INVALID) == 11 &&
      avd_cpu_dispatch_counted_opcode(3, AVD_CPU_DISPATCH_FAMILY_NOP) == 3
    );

    // Thread change classification
    ReportTestResult(
      "CPU thread change killed-one policy",
      avd_cpu_thread_change_kind(5, 4) == AVD_CPU_THREAD_CHANGE_KILLED_ONE &&
      avd_cpu_thread_change_kind(2, 1) == AVD_CPU_THREAD_CHANGE_KILLED_ONE
    );
    ReportTestResult(
      "CPU thread change divide policy",
      avd_cpu_thread_change_kind(3, 1) == AVD_CPU_THREAD_CHANGE_DIVIDE &&
      avd_cpu_thread_change_kind(10, 1) == AVD_CPU_THREAD_CHANGE_DIVIDE
    );
    ReportTestResult(
      "CPU thread change error policy",
      avd_cpu_thread_change_kind(5, 2) == AVD_CPU_THREAD_CHANGE_ERROR &&
      avd_cpu_thread_change_kind(5, 3) == AVD_CPU_THREAD_CHANGE_ERROR
    );
    ReportTestResult(
      "CPU thread change none policy",
      avd_cpu_thread_change_kind(3, 3) == AVD_CPU_THREAD_CHANGE_NONE &&
      avd_cpu_thread_change_kind(3, 4) == AVD_CPU_THREAD_CHANGE_NONE
    );

    // Max-executed death policy
    ReportTestResult(
      "CPU max-executed death positive",
      avd_cpu_should_die_max_executed(100, 100, 0) == 1 &&
      avd_cpu_should_die_max_executed(100, 200, 0) == 1 &&
      avd_cpu_should_die_max_executed(0, 0, 1) == 1
    );
    ReportTestResult(
      "CPU max-executed death negative",
      avd_cpu_should_die_max_executed(100, 50, 0) == 0 &&
      avd_cpu_should_die_max_executed(0, 999, 0) == 0
    );

    // No-active-promoter exec suppression
    ReportTestResult(
      "CPU no-promoter suppression positive",
      avd_cpu_should_suppress_no_promoter(1, 2, -1) == 1
    );
    ReportTestResult(
      "CPU no-promoter suppression negative",
      avd_cpu_should_suppress_no_promoter(0, 2, -1) == 0 &&
      avd_cpu_should_suppress_no_promoter(1, 1, -1) == 0 &&
      avd_cpu_should_suppress_no_promoter(1, 2, 0) == 0
    );

    // Promoter max-inst termination
    ReportTestResult(
      "CPU promoter termination positive",
      avd_cpu_should_terminate_promoter(10, 10) == 1 &&
      avd_cpu_should_terminate_promoter(10, 15) == 1
    );
    ReportTestResult(
      "CPU promoter termination negative",
      avd_cpu_should_terminate_promoter(0, 100) == 0 &&
      avd_cpu_should_terminate_promoter(10, 5) == 0
    );

    // Task switch penalty
    ReportTestResult(
      "CPU task switch penalty computation",
      avd_cpu_task_switch_penalty(1, 3, 10) == 30 &&
      avd_cpu_task_switch_penalty(0, 3, 10) == 0 &&
      avd_cpu_task_switch_penalty(1, 0, 10) == 0
    );

    // Cardinal direction from gradient
    ReportTestResult(
      "CPU gradient facing all directions",
      avd_cpu_gradient_facing(1, 0) == 0 &&   // N
      avd_cpu_gradient_facing(1, -1) == 1 &&  // NE
      avd_cpu_gradient_facing(0, -1) == 2 &&  // E
      avd_cpu_gradient_facing(-1, -1) == 3 && // SE
      avd_cpu_gradient_facing(-1, 0) == 4 &&  // S
      avd_cpu_gradient_facing(-1, 1) == 5 &&  // SW
      avd_cpu_gradient_facing(0, 1) == 6 &&   // W
      avd_cpu_gradient_facing(1, 1) == 7 &&   // NW
      avd_cpu_gradient_facing(0, 0) == -1     // zero
    );

    // Allocation validity
    ReportTestResult(
      "CPU alloc validity OK and failures",
      avd_cpu_alloc_validity(100, 100, 10, 500, 200, 200) == AVD_CPU_ALLOC_OK &&
      avd_cpu_alloc_validity(0, 100, 10, 500, 200, 200) == AVD_CPU_ALLOC_TOO_SMALL &&
      avd_cpu_alloc_validity(500, 100, 10, 500, 600, 600) == AVD_CPU_ALLOC_OUT_OF_RANGE &&
      avd_cpu_alloc_validity(201, 100, 10, 500, 200, 200) == AVD_CPU_ALLOC_TOO_LARGE &&
      avd_cpu_alloc_validity(50, 201, 10, 500, 300, 200) == AVD_CPU_ALLOC_PARENT_TOO_LARGE
    );

    // Register wrap
    ReportTestResult(
      "CPU next/prev register wrap",
      avd_cpu_next_register(0, 3) == 1 &&
      avd_cpu_next_register(2, 3) == 0 &&
      avd_cpu_prev_register(0, 3) == 2 &&
      avd_cpu_prev_register(2, 3) == 1
    );

    // Unary math domain guard
    ReportTestResult(
      "CPU unary math domain guard",
      avd_cpu_unary_math_domain(5, 2) == AVD_CPU_MATH_COMPUTE &&
      avd_cpu_unary_math_domain(1, 2) == AVD_CPU_MATH_NOOP &&
      avd_cpu_unary_math_domain(-1, 2) == AVD_CPU_MATH_FAULT_NEGATIVE &&
      avd_cpu_unary_math_domain(1, 1) == AVD_CPU_MATH_COMPUTE &&
      avd_cpu_unary_math_domain(0, 1) == AVD_CPU_MATH_NOOP
    );

    // Div/mod guard
    ReportTestResult(
      "CPU div guard policy",
      avd_cpu_div_guard(10, 3, -2147483647) == AVD_CPU_DIV_OK &&
      avd_cpu_div_guard(10, 0, -2147483647) == AVD_CPU_DIV_ZERO &&
      avd_cpu_div_guard(-2147483647, -1, -2147483647) == AVD_CPU_DIV_OVERFLOW
    );
  }
};

class cPopulationActionPolicyHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cPopulationAction Policy Helpers"; }
protected:
  void RunTests()
  {
    ReportTestResult(
      "PopulationAction deme-loop start index policy",
      avd_popaction_deme_loop_start_index(1) == 1 &&
      avd_popaction_deme_loop_start_index(0) == 0 &&
      avd_popaction_deme_loop_start_index(2) == 0
    );
    ReportTestResult(
      "PopulationAction seed-deme action policy",
      avd_popaction_seed_deme_action(1, 0) == AVD_POPACTION_SEED_ACTION_SKIP_AND_COUNT &&
      avd_popaction_seed_deme_action(1, 1) == AVD_POPACTION_SEED_ACTION_PROCEED &&
      avd_popaction_seed_deme_action(0, 0) == AVD_POPACTION_SEED_ACTION_PROCEED
    );
    ReportTestResult(
      "PopulationAction cell-end normalization policy",
      avd_popaction_normalize_cell_end(0, -1) == 1 &&
      avd_popaction_normalize_cell_end(5, -1) == 6 &&
      avd_popaction_normalize_cell_end(5, 9) == 9
    );
    ReportTestResult(
      "PopulationAction cell-range validity policy",
      avd_popaction_is_valid_cell_range(0, 1, 10) == 1 &&
      avd_popaction_is_valid_cell_range(-1, 1, 10) == 0 &&
      avd_popaction_is_valid_cell_range(2, 2, 10) == 0 &&
      avd_popaction_is_valid_cell_range(0, 11, 10) == 0
    );
    ReportTestResult(
      "PopulationAction cell-range-with-stride validity policy",
      avd_popaction_is_valid_cell_range_with_stride(0, 5, 10, 1) == 1 &&
      avd_popaction_is_valid_cell_range_with_stride(0, 5, 10, 0) == 0 &&
      avd_popaction_is_valid_cell_range_with_stride(0, 5, 10, -1) == 0 &&
      avd_popaction_is_valid_cell_range_with_stride(5, 5, 10, 1) == 0
    );
    ReportTestResult(
      "PopulationAction filename-token requiredness policy",
      avd_popaction_is_missing_filename_token(0) == 1 &&
      avd_popaction_is_missing_filename_token(1) == 0 &&
      avd_popaction_is_missing_filename_token(12) == 0 &&
      avd_popaction_is_missing_filename_token(-1) == 0
    );
    ReportTestResult(
      "PopulationAction well-mixed cell-count validity policy",
      avd_popaction_is_valid_well_mixed_cell_count(0, 10) == 1 &&
      avd_popaction_is_valid_well_mixed_cell_count(10, 10) == 1 &&
      avd_popaction_is_valid_well_mixed_cell_count(-1, 10) == 0 &&
      avd_popaction_is_valid_well_mixed_cell_count(11, 10) == 0
    );
    ReportTestResult(
      "PopulationAction group cell-id validity policy",
      avd_popaction_is_valid_group_cell_id(0, 10) == 1 &&
      avd_popaction_is_valid_group_cell_id(9, 10) == 1 &&
      avd_popaction_is_valid_group_cell_id(-1, 10) == 0 &&
      avd_popaction_is_valid_group_cell_id(10, 10) == 0 &&
      avd_popaction_is_valid_group_cell_id(0, 0) == 0 &&
      avd_popaction_is_valid_group_cell_id(7, 10) == avd_popaction_is_valid_single_cell_id(7, 10) &&
      avd_popaction_is_valid_group_cell_id(10, 10) == avd_popaction_is_valid_single_cell_id(10, 10)
    );
    ReportTestResult(
      "PopulationAction single-cell id validity policy",
      avd_popaction_is_valid_single_cell_id(0, 10) == 1 &&
      avd_popaction_is_valid_single_cell_id(9, 10) == 1 &&
      avd_popaction_is_valid_single_cell_id(-1, 10) == 0 &&
      avd_popaction_is_valid_single_cell_id(10, 10) == 0 &&
      avd_popaction_is_valid_single_cell_id(0, 0) == 0
    );
    ReportTestResult(
      "PopulationAction parasite skip gating policy",
      avd_popaction_should_skip_parasite_injection(1, 0) == 0 &&
      avd_popaction_should_skip_parasite_injection(1, 1) == 1 &&
      avd_popaction_should_skip_parasite_injection(0, 1) == 0 &&
      avd_popaction_should_skip_parasite_injection(2, 3) == 1 &&
      avd_popaction_should_skip_parasite_injection(1, -1) == 1
    );
    ReportTestResult(
      "PopulationAction parasite filename requiredness policy",
      avd_popaction_is_missing_parasite_filename_token(0) == 1 &&
      avd_popaction_is_missing_parasite_filename_token(1) == 0 &&
      avd_popaction_is_missing_parasite_filename_token(7) == 0 &&
      avd_popaction_is_missing_parasite_filename_token(-1) == 0
    );
    ReportTestResult(
      "PopulationAction parasite pair filename requiredness policy",
      avd_popaction_has_missing_parasite_pair_filenames(0, 2) == 1 &&
      avd_popaction_has_missing_parasite_pair_filenames(2, 0) == 1 &&
      avd_popaction_has_missing_parasite_pair_filenames(0, 0) == 1 &&
      avd_popaction_has_missing_parasite_pair_filenames(2, 3) == 0
    );
    ReportTestResult(
      "PopulationAction parasite label requiredness policy",
      avd_popaction_is_missing_parasite_label_token(0) == 1 &&
      avd_popaction_is_missing_parasite_label_token(1) == 0 &&
      avd_popaction_is_missing_parasite_label_token(5) == 0 &&
      avd_popaction_is_missing_parasite_label_token(-1) == 0
    );
    ReportTestResult(
      "PopulationAction parasite sequence requiredness policy",
      avd_popaction_is_missing_parasite_sequence_token(0) == 1 &&
      avd_popaction_is_missing_parasite_sequence_token(1) == 0 &&
      avd_popaction_is_missing_parasite_sequence_token(8) == 0 &&
      avd_popaction_is_missing_parasite_sequence_token(-1) == 0
    );
    ReportTestResult(
      "PopulationAction parasite warning selector policy",
      avd_popaction_parasite_invalid_range_warning_kind(AVD_POPACTION_PARASITE_WARNING_ACTION_INJECT_PARASITE) == AVD_POPACTION_PARASITE_WARNING_KIND_INJECT_PARASITE &&
      avd_popaction_parasite_invalid_range_warning_kind(AVD_POPACTION_PARASITE_WARNING_ACTION_INJECT_PARASITE_SEQUENCE) == AVD_POPACTION_PARASITE_WARNING_KIND_INJECT_PARASITE &&
      avd_popaction_parasite_invalid_range_warning_kind(AVD_POPACTION_PARASITE_WARNING_ACTION_INJECT_PARASITE_PAIR) == AVD_POPACTION_PARASITE_WARNING_KIND_INJECT_PARASITE_PAIR &&
      avd_popaction_parasite_invalid_range_warning_kind(AVD_POPACTION_PARASITE_WARNING_ACTION_INVALID) == AVD_POPACTION_PARASITE_WARNING_KIND_INVALID &&
      avd_popaction_parasite_invalid_range_warning_kind(7) == AVD_POPACTION_PARASITE_WARNING_KIND_INVALID
    );
    ReportTestResult(
      "PopulationAction parasite warning short-circuit combiner policy",
      avd_popaction_parasite_warning_short_circuit_kind(AVD_POPACTION_PARASITE_WARNING_ACTION_INJECT_PARASITE, 1) == AVD_POPACTION_PARASITE_WARNING_KIND_INJECT_PARASITE &&
      avd_popaction_parasite_warning_short_circuit_kind(AVD_POPACTION_PARASITE_WARNING_ACTION_INJECT_PARASITE_SEQUENCE, 1) == AVD_POPACTION_PARASITE_WARNING_KIND_INJECT_PARASITE &&
      avd_popaction_parasite_warning_short_circuit_kind(AVD_POPACTION_PARASITE_WARNING_ACTION_INJECT_PARASITE_PAIR, 1) == AVD_POPACTION_PARASITE_WARNING_KIND_INJECT_PARASITE_PAIR &&
      avd_popaction_parasite_warning_short_circuit_kind(AVD_POPACTION_PARASITE_WARNING_ACTION_INVALID, 1) == AVD_POPACTION_PARASITE_WARNING_KIND_INVALID &&
      avd_popaction_parasite_warning_short_circuit_kind(AVD_POPACTION_PARASITE_WARNING_ACTION_INJECT_PARASITE, 0) == AVD_POPACTION_PARASITE_WARNING_KIND_INVALID &&
      avd_popaction_parasite_warning_short_circuit_kind(AVD_POPACTION_PARASITE_WARNING_ACTION_INJECT_PARASITE_PAIR, 0) == AVD_POPACTION_PARASITE_WARNING_KIND_INVALID
    );
    ReportTestResult(
      "PopulationAction parasite missing-token error selector policy",
      avd_popaction_parasite_missing_token_error_kind(AVD_POPACTION_PARASITE_MISSING_TOKEN_FILENAME) == AVD_POPACTION_PARASITE_ERROR_KIND_ORGANISM_FILE &&
      avd_popaction_parasite_missing_token_error_kind(AVD_POPACTION_PARASITE_MISSING_TOKEN_LABEL) == AVD_POPACTION_PARASITE_ERROR_KIND_LABEL &&
      avd_popaction_parasite_missing_token_error_kind(AVD_POPACTION_PARASITE_MISSING_TOKEN_SEQUENCE) == AVD_POPACTION_PARASITE_ERROR_KIND_SEQUENCE &&
      avd_popaction_parasite_missing_token_error_kind(AVD_POPACTION_PARASITE_MISSING_TOKEN_INVALID) == AVD_POPACTION_PARASITE_ERROR_KIND_INVALID &&
      avd_popaction_parasite_missing_token_error_kind(9) == AVD_POPACTION_PARASITE_ERROR_KIND_INVALID
    );
    ReportTestResult(
      "PopulationAction parasite missing-token short-circuit combiner policy",
      avd_popaction_parasite_missing_token_short_circuit_kind(AVD_POPACTION_PARASITE_MISSING_ACTION_INJECT_PARASITE, 1, 1, 1) == AVD_POPACTION_PARASITE_MISSING_TOKEN_FILENAME &&
      avd_popaction_parasite_missing_token_short_circuit_kind(AVD_POPACTION_PARASITE_MISSING_ACTION_INJECT_PARASITE, 0, 1, 0) == AVD_POPACTION_PARASITE_MISSING_TOKEN_LABEL &&
      avd_popaction_parasite_missing_token_short_circuit_kind(AVD_POPACTION_PARASITE_MISSING_ACTION_INJECT_PARASITE_SEQUENCE, 0, 1, 1) == AVD_POPACTION_PARASITE_MISSING_TOKEN_SEQUENCE &&
      avd_popaction_parasite_missing_token_short_circuit_kind(AVD_POPACTION_PARASITE_MISSING_ACTION_INJECT_PARASITE_SEQUENCE, 1, 1, 0) == AVD_POPACTION_PARASITE_MISSING_TOKEN_LABEL &&
      avd_popaction_parasite_missing_token_short_circuit_kind(AVD_POPACTION_PARASITE_MISSING_ACTION_INJECT_PARASITE_PAIR, 1, 0, 0) == AVD_POPACTION_PARASITE_MISSING_TOKEN_FILENAME &&
      avd_popaction_parasite_missing_token_short_circuit_kind(AVD_POPACTION_PARASITE_MISSING_ACTION_INJECT_PARASITE_PAIR, 0, 1, 1) == AVD_POPACTION_PARASITE_MISSING_TOKEN_LABEL &&
      avd_popaction_parasite_missing_token_short_circuit_kind(AVD_POPACTION_PARASITE_MISSING_ACTION_INJECT_PARASITE, 0, 0, 0) == AVD_POPACTION_PARASITE_MISSING_TOKEN_INVALID &&
      avd_popaction_parasite_missing_token_short_circuit_kind(AVD_POPACTION_PARASITE_MISSING_ACTION_INVALID, 1, 1, 1) == AVD_POPACTION_PARASITE_MISSING_TOKEN_INVALID
    );
  }
};

class cPrintActionPolicyHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cPrintAction Policy Helpers"; }
protected:
  void RunTests()
  {
    ReportTestResult(
      "PrintAction instruction filename mode policy",
      avd_printaction_instruction_filename_mode(0, 0) == AVD_PRINTACTION_FILENAME_MODE_DEFAULT_PLAIN &&
      avd_printaction_instruction_filename_mode(1, 0) == AVD_PRINTACTION_FILENAME_MODE_KEEP_PROVIDED &&
      avd_printaction_instruction_filename_mode(1, 1) == AVD_PRINTACTION_FILENAME_MODE_FORMAT_WITH_INSTSET &&
      avd_printaction_instruction_filename_mode(0, 1) == AVD_PRINTACTION_FILENAME_MODE_DEFAULT_PLAIN &&
      avd_printaction_instruction_filename_mode(2, 0) == AVD_PRINTACTION_FILENAME_MODE_KEEP_PROVIDED
    );
    ReportTestResult(
      "PrintAction instruction output sink-selection policy",
      avd_printaction_instruction_output_sink_kind(0) == AVD_PRINTACTION_OUTPUT_SINK_RECORDER &&
      avd_printaction_instruction_output_sink_kind(1) == AVD_PRINTACTION_OUTPUT_SINK_RECORDER &&
      avd_printaction_instruction_output_sink_kind(2) == AVD_PRINTACTION_OUTPUT_SINK_STATS &&
      avd_printaction_instruction_output_sink_kind(8) == AVD_PRINTACTION_OUTPUT_SINK_STATS &&
      avd_printaction_instruction_output_sink_kind(-1) == AVD_PRINTACTION_OUTPUT_SINK_INVALID &&
      avd_printaction_instruction_output_sink_kind(9) == AVD_PRINTACTION_OUTPUT_SINK_INVALID
    );
  }
};

class cPopulationPolicyHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cPopulation Policy Helpers"; }
protected:
  void RunTests()
  {
    ReportTestResult(
      "cPopulation implicit-deme-repro policy",
      avd_cpop_should_check_implicit_deme_repro(-1) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_should_check_implicit_deme_repro(0) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_should_check_implicit_deme_repro(1) == AVD_CPOP_DEME_BLOCK_RUN &&
      avd_cpop_should_check_implicit_deme_repro(2) == AVD_CPOP_DEME_BLOCK_RUN
    );
    ReportTestResult(
      "cPopulation speculative-deme-block policy",
      avd_cpop_should_run_speculative_deme_block(-1) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_should_run_speculative_deme_block(0) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_should_run_speculative_deme_block(1) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_should_run_speculative_deme_block(2) == AVD_CPOP_DEME_BLOCK_RUN
    );
    ReportTestResult(
      "cPopulation deme counter-update policy",
      avd_cpop_should_update_deme_counters(-1) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_should_update_deme_counters(0) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_should_update_deme_counters(1) == AVD_CPOP_DEME_BLOCK_RUN &&
      avd_cpop_should_update_deme_counters(2) == AVD_CPOP_DEME_BLOCK_RUN
    );
    ReportTestResult(
      "cPopulation multi-deme block policy",
      avd_cpop_should_run_multi_deme_block(-1) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_should_run_multi_deme_block(0) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_should_run_multi_deme_block(1) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_should_run_multi_deme_block(2) == AVD_CPOP_DEME_BLOCK_RUN
    );
    ReportTestResult(
      "cPopulation speculative/multi-deme alignment policy",
      avd_cpop_should_run_speculative_deme_block(-1) == avd_cpop_should_run_multi_deme_block(-1) &&
      avd_cpop_should_run_speculative_deme_block(0) == avd_cpop_should_run_multi_deme_block(0) &&
      avd_cpop_should_run_speculative_deme_block(1) == avd_cpop_should_run_multi_deme_block(1) &&
      avd_cpop_should_run_speculative_deme_block(2) == avd_cpop_should_run_multi_deme_block(2)
    );
    ReportTestResult(
      "cPopulation deme-routing short-circuit combiner policy",
      avd_cpop_deme_routing_short_circuit_kind(AVD_CPOP_ROUTING_MODE_PROCESS_STEP, -1) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_deme_routing_short_circuit_kind(AVD_CPOP_ROUTING_MODE_PROCESS_STEP, 1) == AVD_CPOP_DEME_BLOCK_RUN &&
      avd_cpop_deme_routing_short_circuit_kind(AVD_CPOP_ROUTING_MODE_SPECULATIVE_STEP, 1) == AVD_CPOP_DEME_BLOCK_SKIP &&
      avd_cpop_deme_routing_short_circuit_kind(AVD_CPOP_ROUTING_MODE_SPECULATIVE_STEP, 2) == AVD_CPOP_DEME_BLOCK_RUN &&
      avd_cpop_deme_routing_short_circuit_kind(AVD_CPOP_ROUTING_MODE_INVALID, 2) == AVD_CPOP_DEME_BLOCK_SKIP
    );

    // Pred/prey tracking gate
    ReportTestResult(
      "cPopulation pred/prey tracking active policy",
      avd_cpop_is_pred_prey_tracking_active(-2) == 1 &&
      avd_cpop_is_pred_prey_tracking_active(0) == 1 &&
      avd_cpop_is_pred_prey_tracking_active(5) == 1 &&
      avd_cpop_is_pred_prey_tracking_active(-1) == 0 &&
      avd_cpop_is_pred_prey_tracking_active(-3) == 0
    );

    // Forager type classification
    ReportTestResult(
      "cPopulation forager type classification policy",
      avd_cpop_forager_type_kind(1, 0) == AVD_CPOP_FORAGER_TYPE_PREY &&
      avd_cpop_forager_type_kind(1, 1) == AVD_CPOP_FORAGER_TYPE_PREY &&
      avd_cpop_forager_type_kind(0, 1) == AVD_CPOP_FORAGER_TYPE_TOP_PRED &&
      avd_cpop_forager_type_kind(0, 0) == AVD_CPOP_FORAGER_TYPE_PRED
    );

    // Deadly boundary detection
    ReportTestResult(
      "cPopulation deadly boundary edge detection",
      avd_cpop_is_deadly_boundary(1, 1, 0, 5, 10, 10) == 1 &&
      avd_cpop_is_deadly_boundary(1, 1, 5, 0, 10, 10) == 1 &&
      avd_cpop_is_deadly_boundary(1, 1, 9, 5, 10, 10) == 1 &&
      avd_cpop_is_deadly_boundary(1, 1, 5, 9, 10, 10) == 1 &&
      avd_cpop_is_deadly_boundary(1, 1, 5, 5, 10, 10) == 0
    );
    ReportTestResult(
      "cPopulation deadly boundary disabled guard",
      avd_cpop_is_deadly_boundary(0, 1, 0, 0, 10, 10) == 0 &&
      avd_cpop_is_deadly_boundary(1, 2, 0, 0, 10, 10) == 0
    );

    // Prey target exclusion
    ReportTestResult(
      "cPopulation valid prey target policy",
      avd_cpop_is_valid_prey_target(0, -5) == 1 &&
      avd_cpop_is_valid_prey_target(-1, -1) == 1 &&
      avd_cpop_is_valid_prey_target(-1, -2) == 0 &&
      avd_cpop_is_valid_prey_target(-2, 0) == 0
    );

    // Merit bonus gate
    ReportTestResult(
      "cPopulation merit bonus enabled policy",
      avd_cpop_is_merit_bonus_enabled(-1) == 0 &&
      avd_cpop_is_merit_bonus_enabled(0) == 1 &&
      avd_cpop_is_merit_bonus_enabled(5) == 1
    );

    // Phenotype config gates
    ReportTestResult(
      "cPopulation divide method split gate",
      avd_cpop_is_divide_method_split(1) == 1 &&
      avd_cpop_is_divide_method_split(0) == 0 &&
      avd_cpop_is_divide_method_split(2) == 0
    );
    ReportTestResult(
      "cPopulation generation inc both gate",
      avd_cpop_is_generation_inc_both(1) == 1 &&
      avd_cpop_is_generation_inc_both(0) == 0
    );
    ReportTestResult(
      "cPopulation divide method split-or-birth gate",
      avd_cpop_is_divide_method_split_or_birth(1) == 1 &&
      avd_cpop_is_divide_method_split_or_birth(2) == 1 &&
      avd_cpop_is_divide_method_split_or_birth(0) == 0
    );

    // CopyParentFT loophole guard
    ReportTestResult(
      "cPopulation CopyParentFT loophole guard",
      avd_cpop_should_copy_parent_ft(0, 0, -2) == 0 &&
      avd_cpop_should_copy_parent_ft(2, 0, -2) == 0 &&
      avd_cpop_should_copy_parent_ft(1, 0, -2) == 1 &&
      avd_cpop_should_copy_parent_ft(0, -2, -2) == 1 &&
      avd_cpop_should_copy_parent_ft(0, 0, 0) == 1
    );

    // Max-pred kill gate
    ReportTestResult(
      "cPopulation max pred kill gate",
      avd_cpop_should_kill_rand_pred(-2, 50, 50) == 1 &&
      avd_cpop_should_kill_rand_pred(-1, 50, 50) == 0 &&
      avd_cpop_should_kill_rand_pred(-2, 0, 50) == 0
    );

    // Message buffer overflow
    ReportTestResult(
      "cPopulation msg buffer overflow action",
      avd_cpop_msg_buffer_overflow_action(0) == AVD_CPOP_MSG_BUFFER_DROP_OLDEST &&
      avd_cpop_msg_buffer_overflow_action(1) == AVD_CPOP_MSG_BUFFER_DROP_NEW &&
      avd_cpop_msg_buffer_overflow_action(2) == AVD_CPOP_MSG_BUFFER_INVALID
    );
    ReportTestResult(
      "cPopulation msg buffer full check",
      avd_cpop_is_msg_buffer_full(10, 10) == 1 &&
      avd_cpop_is_msg_buffer_full(10, 5) == 0 &&
      avd_cpop_is_msg_buffer_full(-1, 100) == 0
    );

    // Forage target transition
    ReportTestResult(
      "cPopulation forage target transition classification",
      avd_cpop_forage_target_transition(-2, 0) == AVD_CPOP_FT_TRANSITION_PREY_TO_PRED &&
      avd_cpop_forage_target_transition(-2, -3) == AVD_CPOP_FT_TRANSITION_TOP_PRED_TO_PRED &&
      avd_cpop_forage_target_transition(-3, 0) == AVD_CPOP_FT_TRANSITION_PREY_TO_TOP_PRED &&
      avd_cpop_forage_target_transition(-3, -2) == AVD_CPOP_FT_TRANSITION_PRED_TO_TOP_PRED &&
      avd_cpop_forage_target_transition(0, -2) == AVD_CPOP_FT_TRANSITION_PRED_TO_PREY &&
      avd_cpop_forage_target_transition(0, -3) == AVD_CPOP_FT_TRANSITION_TOP_PRED_TO_PREY &&
      avd_cpop_forage_target_transition(0, 1) == AVD_CPOP_FT_TRANSITION_NONE &&
      avd_cpop_forage_target_transition(-2, -2) == AVD_CPOP_FT_TRANSITION_NONE
    );

    // Deme resource reset policy
    ReportTestResult(
      "cPopulation deme reset resources policy",
      avd_cpop_deme_reset_resources_kind(0) == AVD_CPOP_DEME_RESET_BOTH &&
      avd_cpop_deme_reset_resources_kind(1) == AVD_CPOP_DEME_RESET_TARGET_ONLY &&
      avd_cpop_deme_reset_resources_kind(2) == AVD_CPOP_DEME_RESET_NEITHER &&
      avd_cpop_deme_reset_resources_kind(3) == AVD_CPOP_DEME_RESET_INVALID
    );

    // Max prey kill gate
    ReportTestResult(
      "cPopulation max prey kill gate",
      avd_cpop_should_kill_rand_prey(100, 100, 1) == 1 &&
      avd_cpop_should_kill_rand_prey(100, 99, 1) == 0 &&
      avd_cpop_should_kill_rand_prey(0, 100, 1) == 0 &&
      avd_cpop_should_kill_rand_prey(100, 100, 0) == 0
    );

    // Test-birth kill gate
    ReportTestResult(
      "cPopulation test-birth kill gate",
      avd_cpop_should_kill_test_birth(12, 0) == 1 &&
      avd_cpop_should_kill_test_birth(13, 0) == 1 &&
      avd_cpop_should_kill_test_birth(12, 1) == 0 &&
      avd_cpop_should_kill_test_birth(0, 0) == 0
    );
  }
};

class cAnalyzePolicyHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cAnalyze Policy Helpers"; }
protected:
  void RunTests()
  {
    ReportTestResult(
      "Analyze relation mask equals policy",
      avd_analyze_relation_mask("==") == AVD_ANALYZE_REL_MASK_EQUAL
    );
    ReportTestResult(
      "Analyze relation mask not-equals policy",
      avd_analyze_relation_mask("!=") == (AVD_ANALYZE_REL_MASK_LESS | AVD_ANALYZE_REL_MASK_GREATER)
    );
    ReportTestResult(
      "Analyze relation mask boundary policies",
      avd_analyze_relation_mask("<") == AVD_ANALYZE_REL_MASK_LESS &&
      avd_analyze_relation_mask(">") == AVD_ANALYZE_REL_MASK_GREATER &&
      avd_analyze_relation_mask("<=") == (AVD_ANALYZE_REL_MASK_LESS | AVD_ANALYZE_REL_MASK_EQUAL) &&
      avd_analyze_relation_mask(">=") == (AVD_ANALYZE_REL_MASK_EQUAL | AVD_ANALYZE_REL_MASK_GREATER)
    );
    ReportTestResult(
      "Analyze relation mask invalid/null guard",
      avd_analyze_relation_mask("~=") == -1 &&
      avd_analyze_relation_mask(NULL) == -1
    );
    ReportTestResult(
      "Analyze html extension policy",
      avd_analyze_is_html_extension("html") == 1 &&
      avd_analyze_is_html_extension("txt") == 0 &&
      avd_analyze_is_html_extension("HTML") == 0
    );
    ReportTestResult(
      "Analyze html extension null guard",
      avd_analyze_is_html_extension(NULL) == 0
    );
    ReportTestResult(
      "Analyze html filename-token policy",
      avd_analyze_is_html_filename_token("html") == 1 &&
      avd_analyze_is_html_filename_token("txt") == 0 &&
      avd_analyze_is_html_filename_token("HTML") == 0
    );
    ReportTestResult(
      "Analyze html filename-token null guard",
      avd_analyze_is_html_filename_token(NULL) == 0
    );
    ReportTestResult(
      "Analyze output file-type resolution short-circuit kind policy",
      avd_analyze_output_file_type_short_circuit_kind(0) == AVD_ANALYZE_OUTPUT_FILE_TYPE_KIND_KEEP_CURRENT &&
      avd_analyze_output_file_type_short_circuit_kind(1) == AVD_ANALYZE_OUTPUT_FILE_TYPE_KIND_HTML &&
      avd_analyze_output_file_type_short_circuit_kind(2) == AVD_ANALYZE_OUTPUT_FILE_TYPE_KIND_HTML &&
      avd_analyze_output_file_type_short_circuit_kind(-1) == AVD_ANALYZE_OUTPUT_FILE_TYPE_KIND_HTML
    );
    ReportTestResult(
      "Analyze output sink-selection short-circuit kind policy",
      avd_analyze_output_sink_short_circuit_kind(0) == AVD_ANALYZE_OUTPUT_SINK_KIND_FILE &&
      avd_analyze_output_sink_short_circuit_kind(1) == AVD_ANALYZE_OUTPUT_SINK_KIND_COUT &&
      avd_analyze_output_sink_short_circuit_kind(2) == AVD_ANALYZE_OUTPUT_SINK_KIND_COUT &&
      avd_analyze_output_sink_short_circuit_kind(-1) == AVD_ANALYZE_OUTPUT_SINK_KIND_COUT
    );
    ReportTestResult(
      "Analyze output file-handle mode short-circuit kind policy",
      avd_analyze_output_file_handle_mode_short_circuit_kind(AVD_ANALYZE_OUTPUT_HANDLE_ACTION_DETAIL) == AVD_ANALYZE_OUTPUT_HANDLE_MODE_CREATE &&
      avd_analyze_output_file_handle_mode_short_circuit_kind(AVD_ANALYZE_OUTPUT_HANDLE_ACTION_DETAIL_TIMELINE) == AVD_ANALYZE_OUTPUT_HANDLE_MODE_STATIC &&
      avd_analyze_output_file_handle_mode_short_circuit_kind(AVD_ANALYZE_OUTPUT_HANDLE_ACTION_HISTOGRAM) == AVD_ANALYZE_OUTPUT_HANDLE_MODE_STATIC &&
      avd_analyze_output_file_handle_mode_short_circuit_kind(AVD_ANALYZE_OUTPUT_HANDLE_ACTION_INVALID) == AVD_ANALYZE_OUTPUT_HANDLE_MODE_INVALID &&
      avd_analyze_output_file_handle_mode_short_circuit_kind(3) == AVD_ANALYZE_OUTPUT_HANDLE_MODE_INVALID
    );
    ReportTestResult(
      "Analyze output token-presence short-circuit kind policy",
      avd_analyze_output_token_presence_short_circuit_kind(0) == AVD_ANALYZE_OUTPUT_TOKEN_ABSENT &&
      avd_analyze_output_token_presence_short_circuit_kind(1) == AVD_ANALYZE_OUTPUT_TOKEN_PRESENT &&
      avd_analyze_output_token_presence_short_circuit_kind(2) == AVD_ANALYZE_OUTPUT_TOKEN_PRESENT &&
      avd_analyze_output_token_presence_short_circuit_kind(-1) == AVD_ANALYZE_OUTPUT_TOKEN_PRESENT
    );
    ReportTestResult(
      "Analyze file type token policy",
      avd_analyze_apply_file_type_token_policy(0, 0, 30, 10, 20) == 30 &&
      avd_analyze_apply_file_type_token_policy(1, 0, 30, 10, 20) == 10 &&
      avd_analyze_apply_file_type_token_policy(0, 1, 30, 10, 20) == 20 &&
      avd_analyze_apply_file_type_token_policy(1, 1, 30, 10, 20) == 20
    );
    ReportTestResult(
      "Analyze file type token short-circuit kind policy",
      avd_analyze_file_type_token_short_circuit_kind(0, 0) == AVD_ANALYZE_FILE_TYPE_TOKEN_KIND_INVALID &&
      avd_analyze_file_type_token_short_circuit_kind(1, 0) == AVD_ANALYZE_FILE_TYPE_TOKEN_KIND_TEXT &&
      avd_analyze_file_type_token_short_circuit_kind(0, 1) == AVD_ANALYZE_FILE_TYPE_TOKEN_KIND_HTML &&
      avd_analyze_file_type_token_short_circuit_kind(1, 1) == AVD_ANALYZE_FILE_TYPE_TOKEN_KIND_HTML &&
      avd_analyze_file_type_token_short_circuit_kind(2, 0) == AVD_ANALYZE_FILE_TYPE_TOKEN_KIND_TEXT &&
      avd_analyze_file_type_token_short_circuit_kind(0, -1) == AVD_ANALYZE_FILE_TYPE_TOKEN_KIND_HTML
    );
  }
};
class cStatsPolicyHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cStats Policy Helpers"; }
protected:
  void RunTests()
  {
    // Dual task filename classification
    ReportTestResult(
      "Stats dual task filename positive",
      avd_stats_is_dual_task_filename("tasksq.dat") == 1
    );
    ReportTestResult(
      "Stats dual task filename negative",
      avd_stats_is_dual_task_filename("tasks.dat") == 0 &&
      avd_stats_is_dual_task_filename("taskquality.dat") == 0 &&
      avd_stats_is_dual_task_filename("") == 0 &&
      avd_stats_is_dual_task_filename(NULL) == 0
    );
    ReportTestResult(
      "Stats dual internal task filename positive",
      avd_stats_is_dual_internal_task_filename("in_tasksq.dat") == 1
    );
    ReportTestResult(
      "Stats dual internal task filename negative",
      avd_stats_is_dual_internal_task_filename("in_tasks.dat") == 0 &&
      avd_stats_is_dual_internal_task_filename("tasksq.dat") == 0 &&
      avd_stats_is_dual_internal_task_filename(NULL) == 0
    );

    // Spatial resource geometry classification
    ReportTestResult(
      "Stats spatial resource excludes GLOBAL and PARTIAL",
      avd_stats_is_spatial_resource(0) == 0 &&  // GLOBAL
      avd_stats_is_spatial_resource(5) == 0      // PARTIAL
    );
    ReportTestResult(
      "Stats spatial resource includes all spatial geometries",
      avd_stats_is_spatial_resource(1) == 1 &&  // GRID
      avd_stats_is_spatial_resource(2) == 1 &&  // TORUS
      avd_stats_is_spatial_resource(3) == 1 &&  // CLIQUE
      avd_stats_is_spatial_resource(4) == 1 &&  // HEX
      avd_stats_is_spatial_resource(6) == 1 &&  // LATTICE
      avd_stats_is_spatial_resource(7) == 1 &&  // RANDOM_CONNECTED
      avd_stats_is_spatial_resource(8) == 1     // SCALE_FREE
    );

    // Task quality average
    ReportTestResult(
      "Stats task quality average normal",
      avd_stats_task_quality_average(10.0, 5) == 2.0
    );
    ReportTestResult(
      "Stats task quality average zero count",
      avd_stats_task_quality_average(10.0, 0) == 0.0 &&
      avd_stats_task_quality_average(0.0, 0) == 0.0
    );
    ReportTestResult(
      "Stats task quality average negative count guard",
      avd_stats_task_quality_average(10.0, -1) == 0.0
    );

    // Wall gradient resource classification
    ReportTestResult(
      "Stats wall gradient positive",
      avd_stats_is_wall_gradient(1, 2) == 1
    );
    ReportTestResult(
      "Stats wall gradient negative",
      avd_stats_is_wall_gradient(0, 2) == 0 &&
      avd_stats_is_wall_gradient(1, 0) == 0 &&
      avd_stats_is_wall_gradient(1, 3) == 0 &&
      avd_stats_is_wall_gradient(0, 0) == 0
    );

    // Den habitat classification
    ReportTestResult(
      "Stats den habitat positive",
      avd_stats_is_den_habitat(3) == 1 &&
      avd_stats_is_den_habitat(4) == 1
    );
    ReportTestResult(
      "Stats den habitat negative",
      avd_stats_is_den_habitat(0) == 0 &&
      avd_stats_is_den_habitat(1) == 0 &&
      avd_stats_is_den_habitat(2) == 0 &&
      avd_stats_is_den_habitat(5) == 0 &&
      avd_stats_is_den_habitat(-1) == 0
    );
  }
};

class cEnvironmentPolicyHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cEnvironment Policy Helpers"; }
protected:
  void RunTests()
  {
    // Process type classification parity
    ReportTestResult(
      "Env process type known values",
      avd_env_process_type("add") == AVD_ENV_PROCTYPE_ADD &&
      avd_env_process_type("mult") == AVD_ENV_PROCTYPE_MULT &&
      avd_env_process_type("pow") == AVD_ENV_PROCTYPE_POW &&
      avd_env_process_type("lin") == AVD_ENV_PROCTYPE_LIN &&
      avd_env_process_type("energy") == AVD_ENV_PROCTYPE_ENERGY &&
      avd_env_process_type("enzyme") == AVD_ENV_PROCTYPE_ENZYME &&
      avd_env_process_type("exp") == AVD_ENV_PROCTYPE_EXP
    );
    ReportTestResult(
      "Env process type nReaction enum parity",
      AVD_ENV_PROCTYPE_ADD == 0 &&
      AVD_ENV_PROCTYPE_MULT == 1 &&
      AVD_ENV_PROCTYPE_POW == 2 &&
      AVD_ENV_PROCTYPE_LIN == 3 &&
      AVD_ENV_PROCTYPE_ENERGY == 4 &&
      AVD_ENV_PROCTYPE_ENZYME == 5 &&
      AVD_ENV_PROCTYPE_EXP == 6
    );
    ReportTestResult(
      "Env process type unknown and null guard",
      avd_env_process_type("subtract") == AVD_ENV_PROCTYPE_UNKNOWN &&
      avd_env_process_type("") == AVD_ENV_PROCTYPE_UNKNOWN &&
      avd_env_process_type(NULL) == AVD_ENV_PROCTYPE_UNKNOWN
    );
    ReportTestResult(
      "Env process type case sensitivity",
      avd_env_process_type("ADD") == AVD_ENV_PROCTYPE_UNKNOWN &&
      avd_env_process_type("Mult") == AVD_ENV_PROCTYPE_UNKNOWN
    );

    // PhenPlast bonus method classification parity
    ReportTestResult(
      "Env phenplast bonus known values",
      avd_env_phenplast_bonus_method("default") == AVD_ENV_PHENPLAST_DEFAULT &&
      avd_env_phenplast_bonus_method("nobonus") == AVD_ENV_PHENPLAST_NO_BONUS &&
      avd_env_phenplast_bonus_method("fracbonus") == AVD_ENV_PHENPLAST_FRAC_BONUS &&
      avd_env_phenplast_bonus_method("fullbonus") == AVD_ENV_PHENPLAST_FULL_BONUS
    );
    ReportTestResult(
      "Env phenplast bonus ePHENPLAST enum parity",
      AVD_ENV_PHENPLAST_DEFAULT == 0 &&
      AVD_ENV_PHENPLAST_NO_BONUS == 1 &&
      AVD_ENV_PHENPLAST_FRAC_BONUS == 2 &&
      AVD_ENV_PHENPLAST_FULL_BONUS == 3
    );
    ReportTestResult(
      "Env phenplast bonus unknown and null guard",
      avd_env_phenplast_bonus_method("halfbonus") == AVD_ENV_PHENPLAST_UNKNOWN &&
      avd_env_phenplast_bonus_method("") == AVD_ENV_PHENPLAST_UNKNOWN &&
      avd_env_phenplast_bonus_method(NULL) == AVD_ENV_PHENPLAST_UNKNOWN
    );

    // Reaction entry type classification parity
    ReportTestResult(
      "Env reaction entry type known values",
      avd_env_reaction_entry_type("process") == AVD_ENV_ENTRY_TYPE_PROCESS &&
      avd_env_reaction_entry_type("requisite") == AVD_ENV_ENTRY_TYPE_REQUISITE &&
      avd_env_reaction_entry_type("context_requisite") == AVD_ENV_ENTRY_TYPE_CONTEXT_REQUISITE
    );
    ReportTestResult(
      "Env reaction entry type unknown and null guard",
      avd_env_reaction_entry_type("trigger") == AVD_ENV_ENTRY_TYPE_UNKNOWN &&
      avd_env_reaction_entry_type("") == AVD_ENV_ENTRY_TYPE_UNKNOWN &&
      avd_env_reaction_entry_type(NULL) == AVD_ENV_ENTRY_TYPE_UNKNOWN
    );
    ReportTestResult(
      "Env reaction entry type case sensitivity",
      avd_env_reaction_entry_type("PROCESS") == AVD_ENV_ENTRY_TYPE_UNKNOWN &&
      avd_env_reaction_entry_type("Requisite") == AVD_ENV_ENTRY_TYPE_UNKNOWN
    );

    // Gradient temp height
    ReportTestResult(
      "Env gradient temp height policy",
      avd_env_gradient_temp_height(-1.0, 5) == 1 &&
      avd_env_gradient_temp_height(0.0, 5) == 5 &&
      avd_env_gradient_temp_height(1.0, 10) == 10
    );

    // Gradient should-fillin gate
    ReportTestResult(
      "Env gradient should fillin policy",
      avd_env_gradient_should_fillin(2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0) == 1 &&
      avd_env_gradient_should_fillin(1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0) == 1 &&
      avd_env_gradient_should_fillin(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1) == 1 &&
      avd_env_gradient_should_fillin(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0) == 0
    );

    // Gradient update action
    ReportTestResult(
      "Env gradient update action policy",
      avd_env_gradient_update_action(2, 0) == AVD_ENV_GRADIENT_ACTION_BARRIER &&
      avd_env_gradient_update_action(1, 0) == AVD_ENV_GRADIENT_ACTION_HILLS &&
      avd_env_gradient_update_action(0, 1) == AVD_ENV_GRADIENT_ACTION_PROBABILISTIC &&
      avd_env_gradient_update_action(0, 0) == AVD_ENV_GRADIENT_ACTION_PEAK &&
      avd_env_gradient_update_action(2, 1) == AVD_ENV_GRADIENT_ACTION_BARRIER
    );

    // Geometry type classification
    ReportTestResult(
      "Env geometry type known values",
      avd_env_geometry_type("global") == AVD_ENV_GEOMETRY_GLOBAL &&
      avd_env_geometry_type("grid") == AVD_ENV_GEOMETRY_GRID &&
      avd_env_geometry_type("torus") == AVD_ENV_GEOMETRY_TORUS &&
      avd_env_geometry_type("partial") == AVD_ENV_GEOMETRY_PARTIAL
    );
    ReportTestResult(
      "Env geometry type unknown and null guard",
      avd_env_geometry_type("clique") == AVD_ENV_GEOMETRY_UNKNOWN &&
      avd_env_geometry_type("") == AVD_ENV_GEOMETRY_UNKNOWN &&
      avd_env_geometry_type(NULL) == AVD_ENV_GEOMETRY_UNKNOWN
    );

    // Bool-string parser
    ReportTestResult(
      "Env bool string parser known values",
      avd_env_parse_bool_string("true") == AVD_ENV_BOOL_TRUE &&
      avd_env_parse_bool_string("1") == AVD_ENV_BOOL_TRUE &&
      avd_env_parse_bool_string("false") == AVD_ENV_BOOL_FALSE &&
      avd_env_parse_bool_string("0") == AVD_ENV_BOOL_FALSE
    );
    ReportTestResult(
      "Env bool string parser invalid and null guard",
      avd_env_parse_bool_string("yes") == AVD_ENV_BOOL_INVALID &&
      avd_env_parse_bool_string("") == AVD_ENV_BOOL_INVALID &&
      avd_env_parse_bool_string(NULL) == AVD_ENV_BOOL_INVALID
    );
  }
};

class cDemePolicyHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cDeme Policy Helpers"; }
protected:
  void RunTests()
  {
    // Base merit computation
    ReportTestResult(
      "Deme base merit const method",
      avd_deme_base_merit(0, 100.0) == 100.0 &&
      avd_deme_base_merit(0, 0.5) == 0.5
    );
    ReportTestResult(
      "Deme base merit other methods",
      avd_deme_base_merit(1, 100.0) == 1.0 &&
      avd_deme_base_merit(2, 100.0) == 1.0
    );

    // Germline join gate
    ReportTestResult(
      "Deme germline join gate",
      avd_deme_should_join_germline_first(0) == 1 &&
      avd_deme_should_join_germline_first(6) == 1 &&
      avd_deme_should_join_germline_first(7) == 0 &&
      avd_deme_should_join_germline_first(8) == 0 &&
      avd_deme_should_join_germline_first(9) == 1
    );

    // Reaction weight
    ReportTestResult(
      "Deme reaction weight with slope",
      avd_deme_reaction_weight(2.0, 3) == 6.0 &&
      avd_deme_reaction_weight(0.5, 4) == 2.0 &&
      avd_deme_reaction_weight(2.0, 0) == 0.0
    );
    ReportTestResult(
      "Deme reaction weight without slope",
      avd_deme_reaction_weight(0.0, 3) == 1.0 &&
      avd_deme_reaction_weight(-1.0, 3) == 1.0
    );
  }
};

class cOrgSensorPolicyHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "cOrgSensor Policy Helpers"; }
protected:
  void RunTests()
  {
    // Search type normalization
    ReportTestResult(
      "Sensor search type env resource default",
      avd_sensor_normalize_search_type(0, -1, 0, 0) == 0 &&
      avd_sensor_normalize_search_type(0, 2, 0, 0) == 0 &&
      avd_sensor_normalize_search_type(0, 0, 0, 0) == 0 &&
      avd_sensor_normalize_search_type(0, 1, 0, 0) == 1
    );
    ReportTestResult(
      "Sensor search type prey in pred experiment",
      avd_sensor_normalize_search_type(-2, -3, 1, 0) == 1 &&
      avd_sensor_normalize_search_type(-2, 3, 1, 0) == 1 &&
      avd_sensor_normalize_search_type(-2, 2, 1, 0) == 2
    );
    ReportTestResult(
      "Sensor search type predator in pred experiment",
      avd_sensor_normalize_search_type(-2, -3, 1, 1) == -1 &&
      avd_sensor_normalize_search_type(-2, 3, 1, 1) == -1 &&
      avd_sensor_normalize_search_type(-2, 0, 1, 1) == 0
    );
    ReportTestResult(
      "Sensor search type non-pred experiment",
      avd_sensor_normalize_search_type(-2, 1, 0, 0) == 0 &&
      avd_sensor_normalize_search_type(-2, -1, 0, 0) == -1
    );

    // Distance clamping
    ReportTestResult(
      "Sensor distance clamping",
      avd_sensor_clamp_distance(-1, 10) == 1 &&
      avd_sensor_clamp_distance(15, 10) == 10 &&
      avd_sensor_clamp_distance(5, 10) == 5
    );

    // Max distance
    ReportTestResult(
      "Sensor max distance computation",
      avd_sensor_max_distance(-1, 60, 40) == 30 &&
      avd_sensor_max_distance(20, 60, 40) == 20 &&
      avd_sensor_max_distance(100, 60, 40) == 60
    );

    // ID sought clamp
    ReportTestResult(
      "Sensor id sought clamping",
      avd_sensor_clamp_id_sought(-2) == -1 &&
      avd_sensor_clamp_id_sought(-1) == -1 &&
      avd_sensor_clamp_id_sought(0) == 0 &&
      avd_sensor_clamp_id_sought(5) == 5
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
  TEST(cTaskLibRewardHelper);
  TEST(cHardwareCPUDispatchPolicyHelper);
  TEST(cPopulationActionPolicyHelper);
  TEST(cPrintActionPolicyHelper);
  TEST(cPopulationPolicyHelper);
  TEST(cAnalyzePolicyHelper);
  TEST(cEnvironmentPolicyHelper);
  TEST(cStatsPolicyHelper);
  TEST(cDemePolicyHelper);
  TEST(cOrgSensorPolicyHelper);

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
