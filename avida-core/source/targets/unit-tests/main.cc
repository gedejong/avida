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

    cTimeSeriesRecorderDouble loaded(data_id, "3:2.500000,5:4.000000");
    ReportTestResult("String Constructor Parses Count", (loaded.NumPoints() == 2));
    ReportTestResult("String Constructor Parses Data", (fabs(loaded.DataPoint(0) - 2.5) < 1e-12 && fabs(loaded.DataPoint(1) - 4.0) < 1e-12));
    ReportTestResult("String Constructor Parses Time", (loaded.DataTime(0) == 3 && loaded.DataTime(1) == 5));
    ReportTestResult("String Constructor Roundtrip", (loaded.AsString() == "3:2.500000,5:4.000000"));
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
    cMockArgumentedProvider provider;
    Avida::Data::PackagePtr pkg;

    pkg = provider.GetProvidedValue("demo[]");
    ReportTestResult("Argumented empty argument dispatch", pkg && provider.last_id == "demo[]" && provider.last_arg == "" && pkg->IntValue() == 0);

    pkg = provider.GetProvidedValue("demo[value]");
    ReportTestResult("Argumented parse dispatch", pkg && provider.last_id == "demo[]" && provider.last_arg == "value" && pkg->IntValue() == 5);

    pkg = provider.GetProvidedValue("demo]");
    ReportTestResult("Malformed argumented id returns null", !pkg);
  }
};

class cManagerDataIdHelperTests : public cUnitTest
{
public:
  const char* GetUnitName() { return "Data::Manager ID Helpers"; }
protected:
  void RunTests()
  {
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
  TEST(cTimeSeriesRecorder);
  TEST(cProvider);
  TEST(cManagerDataIdHelper);
  
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
  
  delete str;
}
