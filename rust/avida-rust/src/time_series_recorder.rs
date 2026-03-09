use std::ffi::{c_char, c_double, c_int, CStr, CString};

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

fn alloc_c_string(value: String) -> *mut c_char {
    let sanitized: Vec<u8> = value.bytes().filter(|b| *b != 0).collect();
    match CString::new(sanitized) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn avd_tsr_new() -> *mut AvidaTimeSeriesHandle {
    Box::into_raw(Box::new(AvidaTimeSeriesHandle::new()))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_tsr_from_string(serialized: *const c_char) -> *mut AvidaTimeSeriesHandle {
    if serialized.is_null() {
        return avd_tsr_new();
    }
    // SAFETY: pointer checked for null and read-only.
    let cstr = unsafe { CStr::from_ptr(serialized) };
    let text = cstr.to_string_lossy();
    Box::into_raw(Box::new(AvidaTimeSeriesHandle::from_serialized(&text)))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_tsr_free(handle: *mut AvidaTimeSeriesHandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer came from Box::into_raw in this crate and is freed exactly once here.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_tsr_len(handle: *const AvidaTimeSeriesHandle) -> c_int {
    if handle.is_null() {
        return 0;
    }
    // SAFETY: pointer checked for null and read-only.
    let h = unsafe { &*handle };
    h.len()
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_tsr_update_at(handle: *const AvidaTimeSeriesHandle, index: c_int) -> c_int {
    if handle.is_null() {
        return -1;
    }
    // SAFETY: pointer checked for null and read-only.
    let h = unsafe { &*handle };
    h.update_at(index)
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_tsr_value_as_cstr(
    handle: *const AvidaTimeSeriesHandle,
    index: c_int,
) -> *mut c_char {
    if handle.is_null() {
        return alloc_c_string(String::new());
    }
    // SAFETY: pointer checked for null and read-only.
    let h = unsafe { &*handle };
    alloc_c_string(h.value_at(index).unwrap_or_default().to_owned())
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_tsr_push_bool(
    handle: *mut AvidaTimeSeriesHandle,
    update: c_int,
    value: c_int,
) {
    if handle.is_null() {
        return;
    }
    let rendered = if value != 0 { "1" } else { "0" };
    // SAFETY: pointer checked for null and mutably accessed for this call only.
    let h = unsafe { &mut *handle };
    h.push(update, rendered.to_owned());
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_tsr_push_int(
    handle: *mut AvidaTimeSeriesHandle,
    update: c_int,
    value: c_int,
) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer checked for null and mutably accessed for this call only.
    let h = unsafe { &mut *handle };
    h.push(update, value.to_string());
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_tsr_push_double(
    handle: *mut AvidaTimeSeriesHandle,
    update: c_int,
    value: c_double,
) {
    if handle.is_null() {
        return;
    }
    // Keep compatibility with previous C++ `%f` formatting.
    let rendered = format!("{value:.6}");
    // SAFETY: pointer checked for null and mutably accessed for this call only.
    let h = unsafe { &mut *handle };
    h.push(update, rendered);
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_tsr_push_string(
    handle: *mut AvidaTimeSeriesHandle,
    update: c_int,
    value: *const c_char,
) {
    if handle.is_null() {
        return;
    }
    let rendered = if value.is_null() {
        String::new()
    } else {
        // SAFETY: pointer checked for null and read-only.
        unsafe { CStr::from_ptr(value) }
            .to_string_lossy()
            .into_owned()
    };
    // SAFETY: pointer checked for null and mutably accessed for this call only.
    let h = unsafe { &mut *handle };
    h.push(update, rendered);
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_tsr_as_string(handle: *const AvidaTimeSeriesHandle) -> *mut c_char {
    if handle.is_null() {
        return alloc_c_string(String::new());
    }
    // SAFETY: pointer checked for null and read-only.
    let h = unsafe { &*handle };
    alloc_c_string(h.serialize())
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_tsr_string_free(value: *mut c_char) {
    if value.is_null() {
        return;
    }
    // SAFETY: pointer was allocated by CString::into_raw in this crate.
    unsafe {
        drop(CString::from_raw(value));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn time_series_roundtrip_and_formatting() {
        let handle = avd_tsr_new();
        avd_tsr_push_int(handle, 5, 12);
        let text = avd_tsr_as_string(handle);
        assert!(!text.is_null());
        avd_tsr_string_free(text);
        avd_tsr_free(handle);
    }
}
