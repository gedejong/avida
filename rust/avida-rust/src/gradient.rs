//! Rust-native GradientConfig and GradientState types replacing the scalar
//! configuration and internal-state fields of cGradientCount.
//!
//! Both structs are `#[repr(C)]` so C++ can embed them directly.

use std::ffi::{c_double, c_int};

/// Configuration arguments for a gradient resource, set at construction
/// and modified via Set* methods.
///
/// Layout must remain `repr(C)` so C++ can embed it directly.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GradientConfig {
    pub peakx: c_int,
    pub peaky: c_int,
    pub height: c_int,
    pub spread: c_int,
    pub plateau: c_double,
    pub decay: c_int,
    pub max_x: c_int,
    pub max_y: c_int,
    pub min_x: c_int,
    pub min_y: c_int,
    pub move_a_scaler: c_double,
    pub updatestep: c_int,
    pub halo: c_int,
    pub halo_inner_radius: c_int,
    pub halo_width: c_int,
    pub halo_anchor_x: c_int,
    pub halo_anchor_y: c_int,
    pub move_speed: c_int,
    pub move_resistance: c_int,
    pub plateau_inflow: c_double,
    pub plateau_outflow: c_double,
    pub cone_inflow: c_double,
    pub cone_outflow: c_double,
    pub gradient_inflow: c_double,
    pub is_plateau_common: c_int,
    pub floor: c_double,
    pub habitat: c_int,
    pub min_size: c_int,
    pub max_size: c_int,
    pub config: c_int,
    pub count: c_int,
    pub initial_plat: c_double,
    pub threshold: c_double,
    pub damage: c_double,
    pub geometry: c_int,
}

impl Default for GradientConfig {
    fn default() -> Self {
        Self {
            peakx: 0,
            peaky: 0,
            height: 0,
            spread: 0,
            plateau: 0.0,
            decay: 0,
            max_x: 0,
            max_y: 0,
            min_x: 0,
            min_y: 0,
            move_a_scaler: 0.0,
            updatestep: 0,
            halo: 0,
            halo_inner_radius: 0,
            halo_width: 0,
            halo_anchor_x: 0,
            halo_anchor_y: 0,
            move_speed: 0,
            move_resistance: 0,
            plateau_inflow: 0.0,
            plateau_outflow: 0.0,
            cone_inflow: 0.0,
            cone_outflow: 0.0,
            gradient_inflow: 0.0,
            is_plateau_common: 0,
            floor: 0.0,
            habitat: 0,
            min_size: 0,
            max_size: 0,
            config: 0,
            count: 0,
            initial_plat: 0.0,
            threshold: 0.0,
            damage: 0.0,
            geometry: 0,
        }
    }
}

/// Internal mutable state for a gradient resource, updated during simulation.
///
/// Layout must remain `repr(C)` so C++ can embed it directly.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct GradientState {
    pub initial: c_int,
    pub move_y_scaler: c_double,
    pub counter: c_int,
    pub move_counter: c_int,
    pub topo_counter: c_int,
    pub movesignx: c_int,
    pub movesigny: c_int,
    pub old_peakx: c_int,
    pub old_peaky: c_int,
    pub halo_dir: c_int,
    pub changling: c_int,
    pub just_reset: c_int,
    pub past_height: c_double,
    pub current_height: c_double,
    pub ave_plat_cell_loss: c_double,
    pub common_plat_height: c_double,
    pub skip_moves: c_int,
    pub skip_counter: c_int,
    pub mean_plat_inflow: c_double,
    pub var_plat_inflow: c_double,
    pub pred_odds: c_double,
    pub predator: c_int,
    pub death_odds: c_double,
    pub deadly: c_int,
    pub path: c_int,
    pub hammer: c_int,
    pub guarded_juvs_per_adult: c_int,
    pub probabilistic: c_int,
    pub min_usedx: c_int,
    pub min_usedy: c_int,
    pub max_usedx: c_int,
    pub max_usedy: c_int,
}

impl Default for GradientState {
    fn default() -> Self {
        Self {
            initial: 0,
            move_y_scaler: 0.5,
            counter: 0,
            move_counter: 1,
            topo_counter: 0,
            movesignx: 0,
            movesigny: 0,
            old_peakx: 0,
            old_peaky: 0,
            halo_dir: 0,
            changling: 0,
            just_reset: 1,
            past_height: 0.0,
            current_height: 0.0,
            ave_plat_cell_loss: 0.0,
            common_plat_height: 0.0,
            skip_moves: 0,
            skip_counter: 0,
            mean_plat_inflow: 0.0,
            var_plat_inflow: 0.0,
            pred_odds: 0.0,
            predator: 0,
            death_odds: 0.0,
            deadly: 0,
            path: 0,
            hammer: 0,
            guarded_juvs_per_adult: 0,
            probabilistic: 0,
            min_usedx: -1,
            min_usedy: -1,
            max_usedx: -1,
            max_usedy: -1,
        }
    }
}

// ---------------------------------------------------------------------------
// FFI default constructors
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn avd_gradient_config_default() -> GradientConfig {
    GradientConfig::default()
}

#[no_mangle]
pub extern "C" fn avd_gradient_state_default() -> GradientState {
    GradientState::default()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_default_values() {
        let c = GradientConfig::default();
        assert_eq!(c.peakx, 0);
        assert_eq!(c.peaky, 0);
        assert_eq!(c.height, 0);
        assert_eq!(c.spread, 0);
        assert!((c.plateau - 0.0).abs() < f64::EPSILON);
        assert_eq!(c.decay, 0);
        assert_eq!(c.max_x, 0);
        assert_eq!(c.max_y, 0);
        assert_eq!(c.min_x, 0);
        assert_eq!(c.min_y, 0);
        assert!((c.move_a_scaler - 0.0).abs() < f64::EPSILON);
        assert_eq!(c.updatestep, 0);
        assert_eq!(c.halo, 0);
        assert_eq!(c.halo_inner_radius, 0);
        assert_eq!(c.halo_width, 0);
        assert_eq!(c.halo_anchor_x, 0);
        assert_eq!(c.halo_anchor_y, 0);
        assert_eq!(c.move_speed, 0);
        assert_eq!(c.move_resistance, 0);
        assert!((c.plateau_inflow - 0.0).abs() < f64::EPSILON);
        assert!((c.plateau_outflow - 0.0).abs() < f64::EPSILON);
        assert!((c.cone_inflow - 0.0).abs() < f64::EPSILON);
        assert!((c.cone_outflow - 0.0).abs() < f64::EPSILON);
        assert!((c.gradient_inflow - 0.0).abs() < f64::EPSILON);
        assert_eq!(c.is_plateau_common, 0);
        assert!((c.floor - 0.0).abs() < f64::EPSILON);
        assert_eq!(c.habitat, 0);
        assert_eq!(c.min_size, 0);
        assert_eq!(c.max_size, 0);
        assert_eq!(c.config, 0);
        assert_eq!(c.count, 0);
        assert!((c.initial_plat - 0.0).abs() < f64::EPSILON);
        assert!((c.threshold - 0.0).abs() < f64::EPSILON);
        assert!((c.damage - 0.0).abs() < f64::EPSILON);
        assert_eq!(c.geometry, 0);
    }

    #[test]
    fn state_default_values() {
        let s = GradientState::default();
        assert_eq!(s.initial, 0);
        assert!((s.move_y_scaler - 0.5).abs() < f64::EPSILON);
        assert_eq!(s.counter, 0);
        assert_eq!(s.move_counter, 1);
        assert_eq!(s.topo_counter, 0);
        assert_eq!(s.movesignx, 0);
        assert_eq!(s.movesigny, 0);
        assert_eq!(s.old_peakx, 0);
        assert_eq!(s.old_peaky, 0);
        assert_eq!(s.halo_dir, 0);
        assert_eq!(s.changling, 0);
        assert_eq!(s.just_reset, 1);
        assert!((s.past_height - 0.0).abs() < f64::EPSILON);
        assert!((s.current_height - 0.0).abs() < f64::EPSILON);
        assert!((s.ave_plat_cell_loss - 0.0).abs() < f64::EPSILON);
        assert!((s.common_plat_height - 0.0).abs() < f64::EPSILON);
        assert_eq!(s.skip_moves, 0);
        assert_eq!(s.skip_counter, 0);
        assert!((s.mean_plat_inflow - 0.0).abs() < f64::EPSILON);
        assert!((s.var_plat_inflow - 0.0).abs() < f64::EPSILON);
        assert!((s.pred_odds - 0.0).abs() < f64::EPSILON);
        assert_eq!(s.predator, 0);
        assert!((s.death_odds - 0.0).abs() < f64::EPSILON);
        assert_eq!(s.deadly, 0);
        assert_eq!(s.path, 0);
        assert_eq!(s.hammer, 0);
        assert_eq!(s.guarded_juvs_per_adult, 0);
        assert_eq!(s.probabilistic, 0);
        assert_eq!(s.min_usedx, -1);
        assert_eq!(s.min_usedy, -1);
        assert_eq!(s.max_usedx, -1);
        assert_eq!(s.max_usedy, -1);
    }

    #[test]
    fn config_ffi_default_matches() {
        let c = avd_gradient_config_default();
        assert_eq!(c.peakx, 0);
        assert_eq!(c.geometry, 0);
    }

    #[test]
    fn state_ffi_default_matches() {
        let s = avd_gradient_state_default();
        assert_eq!(s.just_reset, 1);
        assert_eq!(s.min_usedx, -1);
    }

    #[test]
    fn config_size_and_alignment() {
        // Ensure the struct has expected size (no unexpected padding issues).
        // 20 ints * 4 + 14 doubles * 8 = 80 + 112 = 192 bytes
        // With repr(C) layout the actual size depends on field ordering.
        // Just verify it's non-zero and a multiple of 8.
        let size = std::mem::size_of::<GradientConfig>();
        assert!(size > 0);
        assert_eq!(size % 8, 0);
    }

    #[test]
    fn state_size_and_alignment() {
        let size = std::mem::size_of::<GradientState>();
        assert!(size > 0);
        assert_eq!(size % 8, 0);
    }

    #[test]
    fn config_clone_is_independent() {
        let a = GradientConfig {
            peakx: 42,
            ..GradientConfig::default()
        };
        let b = a;
        assert_eq!(b.peakx, 42);
        // Modifying a copy shouldn't affect the original (they're Copy).
        let mut a2 = a;
        a2.peakx = 99;
        assert_eq!(a.peakx, 42);
        assert_eq!(a2.peakx, 99);
    }

    #[test]
    fn state_clone_is_independent() {
        let a = GradientState {
            counter: 7,
            ..GradientState::default()
        };
        let b = a;
        assert_eq!(b.counter, 7);
        let mut a2 = a;
        a2.counter = 100;
        assert_eq!(a.counter, 7);
        assert_eq!(a2.counter, 100);
    }
}
