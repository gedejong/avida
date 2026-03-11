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

    let mut entry: c_int = 0;
    while (entry as usize) < updates.len() {
        if updates[entry as usize] > update {
            break;
        }
        entry += 1;
    }
    if entry > 0 {
        entry - 1
    } else {
        0
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rh_select_entry_index(
    updates: *const c_int,
    count: c_int,
    update: c_int,
    exact: c_int,
) -> c_int {
    if updates.is_null() || count <= 0 {
        return -1;
    }
    let count_usize = match usize::try_from(count) {
        Ok(v) => v,
        Err(_) => return -1,
    };
    // SAFETY: updates is non-null and treated as read-only for count elements.
    let updates_slice = unsafe { std::slice::from_raw_parts(updates, count_usize) };
    select_entry_index_internal(updates_slice, update, exact != 0)
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_rh_value_at_or_zero(values: *const f64, count: c_int, index: c_int) -> f64 {
    if values.is_null() || count <= 0 || index < 0 || index >= count {
        return 0.0;
    }
    let idx = index as usize;
    // SAFETY: values is non-null and idx is validated within [0, count).
    unsafe { *values.add(idx) }
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
    }
}
