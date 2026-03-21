/*
 *  AvidaArray.h
 *  Avida
 *
 *  Compatibility shim: std::vector with Apto::Array API.
 *  Used during migration from Apto containers to C++ standard library.
 *  Drop-in replacement for Apto::Array<T> and Apto::Array<T, Apto::Smart>.
 *
 */

#ifndef AvidaArray_h
#define AvidaArray_h

#include <algorithm>
#include <cassert>
#include <cstring>
#include <vector>

#include "apto/core/Array.h"


template <class T>
class AvidaArray
{
private:
  std::vector<T> m_data;

public:
  typedef T ValueType;
  AvidaArray() {}
  explicit AvidaArray(int size) : m_data(size) {}
  AvidaArray(int size, const T& value) : m_data(size, value) {}
  AvidaArray(const AvidaArray& rhs) : m_data(rhs.m_data) {}

  // Construct from Apto::Array for compatibility during migration
  template <template <class> class SP>
  AvidaArray(const Apto::Array<T, SP>& rhs) : m_data(rhs.GetSize()) {
    for (int i = 0; i < static_cast<int>(m_data.size()); i++) m_data[i] = rhs[i];
  }

  template <class T2>
  explicit AvidaArray(const AvidaArray<T2>& rhs) : m_data(rhs.GetSize()) {
    for (int i = 0; i < GetSize(); i++) m_data[i] = static_cast<T>(rhs[i]);
  }

  AvidaArray& operator=(const AvidaArray& rhs) { m_data = rhs.m_data; return *this; }

  template <class T2>
  AvidaArray& operator=(const AvidaArray<T2>& rhs) {
    m_data.resize(rhs.GetSize());
    for (int i = 0; i < GetSize(); i++) m_data[i] = static_cast<T>(rhs[i]);
    return *this;
  }

  int GetSize() const { return static_cast<int>(m_data.size()); }

  void Resize(int new_size) { m_data.resize(new_size); }
  void Resize(int new_size, const T& value) { m_data.resize(new_size, value); }
  void ResizeClear(int new_size) { m_data.assign(new_size, T()); }

  void SetAll(const T& value) { std::fill(m_data.begin(), m_data.end(), value); }

  void SetReserve(int size) { m_data.reserve(size); }

  void Push(const T& value) { m_data.push_back(value); }
  T Pop() { T val = m_data.back(); m_data.pop_back(); return val; }

  void Swap(int i, int j) { std::swap(m_data[i], m_data[j]); }

  // Apto::Array compatibility aliases
  T Get(int index) const { return m_data[index]; }
  void Set(int index, const T& value) { m_data[index] = value; }

  bool operator==(const AvidaArray& rhs) const { return m_data == rhs.m_data; }
  bool operator!=(const AvidaArray& rhs) const { return m_data != rhs.m_data; }

  typename std::vector<T>::reference operator[](int index) { return m_data[index]; }
  typename std::vector<T>::const_reference operator[](int index) const { return m_data[index]; }

  // Implicit conversion to Apto::Array for compatibility during migration
  template <template <class> class SP>
  operator Apto::Array<T, SP>() const {
    Apto::Array<T, SP> result(GetSize());
    for (int i = 0; i < GetSize(); i++) result[i] = m_data[i];
    return result;
  }

  // Assignment from Apto::Array for compatibility during migration
  template <template <class> class SP>
  AvidaArray& operator=(const Apto::Array<T, SP>& rhs) {
    m_data.resize(rhs.GetSize());
    for (int i = 0; i < GetSize(); i++) m_data[i] = rhs[i];
    return *this;
  }

  // Concatenation operators (matches Apto::Array semantics)
  AvidaArray operator+(const AvidaArray& rhs) const {
    AvidaArray result(GetSize() + rhs.GetSize());
    for (int i = 0; i < GetSize(); i++) result[i] = m_data[i];
    for (int i = 0; i < rhs.GetSize(); i++) result[GetSize() + i] = rhs[i];
    return result;
  }

  AvidaArray& operator+=(const AvidaArray& rhs) {
    int old_size = GetSize();
    m_data.resize(old_size + rhs.GetSize());
    for (int i = 0; i < rhs.GetSize(); i++) m_data[old_size + i] = rhs[i];
    return *this;
  }

  // Raw data access for FFI bridging
  T* GetData() { return m_data.data(); }
  const T* GetData() const { return m_data.data(); }

  // Iterator support
  typename std::vector<T>::iterator begin() { return m_data.begin(); }
  typename std::vector<T>::iterator end() { return m_data.end(); }
  typename std::vector<T>::const_iterator begin() const { return m_data.begin(); }
  typename std::vector<T>::const_iterator end() const { return m_data.end(); }
};

#endif
