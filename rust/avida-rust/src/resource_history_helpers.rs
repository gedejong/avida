use crate::common::with_slice;
use std::ffi::c_int;

fn select_entry_index_internal(updates: &[c_int], update: c_int, exact: bool) -> c_int {
    if updates.is_empty() {
        return -1;
    }

    if exact {
        for (i, u) in updates.iter().enumerate() {
            if *u == update {
                return i as c_int;
            }
        }
        return -1;
    }

    let mut best_idx: usize = 0;
    for (idx, value) in updates.iter().enumerate() {
        if *value > update {
            break;
        }
        best_idx = idx;
    }
    best_idx as c_int
}

#[no_mangle]
pub extern "C" fn avd_rh_select_entry_index(
    updates: *const c_int,
    count: c_int,
    update: c_int,
    exact: c_int,
) -> c_int {
    with_slice(updates, count, -1, |updates_slice| {
        select_entry_index_internal(updates_slice, update, exact != 0)
    })
}

#[no_mangle]
pub extern "C" fn avd_rh_value_at_or_zero(values: *const f64, count: c_int, index: c_int) -> f64 {
    with_slice(values, count, 0.0, |values_slice| {
        if index < 0 {
            return 0.0;
        }
        let idx = match usize::try_from(index) {
            Ok(v) => v,
            Err(_) => return 0.0,
        };
        values_slice.get(idx).copied().unwrap_or(0.0)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_entry_exact_and_nearest_behaviors() {
        let updates = [10, 20, 30];
        assert_eq!(avd_rh_select_entry_index(updates.as_ptr(), 3, 20, 1), 1);
        assert_eq!(avd_rh_select_entry_index(updates.as_ptr(), 3, 25, 1), -1);
        assert_eq!(avd_rh_select_entry_index(updates.as_ptr(), 3, 5, 0), 0);
        assert_eq!(avd_rh_select_entry_index(updates.as_ptr(), 3, 10, 0), 0);
        assert_eq!(avd_rh_select_entry_index(updates.as_ptr(), 3, 25, 0), 1);
        assert_eq!(avd_rh_select_entry_index(updates.as_ptr(), 3, 40, 0), 2);
    }

    #[test]
    fn select_entry_nearest_keeps_last_lte_on_duplicates() {
        let updates = [10, 20, 20, 30];
        assert_eq!(avd_rh_select_entry_index(updates.as_ptr(), 4, 20, 0), 2);
        assert_eq!(avd_rh_select_entry_index(updates.as_ptr(), 4, 21, 0), 2);
    }

    #[test]
    fn select_entry_handles_invalid_inputs() {
        let updates = [7, 11];
        assert_eq!(avd_rh_select_entry_index(std::ptr::null(), 2, 7, 0), -1);
        assert_eq!(avd_rh_select_entry_index(updates.as_ptr(), 0, 7, 0), -1);
        assert_eq!(avd_rh_select_entry_index(updates.as_ptr(), -1, 7, 0), -1);
    }

    #[test]
    fn value_lookup_or_zero_parity() {
        let values = [1.5, 2.5, 3.5];
        assert!((avd_rh_value_at_or_zero(values.as_ptr(), 3, 0) - 1.5).abs() < 1e-12);
        assert!((avd_rh_value_at_or_zero(values.as_ptr(), 3, 2) - 3.5).abs() < 1e-12);
        assert_eq!(avd_rh_value_at_or_zero(values.as_ptr(), 3, -1), 0.0);
        assert_eq!(avd_rh_value_at_or_zero(values.as_ptr(), 3, 3), 0.0);
        assert_eq!(avd_rh_value_at_or_zero(std::ptr::null(), 3, 1), 0.0);
        assert_eq!(avd_rh_value_at_or_zero(values.as_ptr(), 0, 0), 0.0);
        assert_eq!(avd_rh_value_at_or_zero(values.as_ptr(), -1, 0), 0.0);
    }
}
