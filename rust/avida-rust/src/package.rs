use std::ffi::{c_char, c_double, c_int, CStr, CString};

fn alloc_c_string(value: String) -> *mut c_char {
    let sanitized: Vec<u8> = value.bytes().filter(|b| *b != 0).collect();
    match CString::new(sanitized) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn avd_pkg_array_bool_value(count: c_int) -> c_int {
    if count != 0 {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn avd_pkg_array_int_value(count: c_int) -> c_int {
    count
}

#[no_mangle]
pub extern "C" fn avd_pkg_array_double_value() -> c_double {
    f64::NAN
}

#[no_mangle]
pub extern "C" fn avd_pkg_array_descriptor(count: c_int) -> *mut c_char {
    alloc_c_string(format!("array({count})"))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_pkg_array_string_value(
    entries: *const *const c_char,
    count: c_int,
) -> *mut c_char {
    if count <= 0 || entries.is_null() {
        return alloc_c_string(String::new());
    }

    let mut rendered = String::new();
    for i in 0..count {
        // SAFETY: entries pointer is checked for null and indexed within count bounds.
        let entry_ptr = unsafe { *entries.add(i as usize) };
        let text = if entry_ptr.is_null() {
            String::new()
        } else {
            // SAFETY: entry_ptr is expected to be a valid NUL-terminated C string.
            unsafe { CStr::from_ptr(entry_ptr) }
                .to_string_lossy()
                .into_owned()
        };
        if i == 0 {
            rendered.push('\'');
            rendered.push_str(&text);
            rendered.push('\'');
        } else {
            rendered.push_str(",'");
            rendered.push_str(&text);
            rendered.push('\'');
        }
    }
    alloc_c_string(rendered)
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_pkg_string_free(value: *mut c_char) {
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
    fn package_array_formatting_and_descriptor() {
        let descriptor = avd_pkg_array_descriptor(2);
        assert!(!descriptor.is_null());
        avd_pkg_string_free(descriptor);
    }
}
