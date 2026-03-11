/*
 *  data/TimeSeriesRecorder.cc
 *  avida-core
 *
 *  Created by David on 5/20/11.
 *  Copyright 2011 Michigan State University. All rights reserved.
 *  http://avida.devosoft.org/
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
 *  Authors: David M. Bryson <david@programerror.com>
 *
 */

#include "avida/data/TimeSeriesRecorder.h"

#include "avida/data/Package.h"
#include "rust/running_stats_ffi.h"

namespace {
Apto::String RustOwnedStringToApto(char* raw)
{
  if (!raw) return Apto::String();
  Apto::String value(raw);
  avd_tsr_string_free(raw);
  return value;
}
}

namespace Avida {
  namespace Data {
    
    template <>
    TimeSeriesRecorder<PackagePtr>::TimeSeriesRecorder(const DataID& data_id)
      : m_data_id(data_id)
      , m_rust_handle(avd_tsr_new())
    {
      DataSetPtr ds(new DataSet);
      ds->Insert(m_data_id);
      m_requested = ds;
    }

    template <>
    TimeSeriesRecorder<bool>::TimeSeriesRecorder(const DataID& data_id)
      : m_data_id(data_id)
      , m_rust_handle(avd_tsr_new())
    {
      DataSetPtr ds(new DataSet);
      ds->Insert(m_data_id);
      m_requested = ds;
    }
    
    template <>
    TimeSeriesRecorder<int>::TimeSeriesRecorder(const DataID& data_id)
      : m_data_id(data_id)
      , m_rust_handle(avd_tsr_new())
    {
      DataSetPtr ds(new DataSet);
      ds->Insert(m_data_id);
      m_requested = ds;
    }

    template <>
    TimeSeriesRecorder<double>::TimeSeriesRecorder(const DataID& data_id)
      : m_data_id(data_id)
      , m_rust_handle(avd_tsr_new())
    {
      DataSetPtr ds(new DataSet);
      ds->Insert(m_data_id);
      m_requested = ds;
    }

    template <>
    TimeSeriesRecorder<Apto::String>::TimeSeriesRecorder(const DataID& data_id)
      : m_data_id(data_id)
      , m_rust_handle(avd_tsr_new())
    {
      DataSetPtr ds(new DataSet);
      ds->Insert(m_data_id);
      m_requested = ds;
    }
    
    
    template <>
    TimeSeriesRecorder<PackagePtr>::TimeSeriesRecorder(const DataID& data_id, Apto::String str)
      : m_data_id(data_id)
      , m_rust_handle(avd_tsr_from_string((const char*) str))
    {
      DataSetPtr ds(new DataSet);
      ds->Insert(m_data_id);
      m_requested = ds;
    }
    
    template <>
    TimeSeriesRecorder<bool>::TimeSeriesRecorder(const DataID& data_id, Apto::String str)
      : m_data_id(data_id)
      , m_rust_handle(avd_tsr_from_string((const char*) str))
    {
      DataSetPtr ds(new DataSet);
      ds->Insert(m_data_id);
      m_requested = ds;
    }
    
    template <>
    TimeSeriesRecorder<int>::TimeSeriesRecorder(const DataID& data_id, Apto::String str)
      : m_data_id(data_id)
      , m_rust_handle(avd_tsr_from_string((const char*) str))
    {
      DataSetPtr ds(new DataSet);
      ds->Insert(m_data_id);
      m_requested = ds;
    }
    
    template <>
    TimeSeriesRecorder<double>::TimeSeriesRecorder(const DataID& data_id, Apto::String str)
      : m_data_id(data_id)
      , m_rust_handle(avd_tsr_from_string((const char*) str))
    {
      DataSetPtr ds(new DataSet);
      ds->Insert(m_data_id);
      m_requested = ds;
    }
    
    template <>
    TimeSeriesRecorder<Apto::String>::TimeSeriesRecorder(const DataID& data_id, Apto::String str)
      : m_data_id(data_id)
      , m_rust_handle(avd_tsr_from_string((const char*) str))
    {
      DataSetPtr ds(new DataSet);
      ds->Insert(m_data_id);
      m_requested = ds;
    }
    
    template <class T>
    TimeSeriesRecorder<T>::~TimeSeriesRecorder()
    {
      avd_tsr_free(m_rust_handle);
      m_rust_handle = NULL;
    }

    template <class T>
    int TimeSeriesRecorder<T>::NumPoints() const
    {
      return avd_tsr_len(m_rust_handle);
    }

    template <class T>
    Update TimeSeriesRecorder<T>::DataTime(int idx) const
    {
      return avd_tsr_update_at(m_rust_handle, idx);
    }
    

    
    template <>
    void TimeSeriesRecorder<PackagePtr>::NotifyData(Update update, DataRetrievalFunctor retrieve_data)
    {
      if (shouldRecordValue(update)) {
        PackagePtr value = retrieve_data(m_data_id);
        avd_tsr_push_string(m_rust_handle, update, (const char*) value->StringValue());
        didRecordValue();
      }
    }
    
    
    template <>
    void TimeSeriesRecorder<bool>::NotifyData(Update update, DataRetrievalFunctor retrieve_data)
    {
      if (shouldRecordValue(update)) {
        bool value = retrieve_data(m_data_id)->BoolValue();
        avd_tsr_push_bool(m_rust_handle, update, value ? 1 : 0);
        didRecordValue();
      }
    }
    
    template <>
    void TimeSeriesRecorder<int>::NotifyData(Update update, DataRetrievalFunctor retrieve_data)
    {
      if (shouldRecordValue(update)) {
        int value = retrieve_data(m_data_id)->IntValue();
        avd_tsr_push_int(m_rust_handle, update, value);
        didRecordValue();
      }
    }
    
    template <>
    void TimeSeriesRecorder<double>::NotifyData(Update update, DataRetrievalFunctor retrieve_data)
    {
      if (shouldRecordValue(update)) {
        double value = retrieve_data(m_data_id)->DoubleValue();
        avd_tsr_push_double(m_rust_handle, update, value);
        didRecordValue();
      }
    }

    template <>
    void TimeSeriesRecorder<Apto::String>::NotifyData(Update update, DataRetrievalFunctor retrieve_data)
    {
      if (shouldRecordValue(update)) {
        Apto::String value = retrieve_data(m_data_id)->StringValue();
        avd_tsr_push_string(m_rust_handle, update, (const char*) value);
        didRecordValue();
      }
    }

    template <>
    PackagePtr TimeSeriesRecorder<PackagePtr>::DataPoint(int idx) const
    {
      Apto::String value = RustOwnedStringToApto(avd_tsr_value_as_cstr(m_rust_handle, idx));
      return PackagePtr(new Wrap<Apto::String>(value));
    }

    template <>
    bool TimeSeriesRecorder<bool>::DataPoint(int idx) const
    {
      int value = 0;
      if (avd_tsr_value_as_bool(m_rust_handle, idx, &value) == 0) return false;
      return value != 0;
    }

    template <>
    int TimeSeriesRecorder<int>::DataPoint(int idx) const
    {
      int value = 0;
      if (avd_tsr_value_as_int(m_rust_handle, idx, &value) == 0) return 0;
      return value;
    }

    template <>
    double TimeSeriesRecorder<double>::DataPoint(int idx) const
    {
      double value = 0.0;
      if (avd_tsr_value_as_double(m_rust_handle, idx, &value) == 0) return 0.0;
      return value;
    }

    template <>
    Apto::String TimeSeriesRecorder<Apto::String>::DataPoint(int idx) const
    {
      return RustOwnedStringToApto(avd_tsr_value_as_cstr(m_rust_handle, idx));
    }
    
    
    template <>
    Apto::String TimeSeriesRecorder<PackagePtr>::AsString() const
    {
      return RustOwnedStringToApto(avd_tsr_as_string(m_rust_handle));
    }

    template <>
    Apto::String TimeSeriesRecorder<bool>::AsString() const
    {
      return RustOwnedStringToApto(avd_tsr_as_string(m_rust_handle));
    }

    template <>
    Apto::String TimeSeriesRecorder<int>::AsString() const
    {
      return RustOwnedStringToApto(avd_tsr_as_string(m_rust_handle));
    }

    template <>
    Apto::String TimeSeriesRecorder<double>::AsString() const
    {
      return RustOwnedStringToApto(avd_tsr_as_string(m_rust_handle));
    }
  
    template <>
    Apto::String TimeSeriesRecorder<Apto::String>::AsString() const
    {
      return RustOwnedStringToApto(avd_tsr_as_string(m_rust_handle));
    }
};
};


// Explicitly instantiate classes
template class Avida::Data::TimeSeriesRecorder<Avida::Data::PackagePtr>;
template class Avida::Data::TimeSeriesRecorder<bool>;
template class Avida::Data::TimeSeriesRecorder<int>;
template class Avida::Data::TimeSeriesRecorder<double>;
template class Avida::Data::TimeSeriesRecorder<Apto::String>;

// Explicitly instantiate member functions   @DMB disabled, c++0x extension warning under Apple LLVM 3.0
//template void Avida::Data::TimeSeriesRecorder<Avida::Data::PackagePtr>::NotifyData(Update, DataRetrievalFunctor);
//template void Avida::Data::TimeSeriesRecorder<bool>::NotifyData(Update, DataRetrievalFunctor);
//template void Avida::Data::TimeSeriesRecorder<int>::NotifyData(Update, DataRetrievalFunctor);
//template void Avida::Data::TimeSeriesRecorder<double>::NotifyData(Update, DataRetrievalFunctor);
//template void Avida::Data::TimeSeriesRecorder<Apto::String>::NotifyData(Update, DataRetrievalFunctor);
