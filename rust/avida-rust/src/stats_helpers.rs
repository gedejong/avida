use crate::common::with_cstr;
use std::os::raw::{c_char, c_int};

// --- Dual-print task filename classification ---

/// Returns 1 if the filename token is `"tasksq.dat"`, which triggers dual task+quality printing.
/// Returns 0 otherwise (including null).
#[no_mangle]
pub extern "C" fn avd_stats_is_dual_task_filename(filename: *const c_char) -> c_int {
    with_cstr(filename, 0, |s| match s.to_bytes() {
        b"tasksq.dat" => 1,
        _ => 0,
    })
}

/// Returns 1 if the filename token is `"in_tasksq.dat"`, which triggers dual internal task+quality printing.
/// Returns 0 otherwise (including null).
#[no_mangle]
pub extern "C" fn avd_stats_is_dual_internal_task_filename(filename: *const c_char) -> c_int {
    with_cstr(filename, 0, |s| match s.to_bytes() {
        b"in_tasksq.dat" => 1,
        _ => 0,
    })
}

// --- Spatial resource geometry classification ---
// nGeometry::GLOBAL = 0, GRID = 1, TORUS = 2, CLIQUE = 3, HEX = 4, PARTIAL = 5,
// LATTICE = 6, RANDOM_CONNECTED = 7, SCALE_FREE = 8

const GEOMETRY_GLOBAL: c_int = 0;
const GEOMETRY_PARTIAL: c_int = 5;

/// Returns 1 if the geometry represents a spatial resource (not GLOBAL, not PARTIAL).
/// Used to decide whether to sum spatial cell values vs use the global count directly.
#[no_mangle]
pub extern "C" fn avd_stats_is_spatial_resource(geometry: c_int) -> c_int {
    if geometry != GEOMETRY_GLOBAL && geometry != GEOMETRY_PARTIAL {
        1
    } else {
        0
    }
}

// --- Task quality average ---

/// Computes safe task quality average: quality/count if count > 0, else 0.0.
#[no_mangle]
pub extern "C" fn avd_stats_task_quality_average(quality: f64, count: c_int) -> f64 {
    if count > 0 {
        quality / f64::from(count)
    } else {
        0.0
    }
}

// --- Gradient resource habitat classifiers ---

/// Returns 1 if the resource is a wall gradient resource (is_gradient != 0 and habitat == 2).
#[no_mangle]
pub extern "C" fn avd_stats_is_wall_gradient(is_gradient: c_int, habitat: c_int) -> c_int {
    if is_gradient != 0 && habitat == 2 {
        1
    } else {
        0
    }
}

/// Returns 1 if the habitat is a den/nest type (habitat == 3 or habitat == 4).
#[no_mangle]
pub extern "C" fn avd_stats_is_den_habitat(habitat: c_int) -> c_int {
    if habitat == 3 || habitat == 4 {
        1
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    fn cstr(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    // --- Dual task filename tests ---

    #[test]
    fn dual_task_filename_policy() {
        let yes = cstr("tasksq.dat");
        assert_eq!(avd_stats_is_dual_task_filename(yes.as_ptr()), 1);
        let no = cstr("tasks.dat");
        assert_eq!(avd_stats_is_dual_task_filename(no.as_ptr()), 0);
        let empty = cstr("");
        assert_eq!(avd_stats_is_dual_task_filename(empty.as_ptr()), 0);
        assert_eq!(avd_stats_is_dual_task_filename(ptr::null()), 0);
    }

    #[test]
    fn dual_internal_task_filename_policy() {
        let yes = cstr("in_tasksq.dat");
        assert_eq!(avd_stats_is_dual_internal_task_filename(yes.as_ptr()), 1);
        let no = cstr("in_tasks.dat");
        assert_eq!(avd_stats_is_dual_internal_task_filename(no.as_ptr()), 0);
        let wrong = cstr("tasksq.dat");
        assert_eq!(avd_stats_is_dual_internal_task_filename(wrong.as_ptr()), 0);
        assert_eq!(avd_stats_is_dual_internal_task_filename(ptr::null()), 0);
    }

    // --- Spatial resource geometry tests ---

    #[test]
    fn spatial_resource_geometry_policy() {
        // GLOBAL(0) and PARTIAL(5) are NOT spatial
        assert_eq!(avd_stats_is_spatial_resource(0), 0);
        assert_eq!(avd_stats_is_spatial_resource(5), 0);
        // Everything else is spatial
        assert_eq!(avd_stats_is_spatial_resource(1), 1); // GRID
        assert_eq!(avd_stats_is_spatial_resource(2), 1); // TORUS
        assert_eq!(avd_stats_is_spatial_resource(3), 1); // CLIQUE
        assert_eq!(avd_stats_is_spatial_resource(4), 1); // HEX
        assert_eq!(avd_stats_is_spatial_resource(6), 1); // LATTICE
        assert_eq!(avd_stats_is_spatial_resource(7), 1); // RANDOM_CONNECTED
        assert_eq!(avd_stats_is_spatial_resource(8), 1); // SCALE_FREE
    }

    #[test]
    fn spatial_resource_geometry_boundary() {
        assert_eq!(avd_stats_is_spatial_resource(-1), 1); // unknown still "spatial" by exclusion
        assert_eq!(avd_stats_is_spatial_resource(99), 1);
    }

    // --- Task quality average tests ---

    #[test]
    fn task_quality_average_normal() {
        let avg = avd_stats_task_quality_average(10.0, 5);
        assert!((avg - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn task_quality_average_zero_count() {
        assert_eq!(avd_stats_task_quality_average(10.0, 0), 0.0);
        assert_eq!(avd_stats_task_quality_average(0.0, 0), 0.0);
    }

    #[test]
    fn task_quality_average_negative_count() {
        // Negative count should return 0.0 (count > 0 guard)
        assert_eq!(avd_stats_task_quality_average(10.0, -1), 0.0);
    }

    #[test]
    fn task_quality_average_zero_quality() {
        assert_eq!(avd_stats_task_quality_average(0.0, 5), 0.0);
    }

    // --- Wall gradient tests ---

    #[test]
    fn wall_gradient_policy() {
        assert_eq!(avd_stats_is_wall_gradient(1, 2), 1); // gradient + habitat 2
        assert_eq!(avd_stats_is_wall_gradient(0, 2), 0); // not gradient
        assert_eq!(avd_stats_is_wall_gradient(1, 0), 0); // gradient but wrong habitat
        assert_eq!(avd_stats_is_wall_gradient(1, 3), 0); // gradient but habitat 3
        assert_eq!(avd_stats_is_wall_gradient(0, 0), 0); // neither
    }

    // --- Den habitat tests ---

    #[test]
    fn den_habitat_policy() {
        assert_eq!(avd_stats_is_den_habitat(3), 1);
        assert_eq!(avd_stats_is_den_habitat(4), 1);
        assert_eq!(avd_stats_is_den_habitat(0), 0);
        assert_eq!(avd_stats_is_den_habitat(1), 0);
        assert_eq!(avd_stats_is_den_habitat(2), 0);
        assert_eq!(avd_stats_is_den_habitat(5), 0);
        assert_eq!(avd_stats_is_den_habitat(-1), 0);
    }
}
