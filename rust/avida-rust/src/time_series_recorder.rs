use crate::common::{
    alloc_c_string, apto_bool_from_bytes, apto_double_from_str, apto_int_from_str, boxed_free,
    boxed_new, free_c_string, set_out, with_cstr, with_tsr_mut, with_tsr_ref,
};
use std::ffi::{c_char, c_double, c_int};

fn apto_bool_from_str(value: &str) -> c_int {
    apto_bool_from_bytes(value.as_bytes())
}

#[repr(C)]
pub struct AvidaTimeSeriesHandle {
    updates: Vec<c_int>,
    values: Vec<String>,
}

impl AvidaTimeSeriesHandle {
    fn new() -> Self {
        Self {
            updates: Vec::new(),
            values: Vec::new(),
        }
    }

    fn from_serialized(serialized: &str) -> Self {
        let mut handle = Self::new();
        if serialized.is_empty() {
            return handle;
        }

        for entry in serialized.split(',') {
            if entry.is_empty() {
                continue;
            }
            let (update_str, value_str) = match entry.split_once(':') {
                Some((u, v)) => (u, v),
                None => ("0", entry),
            };
            let update: c_int = update_str.parse::<i32>().unwrap_or(0);
            handle.updates.push(update);
            handle.values.push(value_str.to_owned());
        }

        handle
    }

    fn push(&mut self, update: c_int, value: String) {
        self.updates.push(update);
        self.values.push(value);
    }

    fn len(&self) -> c_int {
        self.updates.len() as c_int
    }

    fn update_at(&self, index: c_int) -> c_int {
        if index < 0 {
            return -1;
        }
        self.updates.get(index as usize).copied().unwrap_or(-1)
    }

    fn value_at(&self, index: c_int) -> Option<&str> {
        if index < 0 {
            return None;
        }
        self.values.get(index as usize).map(String::as_str)
    }

    fn value_as_bool(&self, index: c_int) -> Option<c_int> {
        let value = self.value_at(index)?;
        Some(apto_bool_from_str(value))
    }

    fn value_as_int(&self, index: c_int) -> Option<c_int> {
        let value = self.value_at(index)?;
        Some(apto_int_from_str(value))
    }

    fn value_as_double(&self, index: c_int) -> Option<c_double> {
        let value = self.value_at(index)?;
        Some(apto_double_from_str(value))
    }

    fn serialize(&self) -> String {
        let mut out = String::new();
        for (idx, (update, value)) in self.updates.iter().zip(self.values.iter()).enumerate() {
            if idx > 0 {
                out.push(',');
            }
            out.push_str(&format!("{update}:{value}"));
        }
        out
    }
}

#[no_mangle]
pub extern "C" fn avd_tsr_new() -> *mut AvidaTimeSeriesHandle {
    boxed_new(AvidaTimeSeriesHandle::new())
}

#[no_mangle]
pub extern "C" fn avd_tsr_from_string(serialized: *const c_char) -> *mut AvidaTimeSeriesHandle {
    with_cstr(serialized, avd_tsr_new(), |cstr| {
        let text = cstr.to_string_lossy();
        boxed_new(AvidaTimeSeriesHandle::from_serialized(&text))
    })
}

#[no_mangle]
pub extern "C" fn avd_tsr_free(handle: *mut AvidaTimeSeriesHandle) {
    boxed_free(handle);
}

#[no_mangle]
pub extern "C" fn avd_tsr_len(handle: *const AvidaTimeSeriesHandle) -> c_int {
    with_tsr_ref(handle, 0, |h| h.len())
}

#[no_mangle]
pub extern "C" fn avd_tsr_update_at(handle: *const AvidaTimeSeriesHandle, index: c_int) -> c_int {
    with_tsr_ref(handle, -1, |h| h.update_at(index))
}

#[no_mangle]
pub extern "C" fn avd_tsr_value_as_cstr(
    handle: *const AvidaTimeSeriesHandle,
    index: c_int,
) -> *mut c_char {
    with_tsr_ref(handle, alloc_c_string(String::new()), |h| {
        alloc_c_string(h.value_at(index).unwrap_or_default().to_owned())
    })
}

fn write_typed_value<T: Copy>(out_value: *mut T, parsed: Option<T>) -> c_int {
    if out_value.is_null() {
        return 0;
    }
    match parsed {
        Some(v) => {
            if !set_out(out_value, v) {
                return 0;
            }
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn avd_tsr_value_as_bool(
    handle: *const AvidaTimeSeriesHandle,
    index: c_int,
    out_value: *mut c_int,
) -> c_int {
    let parsed = with_tsr_ref(handle, None, |h| h.value_as_bool(index));
    write_typed_value(out_value, parsed)
}

#[no_mangle]
pub extern "C" fn avd_tsr_value_as_int(
    handle: *const AvidaTimeSeriesHandle,
    index: c_int,
    out_value: *mut c_int,
) -> c_int {
    let parsed = with_tsr_ref(handle, None, |h| h.value_as_int(index));
    write_typed_value(out_value, parsed)
}

#[no_mangle]
pub extern "C" fn avd_tsr_value_as_double(
    handle: *const AvidaTimeSeriesHandle,
    index: c_int,
    out_value: *mut c_double,
) -> c_int {
    let parsed = with_tsr_ref(handle, None, |h| h.value_as_double(index));
    write_typed_value(out_value, parsed)
}

#[no_mangle]
pub extern "C" fn avd_tsr_push_bool(
    handle: *mut AvidaTimeSeriesHandle,
    update: c_int,
    value: c_int,
) {
    let rendered = if value != 0 { "1" } else { "0" };
    with_tsr_mut(handle, |h| h.push(update, rendered.to_owned()));
}

#[no_mangle]
pub extern "C" fn avd_tsr_push_int(
    handle: *mut AvidaTimeSeriesHandle,
    update: c_int,
    value: c_int,
) {
    with_tsr_mut(handle, |h| h.push(update, value.to_string()));
}

#[no_mangle]
pub extern "C" fn avd_tsr_push_double(
    handle: *mut AvidaTimeSeriesHandle,
    update: c_int,
    value: c_double,
) {
    // Keep compatibility with previous C++ `%f` formatting.
    let rendered = format!("{value:.6}");
    with_tsr_mut(handle, |h| h.push(update, rendered));
}

#[no_mangle]
pub extern "C" fn avd_tsr_push_string(
    handle: *mut AvidaTimeSeriesHandle,
    update: c_int,
    value: *const c_char,
) {
    let rendered = with_cstr(value, String::new(), |s| s.to_string_lossy().into_owned());
    with_tsr_mut(handle, |h| h.push(update, rendered));
}

#[no_mangle]
pub extern "C" fn avd_tsr_as_string(handle: *const AvidaTimeSeriesHandle) -> *mut c_char {
    with_tsr_ref(handle, alloc_c_string(String::new()), |h| {
        alloc_c_string(h.serialize())
    })
}

#[no_mangle]
pub extern "C" fn avd_tsr_string_free(value: *mut c_char) {
    free_c_string(value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    #[test]
    fn time_series_roundtrip_and_formatting() {
        let handle = avd_tsr_new();
        avd_tsr_push_int(handle, 5, 12);
        avd_tsr_push_double(handle, 7, 3.5);
        avd_tsr_push_bool(handle, 9, 1);
        let text = avd_tsr_as_string(handle);
        assert!(!text.is_null());
        let mut out_i: c_int = 0;
        let mut out_d: c_double = 0.0;
        let mut out_b: c_int = 0;
        assert_eq!(avd_tsr_value_as_int(handle, 0, &mut out_i), 1);
        assert_eq!(out_i, 12);
        assert_eq!(avd_tsr_value_as_double(handle, 1, &mut out_d), 1);
        assert!((out_d - 3.5).abs() < 1e-12);
        assert_eq!(avd_tsr_value_as_bool(handle, 2, &mut out_b), 1);
        assert_eq!(out_b, 1);
        avd_tsr_string_free(text);
        avd_tsr_free(handle);
    }

    #[test]
    fn typed_getters_reject_invalid_values() {
        let handle = avd_tsr_from_string(
            CString::new("1:abc,2:9.5,3:0")
                .expect("literal has no NUL")
                .as_ptr(),
        );
        let mut out_i: c_int = 0;
        let mut out_d: c_double = 0.0;
        let mut out_b: c_int = 0;
        assert_eq!(avd_tsr_value_as_int(handle, 0, &mut out_i), 1);
        assert_eq!(out_i, 0);
        assert_eq!(avd_tsr_value_as_double(handle, 1, &mut out_d), 1);
        assert!((out_d - 9.5).abs() < 1e-12);
        assert_eq!(avd_tsr_value_as_bool(handle, 2, &mut out_b), 1);
        assert_eq!(out_b, 0);
        avd_tsr_free(handle);
    }

    #[test]
    fn typed_getters_validate_pointer_and_index() {
        let mut out_i: c_int = 99;
        let out_b: c_int = 99;
        let out_d: c_double = 99.0;
        assert_eq!(avd_tsr_value_as_int(std::ptr::null(), 0, &mut out_i), 0);
        assert_eq!(out_i, 99);
        assert_eq!(
            avd_tsr_value_as_int(std::ptr::null(), 0, std::ptr::null_mut()),
            0
        );
        assert_eq!(
            avd_tsr_value_as_bool(std::ptr::null(), 0, std::ptr::null_mut()),
            0
        );
        assert_eq!(
            avd_tsr_value_as_double(std::ptr::null(), 0, std::ptr::null_mut()),
            0
        );

        let handle = avd_tsr_new();
        avd_tsr_push_int(handle, 3, 42);
        avd_tsr_push_bool(handle, 4, 1);
        avd_tsr_push_double(handle, 5, 2.5);
        assert_eq!(avd_tsr_value_as_int(handle, -1, &mut out_i), 0);
        assert_eq!(avd_tsr_value_as_int(handle, 7, &mut out_i), 0);
        assert_eq!(avd_tsr_value_as_bool(handle, 1, std::ptr::null_mut()), 0);
        assert_eq!(avd_tsr_value_as_double(handle, 2, std::ptr::null_mut()), 0);
        assert_eq!(out_i, 99);
        assert_eq!(out_b, 99);
        assert_eq!(out_d, 99.0);
        avd_tsr_free(handle);
    }

    #[test]
    fn string_getters_return_empty_for_null_handle() {
        let value_ptr = avd_tsr_value_as_cstr(std::ptr::null(), 0);
        assert!(!value_ptr.is_null());
        avd_tsr_string_free(value_ptr);

        let serialized_ptr = avd_tsr_as_string(std::ptr::null());
        assert!(!serialized_ptr.is_null());
        avd_tsr_string_free(serialized_ptr);
    }

    #[test]
    fn malformed_serialized_entries_are_stable() {
        let handle = avd_tsr_from_string(
            CString::new("oops,4:12")
                .expect("literal has no NUL")
                .as_ptr(),
        );
        assert_eq!(avd_tsr_len(handle), 2);
        assert_eq!(avd_tsr_update_at(handle, 0), 0);
        assert_eq!(avd_tsr_update_at(handle, 1), 4);
        let mut out_i: c_int = 0;
        assert_eq!(avd_tsr_value_as_int(handle, 0, &mut out_i), 1);
        assert_eq!(out_i, 0);
        assert_eq!(avd_tsr_value_as_int(handle, 1, &mut out_i), 1);
        assert_eq!(out_i, 12);
        avd_tsr_free(handle);
    }

    #[test]
    fn typed_getters_match_apto_stras_coercion_matrix() {
        let handle = avd_tsr_from_string(
            CString::new("1:1,2:T,3:true,4: true,5:2,6:0x10,7:7x,8:1e2,9:nan")
                .expect("literal has no NUL")
                .as_ptr(),
        );
        let mut out_i: c_int = 0;
        let mut out_d: c_double = 0.0;
        let mut out_b: c_int = 0;

        assert_eq!(avd_tsr_value_as_bool(handle, 0, &mut out_b), 1);
        assert_eq!(out_b, 1);
        assert_eq!(avd_tsr_value_as_bool(handle, 1, &mut out_b), 1);
        assert_eq!(out_b, 1);
        assert_eq!(avd_tsr_value_as_bool(handle, 2, &mut out_b), 1);
        assert_eq!(out_b, 1);
        assert_eq!(avd_tsr_value_as_bool(handle, 3, &mut out_b), 1);
        assert_eq!(out_b, 0);
        assert_eq!(avd_tsr_value_as_bool(handle, 4, &mut out_b), 1);
        assert_eq!(out_b, 0);

        assert_eq!(avd_tsr_value_as_int(handle, 5, &mut out_i), 1);
        assert_eq!(out_i, 16);
        assert_eq!(avd_tsr_value_as_int(handle, 6, &mut out_i), 1);
        assert_eq!(out_i, 7);
        assert_eq!(avd_tsr_value_as_int(handle, 7, &mut out_i), 1);
        assert_eq!(out_i, 1);

        assert_eq!(avd_tsr_value_as_double(handle, 8, &mut out_d), 1);
        assert!(out_d.is_nan());
        avd_tsr_free(handle);
    }
}
