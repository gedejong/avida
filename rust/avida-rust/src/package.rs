use crate::common::{
    alloc_c_string, apto_bool_from_cstr, apto_double_from_cstr, apto_int_from_cstr, free_c_string,
    with_cstr, with_slice,
};
use std::ffi::{c_char, c_double, c_int, CStr};

unsafe extern "C" {
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
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
pub extern "C" fn avd_pkg_str_as_bool(value: *const c_char) -> c_int {
    with_cstr(value, 0, apto_bool_from_cstr)
}

#[no_mangle]
pub extern "C" fn avd_pkg_str_as_int(value: *const c_char) -> c_int {
    with_cstr(value, 0, apto_int_from_cstr)
}

#[no_mangle]
pub extern "C" fn avd_pkg_str_as_double(value: *const c_char) -> c_double {
    with_cstr(value, 0.0, apto_double_from_cstr)
}

#[no_mangle]
pub extern "C" fn avd_pkg_bool_to_string(value: c_int) -> *mut c_char {
    alloc_c_string(if value != 0 { "true" } else { "false" }.to_string())
}

#[no_mangle]
pub extern "C" fn avd_pkg_int_to_string(value: c_int) -> *mut c_char {
    alloc_c_string(value.to_string())
}

#[no_mangle]
pub extern "C" fn avd_pkg_double_to_string(value: c_double) -> *mut c_char {
    let mut buffer = [0 as c_char; 128];
    // SAFETY: buffer is valid for writes and format string is static NUL-terminated.
    unsafe {
        snprintf(buffer.as_mut_ptr(), buffer.len(), c"%g".as_ptr(), value);
    }
    // SAFETY: snprintf guarantees NUL-termination when n > 0.
    let rendered = unsafe { CStr::from_ptr(buffer.as_ptr()) }
        .to_string_lossy()
        .into_owned();
    alloc_c_string(rendered)
}

#[no_mangle]
pub extern "C" fn avd_pkg_array_descriptor(count: c_int) -> *mut c_char {
    alloc_c_string(format!("array({count})"))
}

#[no_mangle]
pub extern "C" fn avd_pkg_array_string_value(
    entries: *const *const c_char,
    count: c_int,
) -> *mut c_char {
    with_slice(
        entries,
        count,
        alloc_c_string(String::new()),
        |entry_slice| {
            let mut rendered = String::new();
            for (i, entry_ptr) in entry_slice.iter().enumerate() {
                let text = with_cstr(*entry_ptr, String::new(), |s| {
                    s.to_string_lossy().into_owned()
                });
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
        },
    )
}

#[no_mangle]
pub extern "C" fn avd_pkg_string_free(value: *mut c_char) {
    free_c_string(value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    fn owned_c_string_to_rust(ptr: *mut c_char) -> String {
        // SAFETY: pointer is expected to come from alloc_c_string in this module.
        let s = unsafe { CStr::from_ptr(ptr) }
            .to_string_lossy()
            .into_owned();
        avd_pkg_string_free(ptr);
        s
    }

    fn expected_c_g_format(value: c_double) -> String {
        let mut buffer = [0 as c_char; 128];
        // SAFETY: buffer is valid for writes and format string is a static C string.
        unsafe {
            snprintf(buffer.as_mut_ptr(), buffer.len(), c"%g".as_ptr(), value);
            CStr::from_ptr(buffer.as_ptr())
                .to_string_lossy()
                .into_owned()
        }
    }

    #[test]
    fn package_array_formatting_and_descriptor() {
        let descriptor = avd_pkg_array_descriptor(2);
        assert!(!descriptor.is_null(), "descriptor should be allocated");
        // SAFETY: descriptor was allocated in this module.
        let descriptor_s = unsafe { CStr::from_ptr(descriptor) }
            .to_string_lossy()
            .into_owned();
        assert_eq!(descriptor_s, "array(2)");
        avd_pkg_string_free(descriptor);

        let a = CString::new("a").expect("literal has no NUL");
        let b = CString::new("b").expect("literal has no NUL");
        let entries = [a.as_ptr(), b.as_ptr()];
        let rendered = avd_pkg_array_string_value(entries.as_ptr(), entries.len() as c_int);
        // SAFETY: rendered was allocated in this module.
        let rendered_s = unsafe { CStr::from_ptr(rendered) }
            .to_string_lossy()
            .into_owned();
        assert_eq!(rendered_s, "'a','b'");
        avd_pkg_string_free(rendered);
    }

    #[test]
    fn package_string_parse_helpers_match_legacy_expectations() {
        let t = CString::new("true").expect("literal has no NUL");
        let t_caps = CString::new("T").expect("literal has no NUL");
        let f = CString::new("false").expect("literal has no NUL");
        assert_eq!(avd_pkg_str_as_bool(t.as_ptr()), 1);
        assert_eq!(avd_pkg_str_as_bool(t_caps.as_ptr()), 1);
        assert_eq!(avd_pkg_str_as_bool(f.as_ptr()), 0);

        let dec = CString::new("42").expect("literal has no NUL");
        let hex = CString::new("0x10").expect("literal has no NUL");
        let malformed = CString::new("abc").expect("literal has no NUL");
        assert_eq!(avd_pkg_str_as_int(dec.as_ptr()), 42);
        assert_eq!(avd_pkg_str_as_int(hex.as_ptr()), 16);
        assert_eq!(avd_pkg_str_as_int(malformed.as_ptr()), 0);

        let dbl = CString::new("2.5").expect("literal has no NUL");
        assert!((avd_pkg_str_as_double(dbl.as_ptr()) - 2.5).abs() < 1e-12);
        assert_eq!(avd_pkg_str_as_double(malformed.as_ptr()), 0.0);
    }

    #[test]
    fn package_string_format_helpers_match_legacy_expectations() {
        assert_eq!(owned_c_string_to_rust(avd_pkg_bool_to_string(1)), "true");
        assert_eq!(owned_c_string_to_rust(avd_pkg_bool_to_string(0)), "false");

        assert_eq!(owned_c_string_to_rust(avd_pkg_int_to_string(42)), "42");
        assert_eq!(owned_c_string_to_rust(avd_pkg_int_to_string(-7)), "-7");

        assert_eq!(owned_c_string_to_rust(avd_pkg_double_to_string(2.5)), "2.5");
        assert_eq!(
            owned_c_string_to_rust(avd_pkg_double_to_string(1234567.0)),
            "1.23457e+06"
        );

        let nan_s = owned_c_string_to_rust(avd_pkg_double_to_string(f64::NAN));
        let inf_s = owned_c_string_to_rust(avd_pkg_double_to_string(f64::INFINITY));
        let ninf_s = owned_c_string_to_rust(avd_pkg_double_to_string(f64::NEG_INFINITY));
        assert_eq!(nan_s.to_ascii_lowercase(), "nan");
        assert_eq!(inf_s.to_ascii_lowercase(), "inf");
        assert_eq!(ninf_s.to_ascii_lowercase(), "-inf");
    }

    #[test]
    fn package_string_format_matrix_covers_boundaries_and_thresholds() {
        let bool_cases = [(0, "false"), (1, "true"), (-1, "true")];
        for (input, expected) in bool_cases {
            assert_eq!(
                owned_c_string_to_rust(avd_pkg_bool_to_string(input)),
                expected
            );
        }

        let int_cases = [0, 1, -1, i32::MAX, i32::MIN];
        for value in int_cases {
            assert_eq!(
                owned_c_string_to_rust(avd_pkg_int_to_string(value)),
                value.to_string()
            );
        }

        let double_cases = [
            0.0,
            -0.0,
            f64::MIN_POSITIVE,
            -f64::MIN_POSITIVE,
            f64::MIN_POSITIVE / 2.0,
            -(f64::MIN_POSITIVE / 2.0),
            9.999_99e-5,
            1.0e-4,
            9.999_99e5,
            1.0e6,
            1.234_567_89,
            -1.234_567_89,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NAN,
        ];
        for value in double_cases {
            let got = owned_c_string_to_rust(avd_pkg_double_to_string(value));
            let expected = expected_c_g_format(value);
            if value.is_nan() {
                assert_eq!(got.to_ascii_lowercase(), expected.to_ascii_lowercase());
            } else {
                assert_eq!(got, expected);
            }
        }
    }
}
