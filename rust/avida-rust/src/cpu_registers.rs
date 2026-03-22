// All public functions in this module are `extern "C"` FFI entry-points that
// accept raw pointers from C++.  They guard against null/oob before any
// dereference, so the clippy lint is a false positive at the module boundary.
#![allow(clippy::not_unsafe_ptr_arg_deref)]

use std::ffi::c_int;

pub const NUM_REGISTERS: usize = 3;

/// FFI-safe register file for cHardwareCPU threads.
///
/// This is a `#[repr(C)]` struct that overlays the existing `int reg[3]`
/// array in `cLocalThread`, allowing Rust to own the arithmetic while
/// C++ keeps the surrounding thread machinery.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CpuRegisters {
    pub reg: [c_int; NUM_REGISTERS],
}

impl Default for CpuRegisters {
    fn default() -> Self {
        Self {
            reg: [0; NUM_REGISTERS],
        }
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Returns `true` when the pointer is non-null and index is in-bounds.
#[inline]
fn valid(regs: *mut CpuRegisters, idx: c_int) -> bool {
    !regs.is_null() && idx >= 0 && (idx as usize) < NUM_REGISTERS
}

#[inline]
fn valid2(regs: *mut CpuRegisters, a: c_int, b: c_int) -> bool {
    valid(regs, a) && valid(regs, b)
}

#[inline]
fn valid3(regs: *mut CpuRegisters, a: c_int, b: c_int, c: c_int) -> bool {
    valid(regs, a) && valid(regs, b) && valid(regs, c)
}

/// Shorthand to get a mutable reference to the register array.
///
/// # Safety
/// Caller must ensure `regs` is non-null and points to a valid `CpuRegisters`.
#[inline]
unsafe fn r(regs: *mut CpuRegisters) -> &'static mut [c_int; NUM_REGISTERS] {
    // SAFETY: caller has verified non-null; pointer comes from C++ `&reg[0]`.
    unsafe { &mut (*regs).reg }
}

// ---------------------------------------------------------------------------
// FFI — construction / access
// ---------------------------------------------------------------------------

/// Return a default (all-zero) register file.
#[no_mangle]
pub extern "C" fn avd_cpu_reg_default() -> CpuRegisters {
    CpuRegisters::default()
}

/// Read a single register. Returns 0 on null/oob.
#[no_mangle]
pub extern "C" fn avd_cpu_reg_get(regs: *const CpuRegisters, idx: c_int) -> c_int {
    if regs.is_null() || idx < 0 || (idx as usize) >= NUM_REGISTERS {
        return 0;
    }
    // SAFETY: pointer verified non-null and idx is in-bounds.
    unsafe { (*regs).reg[idx as usize] }
}

/// Write a single register. No-op on null/oob.
#[no_mangle]
pub extern "C" fn avd_cpu_reg_set(regs: *mut CpuRegisters, idx: c_int, val: c_int) {
    if !valid(regs, idx) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        r(regs)[idx as usize] = val;
    }
}

/// Reset all registers to zero.
#[no_mangle]
pub extern "C" fn avd_cpu_reg_reset(regs: *mut CpuRegisters) {
    if regs.is_null() {
        return;
    }
    // SAFETY: non-null verified.
    unsafe {
        (*regs).reg = [0; NUM_REGISTERS];
    }
}

// ---------------------------------------------------------------------------
// FFI — unary operations (dst only)
// ---------------------------------------------------------------------------

/// `reg[dst] += 1` (wrapping)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_inc(regs: *mut CpuRegisters, dst: c_int) {
    if !valid(regs, dst) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let v = &mut r(regs)[dst as usize];
        *v = v.wrapping_add(1);
    }
}

/// `reg[dst] -= 1` (wrapping)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_dec(regs: *mut CpuRegisters, dst: c_int) {
    if !valid(regs, dst) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let v = &mut r(regs)[dst as usize];
        *v = v.wrapping_sub(1);
    }
}

/// `reg[dst] = 0`
#[no_mangle]
pub extern "C" fn avd_cpu_reg_zero(regs: *mut CpuRegisters, dst: c_int) {
    if !valid(regs, dst) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        r(regs)[dst as usize] = 0;
    }
}

/// `reg[dst] = 1`
#[no_mangle]
pub extern "C" fn avd_cpu_reg_one(regs: *mut CpuRegisters, dst: c_int) {
    if !valid(regs, dst) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        r(regs)[dst as usize] = 1;
    }
}

/// `reg[dst] = !0` (all bits set, i.e. -1 for signed int)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_all1s(regs: *mut CpuRegisters, dst: c_int) {
    if !valid(regs, dst) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        r(regs)[dst as usize] = !0;
    }
}

/// `reg[dst] = 0 - reg[dst]` (wrapping negation)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_neg(regs: *mut CpuRegisters, dst: c_int) {
    if !valid(regs, dst) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let v = &mut r(regs)[dst as usize];
        *v = v.wrapping_neg();
    }
}

/// `reg[dst] = reg[dst] * reg[dst]` (wrapping)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_square(regs: *mut CpuRegisters, dst: c_int) {
    if !valid(regs, dst) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let v = r(regs)[dst as usize];
        r(regs)[dst as usize] = v.wrapping_mul(v);
    }
}

/// `reg[dst] = ~reg[dst]` (bitwise NOT)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_not(regs: *mut CpuRegisters, dst: c_int) {
    if !valid(regs, dst) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        r(regs)[dst as usize] = !r(regs)[dst as usize];
    }
}

/// `reg[dst] >>= 1` (arithmetic right shift, matches C++ signed >>)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_shift_r(regs: *mut CpuRegisters, dst: c_int) {
    if !valid(regs, dst) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        r(regs)[dst as usize] >>= 1;
    }
}

/// `reg[dst] <<= 1` (left shift)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_shift_l(regs: *mut CpuRegisters, dst: c_int) {
    if !valid(regs, dst) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        r(regs)[dst as usize] <<= 1;
    }
}

/// `reg[dst] |= 1` (set the lowest bit)
///
/// Matches the C++ `Inst_Bit1` which does `GetRegister(reg_used) |= 1`.
#[no_mangle]
pub extern "C" fn avd_cpu_reg_bit1(regs: *mut CpuRegisters, dst: c_int) {
    if !valid(regs, dst) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        r(regs)[dst as usize] |= 1;
    }
}

// ---------------------------------------------------------------------------
// FFI — binary operations (dst, op1, op2)
// ---------------------------------------------------------------------------

/// `reg[dst] = reg[op1] + reg[op2]` (wrapping)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_add(regs: *mut CpuRegisters, dst: c_int, op1: c_int, op2: c_int) {
    if !valid3(regs, dst, op1, op2) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let a = r(regs)[op1 as usize];
        let b = r(regs)[op2 as usize];
        r(regs)[dst as usize] = a.wrapping_add(b);
    }
}

/// `reg[dst] = reg[op1] - reg[op2]` (wrapping)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_sub(regs: *mut CpuRegisters, dst: c_int, op1: c_int, op2: c_int) {
    if !valid3(regs, dst, op1, op2) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let a = r(regs)[op1 as usize];
        let b = r(regs)[op2 as usize];
        r(regs)[dst as usize] = a.wrapping_sub(b);
    }
}

/// `reg[dst] = reg[op1] * reg[op2]` (wrapping)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_mult(regs: *mut CpuRegisters, dst: c_int, op1: c_int, op2: c_int) {
    if !valid3(regs, dst, op1, op2) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let a = r(regs)[op1 as usize];
        let b = r(regs)[op2 as usize];
        r(regs)[dst as usize] = a.wrapping_mul(b);
    }
}

/// `reg[dst] = ~(reg[op1] & reg[op2])` (NAND)
#[no_mangle]
pub extern "C" fn avd_cpu_reg_nand(regs: *mut CpuRegisters, dst: c_int, op1: c_int, op2: c_int) {
    if !valid3(regs, dst, op1, op2) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let a = r(regs)[op1 as usize];
        let b = r(regs)[op2 as usize];
        r(regs)[dst as usize] = !(a & b);
    }
}

/// `reg[dst] = reg[op1] & reg[op2]`
#[no_mangle]
pub extern "C" fn avd_cpu_reg_and(regs: *mut CpuRegisters, dst: c_int, op1: c_int, op2: c_int) {
    if !valid3(regs, dst, op1, op2) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        r(regs)[dst as usize] = r(regs)[op1 as usize] & r(regs)[op2 as usize];
    }
}

/// `reg[dst] = reg[op1] ^ reg[op2]`
#[no_mangle]
pub extern "C" fn avd_cpu_reg_xor(regs: *mut CpuRegisters, dst: c_int, op1: c_int, op2: c_int) {
    if !valid3(regs, dst, op1, op2) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        r(regs)[dst as usize] = r(regs)[op1 as usize] ^ r(regs)[op2 as usize];
    }
}

/// `reg[dst] = reg[op1] | reg[op2]`
#[no_mangle]
pub extern "C" fn avd_cpu_reg_or(regs: *mut CpuRegisters, dst: c_int, op1: c_int, op2: c_int) {
    if !valid3(regs, dst, op1, op2) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        r(regs)[dst as usize] = r(regs)[op1 as usize] | r(regs)[op2 as usize];
    }
}

// ---------------------------------------------------------------------------
// FFI — register ops (swap, copy, order, setbit, clearbit)
// ---------------------------------------------------------------------------

/// Swap `reg[r1]` and `reg[r2]`.
#[no_mangle]
pub extern "C" fn avd_cpu_reg_swap(regs: *mut CpuRegisters, r1: c_int, r2: c_int) {
    if !valid2(regs, r1, r2) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let arr = r(regs);
        arr.swap(r1 as usize, r2 as usize);
    }
}

/// `reg[dst] = reg[src]`
#[no_mangle]
pub extern "C" fn avd_cpu_reg_copy(regs: *mut CpuRegisters, dst: c_int, src: c_int) {
    if !valid2(regs, dst, src) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let val = r(regs)[src as usize];
        r(regs)[dst as usize] = val;
    }
}

/// If `reg[r1] > reg[r2]`, swap them so they are in ascending order.
#[no_mangle]
pub extern "C" fn avd_cpu_reg_order(regs: *mut CpuRegisters, r1: c_int, r2: c_int) {
    if !valid2(regs, r1, r2) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let arr = r(regs);
        if arr[r1 as usize] > arr[r2 as usize] {
            arr.swap(r1 as usize, r2 as usize);
        }
    }
}

/// Set the bit in `reg[to_set]` at position `reg[bit_reg] % 32`.
///
/// Matches C++: `max(0, reg[bit_reg]) % (sizeof(int)*8)` then `reg[to_set] |= 1 << bit_to_set`.
#[no_mangle]
pub extern "C" fn avd_cpu_reg_setbit(regs: *mut CpuRegisters, to_set: c_int, bit_reg: c_int) {
    if !valid2(regs, to_set, bit_reg) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let arr = r(regs);
        let bit_pos = (0_i32.max(arr[bit_reg as usize]) as u32) % 32;
        arr[to_set as usize] |= 1_i32 << bit_pos;
    }
}

/// Clear the bit in `reg[to_clear]` at position `reg[bit_reg] % 32`.
///
/// Matches C++: `max(0, reg[bit_reg]) % (sizeof(int)*8)` then `reg[to_clear] &= ~(1 << bit_to_clear)`.
#[no_mangle]
pub extern "C" fn avd_cpu_reg_clearbit(regs: *mut CpuRegisters, to_clear: c_int, bit_reg: c_int) {
    if !valid2(regs, to_clear, bit_reg) {
        return;
    }
    // SAFETY: validated above.
    unsafe {
        let arr = r(regs);
        let bit_pos = (0_i32.max(arr[bit_reg as usize]) as u32) % 32;
        arr[to_clear as usize] &= !(1_i32 << bit_pos);
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make(a: c_int, b: c_int, c: c_int) -> CpuRegisters {
        CpuRegisters { reg: [a, b, c] }
    }

    // ---- construction / access ----

    #[test]
    fn default_is_zero() {
        let r = CpuRegisters::default();
        assert_eq!(r.reg, [0, 0, 0]);
    }

    #[test]
    fn ffi_default_is_zero() {
        let r = avd_cpu_reg_default();
        assert_eq!(r.reg, [0, 0, 0]);
    }

    #[test]
    fn get_set_roundtrip() {
        let mut r = avd_cpu_reg_default();
        avd_cpu_reg_set(&mut r, 0, 42);
        avd_cpu_reg_set(&mut r, 1, -7);
        avd_cpu_reg_set(&mut r, 2, 100);
        assert_eq!(avd_cpu_reg_get(&r, 0), 42);
        assert_eq!(avd_cpu_reg_get(&r, 1), -7);
        assert_eq!(avd_cpu_reg_get(&r, 2), 100);
    }

    #[test]
    fn get_oob_returns_zero() {
        let r = make(10, 20, 30);
        assert_eq!(avd_cpu_reg_get(&r, -1), 0);
        assert_eq!(avd_cpu_reg_get(&r, 3), 0);
        assert_eq!(avd_cpu_reg_get(std::ptr::null(), 0), 0);
    }

    #[test]
    fn set_oob_is_noop() {
        let mut r = make(1, 2, 3);
        avd_cpu_reg_set(&mut r, -1, 99);
        avd_cpu_reg_set(&mut r, 3, 99);
        assert_eq!(r.reg, [1, 2, 3]);
    }

    #[test]
    fn reset_clears_all() {
        let mut r = make(10, 20, 30);
        avd_cpu_reg_reset(&mut r);
        assert_eq!(r.reg, [0, 0, 0]);
    }

    // ---- null safety ----

    #[test]
    fn null_pointer_noops() {
        let null: *mut CpuRegisters = std::ptr::null_mut();
        // These should all be no-ops, not crash.
        avd_cpu_reg_set(null, 0, 1);
        avd_cpu_reg_reset(null);
        avd_cpu_reg_inc(null, 0);
        avd_cpu_reg_dec(null, 0);
        avd_cpu_reg_zero(null, 0);
        avd_cpu_reg_one(null, 0);
        avd_cpu_reg_all1s(null, 0);
        avd_cpu_reg_neg(null, 0);
        avd_cpu_reg_square(null, 0);
        avd_cpu_reg_not(null, 0);
        avd_cpu_reg_shift_r(null, 0);
        avd_cpu_reg_shift_l(null, 0);
        avd_cpu_reg_bit1(null, 0);
        avd_cpu_reg_add(null, 0, 1, 2);
        avd_cpu_reg_sub(null, 0, 1, 2);
        avd_cpu_reg_mult(null, 0, 1, 2);
        avd_cpu_reg_nand(null, 0, 1, 2);
        avd_cpu_reg_and(null, 0, 1, 2);
        avd_cpu_reg_xor(null, 0, 1, 2);
        avd_cpu_reg_or(null, 0, 1, 2);
        avd_cpu_reg_swap(null, 0, 1);
        avd_cpu_reg_copy(null, 0, 1);
        avd_cpu_reg_order(null, 0, 1);
        avd_cpu_reg_setbit(null, 0, 1);
        avd_cpu_reg_clearbit(null, 0, 1);
    }

    // ---- inc / dec ----

    #[test]
    fn inc_basic() {
        let mut r = make(0, 5, -1);
        avd_cpu_reg_inc(&mut r, 0);
        avd_cpu_reg_inc(&mut r, 1);
        avd_cpu_reg_inc(&mut r, 2);
        assert_eq!(r.reg, [1, 6, 0]);
    }

    #[test]
    fn inc_wraps_at_max() {
        let mut r = make(c_int::MAX, 0, 0);
        avd_cpu_reg_inc(&mut r, 0);
        assert_eq!(r.reg[0], c_int::MIN);
    }

    #[test]
    fn dec_basic() {
        let mut r = make(1, 0, -1);
        avd_cpu_reg_dec(&mut r, 0);
        avd_cpu_reg_dec(&mut r, 1);
        avd_cpu_reg_dec(&mut r, 2);
        assert_eq!(r.reg, [0, -1, -2]);
    }

    #[test]
    fn dec_wraps_at_min() {
        let mut r = make(c_int::MIN, 0, 0);
        avd_cpu_reg_dec(&mut r, 0);
        assert_eq!(r.reg[0], c_int::MAX);
    }

    // ---- zero / one / all1s ----

    #[test]
    fn zero_sets_zero() {
        let mut r = make(99, 0, 0);
        avd_cpu_reg_zero(&mut r, 0);
        assert_eq!(r.reg[0], 0);
    }

    #[test]
    fn one_sets_one() {
        let mut r = make(99, 0, 0);
        avd_cpu_reg_one(&mut r, 0);
        assert_eq!(r.reg[0], 1);
    }

    #[test]
    fn all1s_sets_all_bits() {
        let mut r = make(0, 0, 0);
        avd_cpu_reg_all1s(&mut r, 0);
        assert_eq!(r.reg[0], -1); // !0 for signed int = -1
    }

    // ---- neg ----

    #[test]
    fn neg_basic() {
        let mut r = make(5, -3, 0);
        avd_cpu_reg_neg(&mut r, 0);
        avd_cpu_reg_neg(&mut r, 1);
        avd_cpu_reg_neg(&mut r, 2);
        assert_eq!(r.reg, [-5, 3, 0]);
    }

    #[test]
    fn neg_min_wraps() {
        let mut r = make(c_int::MIN, 0, 0);
        avd_cpu_reg_neg(&mut r, 0);
        // wrapping_neg of MIN = MIN
        assert_eq!(r.reg[0], c_int::MIN);
    }

    // ---- square ----

    #[test]
    fn square_basic() {
        let mut r = make(3, -4, 0);
        avd_cpu_reg_square(&mut r, 0);
        avd_cpu_reg_square(&mut r, 1);
        avd_cpu_reg_square(&mut r, 2);
        assert_eq!(r.reg, [9, 16, 0]);
    }

    #[test]
    fn square_overflow_wraps() {
        let mut r = make(c_int::MAX, 0, 0);
        avd_cpu_reg_square(&mut r, 0);
        assert_eq!(r.reg[0], c_int::MAX.wrapping_mul(c_int::MAX));
    }

    // ---- not ----

    #[test]
    fn not_basic() {
        let mut r = make(0, -1, 42);
        avd_cpu_reg_not(&mut r, 0);
        avd_cpu_reg_not(&mut r, 1);
        avd_cpu_reg_not(&mut r, 2);
        assert_eq!(r.reg[0], -1); // ~0 = -1
        assert_eq!(r.reg[1], 0); // ~(-1) = 0
        assert_eq!(r.reg[2], !42);
    }

    // ---- shift_r / shift_l ----

    #[test]
    fn shift_r_basic() {
        let mut r = make(8, -2, 1);
        avd_cpu_reg_shift_r(&mut r, 0);
        avd_cpu_reg_shift_r(&mut r, 1);
        avd_cpu_reg_shift_r(&mut r, 2);
        assert_eq!(r.reg[0], 4);
        assert_eq!(r.reg[1], -1); // arithmetic shift right of -2
        assert_eq!(r.reg[2], 0);
    }

    #[test]
    fn shift_l_basic() {
        let mut r = make(1, -1, 0);
        avd_cpu_reg_shift_l(&mut r, 0);
        avd_cpu_reg_shift_l(&mut r, 1);
        avd_cpu_reg_shift_l(&mut r, 2);
        assert_eq!(r.reg[0], 2);
        assert_eq!(r.reg[1], -2);
        assert_eq!(r.reg[2], 0);
    }

    // ---- bit1 ----

    #[test]
    fn bit1_sets_lowest_bit() {
        let mut r = make(0, 4, 5);
        avd_cpu_reg_bit1(&mut r, 0);
        avd_cpu_reg_bit1(&mut r, 1);
        avd_cpu_reg_bit1(&mut r, 2);
        assert_eq!(r.reg[0], 1); // 0 | 1 = 1
        assert_eq!(r.reg[1], 5); // 4 | 1 = 5
        assert_eq!(r.reg[2], 5); // 5 | 1 = 5 (already set)
    }

    // ---- binary: add / sub / mult ----

    #[test]
    fn add_basic() {
        let mut r = make(10, 20, 30);
        avd_cpu_reg_add(&mut r, 0, 1, 2);
        assert_eq!(r.reg[0], 50);
    }

    #[test]
    fn add_wrapping() {
        let mut r = make(0, c_int::MAX, 1);
        avd_cpu_reg_add(&mut r, 0, 1, 2);
        assert_eq!(r.reg[0], c_int::MIN);
    }

    #[test]
    fn sub_basic() {
        let mut r = make(0, 50, 20);
        avd_cpu_reg_sub(&mut r, 0, 1, 2);
        assert_eq!(r.reg[0], 30);
    }

    #[test]
    fn sub_wrapping() {
        let mut r = make(0, c_int::MIN, 1);
        avd_cpu_reg_sub(&mut r, 0, 1, 2);
        assert_eq!(r.reg[0], c_int::MAX);
    }

    #[test]
    fn mult_basic() {
        let mut r = make(0, 7, 6);
        avd_cpu_reg_mult(&mut r, 0, 1, 2);
        assert_eq!(r.reg[0], 42);
    }

    #[test]
    fn mult_wrapping() {
        let mut r = make(0, c_int::MAX, 2);
        avd_cpu_reg_mult(&mut r, 0, 1, 2);
        assert_eq!(r.reg[0], c_int::MAX.wrapping_mul(2));
    }

    // ---- binary: nand / and / xor / or ----

    #[test]
    fn nand_basic() {
        let mut r = make(0, 0x0F, 0xFF);
        avd_cpu_reg_nand(&mut r, 0, 1, 2);
        assert_eq!(r.reg[0], !(0x0F & 0xFF));
    }

    #[test]
    fn and_basic() {
        let mut r = make(0, 0x0F, 0xFF);
        avd_cpu_reg_and(&mut r, 0, 1, 2);
        assert_eq!(r.reg[0], 0x0F);
    }

    #[test]
    fn xor_basic() {
        let mut r = make(0, 0x0F, 0xFF);
        avd_cpu_reg_xor(&mut r, 0, 1, 2);
        assert_eq!(r.reg[0], 0xF0);
    }

    #[test]
    fn or_basic() {
        let mut r = make(0, 0x0F, 0xF0);
        avd_cpu_reg_or(&mut r, 0, 1, 2);
        assert_eq!(r.reg[0], 0xFF);
    }

    // ---- swap ----

    #[test]
    fn swap_basic() {
        let mut r = make(10, 20, 30);
        avd_cpu_reg_swap(&mut r, 0, 1);
        assert_eq!(r.reg, [20, 10, 30]);
    }

    #[test]
    fn swap_same_index() {
        let mut r = make(10, 20, 30);
        avd_cpu_reg_swap(&mut r, 1, 1);
        assert_eq!(r.reg, [10, 20, 30]);
    }

    // ---- copy ----

    #[test]
    fn copy_basic() {
        let mut r = make(10, 20, 30);
        avd_cpu_reg_copy(&mut r, 0, 2);
        assert_eq!(r.reg, [30, 20, 30]);
    }

    // ---- order ----

    #[test]
    fn order_already_sorted() {
        let mut r = make(0, 5, 10);
        avd_cpu_reg_order(&mut r, 1, 2);
        assert_eq!(r.reg, [0, 5, 10]);
    }

    #[test]
    fn order_needs_swap() {
        let mut r = make(0, 10, 5);
        avd_cpu_reg_order(&mut r, 1, 2);
        assert_eq!(r.reg, [0, 5, 10]);
    }

    #[test]
    fn order_equal_noop() {
        let mut r = make(0, 7, 7);
        avd_cpu_reg_order(&mut r, 1, 2);
        assert_eq!(r.reg, [0, 7, 7]);
    }

    // ---- setbit / clearbit ----

    #[test]
    fn setbit_basic() {
        let mut r = make(0, 0, 3); // bit_reg=2 has value 3 -> set bit 3
        avd_cpu_reg_setbit(&mut r, 0, 2);
        assert_eq!(r.reg[0], 1 << 3);
    }

    #[test]
    fn setbit_negative_bit_reg_uses_zero() {
        // max(0, -5) = 0, so bit position = 0 % 32 = 0
        let mut r = make(0, -5, 0);
        avd_cpu_reg_setbit(&mut r, 0, 1);
        assert_eq!(r.reg[0], 1); // bit 0 set
    }

    #[test]
    fn setbit_large_value_wraps_mod32() {
        let mut r = make(0, 33, 0); // 33 % 32 = 1
        avd_cpu_reg_setbit(&mut r, 0, 1);
        assert_eq!(r.reg[0], 1 << 1);
    }

    #[test]
    fn clearbit_basic() {
        let mut r = make(0xFF, 3, 0); // clear bit 3 from reg[0]
        avd_cpu_reg_clearbit(&mut r, 0, 1);
        assert_eq!(r.reg[0], 0xFF & !(1 << 3));
    }

    #[test]
    fn clearbit_negative_bit_reg_uses_zero() {
        let mut r = make(0xFF, -1, 0);
        avd_cpu_reg_clearbit(&mut r, 0, 1);
        // max(0, -1) = 0, bit 0 cleared
        assert_eq!(r.reg[0], 0xFE);
    }

    // ---- dst == op aliasing ----

    #[test]
    fn add_aliased_dst_is_op() {
        // dst=1, op1=1, op2=2 => reg[1] = reg[1] + reg[2]
        let mut r = make(0, 10, 3);
        avd_cpu_reg_add(&mut r, 1, 1, 2);
        assert_eq!(r.reg[1], 13);
    }

    #[test]
    fn nand_self() {
        // nand(reg[0], reg[0], reg[0]) = ~(reg[0] & reg[0]) = ~reg[0]
        let mut r = make(0x0F, 0, 0);
        avd_cpu_reg_nand(&mut r, 0, 0, 0);
        assert_eq!(r.reg[0], !0x0F);
    }

    // ---- oob on binary ops ----

    #[test]
    fn binary_oob_is_noop() {
        let mut r = make(1, 2, 3);
        avd_cpu_reg_add(&mut r, 3, 0, 1);
        avd_cpu_reg_add(&mut r, 0, -1, 1);
        avd_cpu_reg_swap(&mut r, 0, 5);
        assert_eq!(r.reg, [1, 2, 3]);
    }

    // ---- INT_MIN edge cases ----

    #[test]
    fn not_of_int_min() {
        let mut r = make(c_int::MIN, 0, 0);
        avd_cpu_reg_not(&mut r, 0);
        assert_eq!(r.reg[0], c_int::MAX);
    }

    #[test]
    fn shift_r_negative() {
        // Arithmetic right shift of -1 stays -1
        let mut r = make(-1, 0, 0);
        avd_cpu_reg_shift_r(&mut r, 0);
        assert_eq!(r.reg[0], -1);
    }

    #[test]
    fn shift_l_overflow() {
        // Shifting c_int::MAX left by 1
        let mut r = make(c_int::MAX, 0, 0);
        avd_cpu_reg_shift_l(&mut r, 0);
        assert_eq!(r.reg[0], -2);
    }
}
