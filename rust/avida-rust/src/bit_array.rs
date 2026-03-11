use std::ffi::c_int;

use crate::{
    common::{with_rba_mut, with_rba_ref},
    AvidaRawBitArrayHandle,
};

impl AvidaRawBitArrayHandle {
    fn num_fields(num_bits: i32) -> usize {
        if num_bits <= 0 {
            0
        } else {
            1 + (((num_bits - 1) as usize) >> 5)
        }
    }

    fn get_bit_from(fields: &[u32], index: i32) -> bool {
        if index < 0 {
            return false;
        }
        let field_id = (index as usize) >> 5;
        let pos_id = (index as usize) & 31;
        if field_id >= fields.len() {
            return false;
        }
        (fields[field_id] & (1_u32 << pos_id)) != 0
    }

    fn set_bit_in(fields: &mut [u32], index: i32, value: bool) {
        if index < 0 {
            return;
        }
        let field_id = (index as usize) >> 5;
        let pos_id = (index as usize) & 31;
        if field_id >= fields.len() {
            return;
        }
        let pos_mask = 1_u32 << pos_id;
        if value {
            fields[field_id] |= pos_mask;
        } else {
            fields[field_id] &= !pos_mask;
        }
    }

    fn tail_mask(num_bits: i32) -> u32 {
        let rem = num_bits & 31;
        if rem == 0 {
            u32::MAX
        } else {
            (1_u32 << rem) - 1_u32
        }
    }

    fn mask_tail(&mut self, num_bits: i32) {
        if self.bit_fields.is_empty() {
            return;
        }
        if (num_bits & 31) != 0 {
            let last = self.bit_fields.len() - 1;
            self.bit_fields[last] &= Self::tail_mask(num_bits);
        }
    }
}

#[no_mangle]
pub extern "C" fn avd_rba_new(num_bits: c_int) -> *mut AvidaRawBitArrayHandle {
    let num_fields = AvidaRawBitArrayHandle::num_fields(num_bits);
    Box::into_raw(Box::new(AvidaRawBitArrayHandle {
        bit_fields: vec![0_u32; num_fields],
    }))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rba_clone(
    other: *const AvidaRawBitArrayHandle,
) -> *mut AvidaRawBitArrayHandle {
    if other.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: pointer was checked for null and is only read.
    let other_ref = unsafe { &*other };
    Box::into_raw(Box::new(AvidaRawBitArrayHandle {
        bit_fields: other_ref.bit_fields.clone(),
    }))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rba_free(handle: *mut AvidaRawBitArrayHandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer came from Box::into_raw in this crate and is freed exactly once here.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub extern "C" fn avd_rba_resize(
    handle: *mut AvidaRawBitArrayHandle,
    old_bits: c_int,
    new_bits: c_int,
) {
    with_rba_mut(handle, |h| {
        let num_old_fields = AvidaRawBitArrayHandle::num_fields(old_bits);
        let num_new_fields = AvidaRawBitArrayHandle::num_fields(new_bits);
        if num_old_fields == num_new_fields {
            if num_new_fields > 0 {
                for i in new_bits..old_bits {
                    AvidaRawBitArrayHandle::set_bit_in(&mut h.bit_fields, i, false);
                }
            }
            return;
        }

        let mut new_fields = vec![0_u32; num_new_fields];
        let overlap = num_old_fields.min(num_new_fields);
        new_fields[..overlap].copy_from_slice(&h.bit_fields[..overlap]);

        if num_old_fields > num_new_fields && num_new_fields > 0 {
            let last = num_new_fields - 1;
            let rem = new_bits & 31;
            if rem != 0 {
                new_fields[last] &= AvidaRawBitArrayHandle::tail_mask(new_bits);
            }
        }

        h.bit_fields = new_fields;
    });
}

#[no_mangle]
pub extern "C" fn avd_rba_zero(handle: *mut AvidaRawBitArrayHandle, _num_bits: c_int) {
    with_rba_mut(handle, |h| h.bit_fields.fill(0_u32));
}

#[no_mangle]
pub extern "C" fn avd_rba_ones(handle: *mut AvidaRawBitArrayHandle, num_bits: c_int) {
    with_rba_mut(handle, |h| {
        h.bit_fields.fill(u32::MAX);
        h.mask_tail(num_bits);
    });
}

#[no_mangle]
pub extern "C" fn avd_rba_get_bit(handle: *const AvidaRawBitArrayHandle, index: c_int) -> c_int {
    with_rba_ref(handle, 0, |h| {
        if AvidaRawBitArrayHandle::get_bit_from(&h.bit_fields, index) {
            1
        } else {
            0
        }
    })
}

#[no_mangle]
pub extern "C" fn avd_rba_set_bit(handle: *mut AvidaRawBitArrayHandle, index: c_int, value: c_int) {
    with_rba_mut(handle, |h| {
        AvidaRawBitArrayHandle::set_bit_in(&mut h.bit_fields, index, value != 0)
    });
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rba_is_equal(
    left: *const AvidaRawBitArrayHandle,
    right: *const AvidaRawBitArrayHandle,
    num_bits: c_int,
) -> c_int {
    if left.is_null() || right.is_null() {
        return 0;
    }
    // SAFETY: pointers checked for null and only read.
    let l = unsafe { &*left };
    // SAFETY: pointers checked for null and only read.
    let r = unsafe { &*right };
    let fields = AvidaRawBitArrayHandle::num_fields(num_bits);
    if l.bit_fields.len() < fields || r.bit_fields.len() < fields {
        return 0;
    }
    if l.bit_fields[..fields] == r.bit_fields[..fields] {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn avd_rba_count_bits(
    handle: *const AvidaRawBitArrayHandle,
    num_bits: c_int,
) -> c_int {
    with_rba_ref(handle, 0, |h| {
        let fields = AvidaRawBitArrayHandle::num_fields(num_bits);
        let mut bit_count: i32 = 0;
        for v in h.bit_fields.iter().take(fields) {
            let mut temp = *v;
            while temp != 0 {
                temp &= temp - 1;
                bit_count += 1;
            }
        }
        bit_count
    })
}

#[no_mangle]
pub extern "C" fn avd_rba_count_bits2(
    handle: *const AvidaRawBitArrayHandle,
    num_bits: c_int,
) -> c_int {
    with_rba_ref(handle, 0, |h| {
        let fields = AvidaRawBitArrayHandle::num_fields(num_bits);
        let mut bit_count: i32 = 0;
        for v in h.bit_fields.iter().take(fields) {
            let t1 = *v - ((*v >> 1) & 0x5555_5555);
            let t2 = (t1 & 0x3333_3333) + ((t1 >> 2) & 0x3333_3333);
            bit_count += (((t2 + (t2 >> 4)) & 0x0F0F_0F0F).wrapping_mul(0x0101_0101) >> 24) as i32;
        }
        bit_count
    })
}

#[no_mangle]
pub extern "C" fn avd_rba_find_bit1(
    handle: *const AvidaRawBitArrayHandle,
    num_bits: c_int,
    start_pos: c_int,
) -> c_int {
    with_rba_ref(handle, -1, |h| {
        for i in start_pos..num_bits {
            if AvidaRawBitArrayHandle::get_bit_from(&h.bit_fields, i) {
                return i;
            }
        }
        -1
    })
}

#[no_mangle]
pub extern "C" fn avd_rba_not(handle: *mut AvidaRawBitArrayHandle, num_bits: c_int) {
    with_rba_mut(handle, |h| {
        for v in &mut h.bit_fields {
            *v = !*v;
        }
        h.mask_tail(num_bits);
    });
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rba_and(
    handle: *mut AvidaRawBitArrayHandle,
    other: *const AvidaRawBitArrayHandle,
    num_bits: c_int,
) {
    if other.is_null() {
        return;
    }
    // SAFETY: pointer checked for null and only read.
    let rhs = unsafe { &*other };
    with_rba_mut(handle, |h| {
        let fields = AvidaRawBitArrayHandle::num_fields(num_bits);
        for i in 0..fields.min(h.bit_fields.len()).min(rhs.bit_fields.len()) {
            h.bit_fields[i] &= rhs.bit_fields[i];
        }
    });
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rba_or(
    handle: *mut AvidaRawBitArrayHandle,
    other: *const AvidaRawBitArrayHandle,
    num_bits: c_int,
) {
    if other.is_null() {
        return;
    }
    // SAFETY: pointer checked for null and only read.
    let rhs = unsafe { &*other };
    with_rba_mut(handle, |h| {
        let fields = AvidaRawBitArrayHandle::num_fields(num_bits);
        for i in 0..fields.min(h.bit_fields.len()).min(rhs.bit_fields.len()) {
            h.bit_fields[i] |= rhs.bit_fields[i];
        }
    });
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rba_nand(
    handle: *mut AvidaRawBitArrayHandle,
    other: *const AvidaRawBitArrayHandle,
    num_bits: c_int,
) {
    if other.is_null() {
        return;
    }
    // SAFETY: pointer checked for null and only read.
    let rhs = unsafe { &*other };
    with_rba_mut(handle, |h| {
        let fields = AvidaRawBitArrayHandle::num_fields(num_bits);
        for i in 0..fields.min(h.bit_fields.len()).min(rhs.bit_fields.len()) {
            h.bit_fields[i] = !(h.bit_fields[i] & rhs.bit_fields[i]);
        }
        h.mask_tail(num_bits);
    });
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rba_nor(
    handle: *mut AvidaRawBitArrayHandle,
    other: *const AvidaRawBitArrayHandle,
    num_bits: c_int,
) {
    if other.is_null() {
        return;
    }
    // SAFETY: pointer checked for null and only read.
    let rhs = unsafe { &*other };
    with_rba_mut(handle, |h| {
        let fields = AvidaRawBitArrayHandle::num_fields(num_bits);
        for i in 0..fields.min(h.bit_fields.len()).min(rhs.bit_fields.len()) {
            h.bit_fields[i] = !(h.bit_fields[i] | rhs.bit_fields[i]);
        }
        h.mask_tail(num_bits);
    });
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rba_xor(
    handle: *mut AvidaRawBitArrayHandle,
    other: *const AvidaRawBitArrayHandle,
    num_bits: c_int,
) {
    if other.is_null() {
        return;
    }
    // SAFETY: pointer checked for null and only read.
    let rhs = unsafe { &*other };
    with_rba_mut(handle, |h| {
        let fields = AvidaRawBitArrayHandle::num_fields(num_bits);
        for i in 0..fields.min(h.bit_fields.len()).min(rhs.bit_fields.len()) {
            h.bit_fields[i] ^= rhs.bit_fields[i];
        }
    });
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rba_equ(
    handle: *mut AvidaRawBitArrayHandle,
    other: *const AvidaRawBitArrayHandle,
    num_bits: c_int,
) {
    if other.is_null() {
        return;
    }
    // SAFETY: pointer checked for null and only read.
    let rhs = unsafe { &*other };
    with_rba_mut(handle, |h| {
        let fields = AvidaRawBitArrayHandle::num_fields(num_bits);
        for i in 0..fields.min(h.bit_fields.len()).min(rhs.bit_fields.len()) {
            h.bit_fields[i] = !(h.bit_fields[i] ^ rhs.bit_fields[i]);
        }
        h.mask_tail(num_bits);
    });
}

#[no_mangle]
pub extern "C" fn avd_rba_shift(
    handle: *mut AvidaRawBitArrayHandle,
    num_bits: c_int,
    shift_size: c_int,
) {
    if shift_size == 0 {
        return;
    }
    with_rba_mut(handle, |h| {
        if num_bits <= 0 {
            return;
        }
        let old = h.bit_fields.clone();
        h.bit_fields.fill(0_u32);
        if shift_size > 0 {
            for i in (0..num_bits).rev() {
                let src = i - shift_size;
                let bit = if src >= 0 {
                    AvidaRawBitArrayHandle::get_bit_from(&old, src)
                } else {
                    false
                };
                AvidaRawBitArrayHandle::set_bit_in(&mut h.bit_fields, i, bit);
            }
        } else {
            let right = -shift_size;
            for i in 0..num_bits {
                let src = i + right;
                let bit = if src < num_bits {
                    AvidaRawBitArrayHandle::get_bit_from(&old, src)
                } else {
                    false
                };
                AvidaRawBitArrayHandle::set_bit_in(&mut h.bit_fields, i, bit);
            }
        }
        h.mask_tail(num_bits);
    });
}

#[no_mangle]
pub extern "C" fn avd_rba_increment(handle: *mut AvidaRawBitArrayHandle, num_bits: c_int) {
    with_rba_mut(handle, |h| {
        let fields = AvidaRawBitArrayHandle::num_fields(num_bits);
        if fields == 0 {
            return;
        }
        let mut i = 0usize;
        while i < fields && i < h.bit_fields.len() {
            let (next, overflow) = h.bit_fields[i].overflowing_add(1_u32);
            h.bit_fields[i] = next;
            if !overflow {
                break;
            }
            i += 1;
        }
        if fields > 0 && fields - 1 < h.bit_fields.len() {
            let rem = num_bits & 31;
            if rem != 0 {
                h.bit_fields[fields - 1] &= AvidaRawBitArrayHandle::tail_mask(num_bits);
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::prelude::{BitVec, Lsb0};

    fn bitvec_from_handle(handle: &AvidaRawBitArrayHandle, num_bits: i32) -> BitVec<u32, Lsb0> {
        let mut out = BitVec::<u32, Lsb0>::repeat(false, num_bits as usize);
        for i in 0..num_bits {
            out.set(
                i as usize,
                AvidaRawBitArrayHandle::get_bit_from(&handle.bit_fields, i),
            );
        }
        out
    }

    fn shift_bitvec(src: &BitVec<u32, Lsb0>, num_bits: i32, shift_size: i32) -> BitVec<u32, Lsb0> {
        let mut out = BitVec::<u32, Lsb0>::repeat(false, num_bits as usize);
        if shift_size > 0 {
            for i in (0..num_bits).rev() {
                let src_idx = i - shift_size;
                if src_idx >= 0 {
                    out.set(i as usize, src[src_idx as usize]);
                }
            }
        } else if shift_size < 0 {
            let right = -shift_size;
            for i in 0..num_bits {
                let src_idx = i + right;
                if src_idx < num_bits {
                    out.set(i as usize, src[src_idx as usize]);
                }
            }
        } else {
            out.clone_from(src);
        }
        out
    }

    fn increment_bitvec(bits: &mut BitVec<u32, Lsb0>) {
        let mut carry = true;
        for i in 0..bits.len() {
            if !carry {
                break;
            }
            let next = !bits[i];
            bits.set(i, next);
            carry = !next;
        }
    }

    #[test]
    fn raw_bit_array_shift_and_increment_semantics() {
        let mut bits = AvidaRawBitArrayHandle {
            bit_fields: vec![0_u32; AvidaRawBitArrayHandle::num_fields(34)],
        };
        AvidaRawBitArrayHandle::set_bit_in(&mut bits.bit_fields, 0, true);
        avd_rba_shift(&mut bits, 34, 33);
        assert!(AvidaRawBitArrayHandle::get_bit_from(&bits.bit_fields, 33));
    }

    #[test]
    fn bitvec_shift_increment_count_parity_experiment() {
        let num_bits = 37;
        let mut handle = AvidaRawBitArrayHandle {
            bit_fields: vec![0_u32; AvidaRawBitArrayHandle::num_fields(num_bits)],
        };
        for idx in [0, 1, 5, 31, 36] {
            AvidaRawBitArrayHandle::set_bit_in(&mut handle.bit_fields, idx, true);
        }

        for shift in [-40, -7, -1, 0, 1, 7, 40] {
            let before = bitvec_from_handle(&handle, num_bits);
            avd_rba_shift(&mut handle, num_bits, shift);
            let expected = shift_bitvec(&before, num_bits, shift);
            for i in 0..num_bits {
                assert_eq!(
                    AvidaRawBitArrayHandle::get_bit_from(&handle.bit_fields, i),
                    expected[i as usize]
                );
            }
        }

        let mut expected = bitvec_from_handle(&handle, num_bits);
        for _ in 0..100 {
            avd_rba_increment(&mut handle, num_bits);
            increment_bitvec(&mut expected);
        }
        for i in 0..num_bits {
            assert_eq!(
                AvidaRawBitArrayHandle::get_bit_from(&handle.bit_fields, i),
                expected[i as usize]
            );
        }

        let count = avd_rba_count_bits(&handle, num_bits);
        let count2 = avd_rba_count_bits2(&handle, num_bits);
        assert_eq!(count as usize, expected.count_ones());
        assert_eq!(count2 as usize, expected.count_ones());
    }
}
