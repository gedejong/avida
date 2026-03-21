/*
 *  tMatrix.h
 *  Avida
 *
 *  Called "tMatrix.hh" prior to 12/7/05.
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

#ifndef tMatrix_h
#define tMatrix_h

#include "AvidaArray.h"

#include <cassert>

/**
 * This class provides a matrix template.
 **/

template <class T> class tMatrix {
protected:
  AvidaArray<AvidaArray<T> > data;

public:
  int GetNumRows() const { return data.GetSize(); }
  int GetNumCols() const { return data.GetSize() > 0 ? data[0].GetSize() : 0; }

  void ResizeClear(const int _rows, const int _cols)
  {
    data.Resize(_rows);
    for (int i = 0; i < _rows; i++) data[i].ResizeClear(_cols);
  }

  void Resize(int _rows, int _cols)
  {
    data.Resize(_rows);
    for (int i = 0; i < _rows; i++) data[i].Resize(_cols);
  }

  T & ElementAt(int _row, int _col) { return data[_row][_col]; }
  const T & ElementAt(int _row, int _col) const { return data[_row][_col]; }

        T & operator()(int _r, int _c)       { return ElementAt(_r, _c); }
  const T & operator()(int _r, int _c) const { return ElementAt(_r, _c); }

        AvidaArray<T> & operator[](int row)       { return data[row]; }
  const AvidaArray<T> & operator[](int row) const { return data[row]; }

  void SetAll(const T & value) {
    for (int i = 0; i < data.GetSize(); i++) {
      data[i].SetAll(value);
    }
  }

public:
  explicit tMatrix() { ResizeClear(1,1); }
  explicit tMatrix(const int _rows, const int _cols) { ResizeClear(_rows, _cols); }
  tMatrix(const tMatrix& rhs) : data(rhs.data) { }

  tMatrix& operator= (const tMatrix<T>& rhs) {
    if (GetNumRows() != rhs.GetNumRows() || GetNumCols() != rhs.GetNumCols()) {
      ResizeClear(rhs.GetNumRows(), rhs.GetNumCols());
    }
    for (int row = 0; row < GetNumRows(); row++) {
      for (int col = 0; col < GetNumCols(); col++) {
        data[row][col] = rhs.data[row][col];
      }
    }
    return *this;
  }

  virtual ~tMatrix() { }
};

#endif
