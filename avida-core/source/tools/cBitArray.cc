#include "cBitArray.h"

void cRawBitArray::Copy(const cRawBitArray & in_array, const int num_bits)
{
  (void) num_bits;
  if (m_handle != NULL) {
    avd_rba_free(m_handle);
  }
  m_handle = avd_rba_clone(in_array.m_handle);
  assert(m_handle != NULL);
}


bool cRawBitArray::IsEqual(const cRawBitArray & in_array, int num_bits) const
{
  return avd_rba_is_equal(m_handle, in_array.m_handle, num_bits) != 0;
}


void cRawBitArray::Resize(const int old_bits, const int new_bits)
{
  avd_rba_resize(m_handle, old_bits, new_bits);
}


void cRawBitArray::ResizeSloppy(const int new_bits)
{
  if (m_handle != NULL) {
    avd_rba_free(m_handle);
  }
  m_handle = avd_rba_new(new_bits);
  assert(m_handle != NULL);
}

void cRawBitArray::ResizeClear(const int new_bits)
{
  ResizeSloppy(new_bits);
  Zero(new_bits);
}


// This technique counts the number of bits; it loops through once for each
// bit equal to 1.  This is reasonably fast for sparse arrays.
int cRawBitArray::CountBits(const int num_bits) const
{
  return avd_rba_count_bits(m_handle, num_bits);
}

// This technique is another way of counting bits; It does a bunch of
// clever bit tricks to do it in parallel in each int.
int cRawBitArray::CountBits2(const int num_bits) const
{
  return avd_rba_count_bits2(m_handle, num_bits);
}

int cRawBitArray::FindBit1(const int num_bits, const int start_pos) const
{
  return avd_rba_find_bit1(m_handle, num_bits, start_pos);
}

AvidaArray<int> cRawBitArray::GetOnes(const int num_bits) const
{
  // @CAO -- There are probably better ways to do this with bit tricks.
  AvidaArray<int> out_array(CountBits2(num_bits));
  int cur_pos = 0;
  for (int i = 0; i < num_bits; i++) {
    if (GetBit(i) == true) out_array[cur_pos++] = i;
  }

  return out_array;
}

void cRawBitArray::ShiftLeft(const int num_bits, const int shift_size)
{
  assert(shift_size > 0);
  avd_rba_shift(m_handle, num_bits, shift_size);
}

// ALWAYS shifts in zeroes, irrespective of sign bit (since fields are unsigned)
void cRawBitArray::ShiftRight(const int num_bits, const int shift_size)
{
  assert(shift_size > 0);
  avd_rba_shift(m_handle, num_bits, -shift_size);
}

void cRawBitArray::NOT(const int num_bits)
{
  avd_rba_not(m_handle, num_bits);
}

void cRawBitArray::AND(const cRawBitArray & array2, const int num_bits)
{
  avd_rba_and(m_handle, array2.m_handle, num_bits);
}

void cRawBitArray::OR(const cRawBitArray & array2, const int num_bits)
{
  avd_rba_or(m_handle, array2.m_handle, num_bits);
}

void cRawBitArray::NAND(const cRawBitArray & array2, const int num_bits)
{
  avd_rba_nand(m_handle, array2.m_handle, num_bits);
}

void cRawBitArray::NOR(const cRawBitArray & array2, const int num_bits)
{
  avd_rba_nor(m_handle, array2.m_handle, num_bits);
}

void cRawBitArray::XOR(const cRawBitArray & array2, const int num_bits)
{
  avd_rba_xor(m_handle, array2.m_handle, num_bits);
}

void cRawBitArray::EQU(const cRawBitArray & array2, const int num_bits)
{
  avd_rba_equ(m_handle, array2.m_handle, num_bits);
}

void cRawBitArray::SHIFT(const int num_bits, const int shift_size)
{
  avd_rba_shift(m_handle, num_bits, shift_size);
}

void cRawBitArray::INCREMENT(const int num_bits)
{
  avd_rba_increment(m_handle, num_bits);
}





void cRawBitArray::NOT(const cRawBitArray & array1, const int num_bits)
{
  Copy(array1, num_bits);
  NOT(num_bits);
}

void cRawBitArray::AND(const cRawBitArray & array1,
		       const cRawBitArray & array2, const int num_bits)
{
  Copy(array1, num_bits);
  AND(array2, num_bits);
}

void cRawBitArray::OR(const cRawBitArray & array1,
		      const cRawBitArray & array2, const int num_bits)
{
  Copy(array1, num_bits);
  OR(array2, num_bits);
}

void cRawBitArray::NAND(const cRawBitArray & array1,
			const cRawBitArray & array2, const int num_bits)
{
  Copy(array1, num_bits);
  NAND(array2, num_bits);
}

void cRawBitArray::NOR(const cRawBitArray & array1,
		       const cRawBitArray & array2, const int num_bits)
{
  Copy(array1, num_bits);
  NOR(array2, num_bits);
}

void cRawBitArray::XOR(const cRawBitArray & array1,
		       const cRawBitArray & array2, const int num_bits)
{
  Copy(array1, num_bits);
  XOR(array2, num_bits);
}

void cRawBitArray::EQU(const cRawBitArray & array1, const cRawBitArray & array2, const int num_bits)
{
  Copy(array1, num_bits);
  EQU(array2, num_bits);
}

void cRawBitArray::SHIFT(const cRawBitArray & array1, const int num_bits, const int shift_size)
{
  if (shift_size == 0) return;
  
  Copy(array1, num_bits);
  
  SHIFT(num_bits, shift_size);
}

void cRawBitArray::INCREMENT(const cRawBitArray & array1, const int num_bits)
{
  Copy(array1, num_bits);
  INCREMENT(num_bits);
}


bool cBitArray::operator<(const cBitArray& ar2) const
{
  return CountBits2() < ar2.CountBits2();
}




std::ostream & operator << (std::ostream & out, const cBitArray & bit_array)
{
  bit_array.Print(out);
  return out;
}
