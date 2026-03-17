use crate::common::with_cstr;
use std::os::raw::{c_char, c_int};

const ANALYZE_REL_MASK_LESS: c_int = 1;
const ANALYZE_REL_MASK_EQUAL: c_int = 2;
const ANALYZE_REL_MASK_GREATER: c_int = 4;

#[no_mangle]
pub extern "C" fn avd_analyze_relation_mask(relation: *const c_char) -> c_int {
    with_cstr(relation, -1, |rel| match rel.to_bytes() {
        b"==" => ANALYZE_REL_MASK_EQUAL,
        b"!=" => ANALYZE_REL_MASK_LESS | ANALYZE_REL_MASK_GREATER,
        b"<" => ANALYZE_REL_MASK_LESS,
        b">" => ANALYZE_REL_MASK_GREATER,
        b"<=" => ANALYZE_REL_MASK_LESS | ANALYZE_REL_MASK_EQUAL,
        b">=" => ANALYZE_REL_MASK_EQUAL | ANALYZE_REL_MASK_GREATER,
        _ => -1,
    })
}

#[no_mangle]
pub extern "C" fn avd_analyze_is_html_extension(extension: *const c_char) -> c_int {
    with_cstr(extension, 0, |ext| match ext.to_bytes() {
        b"html" => 1,
        _ => 0,
    })
}

#[no_mangle]
pub extern "C" fn avd_analyze_is_html_filename_token(filename_token: *const c_char) -> c_int {
    with_cstr(filename_token, 0, |token| match token.to_bytes() {
        b"html" => 1,
        _ => 0,
    })
}

#[no_mangle]
pub extern "C" fn avd_analyze_output_file_type_short_circuit_kind(
    has_html_extension: c_int,
) -> c_int {
    if has_html_extension != 0 {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn avd_analyze_output_sink_short_circuit_kind(is_cout_filename: c_int) -> c_int {
    if is_cout_filename != 0 {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn avd_analyze_output_file_handle_mode_short_circuit_kind(
    action_kind: c_int,
) -> c_int {
    match action_kind {
        0 => 0,
        1 | 2 => 1,
        _ => -1,
    }
}

#[no_mangle]
pub extern "C" fn avd_analyze_output_token_presence_short_circuit_kind(
    remaining_arg_size: c_int,
) -> c_int {
    if remaining_arg_size != 0 {
        1
    } else {
        0
    }
}

#[no_mangle]
pub extern "C" fn avd_analyze_file_type_token_short_circuit_kind(
    has_text_token: c_int,
    has_html_token: c_int,
) -> c_int {
    if has_html_token != 0 {
        1
    } else if has_text_token != 0 {
        0
    } else {
        -1
    }
}

#[no_mangle]
pub extern "C" fn avd_analyze_apply_file_type_token_policy(
    has_text_token: c_int,
    has_html_token: c_int,
    current_file_type: c_int,
    text_file_type: c_int,
    html_file_type: c_int,
) -> c_int {
    let mut file_type = current_file_type;
    if has_text_token != 0 {
        file_type = text_file_type;
    }
    if has_html_token != 0 {
        file_type = html_file_type;
    }
    file_type
}

#[cfg(test)]
mod tests {
    use super::{
        avd_analyze_apply_file_type_token_policy, avd_analyze_file_type_token_short_circuit_kind,
        avd_analyze_is_html_extension, avd_analyze_is_html_filename_token,
        avd_analyze_output_file_handle_mode_short_circuit_kind,
        avd_analyze_output_file_type_short_circuit_kind,
        avd_analyze_output_sink_short_circuit_kind,
        avd_analyze_output_token_presence_short_circuit_kind, avd_analyze_relation_mask,
        ANALYZE_REL_MASK_EQUAL, ANALYZE_REL_MASK_GREATER, ANALYZE_REL_MASK_LESS,
    };
    use std::ffi::CString;
    use std::ptr;

    #[test]
    fn analyze_relation_mask_mapping_policy() {
        let eq = CString::new("==").expect("cstr");
        let ne = CString::new("!=").expect("cstr");
        let lt = CString::new("<").expect("cstr");
        let gt = CString::new(">").expect("cstr");
        let le = CString::new("<=").expect("cstr");
        let ge = CString::new(">=").expect("cstr");
        let bad = CString::new("~=").expect("cstr");

        assert_eq!(
            avd_analyze_relation_mask(eq.as_ptr()),
            ANALYZE_REL_MASK_EQUAL
        );
        assert_eq!(
            avd_analyze_relation_mask(ne.as_ptr()),
            ANALYZE_REL_MASK_LESS | ANALYZE_REL_MASK_GREATER
        );
        assert_eq!(
            avd_analyze_relation_mask(lt.as_ptr()),
            ANALYZE_REL_MASK_LESS
        );
        assert_eq!(
            avd_analyze_relation_mask(gt.as_ptr()),
            ANALYZE_REL_MASK_GREATER
        );
        assert_eq!(
            avd_analyze_relation_mask(le.as_ptr()),
            ANALYZE_REL_MASK_LESS | ANALYZE_REL_MASK_EQUAL
        );
        assert_eq!(
            avd_analyze_relation_mask(ge.as_ptr()),
            ANALYZE_REL_MASK_EQUAL | ANALYZE_REL_MASK_GREATER
        );
        assert_eq!(avd_analyze_relation_mask(bad.as_ptr()), -1);
        assert_eq!(avd_analyze_relation_mask(ptr::null()), -1);
    }

    #[test]
    fn analyze_file_extension_html_policy() {
        let html = CString::new("html").expect("cstr");
        let txt = CString::new("txt").expect("cstr");
        let upper = CString::new("HTML").expect("cstr");
        assert_eq!(avd_analyze_is_html_extension(html.as_ptr()), 1);
        assert_eq!(avd_analyze_is_html_extension(txt.as_ptr()), 0);
        assert_eq!(avd_analyze_is_html_extension(upper.as_ptr()), 0);
        assert_eq!(avd_analyze_is_html_extension(ptr::null()), 0);
    }

    #[test]
    fn analyze_filename_token_html_policy() {
        let html = CString::new("html").expect("cstr");
        let dat = CString::new("data").expect("cstr");
        let upper = CString::new("HTML").expect("cstr");
        assert_eq!(avd_analyze_is_html_filename_token(html.as_ptr()), 1);
        assert_eq!(avd_analyze_is_html_filename_token(dat.as_ptr()), 0);
        assert_eq!(avd_analyze_is_html_filename_token(upper.as_ptr()), 0);
        assert_eq!(avd_analyze_is_html_filename_token(ptr::null()), 0);
    }

    #[test]
    fn analyze_file_type_token_policy() {
        let text_type = 10;
        let html_type = 20;
        let current_type = 30;
        assert_eq!(
            avd_analyze_apply_file_type_token_policy(0, 0, current_type, text_type, html_type),
            current_type
        );
        assert_eq!(
            avd_analyze_apply_file_type_token_policy(1, 0, current_type, text_type, html_type),
            text_type
        );
        assert_eq!(
            avd_analyze_apply_file_type_token_policy(0, 1, current_type, text_type, html_type),
            html_type
        );
        assert_eq!(
            avd_analyze_apply_file_type_token_policy(1, 1, current_type, text_type, html_type),
            html_type
        );
    }

    #[test]
    fn analyze_file_type_token_short_circuit_kind_policy() {
        assert_eq!(avd_analyze_file_type_token_short_circuit_kind(0, 0), -1);
        assert_eq!(avd_analyze_file_type_token_short_circuit_kind(1, 0), 0);
        assert_eq!(avd_analyze_file_type_token_short_circuit_kind(0, 1), 1);
        assert_eq!(avd_analyze_file_type_token_short_circuit_kind(1, 1), 1);
        assert_eq!(avd_analyze_file_type_token_short_circuit_kind(2, 0), 0);
        assert_eq!(avd_analyze_file_type_token_short_circuit_kind(0, -1), 1);
    }

    #[test]
    fn analyze_output_file_type_short_circuit_kind_policy() {
        assert_eq!(avd_analyze_output_file_type_short_circuit_kind(0), 0);
        assert_eq!(avd_analyze_output_file_type_short_circuit_kind(1), 1);
        assert_eq!(avd_analyze_output_file_type_short_circuit_kind(2), 1);
        assert_eq!(avd_analyze_output_file_type_short_circuit_kind(-1), 1);
    }

    #[test]
    fn analyze_output_sink_short_circuit_kind_policy() {
        assert_eq!(avd_analyze_output_sink_short_circuit_kind(0), 0);
        assert_eq!(avd_analyze_output_sink_short_circuit_kind(1), 1);
        assert_eq!(avd_analyze_output_sink_short_circuit_kind(2), 1);
        assert_eq!(avd_analyze_output_sink_short_circuit_kind(-1), 1);
    }

    #[test]
    fn analyze_output_file_handle_mode_short_circuit_kind_policy() {
        assert_eq!(avd_analyze_output_file_handle_mode_short_circuit_kind(0), 0);
        assert_eq!(avd_analyze_output_file_handle_mode_short_circuit_kind(1), 1);
        assert_eq!(avd_analyze_output_file_handle_mode_short_circuit_kind(2), 1);
        assert_eq!(
            avd_analyze_output_file_handle_mode_short_circuit_kind(-1),
            -1
        );
        assert_eq!(
            avd_analyze_output_file_handle_mode_short_circuit_kind(3),
            -1
        );
    }

    #[test]
    fn analyze_output_token_presence_short_circuit_kind_policy() {
        assert_eq!(avd_analyze_output_token_presence_short_circuit_kind(0), 0);
        assert_eq!(avd_analyze_output_token_presence_short_circuit_kind(1), 1);
        assert_eq!(avd_analyze_output_token_presence_short_circuit_kind(7), 1);
        assert_eq!(avd_analyze_output_token_presence_short_circuit_kind(-1), 1);
    }
}
