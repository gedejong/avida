//! Birth entry scalar data migrated from cBirthEntry.

use std::ffi::{c_double, c_int};

use crate::merit::Merit;

/// Scalar fields extracted from cBirthEntry, embedded via #[repr(C)].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BirthEntryScalars {
    pub mating_type: c_int,
    pub mating_display_a: c_int,
    pub mating_display_b: c_int,
    pub mate_preference: c_int,
    pub group_id: c_int,
    pub energy4_offspring: c_double,
    pub merit: Merit,
    pub timestamp: c_int,
}

impl Default for BirthEntryScalars {
    fn default() -> Self {
        BirthEntryScalars {
            mating_type: -1, // MATING_TYPE_JUVENILE
            mating_display_a: 0,
            mating_display_b: 0,
            mate_preference: 0, // MATE_PREFERENCE_RANDOM
            group_id: -1,
            energy4_offspring: 0.0,
            merit: Merit::default(),
            timestamp: -1, // empty
        }
    }
}

/// Create a default BirthEntryScalars.
#[no_mangle]
pub extern "C" fn avd_birth_scalars_default() -> BirthEntryScalars {
    BirthEntryScalars::default()
}

/// Check if a birth entry is still valid (not timed out).
///
/// Returns 1 if valid, 0 if expired or empty.
/// `max_wait_time`: from config `MAX_BIRTH_WAIT_TIME`, -1 means no timeout.
/// `cur_update`: current simulation update counter.
#[no_mangle]
pub extern "C" fn avd_birth_is_valid_entry(
    timestamp: c_int,
    max_wait_time: c_int,
    cur_update: c_int,
) -> c_int {
    // No organism in entry
    if timestamp == -1 {
        return 0;
    }
    // No timeout configured
    if max_wait_time == -1 {
        return 1;
    }
    // Check if too many updates have passed
    if cur_update > timestamp + max_wait_time {
        0
    } else {
        1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_values() {
        let s = BirthEntryScalars::default();
        assert_eq!(s.mating_type, -1);
        assert_eq!(s.mating_display_a, 0);
        assert_eq!(s.mate_preference, 0);
        assert_eq!(s.group_id, -1);
        assert!((s.energy4_offspring - 0.0).abs() < f64::EPSILON);
        assert_eq!(s.timestamp, -1);
    }

    #[test]
    fn test_ffi_default() {
        let s = avd_birth_scalars_default();
        assert_eq!(s.timestamp, -1);
        assert_eq!(s.mating_type, -1);
    }

    #[test]
    fn test_clone() {
        let s = BirthEntryScalars {
            mating_type: 5,
            timestamp: 42,
            ..BirthEntryScalars::default()
        };
        let s2 = s;
        assert_eq!(s2.mating_type, 5);
        assert_eq!(s2.timestamp, 42);
    }

    #[test]
    fn test_valid_entry_empty() {
        assert_eq!(avd_birth_is_valid_entry(-1, 100, 50), 0);
    }

    #[test]
    fn test_valid_entry_no_timeout() {
        assert_eq!(avd_birth_is_valid_entry(10, -1, 99999), 1);
    }

    #[test]
    fn test_valid_entry_within_timeout() {
        assert_eq!(avd_birth_is_valid_entry(10, 100, 50), 1);
    }

    #[test]
    fn test_valid_entry_expired() {
        assert_eq!(avd_birth_is_valid_entry(10, 100, 111), 0);
    }

    #[test]
    fn test_valid_entry_at_boundary() {
        assert_eq!(avd_birth_is_valid_entry(10, 100, 110), 1); // exactly at max
    }
}
