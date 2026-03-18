use std::ffi::c_int;

/// Rust-native Merit type replacing cMerit.
///
/// Stores a double value with binary decomposition (bits, base, offset)
/// for efficient bit-level access. Negative values are clamped to 0.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Merit {
    bits: c_int,
    base: u32,
    offset: c_int,
    value: f64,
}

impl Merit {
    const MAX_BITS: c_int = (std::mem::size_of::<u32>() * 8) as c_int;

    fn exp_mult(i: c_int) -> f64 {
        2.0_f64.powi(i - 1)
    }

    pub fn new(in_value: f64) -> Self {
        let mut m = Merit::default();
        m.update_value(in_value);
        m
    }

    fn update_value(&mut self, in_value: f64) {
        let clamped = if in_value < 0.0 { 0.0 } else { in_value };
        self.value = clamped;

        let mut bits: i32 = 0;
        // SAFETY: libc frexp is a pure math function with no UB for any f64 input.
        // The bits pointer is valid and properly aligned.
        let mant = unsafe { libc_frexp(self.value, &mut bits) };
        self.bits = bits;

        if self.bits > Self::MAX_BITS {
            self.offset = self.bits - Self::MAX_BITS;
        } else {
            self.offset = 0;
        }

        self.base = (mant * Self::exp_mult(self.bits - self.offset) * 2.0) as u32;
    }

    pub fn get_double(&self) -> f64 {
        self.value
    }

    pub fn get_uint(&self) -> u32 {
        self.value as u32
    }

    pub fn get_bit(&self, bit_num: c_int) -> c_int {
        if bit_num >= self.offset && bit_num < self.bits {
            ((self.base >> (bit_num - self.offset)) & 1) as c_int
        } else {
            0
        }
    }

    pub fn get_num_bits(&self) -> c_int {
        self.bits
    }

    pub fn calc_fitness(&self, gestation_time: c_int) -> f64 {
        if gestation_time != 0 {
            self.value / f64::from(gestation_time)
        } else {
            0.0
        }
    }

    pub fn clear(&mut self) {
        self.value = 0.0;
        self.base = 0;
        self.offset = 0;
        self.bits = 0;
    }
}

// We need frexp from libm. Use a minimal extern.
extern "C" {
    #[link_name = "frexp"]
    fn libc_frexp(value: f64, exp: *mut i32) -> f64;
}

impl PartialEq for Merit {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl PartialOrd for Merit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.value.partial_cmp(&other.value)
    }
}

// --- FFI interface ---

#[no_mangle]
pub extern "C" fn avd_merit_new(value: f64) -> Merit {
    Merit::new(value)
}

#[no_mangle]
pub extern "C" fn avd_merit_new_int(value: c_int) -> Merit {
    Merit::new(f64::from(value))
}

#[no_mangle]
pub extern "C" fn avd_merit_default() -> Merit {
    Merit::default()
}

#[no_mangle]
pub extern "C" fn avd_merit_get_double(m: &Merit) -> f64 {
    m.get_double()
}

#[no_mangle]
pub extern "C" fn avd_merit_get_uint(m: &Merit) -> u32 {
    m.get_uint()
}

#[no_mangle]
pub extern "C" fn avd_merit_get_bit(m: &Merit, bit_num: c_int) -> c_int {
    m.get_bit(bit_num)
}

#[no_mangle]
pub extern "C" fn avd_merit_get_num_bits(m: &Merit) -> c_int {
    m.get_num_bits()
}

#[no_mangle]
pub extern "C" fn avd_merit_calc_fitness(m: &Merit, gestation_time: c_int) -> f64 {
    m.calc_fitness(gestation_time)
}

#[no_mangle]
pub extern "C" fn avd_merit_set(m: &mut Merit, value: f64) {
    m.update_value(value);
}

#[no_mangle]
pub extern "C" fn avd_merit_add_assign(m: &mut Merit, other: &Merit) {
    m.update_value(m.value + other.value);
}

#[no_mangle]
pub extern "C" fn avd_merit_add_assign_double(m: &mut Merit, value: f64) {
    m.update_value(m.value + value);
}

#[no_mangle]
pub extern "C" fn avd_merit_mul(a: &Merit, b: &Merit) -> Merit {
    Merit::new(a.value * b.value)
}

#[no_mangle]
pub extern "C" fn avd_merit_mul_assign(m: &mut Merit, other: &Merit) {
    m.update_value(m.value * other.value);
}

#[no_mangle]
pub extern "C" fn avd_merit_clear(m: &mut Merit) {
    m.clear();
}

#[no_mangle]
pub extern "C" fn avd_merit_gt(a: &Merit, b: &Merit) -> c_int {
    (a.value > b.value) as c_int
}

#[no_mangle]
pub extern "C" fn avd_merit_lt(a: &Merit, b: &Merit) -> c_int {
    (a.value < b.value) as c_int
}

#[no_mangle]
pub extern "C" fn avd_merit_eq(a: &Merit, b: &Merit) -> c_int {
    (a.value == b.value) as c_int
}

#[no_mangle]
pub extern "C" fn avd_merit_eq_double(m: &Merit, val: f64) -> c_int {
    (m.value == val) as c_int
}

#[no_mangle]
pub extern "C" fn avd_merit_ne_double(m: &Merit, val: f64) -> c_int {
    (m.value != val) as c_int
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn merit_default_is_zero() {
        let m = Merit::default();
        assert_eq!(m.get_double(), 0.0);
        assert_eq!(m.get_num_bits(), 0);
        assert_eq!(m.base, 0);
    }

    #[test]
    fn merit_from_double() {
        let m = Merit::new(100.0);
        assert_eq!(m.get_double(), 100.0);
        assert!(m.get_num_bits() > 0);
    }

    #[test]
    fn merit_negative_clamped() {
        let m = Merit::new(-5.0);
        assert_eq!(m.get_double(), 0.0);
    }

    #[test]
    fn merit_zero() {
        let m = Merit::new(0.0);
        assert_eq!(m.get_double(), 0.0);
        assert_eq!(m.get_num_bits(), 0);
    }

    #[test]
    fn merit_arithmetic() {
        let a = Merit::new(10.0);
        let b = Merit::new(20.0);
        let c = avd_merit_mul(&a, &b);
        assert_eq!(c.get_double(), 200.0);

        let mut d = Merit::new(5.0);
        avd_merit_add_assign_double(&mut d, 3.0);
        assert_eq!(d.get_double(), 8.0);
    }

    #[test]
    fn merit_comparison() {
        let a = Merit::new(10.0);
        let b = Merit::new(20.0);
        assert_eq!(avd_merit_gt(&b, &a), 1);
        assert_eq!(avd_merit_lt(&a, &b), 1);
        assert_eq!(avd_merit_eq(&a, &a), 1);
        assert_eq!(avd_merit_eq_double(&a, 10.0), 1);
        assert_eq!(avd_merit_ne_double(&a, 20.0), 1);
    }

    #[test]
    fn merit_calc_fitness() {
        let m = Merit::new(100.0);
        assert!((m.calc_fitness(10) - 10.0).abs() < f64::EPSILON);
        assert_eq!(m.calc_fitness(0), 0.0);
    }

    #[test]
    fn merit_bit_access() {
        let m = Merit::new(5.0); // binary: 101
                                 // The bit decomposition should give meaningful results
        assert!(m.get_num_bits() > 0);
        // Verify get_bit doesn't crash for valid and invalid indices
        let _ = m.get_bit(0);
        let _ = m.get_bit(m.get_num_bits() - 1);
        assert_eq!(m.get_bit(-1), 0); // negative index
        assert_eq!(m.get_bit(100), 0); // out of range
    }

    #[test]
    fn merit_clear() {
        let mut m = Merit::new(42.0);
        avd_merit_clear(&mut m);
        assert_eq!(m.get_double(), 0.0);
        assert_eq!(m.get_num_bits(), 0);
    }

    #[test]
    fn merit_ffi_roundtrip() {
        let m = avd_merit_new(123.456);
        assert!((avd_merit_get_double(&m) - 123.456).abs() < f64::EPSILON);

        let m2 = avd_merit_new_int(42);
        assert_eq!(avd_merit_get_double(&m2), 42.0);
    }

    #[test]
    fn merit_parity_with_cpp_update_value() {
        // Verify the binary decomposition matches C++ for known values
        let m = Merit::new(1.0);
        assert_eq!(m.get_double(), 1.0);
        assert_eq!(m.get_num_bits(), 1);

        let m2 = Merit::new(2.0);
        assert_eq!(m2.get_num_bits(), 2);

        let m3 = Merit::new(256.0);
        assert_eq!(m3.get_num_bits(), 9);
    }
}
