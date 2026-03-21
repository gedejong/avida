//! Rust-native MutationRates type replacing cMutationRates internals.
//!
//! Pure data bag of 37 f64 fields organized into 6 sub-structs matching the
//! C++ memory layout. The `Test*` methods that use RNG stay in C++ as inline
//! wrappers; only data storage and accessors live here.

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CopyMuts {
    pub mut_prob: f64,
    pub ins_prob: f64,
    pub del_prob: f64,
    pub uniform_prob: f64,
    pub slip_prob: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DivideMuts {
    pub ins_prob: f64,
    pub del_prob: f64,
    pub mut_prob: f64,
    pub uniform_prob: f64,
    pub slip_prob: f64,
    pub trans_prob: f64,
    pub lgt_prob: f64,
    pub divide_mut_prob: f64,
    pub divide_ins_prob: f64,
    pub divide_del_prob: f64,
    pub divide_uniform_prob: f64,
    pub divide_slip_prob: f64,
    pub divide_trans_prob: f64,
    pub divide_lgt_prob: f64,
    pub divide_poisson_mut_mean: f64,
    pub divide_poisson_ins_mean: f64,
    pub divide_poisson_del_mean: f64,
    pub divide_poisson_slip_mean: f64,
    pub divide_poisson_trans_mean: f64,
    pub divide_poisson_lgt_mean: f64,
    pub parent_mut_prob: f64,
    pub parent_ins_prob: f64,
    pub parent_del_prob: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct PointMuts {
    pub ins_prob: f64,
    pub del_prob: f64,
    pub mut_prob: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct InjectMuts {
    pub ins_prob: f64,
    pub del_prob: f64,
    pub mut_prob: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct MetaMuts {
    pub copy_mut_prob: f64,
    pub standard_dev: f64,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct UpdateMuts {
    pub death_prob: f64,
}

// --- Main struct ---

#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct MutationRates {
    pub copy: CopyMuts,
    pub divide: DivideMuts,
    pub point: PointMuts,
    pub inject: InjectMuts,
    pub meta: MetaMuts,
    pub update: UpdateMuts,
}

impl MutationRates {
    pub fn clear(&mut self) {
        *self = Self::default();
    }
}

// --- FFI interface ---

#[no_mangle]
pub extern "C" fn avd_mutation_rates_default() -> MutationRates {
    MutationRates::default()
}

#[no_mangle]
pub extern "C" fn avd_mutation_rates_clear(m: &mut MutationRates) {
    m.clear();
}

// Copy mutation getters/setters
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_copy_mut_prob(m: &MutationRates) -> f64 {
    m.copy.mut_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_copy_mut_prob(m: &mut MutationRates, v: f64) {
    m.copy.mut_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_copy_ins_prob(m: &MutationRates) -> f64 {
    m.copy.ins_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_copy_ins_prob(m: &mut MutationRates, v: f64) {
    m.copy.ins_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_copy_del_prob(m: &MutationRates) -> f64 {
    m.copy.del_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_copy_del_prob(m: &mut MutationRates, v: f64) {
    m.copy.del_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_copy_uniform_prob(m: &MutationRates) -> f64 {
    m.copy.uniform_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_copy_uniform_prob(m: &mut MutationRates, v: f64) {
    m.copy.uniform_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_copy_slip_prob(m: &MutationRates) -> f64 {
    m.copy.slip_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_copy_slip_prob(m: &mut MutationRates, v: f64) {
    m.copy.slip_prob = v;
}

// Divide per-site getters/setters
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_div_ins_prob(m: &MutationRates) -> f64 {
    m.divide.ins_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_div_ins_prob(m: &mut MutationRates, v: f64) {
    m.divide.ins_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_div_del_prob(m: &MutationRates) -> f64 {
    m.divide.del_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_div_del_prob(m: &mut MutationRates, v: f64) {
    m.divide.del_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_div_mut_prob(m: &MutationRates) -> f64 {
    m.divide.mut_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_div_mut_prob(m: &mut MutationRates, v: f64) {
    m.divide.mut_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_div_uniform_prob(m: &MutationRates) -> f64 {
    m.divide.uniform_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_div_uniform_prob(m: &mut MutationRates, v: f64) {
    m.divide.uniform_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_div_slip_prob(m: &MutationRates) -> f64 {
    m.divide.slip_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_div_slip_prob(m: &mut MutationRates, v: f64) {
    m.divide.slip_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_div_trans_prob(m: &MutationRates) -> f64 {
    m.divide.trans_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_div_trans_prob(m: &mut MutationRates, v: f64) {
    m.divide.trans_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_div_lgt_prob(m: &MutationRates) -> f64 {
    m.divide.lgt_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_div_lgt_prob(m: &mut MutationRates, v: f64) {
    m.divide.lgt_prob = v;
}

// Divide per-divide getters/setters
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_mut_prob(m: &MutationRates) -> f64 {
    m.divide.divide_mut_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_mut_prob(m: &mut MutationRates, v: f64) {
    m.divide.divide_mut_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_ins_prob(m: &MutationRates) -> f64 {
    m.divide.divide_ins_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_ins_prob(m: &mut MutationRates, v: f64) {
    m.divide.divide_ins_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_del_prob(m: &MutationRates) -> f64 {
    m.divide.divide_del_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_del_prob(m: &mut MutationRates, v: f64) {
    m.divide.divide_del_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_uniform_prob(m: &MutationRates) -> f64 {
    m.divide.divide_uniform_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_uniform_prob(m: &mut MutationRates, v: f64) {
    m.divide.divide_uniform_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_slip_prob(m: &MutationRates) -> f64 {
    m.divide.divide_slip_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_slip_prob(m: &mut MutationRates, v: f64) {
    m.divide.divide_slip_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_trans_prob(m: &MutationRates) -> f64 {
    m.divide.divide_trans_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_trans_prob(m: &mut MutationRates, v: f64) {
    m.divide.divide_trans_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_lgt_prob(m: &MutationRates) -> f64 {
    m.divide.divide_lgt_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_lgt_prob(m: &mut MutationRates, v: f64) {
    m.divide.divide_lgt_prob = v;
}

// Divide Poisson mean getters/setters
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_poisson_mut_mean(m: &MutationRates) -> f64 {
    m.divide.divide_poisson_mut_mean
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_poisson_mut_mean(m: &mut MutationRates, v: f64) {
    m.divide.divide_poisson_mut_mean = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_poisson_ins_mean(m: &MutationRates) -> f64 {
    m.divide.divide_poisson_ins_mean
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_poisson_ins_mean(m: &mut MutationRates, v: f64) {
    m.divide.divide_poisson_ins_mean = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_poisson_del_mean(m: &MutationRates) -> f64 {
    m.divide.divide_poisson_del_mean
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_poisson_del_mean(m: &mut MutationRates, v: f64) {
    m.divide.divide_poisson_del_mean = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_poisson_slip_mean(m: &MutationRates) -> f64 {
    m.divide.divide_poisson_slip_mean
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_poisson_slip_mean(m: &mut MutationRates, v: f64) {
    m.divide.divide_poisson_slip_mean = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_poisson_trans_mean(m: &MutationRates) -> f64 {
    m.divide.divide_poisson_trans_mean
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_poisson_trans_mean(m: &mut MutationRates, v: f64) {
    m.divide.divide_poisson_trans_mean = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_divide_poisson_lgt_mean(m: &MutationRates) -> f64 {
    m.divide.divide_poisson_lgt_mean
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_divide_poisson_lgt_mean(m: &mut MutationRates, v: f64) {
    m.divide.divide_poisson_lgt_mean = v;
}

// Parent mutation getters/setters
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_parent_mut_prob(m: &MutationRates) -> f64 {
    m.divide.parent_mut_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_parent_mut_prob(m: &mut MutationRates, v: f64) {
    m.divide.parent_mut_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_parent_ins_prob(m: &MutationRates) -> f64 {
    m.divide.parent_ins_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_parent_ins_prob(m: &mut MutationRates, v: f64) {
    m.divide.parent_ins_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_parent_del_prob(m: &MutationRates) -> f64 {
    m.divide.parent_del_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_parent_del_prob(m: &mut MutationRates, v: f64) {
    m.divide.parent_del_prob = v;
}

// Point mutation getters/setters
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_point_ins_prob(m: &MutationRates) -> f64 {
    m.point.ins_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_point_ins_prob(m: &mut MutationRates, v: f64) {
    m.point.ins_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_point_del_prob(m: &MutationRates) -> f64 {
    m.point.del_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_point_del_prob(m: &mut MutationRates, v: f64) {
    m.point.del_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_point_mut_prob(m: &MutationRates) -> f64 {
    m.point.mut_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_point_mut_prob(m: &mut MutationRates, v: f64) {
    m.point.mut_prob = v;
}

// Inject mutation getters/setters
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_inject_ins_prob(m: &MutationRates) -> f64 {
    m.inject.ins_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_inject_ins_prob(m: &mut MutationRates, v: f64) {
    m.inject.ins_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_inject_del_prob(m: &MutationRates) -> f64 {
    m.inject.del_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_inject_del_prob(m: &mut MutationRates, v: f64) {
    m.inject.del_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_inject_mut_prob(m: &MutationRates) -> f64 {
    m.inject.mut_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_inject_mut_prob(m: &mut MutationRates, v: f64) {
    m.inject.mut_prob = v;
}

// Meta mutation getters/setters
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_meta_copy_mut_prob(m: &MutationRates) -> f64 {
    m.meta.copy_mut_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_meta_copy_mut_prob(m: &mut MutationRates, v: f64) {
    m.meta.copy_mut_prob = v;
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_meta_standard_dev(m: &MutationRates) -> f64 {
    m.meta.standard_dev
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_meta_standard_dev(m: &mut MutationRates, v: f64) {
    m.meta.standard_dev = v;
}

// Update mutation getters/setters
#[no_mangle]
pub extern "C" fn avd_mutation_rates_get_death_prob(m: &MutationRates) -> f64 {
    m.update.death_prob
}
#[no_mangle]
pub extern "C" fn avd_mutation_rates_set_death_prob(m: &mut MutationRates, v: f64) {
    m.update.death_prob = v;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_all_zeros() {
        let m = MutationRates::default();
        assert_eq!(m.copy.mut_prob, 0.0);
        assert_eq!(m.copy.ins_prob, 0.0);
        assert_eq!(m.copy.del_prob, 0.0);
        assert_eq!(m.copy.uniform_prob, 0.0);
        assert_eq!(m.copy.slip_prob, 0.0);
        assert_eq!(m.divide.ins_prob, 0.0);
        assert_eq!(m.divide.divide_poisson_mut_mean, 0.0);
        assert_eq!(m.divide.parent_del_prob, 0.0);
        assert_eq!(m.point.mut_prob, 0.0);
        assert_eq!(m.inject.mut_prob, 0.0);
        assert_eq!(m.meta.copy_mut_prob, 0.0);
        assert_eq!(m.meta.standard_dev, 0.0);
        assert_eq!(m.update.death_prob, 0.0);
    }

    #[test]
    fn clear_resets_to_zero() {
        let mut m = MutationRates::default();
        m.copy.mut_prob = 0.5;
        m.divide.ins_prob = 0.1;
        m.point.del_prob = 0.2;
        m.inject.mut_prob = 0.3;
        m.meta.copy_mut_prob = 0.4;
        m.update.death_prob = 0.05;
        m.clear();
        assert_eq!(m, MutationRates::default());
    }

    #[test]
    fn copy_semantics() {
        let mut m = MutationRates::default();
        m.copy.mut_prob = 0.123;
        m.divide.divide_poisson_lgt_mean = 0.456;
        m.update.death_prob = 0.789;

        let m2 = m;
        assert_eq!(m2.copy.mut_prob, 0.123);
        assert_eq!(m2.divide.divide_poisson_lgt_mean, 0.456);
        assert_eq!(m2.update.death_prob, 0.789);
    }

    #[test]
    fn ffi_default_roundtrip() {
        let m = avd_mutation_rates_default();
        assert_eq!(avd_mutation_rates_get_copy_mut_prob(&m), 0.0);
        assert_eq!(avd_mutation_rates_get_death_prob(&m), 0.0);
    }

    #[test]
    fn ffi_clear() {
        let mut m = avd_mutation_rates_default();
        avd_mutation_rates_set_copy_mut_prob(&mut m, 0.5);
        avd_mutation_rates_set_death_prob(&mut m, 0.1);
        avd_mutation_rates_clear(&mut m);
        assert_eq!(avd_mutation_rates_get_copy_mut_prob(&m), 0.0);
        assert_eq!(avd_mutation_rates_get_death_prob(&m), 0.0);
    }

    #[test]
    fn ffi_all_copy_getters_setters() {
        let mut m = avd_mutation_rates_default();
        avd_mutation_rates_set_copy_mut_prob(&mut m, 0.01);
        avd_mutation_rates_set_copy_ins_prob(&mut m, 0.02);
        avd_mutation_rates_set_copy_del_prob(&mut m, 0.03);
        avd_mutation_rates_set_copy_uniform_prob(&mut m, 0.04);
        avd_mutation_rates_set_copy_slip_prob(&mut m, 0.05);
        assert_eq!(avd_mutation_rates_get_copy_mut_prob(&m), 0.01);
        assert_eq!(avd_mutation_rates_get_copy_ins_prob(&m), 0.02);
        assert_eq!(avd_mutation_rates_get_copy_del_prob(&m), 0.03);
        assert_eq!(avd_mutation_rates_get_copy_uniform_prob(&m), 0.04);
        assert_eq!(avd_mutation_rates_get_copy_slip_prob(&m), 0.05);
    }

    #[test]
    fn ffi_all_divide_per_site_getters_setters() {
        let mut m = avd_mutation_rates_default();
        avd_mutation_rates_set_div_ins_prob(&mut m, 0.1);
        avd_mutation_rates_set_div_del_prob(&mut m, 0.2);
        avd_mutation_rates_set_div_mut_prob(&mut m, 0.3);
        avd_mutation_rates_set_div_uniform_prob(&mut m, 0.4);
        avd_mutation_rates_set_div_slip_prob(&mut m, 0.5);
        avd_mutation_rates_set_div_trans_prob(&mut m, 0.6);
        avd_mutation_rates_set_div_lgt_prob(&mut m, 0.7);
        assert_eq!(avd_mutation_rates_get_div_ins_prob(&m), 0.1);
        assert_eq!(avd_mutation_rates_get_div_del_prob(&m), 0.2);
        assert_eq!(avd_mutation_rates_get_div_mut_prob(&m), 0.3);
        assert_eq!(avd_mutation_rates_get_div_uniform_prob(&m), 0.4);
        assert_eq!(avd_mutation_rates_get_div_slip_prob(&m), 0.5);
        assert_eq!(avd_mutation_rates_get_div_trans_prob(&m), 0.6);
        assert_eq!(avd_mutation_rates_get_div_lgt_prob(&m), 0.7);
    }

    #[test]
    fn ffi_all_divide_per_divide_getters_setters() {
        let mut m = avd_mutation_rates_default();
        avd_mutation_rates_set_divide_mut_prob(&mut m, 0.11);
        avd_mutation_rates_set_divide_ins_prob(&mut m, 0.12);
        avd_mutation_rates_set_divide_del_prob(&mut m, 0.13);
        avd_mutation_rates_set_divide_uniform_prob(&mut m, 0.14);
        avd_mutation_rates_set_divide_slip_prob(&mut m, 0.15);
        avd_mutation_rates_set_divide_trans_prob(&mut m, 0.16);
        avd_mutation_rates_set_divide_lgt_prob(&mut m, 0.17);
        assert_eq!(avd_mutation_rates_get_divide_mut_prob(&m), 0.11);
        assert_eq!(avd_mutation_rates_get_divide_ins_prob(&m), 0.12);
        assert_eq!(avd_mutation_rates_get_divide_del_prob(&m), 0.13);
        assert_eq!(avd_mutation_rates_get_divide_uniform_prob(&m), 0.14);
        assert_eq!(avd_mutation_rates_get_divide_slip_prob(&m), 0.15);
        assert_eq!(avd_mutation_rates_get_divide_trans_prob(&m), 0.16);
        assert_eq!(avd_mutation_rates_get_divide_lgt_prob(&m), 0.17);
    }

    #[test]
    fn ffi_poisson_means() {
        let mut m = avd_mutation_rates_default();
        avd_mutation_rates_set_divide_poisson_mut_mean(&mut m, 1.0);
        avd_mutation_rates_set_divide_poisson_ins_mean(&mut m, 2.0);
        avd_mutation_rates_set_divide_poisson_del_mean(&mut m, 3.0);
        avd_mutation_rates_set_divide_poisson_slip_mean(&mut m, 4.0);
        avd_mutation_rates_set_divide_poisson_trans_mean(&mut m, 5.0);
        avd_mutation_rates_set_divide_poisson_lgt_mean(&mut m, 6.0);
        assert_eq!(avd_mutation_rates_get_divide_poisson_mut_mean(&m), 1.0);
        assert_eq!(avd_mutation_rates_get_divide_poisson_ins_mean(&m), 2.0);
        assert_eq!(avd_mutation_rates_get_divide_poisson_del_mean(&m), 3.0);
        assert_eq!(avd_mutation_rates_get_divide_poisson_slip_mean(&m), 4.0);
        assert_eq!(avd_mutation_rates_get_divide_poisson_trans_mean(&m), 5.0);
        assert_eq!(avd_mutation_rates_get_divide_poisson_lgt_mean(&m), 6.0);
    }

    #[test]
    fn ffi_parent_point_inject_meta_update() {
        let mut m = avd_mutation_rates_default();
        avd_mutation_rates_set_parent_mut_prob(&mut m, 0.01);
        avd_mutation_rates_set_parent_ins_prob(&mut m, 0.02);
        avd_mutation_rates_set_parent_del_prob(&mut m, 0.03);
        avd_mutation_rates_set_point_ins_prob(&mut m, 0.04);
        avd_mutation_rates_set_point_del_prob(&mut m, 0.05);
        avd_mutation_rates_set_point_mut_prob(&mut m, 0.06);
        avd_mutation_rates_set_inject_ins_prob(&mut m, 0.07);
        avd_mutation_rates_set_inject_del_prob(&mut m, 0.08);
        avd_mutation_rates_set_inject_mut_prob(&mut m, 0.09);
        avd_mutation_rates_set_meta_copy_mut_prob(&mut m, 0.10);
        avd_mutation_rates_set_meta_standard_dev(&mut m, 0.11);
        avd_mutation_rates_set_death_prob(&mut m, 0.12);

        assert_eq!(avd_mutation_rates_get_parent_mut_prob(&m), 0.01);
        assert_eq!(avd_mutation_rates_get_parent_ins_prob(&m), 0.02);
        assert_eq!(avd_mutation_rates_get_parent_del_prob(&m), 0.03);
        assert_eq!(avd_mutation_rates_get_point_ins_prob(&m), 0.04);
        assert_eq!(avd_mutation_rates_get_point_del_prob(&m), 0.05);
        assert_eq!(avd_mutation_rates_get_point_mut_prob(&m), 0.06);
        assert_eq!(avd_mutation_rates_get_inject_ins_prob(&m), 0.07);
        assert_eq!(avd_mutation_rates_get_inject_del_prob(&m), 0.08);
        assert_eq!(avd_mutation_rates_get_inject_mut_prob(&m), 0.09);
        assert_eq!(avd_mutation_rates_get_meta_copy_mut_prob(&m), 0.10);
        assert_eq!(avd_mutation_rates_get_meta_standard_dev(&m), 0.11);
        assert_eq!(avd_mutation_rates_get_death_prob(&m), 0.12);
    }

    #[test]
    fn repr_c_layout_field_count() {
        // Verify total size matches 37 f64 fields
        assert_eq!(
            std::mem::size_of::<MutationRates>(),
            37 * std::mem::size_of::<f64>()
        );
    }

    #[test]
    fn sub_struct_sizes() {
        assert_eq!(
            std::mem::size_of::<CopyMuts>(),
            5 * std::mem::size_of::<f64>()
        );
        assert_eq!(
            std::mem::size_of::<DivideMuts>(),
            23 * std::mem::size_of::<f64>()
        );
        assert_eq!(
            std::mem::size_of::<PointMuts>(),
            3 * std::mem::size_of::<f64>()
        );
        assert_eq!(
            std::mem::size_of::<InjectMuts>(),
            3 * std::mem::size_of::<f64>()
        );
        assert_eq!(
            std::mem::size_of::<MetaMuts>(),
            2 * std::mem::size_of::<f64>()
        );
        assert_eq!(
            std::mem::size_of::<UpdateMuts>(),
            std::mem::size_of::<f64>()
        );
    }
}
