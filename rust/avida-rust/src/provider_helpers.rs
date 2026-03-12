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

#[derive(Default)]
struct ProviderIdParse {
    is_standard: bool,
    is_argumented: bool,
    raw_id: String,
    argument: String,
}

fn parse_id(input: &str) -> ProviderIdParse {
    let size = input.len();
    let is_argumented = size > 2 && input.as_bytes()[size - 1] == b']';
    let is_standard = size != 0 && (size < 3 || input.as_bytes()[size - 1] != b']');
    if !is_argumented {
        return ProviderIdParse {
            is_standard,
            is_argumented: false,
            raw_id: String::new(),
            argument: String::new(),
        };
    }

    if let Ok((_, (prefix, argument))) = parse_argumented_id_nom(input) {
        return ProviderIdParse {
            is_standard,
            is_argumented: true,
            raw_id: format!("{prefix}[]"),
            argument: argument.to_owned(),
        };
    }
    ProviderIdParse {
        is_standard,
        is_argumented: true,
        raw_id: String::new(),
        argument: String::new(),
    }
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

fn write_split_outputs(
    out_raw_id: *mut *mut c_char,
    out_argument: *mut *mut c_char,
    raw_id: String,
    argument: String,
) -> bool {
    let raw_ptr = alloc_c_string(raw_id);
    let arg_ptr = alloc_c_string(argument);
    if raw_ptr.is_null() || arg_ptr.is_null() {
        free_c_string(raw_ptr);
        free_c_string(arg_ptr);
        return false;
    }
    if !set_out(out_raw_id, raw_ptr) {
        free_c_string(raw_ptr);
        free_c_string(arg_ptr);
        return false;
    }
    if !set_out(out_argument, arg_ptr) {
        free_c_string(raw_ptr);
        let _ = set_out(out_raw_id, std::ptr::null_mut());
        free_c_string(arg_ptr);
        return false;
    }
    true
}

#[no_mangle]
pub extern "C" fn avd_provider_is_standard_id(data_id: *const c_char) -> c_int {
    if with_cstr(data_id, false, |id| {
        parse_id(&id.to_string_lossy()).is_standard
    }) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn avd_provider_is_argumented_id(data_id: *const c_char) -> c_int {
    if with_cstr(data_id, false, |id| {
        parse_id(&id.to_string_lossy()).is_argumented
    }) {
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
    let parsed = with_cstr(data_id, ProviderIdParse::default(), |id| {
        parse_id(&id.to_string_lossy())
    });
    if !parsed.is_argumented || parsed.raw_id.is_empty() {
        return 0;
    }
    if write_split_outputs(out_raw_id, out_argument, parsed.raw_id, parsed.argument) {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn avd_provider_classify_id(
    data_id: *const c_char,
    out_raw_id: *mut *mut c_char,
    out_argument: *mut *mut c_char,
) -> c_int {
    if !out_raw_id.is_null() && !set_out(out_raw_id, std::ptr::null_mut()) {
        return PROVIDER_ID_KIND_INVALID;
    }
    if !out_argument.is_null() && !set_out(out_argument, std::ptr::null_mut()) {
        return PROVIDER_ID_KIND_INVALID;
    }
    if data_id.is_null() {
        return PROVIDER_ID_KIND_INVALID;
    }

    let parsed = with_cstr(data_id, ProviderIdParse::default(), |id| {
        parse_id(&id.to_string_lossy())
    });
    if parsed.is_standard {
        return PROVIDER_ID_KIND_STANDARD;
    }
    if parsed.is_argumented && !parsed.raw_id.is_empty() {
        if out_raw_id.is_null() || out_argument.is_null() {
            return PROVIDER_ID_KIND_INVALID;
        }
        if write_split_outputs(out_raw_id, out_argument, parsed.raw_id, parsed.argument) {
            return PROVIDER_ID_KIND_ARGUMENTED;
        }
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
    fn provider_id_split_rejects_null_outputs() {
        let input = CString::new("demo[x]").expect("literal has no NUL");
        let mut raw: *mut c_char = std::ptr::null_mut();
        let mut arg: *mut c_char = std::ptr::null_mut();
        assert_eq!(
            avd_provider_split_argumented_id(input.as_ptr(), std::ptr::null_mut(), &mut arg),
            0
        );
        assert_eq!(
            avd_provider_split_argumented_id(input.as_ptr(), &mut raw, std::ptr::null_mut()),
            0
        );
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

    #[test]
    fn provider_id_classify_argumented_rejects_null_outputs() {
        let argumented = CString::new("demo[value]").expect("literal has no NUL");
        assert_eq!(
            avd_provider_classify_id(
                argumented.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut()
            ),
            PROVIDER_ID_KIND_INVALID
        );
    }

    #[test]
    fn provider_id_matrix_covers_edge_shapes() {
        struct Case {
            id: &'static str,
            kind: c_int,
            raw: Option<&'static str>,
            arg: Option<&'static str>,
        }
        let cases = [
            Case {
                id: "",
                kind: PROVIDER_ID_KIND_INVALID,
                raw: None,
                arg: None,
            },
            Case {
                id: "demo",
                kind: PROVIDER_ID_KIND_STANDARD,
                raw: None,
                arg: None,
            },
            Case {
                id: "demo[]",
                kind: PROVIDER_ID_KIND_ARGUMENTED,
                raw: Some("demo[]"),
                arg: Some(""),
            },
            Case {
                id: "demo[value]",
                kind: PROVIDER_ID_KIND_ARGUMENTED,
                raw: Some("demo[]"),
                arg: Some("value"),
            },
            Case {
                id: "demo[[x]]",
                kind: PROVIDER_ID_KIND_ARGUMENTED,
                raw: Some("demo[]"),
                arg: Some("[x]"),
            },
            Case {
                id: "demo[x][y]",
                kind: PROVIDER_ID_KIND_ARGUMENTED,
                raw: Some("demo[]"),
                arg: Some("x][y"),
            },
            Case {
                id: "[x]",
                kind: PROVIDER_ID_KIND_ARGUMENTED,
                raw: Some("[]"),
                arg: Some("x"),
            },
            Case {
                id: "demo]",
                kind: PROVIDER_ID_KIND_INVALID,
                raw: None,
                arg: None,
            },
            Case {
                id: "demo[",
                kind: PROVIDER_ID_KIND_STANDARD,
                raw: None,
                arg: None,
            },
        ];

        for case in cases {
            let id = CString::new(case.id).expect("literal has no NUL");
            let mut raw: *mut c_char = std::ptr::null_mut();
            let mut arg: *mut c_char = std::ptr::null_mut();
            let kind = avd_provider_classify_id(id.as_ptr(), &mut raw, &mut arg);
            assert_eq!(kind, case.kind, "kind mismatch for {}", case.id);
            match case.raw {
                Some(expected_raw) => {
                    assert!(!raw.is_null(), "raw should be set for {}", case.id);
                    // SAFETY: raw is produced by helper allocs.
                    let raw_s = unsafe { CStr::from_ptr(raw) }
                        .to_string_lossy()
                        .into_owned();
                    assert_eq!(raw_s, expected_raw, "raw mismatch for {}", case.id);
                }
                None => assert!(raw.is_null(), "raw should be null for {}", case.id),
            }
            match case.arg {
                Some(expected_arg) => {
                    assert!(!arg.is_null(), "arg should be set for {}", case.id);
                    // SAFETY: arg is produced by helper allocs.
                    let arg_s = unsafe { CStr::from_ptr(arg) }
                        .to_string_lossy()
                        .into_owned();
                    assert_eq!(arg_s, expected_arg, "arg mismatch for {}", case.id);
                }
                None => assert!(arg.is_null(), "arg should be null for {}", case.id),
            }
            avd_provider_string_free(raw);
            avd_provider_string_free(arg);
        }
    }
}
