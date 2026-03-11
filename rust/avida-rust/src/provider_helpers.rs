use crate::common::{alloc_c_string, free_c_string, set_out, with_cstr};
use std::ffi::{c_char, c_int};

use nom::{
    bytes::complete::take_till,
    character::complete::char as parse_char,
    combinator::rest,
    error::{Error, ErrorKind},
    IResult, Parser,
};

const PROVIDER_ID_KIND_INVALID: c_int = 0;
const PROVIDER_ID_KIND_STANDARD: c_int = 1;
const PROVIDER_ID_KIND_ARGUMENTED: c_int = 2;

fn parse_id(input: &str) -> (bool, bool, String, String) {
    let size = input.len();
    let is_argumented = size > 2 && input.as_bytes()[size - 1] == b']';
    let is_standard = size != 0 && (size < 3 || input.as_bytes()[size - 1] != b']');
    if !is_argumented {
        return (is_standard, false, String::new(), String::new());
    }

    if let Ok((_, (prefix, argument))) = parse_argumented_id_nom(input) {
        let raw_id = format!("{prefix}[]");
        return (is_standard, true, raw_id, argument.to_owned());
    }
    (is_standard, true, String::new(), String::new())
}

fn parse_argumented_id_nom(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, prefix) = take_till(|c| c == '[').parse(input)?;
    let (input, _) = parse_char('[').parse(input)?;
    let (input, remainder) = rest.parse(input)?;
    if let Some(argument) = remainder.strip_suffix(']') {
        Ok((input, (prefix, argument)))
    } else {
        Err(nom::Err::Error(Error::new(input, ErrorKind::Char)))
    }
}

#[no_mangle]
pub extern "C" fn avd_provider_is_standard_id(data_id: *const c_char) -> c_int {
    if with_cstr(data_id, false, |id| parse_id(&id.to_string_lossy()).0) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn avd_provider_is_argumented_id(data_id: *const c_char) -> c_int {
    if with_cstr(data_id, false, |id| parse_id(&id.to_string_lossy()).1) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn avd_provider_split_argumented_id(
    data_id: *const c_char,
    out_raw_id: *mut *mut c_char,
    out_argument: *mut *mut c_char,
) -> c_int {
    if out_raw_id.is_null() || out_argument.is_null() {
        return 0;
    }
    let (is_argumented, raw_id, argument) =
        with_cstr(data_id, (false, String::new(), String::new()), |id| {
            let (_, is_arg, raw, arg) = parse_id(&id.to_string_lossy());
            (is_arg, raw, arg)
        });
    if !is_argumented || raw_id.is_empty() {
        return 0;
    }
    set_out(out_raw_id, alloc_c_string(raw_id));
    set_out(out_argument, alloc_c_string(argument));
    1
}

#[no_mangle]
pub extern "C" fn avd_provider_classify_id(
    data_id: *const c_char,
    out_raw_id: *mut *mut c_char,
    out_argument: *mut *mut c_char,
) -> c_int {
    set_out(out_raw_id, std::ptr::null_mut());
    set_out(out_argument, std::ptr::null_mut());
    if data_id.is_null() {
        return PROVIDER_ID_KIND_INVALID;
    }

    let (is_standard, is_argumented, raw_id, argument) = with_cstr(
        data_id,
        (false, false, String::new(), String::new()),
        |id| parse_id(&id.to_string_lossy()),
    );
    if is_standard {
        return PROVIDER_ID_KIND_STANDARD;
    }
    if is_argumented && !raw_id.is_empty() {
        if out_raw_id.is_null() || out_argument.is_null() {
            return PROVIDER_ID_KIND_INVALID;
        }
        set_out(out_raw_id, alloc_c_string(raw_id));
        set_out(out_argument, alloc_c_string(argument));
        return PROVIDER_ID_KIND_ARGUMENTED;
    }
    PROVIDER_ID_KIND_INVALID
}

#[no_mangle]
pub extern "C" fn avd_provider_string_free(value: *mut c_char) {
    free_c_string(value);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};
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

    #[test]
    fn provider_id_split_rejects_malformed_forms() {
        let mut raw: *mut c_char = std::ptr::null_mut();
        let mut arg: *mut c_char = std::ptr::null_mut();
        let malformed = CString::new("demo]").expect("literal has no NUL");
        let ok = avd_provider_split_argumented_id(malformed.as_ptr(), &mut raw, &mut arg);
        assert_eq!(ok, 0);
        assert!(raw.is_null());
        assert!(arg.is_null());
    }

    #[test]
    fn provider_id_classification_edge_cases() {
        let standard = CString::new("core.demo").expect("literal has no NUL");
        let argumented = CString::new("core.demo[]").expect("literal has no NUL");
        let malformed = CString::new("x]").expect("literal has no NUL");
        assert_eq!(avd_provider_is_standard_id(standard.as_ptr()), 1);
        assert_eq!(avd_provider_is_argumented_id(argumented.as_ptr()), 1);
        assert_eq!(avd_provider_is_argumented_id(malformed.as_ptr()), 0);
    }

    #[test]
    fn provider_id_classify_roundtrips_for_dispatch() {
        let standard = CString::new("core.demo").expect("literal has no NUL");
        let argumented = CString::new("demo[value]").expect("literal has no NUL");
        let malformed = CString::new("demo]").expect("literal has no NUL");

        let mut raw: *mut c_char = std::ptr::null_mut();
        let mut arg: *mut c_char = std::ptr::null_mut();
        let kind = avd_provider_classify_id(standard.as_ptr(), &mut raw, &mut arg);
        assert_eq!(kind, PROVIDER_ID_KIND_STANDARD);
        assert!(raw.is_null());
        assert!(arg.is_null());

        let kind = avd_provider_classify_id(argumented.as_ptr(), &mut raw, &mut arg);
        assert_eq!(kind, PROVIDER_ID_KIND_ARGUMENTED);
        // SAFETY: raw/arg were produced by helper allocs.
        let raw_s = unsafe { CStr::from_ptr(raw) }
            .to_string_lossy()
            .into_owned();
        // SAFETY: raw/arg were produced by helper allocs.
        let arg_s = unsafe { CStr::from_ptr(arg) }
            .to_string_lossy()
            .into_owned();
        assert_eq!(raw_s, "demo[]");
        assert_eq!(arg_s, "value");
        avd_provider_string_free(raw);
        avd_provider_string_free(arg);

        raw = std::ptr::null_mut();
        arg = std::ptr::null_mut();
        let kind = avd_provider_classify_id(malformed.as_ptr(), &mut raw, &mut arg);
        assert_eq!(kind, PROVIDER_ID_KIND_INVALID);
        assert!(raw.is_null());
        assert!(arg.is_null());
    }
}
