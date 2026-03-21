//! Bit-exact Rust port of the Avida Knuth subtractive RNG (`Apto::RNG::AvidaRNG`).
//!
//! The generator is a lag-55 subtractive PRNG as described in Knuth TAOCP Vol 2,
//! with constants `UPPER_BOUND = 1_000_000_000` and `MAX_SEED = 161_803_398`.
//! All derived methods (`GetDouble`, `GetRandNormal`, `GetRandPoisson`, `GetRandBinomial`)
//! are ported from the C++ `Apto::Random` base class.

use std::ffi::{c_double, c_int, c_uint};

use crate::common::{boxed_clone, boxed_free, boxed_new, with_handle_mut, with_handle_ref};

// ---------------------------------------------------------------------------
// Constants (matching C++ statics)
// ---------------------------------------------------------------------------

const UPPER_BOUND: u32 = 1_000_000_000;
const MAX_SEED: i32 = 161_803_398;
const FACTOR: f64 = 1.0 / UPPER_BOUND as f64;

/// Statistical-approximation thresholds matching `Random.cc`.
const BINOMIAL_TO_NORMAL: u32 = 50;
const BINOMIAL_TO_POISSON: u32 = 1000;

// ---------------------------------------------------------------------------
// Core struct
// ---------------------------------------------------------------------------

/// Rust-native Avida RNG matching the `Apto::RNG::AvidaRNG` algorithm bit-for-bit.
#[derive(Debug, Clone)]
pub struct AvidaRng {
    m_inext: i32,
    m_inextp: i32,
    m_ma: [i32; 56],
    m_orig_seed: i32,
    m_seed: i32,
    m_rand_norm_exprv: f64,
}

impl AvidaRng {
    /// Create a new RNG seeded with `seed`.
    /// If `seed < 0`, a time-and-pid-based seed is chosen automatically.
    pub fn new(seed: i32) -> Self {
        let mut rng = Self {
            m_inext: 0,
            m_inextp: 0,
            m_ma: [0; 56],
            m_orig_seed: 0,
            m_seed: 0,
            m_rand_norm_exprv: 0.0,
        };
        rng.reset_seed(seed);
        rng
    }

    // -- seed accessors -----------------------------------------------------

    #[inline]
    pub fn seed(&self) -> i32 {
        self.m_seed
    }

    #[inline]
    pub fn original_seed(&self) -> i32 {
        self.m_orig_seed
    }

    // -- ResetSeed ----------------------------------------------------------

    /// Corresponds to `Random::ResetSeed`.
    pub fn reset_seed(&mut self, new_seed: i32) {
        self.m_orig_seed = new_seed;
        let mut effective = new_seed;
        if effective < 0 {
            effective = Self::random_seed();
        }
        self.m_seed = effective;
        if self.m_seed > MAX_SEED {
            self.m_seed %= MAX_SEED;
        }
        self.reset();
        self.m_rand_norm_exprv = -self.get_double().ln();
    }

    // -- Core PRNG ----------------------------------------------------------

    /// Knuth subtractive `reset()`.
    fn reset(&mut self) {
        self.m_inext = 0;
        self.m_inextp = 0;
        self.m_ma = [0; 56];

        let mut mj = MAX_SEED - self.m_seed;
        mj %= UPPER_BOUND as i32;
        self.m_ma[55] = mj;
        let mut mk: i32 = 1;

        for i in 1..55 {
            let j = (21 * i) % 55;
            self.m_ma[j] = mk;
            mk = mj - mk;
            if mk < 0 {
                mk += UPPER_BOUND as i32;
            }
            mj = self.m_ma[j];
        }

        for _ in 0..4 {
            for j in 1..55 {
                self.m_ma[j] -= self.m_ma[1 + (j + 30) % 55];
                if self.m_ma[j] < 0 {
                    self.m_ma[j] += UPPER_BOUND as i32;
                }
            }
        }

        self.m_inext = 0;
        self.m_inextp = 31;
    }

    /// Knuth subtractive `getNext()` — returns value in `[0, UPPER_BOUND)`.
    #[inline]
    fn get_next(&mut self) -> u32 {
        self.m_inext += 1;
        if self.m_inext == 56 {
            self.m_inext = 0;
        }
        self.m_inextp += 1;
        if self.m_inextp == 56 {
            self.m_inextp = 0;
        }
        let mut mj = self.m_ma[self.m_inext as usize] - self.m_ma[self.m_inextp as usize];
        if mj < 0 {
            mj += UPPER_BOUND as i32;
        }
        self.m_ma[self.m_inext as usize] = mj;
        mj as u32
    }

    // -- Derived samplers (matching Random base class) ----------------------

    #[inline]
    pub fn get_double(&mut self) -> f64 {
        self.get_next() as f64 * FACTOR
    }

    #[inline]
    pub fn get_double_max(&mut self, max: f64) -> f64 {
        self.get_double() * max
    }

    #[inline]
    pub fn get_double_range(&mut self, min: f64, max: f64) -> f64 {
        self.get_double() * (max - min) + min
    }

    #[inline]
    pub fn get_uint(&mut self, max: u32) -> u32 {
        self.get_double_max(max as f64) as u32
    }

    #[inline]
    pub fn get_int(&mut self, max: i32) -> i32 {
        self.get_double_max(max as f64) as i32
    }

    #[inline]
    pub fn get_int_range(&mut self, min: i32, max: i32) -> i32 {
        self.get_double_range(min as f64, max as f64).floor() as i32
    }

    /// Bernoulli trial: returns `true` with probability `p`.
    #[inline]
    pub fn p(&mut self, prob: f64) -> bool {
        (self.get_next() as f64) < (prob * UPPER_BOUND as f64)
    }

    /// Draw from unit normal via rejection method with cached exponential.
    pub fn get_rand_normal(&mut self) -> f64 {
        let mut exp_rv2;
        loop {
            exp_rv2 = -self.get_double().ln();
            self.m_rand_norm_exprv -= (exp_rv2 - 1.0) * (exp_rv2 - 1.0) / 2.0;
            if self.m_rand_norm_exprv > 0.0 {
                break;
            }
            self.m_rand_norm_exprv = -self.get_double().ln();
        }
        if self.p(0.5) {
            exp_rv2
        } else {
            -exp_rv2
        }
    }

    /// Draw from `Normal(mean, variance)`.
    pub fn get_rand_normal_params(&mut self, mean: f64, variance: f64) -> f64 {
        mean + self.get_rand_normal() * variance.sqrt()
    }

    /// Draw from Poisson(mean) via rejection method.
    pub fn get_rand_poisson(&mut self, mean: f64) -> u32 {
        let a = (-mean).exp();
        if a <= 0.0 {
            return u32::MAX;
        }
        let mut k: u32 = 0;
        let mut u = self.get_double();
        while u >= a {
            u *= self.get_double();
            k += 1;
        }
        k
    }

    /// Poisson approximation used by `GetRandPoisson(n, p)`.
    fn get_rand_poisson_np(&mut self, n: f64, prob: f64) -> u32 {
        if prob > 0.5 {
            return (n as u32).wrapping_sub(self.get_rand_poisson(n * (1.0 - prob)));
        }
        self.get_rand_poisson(n * prob)
    }

    /// Exact binomial via Bernoulli trials.
    fn get_full_rand_binomial(&mut self, n: f64, prob: f64) -> u32 {
        let mut k: u32 = 0;
        for _ in 0..(n as u32) {
            if self.p(prob) {
                k += 1;
            }
        }
        k
    }

    /// Binomial with automatic approximation (normal / Poisson / exact).
    pub fn get_rand_binomial(&mut self, n: f64, prob: f64) -> u32 {
        if n * prob * (1.0 - prob) >= BINOMIAL_TO_NORMAL as f64 {
            return (self.get_rand_normal_params(n * prob, n * prob * (1.0 - prob)) + 0.5) as u32;
        }
        if n >= BINOMIAL_TO_POISSON as f64 {
            let k = self.get_rand_poisson_np(n, prob);
            if k < u32::MAX {
                return k;
            }
        }
        self.get_full_rand_binomial(n, prob)
    }

    // -- Auto-seed helper ---------------------------------------------------

    /// Matches `Random::getRandomSeed()` — `time ^ (pid << 8)`.
    fn random_seed() -> i32 {
        let seed_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);
        let seed_pid = std::process::id() as i32;
        seed_time ^ (seed_pid << 8)
    }
}

// ===========================================================================
// FFI exports
// ===========================================================================

/// Create a new `AvidaRng` seeded with `seed`.  Returns an owning pointer.
/// If `seed < 0`, a time-and-pid-based seed is chosen.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_new(seed: c_int) -> *mut AvidaRng {
    boxed_new(AvidaRng::new(seed))
}

/// Free a previously allocated `AvidaRng`.  Null-safe.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_free(handle: *mut AvidaRng) {
    boxed_free(handle);
}

/// Clone an `AvidaRng` into a new allocation.  Returns null if `handle` is null.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_clone(handle: *const AvidaRng) -> *mut AvidaRng {
    boxed_clone(handle)
}

/// Return the effective (possibly clamped) seed.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_seed(handle: *const AvidaRng) -> c_int {
    with_handle_ref(handle, 0, |rng| rng.seed())
}

/// Return the original seed passed to `ResetSeed` (may be negative for auto-seed).
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_original_seed(handle: *const AvidaRng) -> c_int {
    with_handle_ref(handle, 0, |rng| rng.original_seed())
}

/// Re-seed the generator.  If `seed < 0`, auto-seeds from time + pid.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_reset_seed(handle: *mut AvidaRng, seed: c_int) {
    with_handle_mut(handle, |rng| rng.reset_seed(seed));
}

/// Uniform `[0, 1)`.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_get_double(handle: *mut AvidaRng) -> c_double {
    if handle.is_null() {
        return 0.0;
    }
    // SAFETY: handle checked for null, exclusive borrow.
    let rng = unsafe { &mut *handle };
    rng.get_double()
}

/// Uniform integer `[0, max)`.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_get_uint(handle: *mut AvidaRng, max: c_uint) -> c_uint {
    if handle.is_null() {
        return 0;
    }
    // SAFETY: handle checked for null, exclusive borrow.
    let rng = unsafe { &mut *handle };
    rng.get_uint(max)
}

/// Integer in `[min, max)` via `floor(GetDouble(min, max))`.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_get_int(handle: *mut AvidaRng, min: c_int, max: c_int) -> c_int {
    if handle.is_null() {
        return 0;
    }
    // SAFETY: handle checked for null, exclusive borrow.
    let rng = unsafe { &mut *handle };
    rng.get_int_range(min, max)
}

/// Bernoulli trial returning 1 with probability `p`, else 0.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_p(handle: *mut AvidaRng, p: c_double) -> c_int {
    if handle.is_null() {
        return 0;
    }
    // SAFETY: handle checked for null, exclusive borrow.
    let rng = unsafe { &mut *handle };
    i32::from(rng.p(p))
}

/// Draw from unit normal distribution.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_get_rand_normal(handle: *mut AvidaRng) -> c_double {
    if handle.is_null() {
        return 0.0;
    }
    // SAFETY: handle checked for null, exclusive borrow.
    let rng = unsafe { &mut *handle };
    rng.get_rand_normal()
}

/// Draw from Poisson(mean).
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_get_rand_poisson(handle: *mut AvidaRng, mean: c_double) -> c_uint {
    if handle.is_null() {
        return 0;
    }
    // SAFETY: handle checked for null, exclusive borrow.
    let rng = unsafe { &mut *handle };
    rng.get_rand_poisson(mean)
}

/// Draw from Binomial(n, p) with automatic approximation.
#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rng_get_rand_binomial(
    handle: *mut AvidaRng,
    n: c_double,
    p: c_double,
) -> c_uint {
    if handle.is_null() {
        return 0;
    }
    // SAFETY: handle checked for null, exclusive borrow.
    let rng = unsafe { &mut *handle };
    rng.get_rand_binomial(n, p)
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -- Bit-exact parity with C++ ------------------------------------------

    /// First 20 raw `getNext()` values after `AvidaRNG(100)` construction,
    /// obtained by compiling and running the reference C++ implementation.
    const EXPECTED_GET_NEXT_SEED_100: [u32; 20] = [
        534045303, 362862289, 972668807, 782752198, 174844879, 779736388, 332642683, 770804462,
        379484628, 890644485, 116947897, 574774596, 759443849, 344052125, 338105228, 998362494,
        504383113, 434527125, 278018010, 81155536,
    ];

    /// m_ma table state after `reset(seed=100)` but before the `GetDouble()` call
    /// inside `ResetSeed` consumes the first `getNext()`.
    #[rustfmt::skip]
    const EXPECTED_MA_AFTER_RESET_100: [i32; 56] = [
        0,
        823679825, 411980170, 392845027, 986543833, 45121118,
        554704626, 235401083, 511272474, 640629924, 188349560,
        953570236,  24465075, 890516656, 686024045, 344995335,
        620009746, 607736692, 134494263, 365562388,  93627565,
        938106683,  90234355, 455109815, 954917584, 329115582,
        476737383, 410337741, 778043384, 185524987, 735186829,
        547974196, 859406175, 877934867,  29982738,  13875026,
        262368920, 379859747, 455664695, 178629791, 869825462,
        808864932,  62925751, 907517178, 315742060, 926580196,
            943210, 281904518, 609374198, 630111150, 931035263,
        815609555, 856951147, 242618044, 688388392, 161803298,
    ];

    /// Verify that the internal `reset()` produces the exact m_ma table.
    #[test]
    fn reset_produces_expected_state_table() {
        // Manually run only reset() without the ResetSeed wrapper, so the
        // GetDouble() inside ResetSeed does not consume a getNext().
        let mut rng = AvidaRng {
            m_inext: 0,
            m_inextp: 0,
            m_ma: [0; 56],
            m_orig_seed: 100,
            m_seed: 100,
            m_rand_norm_exprv: 0.0,
        };
        rng.reset();
        assert_eq!(rng.m_ma, EXPECTED_MA_AFTER_RESET_100);
        assert_eq!(rng.m_inext, 0);
        assert_eq!(rng.m_inextp, 31);
    }

    /// Verify that after full construction the first 20 `get_next()` values
    /// match the C++ output bit-for-bit.
    #[test]
    fn bit_exact_get_next_seed_100() {
        let mut rng = AvidaRng::new(100);
        // After construction, ResetSeed has consumed one getNext() for
        // m_rand_norm_exprv. The next 20 must match the C++ sequence.
        for (i, &expected) in EXPECTED_GET_NEXT_SEED_100.iter().enumerate() {
            let actual = rng.get_next();
            assert_eq!(
                actual, expected,
                "getNext() mismatch at index {i}: got {actual}, expected {expected}"
            );
        }
    }

    /// Two RNGs with the same seed produce identical sequences.
    #[test]
    fn same_seed_same_sequence() {
        let mut a = AvidaRng::new(42);
        let mut b = AvidaRng::new(42);
        for _ in 0..200 {
            assert_eq!(a.get_next(), b.get_next());
        }
    }

    /// Deterministic output: same seed always gives same first value.
    #[test]
    fn deterministic_output() {
        let mut r1 = AvidaRng::new(100);
        let v1 = r1.get_next();
        let mut r2 = AvidaRng::new(100);
        let v2 = r2.get_next();
        assert_eq!(v1, v2);
        assert_eq!(v1, EXPECTED_GET_NEXT_SEED_100[0]);
    }

    /// `GetDouble` is in `[0.0, 1.0)`.
    #[test]
    fn get_double_range() {
        let mut rng = AvidaRng::new(7);
        for _ in 0..10_000 {
            let d = rng.get_double();
            assert!(d >= 0.0, "GetDouble() < 0: {d}");
            assert!(d < 1.0, "GetDouble() >= 1: {d}");
        }
    }

    /// `GetUInt(100)` is in `[0, 100)`.
    #[test]
    fn get_uint_range() {
        let mut rng = AvidaRng::new(11);
        for _ in 0..10_000 {
            let v = rng.get_uint(100);
            assert!(v < 100, "GetUInt(100) out of range: {v}");
        }
    }

    /// Bit-exact `GetDouble` parity with C++.
    /// Expected values are computed as `getNext_value / 1_000_000_000.0` which is
    /// the exact operation performed by the RNG (`getNext() * factor`).
    #[test]
    fn bit_exact_get_double_seed_100() {
        let mut rng = AvidaRng::new(100);
        for (i, &raw) in EXPECTED_GET_NEXT_SEED_100[..10].iter().enumerate() {
            let expected = raw as f64 * FACTOR;
            let got = rng.get_double();
            assert!(
                (got - expected).abs() < 1e-15,
                "GetDouble mismatch at {i}: {got} vs {expected}"
            );
        }
    }

    /// Bit-exact `GetUInt(100)` parity with C++.
    #[test]
    fn bit_exact_get_uint_100_seed_100() {
        let expected: [u32; 10] = [53, 36, 97, 78, 17, 77, 33, 77, 37, 89];
        let mut rng = AvidaRng::new(100);
        for (i, &exp) in expected.iter().enumerate() {
            let got = rng.get_uint(100);
            assert_eq!(got, exp, "GetUInt(100) mismatch at {i}: {got} vs {exp}");
        }
    }

    /// Bit-exact P(0.5) parity with C++.
    #[test]
    fn bit_exact_p_half_seed_100() {
        let expected: [bool; 20] = [
            false, true, false, false, true, false, true, false, true, false, true, false, false,
            true, true, false, false, true, true, true,
        ];
        let mut rng = AvidaRng::new(100);
        for (i, &exp) in expected.iter().enumerate() {
            let got = rng.p(0.5);
            assert_eq!(got, exp, "P(0.5) mismatch at {i}: {got} vs {exp}");
        }
    }

    /// Seed clamping: seeds > MAX_SEED are reduced via modulo.
    #[test]
    fn seed_clamped_above_max_seed() {
        let rng = AvidaRng::new(200_000_000);
        assert!(rng.seed() <= MAX_SEED);
        assert_eq!(rng.seed(), 200_000_000 % MAX_SEED);
    }

    /// `m_rand_norm_exprv` matches C++ after construction with seed=100.
    #[test]
    fn rand_norm_exprv_after_construction() {
        let rng = AvidaRng::new(100);
        let expected = 0.036_380_155_360_250_466_f64;
        assert!(
            (rng.m_rand_norm_exprv - expected).abs() < 1e-15,
            "m_rand_norm_exprv: {} vs {expected}",
            rng.m_rand_norm_exprv
        );
    }

    /// Clone produces identical subsequent output.
    #[test]
    fn clone_parity() {
        let mut rng = AvidaRng::new(99);
        // Advance a few steps
        for _ in 0..50 {
            rng.get_double();
        }
        let mut cloned = rng.clone();
        for _ in 0..100 {
            assert_eq!(rng.get_next(), cloned.get_next());
        }
    }

    /// `GetRandNormal` returns finite values and is roughly symmetric around zero.
    #[test]
    fn get_rand_normal_basic_properties() {
        let mut rng = AvidaRng::new(55);
        let mut sum = 0.0;
        let n = 5_000;
        for _ in 0..n {
            let v = rng.get_rand_normal();
            assert!(v.is_finite(), "GetRandNormal returned non-finite: {v}");
            sum += v;
        }
        let mean = sum / n as f64;
        // With 5000 samples the mean should be close to zero (within ~0.1 with
        // very high probability).
        assert!(
            mean.abs() < 0.15,
            "GetRandNormal mean too far from 0: {mean}"
        );
    }

    /// `GetRandPoisson` basic sanity.
    #[test]
    fn get_rand_poisson_basic() {
        let mut rng = AvidaRng::new(33);
        let mut sum: u64 = 0;
        let n = 5_000;
        let lambda = 3.0;
        for _ in 0..n {
            sum += rng.get_rand_poisson(lambda) as u64;
        }
        let mean = sum as f64 / n as f64;
        assert!(
            (mean - lambda).abs() < 0.3,
            "Poisson mean {mean} too far from {lambda}"
        );
    }

    /// `GetRandBinomial` basic sanity.
    #[test]
    fn get_rand_binomial_basic() {
        let mut rng = AvidaRng::new(77);
        let n_trials = 20.0;
        let p = 0.3;
        let mut sum: u64 = 0;
        let samples = 5_000;
        for _ in 0..samples {
            sum += rng.get_rand_binomial(n_trials, p) as u64;
        }
        let mean = sum as f64 / samples as f64;
        let expected_mean = n_trials * p;
        assert!(
            (mean - expected_mean).abs() < 0.5,
            "Binomial mean {mean} too far from {expected_mean}"
        );
    }

    /// `reset_seed` properly resets the generator.
    #[test]
    fn reset_seed_resets_state() {
        let mut rng = AvidaRng::new(100);
        // Advance
        for _ in 0..200 {
            rng.get_double();
        }
        // Re-seed with same value
        rng.reset_seed(100);
        // Must match a fresh RNG
        let mut fresh = AvidaRng::new(100);
        for _ in 0..50 {
            assert_eq!(rng.get_next(), fresh.get_next());
        }
    }

    // -- FFI null-safety tests ----------------------------------------------

    #[test]
    fn ffi_null_safety() {
        // All FFI functions must tolerate null without crashing.
        avd_rng_free(std::ptr::null_mut());

        assert!(avd_rng_clone(std::ptr::null()).is_null());

        assert_eq!(avd_rng_seed(std::ptr::null()), 0);
        assert_eq!(avd_rng_original_seed(std::ptr::null()), 0);

        avd_rng_reset_seed(std::ptr::null_mut(), 42);

        assert_eq!(avd_rng_get_double(std::ptr::null_mut()), 0.0);
        assert_eq!(avd_rng_get_uint(std::ptr::null_mut(), 100), 0);
        assert_eq!(avd_rng_get_int(std::ptr::null_mut(), 0, 100), 0);
        assert_eq!(avd_rng_p(std::ptr::null_mut(), 0.5), 0);
        assert_eq!(avd_rng_get_rand_normal(std::ptr::null_mut()), 0.0);
        assert_eq!(avd_rng_get_rand_poisson(std::ptr::null_mut(), 3.0), 0);
        assert_eq!(
            avd_rng_get_rand_binomial(std::ptr::null_mut(), 10.0, 0.3),
            0
        );
    }

    /// FFI round-trip: new -> use -> clone -> free.
    #[test]
    fn ffi_round_trip() {
        let h = avd_rng_new(100);
        assert!(!h.is_null());
        assert_eq!(avd_rng_seed(h), 100);
        assert_eq!(avd_rng_original_seed(h), 100);

        let d = avd_rng_get_double(h);
        assert!((0.0..1.0).contains(&d));

        let u = avd_rng_get_uint(h, 50);
        assert!(u < 50);

        let iv = avd_rng_get_int(h, 10, 20);
        assert!((10..20).contains(&iv));

        let pv = avd_rng_p(h, 0.5);
        assert!(pv == 0 || pv == 1);

        let h2 = avd_rng_clone(h);
        assert!(!h2.is_null());

        // Cloned handle should produce same values as original from this point.
        let d1 = avd_rng_get_double(h);
        let d2 = avd_rng_get_double(h2);
        assert_eq!(d1, d2);

        avd_rng_reset_seed(h, 999);
        assert_eq!(avd_rng_seed(h), 999);

        avd_rng_free(h);
        avd_rng_free(h2);
    }
}
