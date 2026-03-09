use std::ffi::{c_char, c_int, CStr, CString};

fn alloc_c_string(value: String) -> *mut c_char {
    let sanitized: Vec<u8> = value.bytes().filter(|b| *b != 0).collect();
    match CString::new(sanitized) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

fn parse_id(input: &str) -> (bool, bool, String, String) {
    let size = input.len();
    let is_argumented = size > 2 && input.as_bytes()[size - 1] == b']';
    let is_standard = size != 0 && (size < 3 || input.as_bytes()[size - 1] != b']');
    if !is_argumented {
        return (is_standard, false, String::new(), String::new());
    }

    if let Some(open_idx) = input.find('[') {
        let start_idx = open_idx + 1;
        let argument = input[start_idx..size - 1].to_owned();
        let raw_id = format!("{}]", &input[..start_idx]);
        return (is_standard, true, raw_id, argument);
    }
    (is_standard, true, String::new(), String::new())
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_provider_is_standard_id(data_id: *const c_char) -> c_int {
    if data_id.is_null() {
        return 0;
    }
    // SAFETY: pointer checked for null and read-only.
    let id = unsafe { CStr::from_ptr(data_id) }.to_string_lossy();
    if parse_id(&id).0 {
        1
    } else {
        0
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_provider_is_argumented_id(data_id: *const c_char) -> c_int {
    if data_id.is_null() {
        return 0;
    }
    // SAFETY: pointer checked for null and read-only.
    let id = unsafe { CStr::from_ptr(data_id) }.to_string_lossy();
    if parse_id(&id).1 {
        1
    } else {
        0
    }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_provider_split_argumented_id(
    data_id: *const c_char,
    out_raw_id: *mut *mut c_char,
    out_argument: *mut *mut c_char,
) -> c_int {
    if data_id.is_null() || out_raw_id.is_null() || out_argument.is_null() {
        return 0;
    }
    // SAFETY: pointer checked for null and read-only.
    let id = unsafe { CStr::from_ptr(data_id) }.to_string_lossy();
    let (_, is_argumented, raw_id, argument) = parse_id(&id);
    if !is_argumented || raw_id.is_empty() {
        return 0;
    }
    // SAFETY: output pointers checked for null and written exactly once.
    unsafe {
        *out_raw_id = alloc_c_string(raw_id);
        *out_argument = alloc_c_string(argument);
    }
    1
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_provider_string_free(value: *mut c_char) {
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
    fn provider_id_split_matches_cpp_expectation() {
        let mut raw: *mut c_char = std::ptr::null_mut();
        let mut arg: *mut c_char = std::ptr::null_mut();
        let input = CString::new("demo[x]").expect("literal has no NUL");
        let ok = avd_provider_split_argumented_id(input.as_ptr(), &mut raw, &mut arg);
        assert_eq!(ok, 1);
        // SAFETY: raw/arg were produced by helper allocs.
        let raw_s = unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned();
        // SAFETY: raw/arg were produced by helper allocs.
        let arg_s = unsafe { CStr::from_ptr(arg) }
            .to_string_lossy()
            .into_owned();
        assert_eq!(raw_s, "demo[]");
        assert_eq!(arg_s, "x");
        avd_provider_string_free(raw);
        avd_provider_string_free(arg);
    }
}
