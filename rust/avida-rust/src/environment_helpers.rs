use crate::common::with_cstr;
use std::os::raw::{c_char, c_int};

// --- Process type classification ---
// Maps nReaction::PROCTYPE_* enum values
const ENV_PROCTYPE_ADD: c_int = 0;
const ENV_PROCTYPE_MULT: c_int = 1;
const ENV_PROCTYPE_POW: c_int = 2;
const ENV_PROCTYPE_LIN: c_int = 3;
const ENV_PROCTYPE_ENERGY: c_int = 4;
const ENV_PROCTYPE_ENZYME: c_int = 5;
const ENV_PROCTYPE_EXP: c_int = 6;
const ENV_PROCTYPE_UNKNOWN: c_int = -1;

/// Classifies a reaction process type string to its nReaction::PROCTYPE_* enum value.
/// Returns -1 for unknown strings or null input.
#[no_mangle]
pub extern "C" fn avd_env_process_type(type_str: *const c_char) -> c_int {
    with_cstr(type_str, ENV_PROCTYPE_UNKNOWN, |s| match s.to_bytes() {
        b"add" => ENV_PROCTYPE_ADD,
        b"mult" => ENV_PROCTYPE_MULT,
        b"pow" => ENV_PROCTYPE_POW,
        b"lin" => ENV_PROCTYPE_LIN,
        b"energy" => ENV_PROCTYPE_ENERGY,
        b"enzyme" => ENV_PROCTYPE_ENZYME,
        b"exp" => ENV_PROCTYPE_EXP,
        _ => ENV_PROCTYPE_UNKNOWN,
    })
}

// --- PhenPlast bonus method classification ---
// Maps ePHENPLAST_BONUS_METHOD enum values
const ENV_PHENPLAST_DEFAULT: c_int = 0;
const ENV_PHENPLAST_NO_BONUS: c_int = 1;
const ENV_PHENPLAST_FRAC_BONUS: c_int = 2;
const ENV_PHENPLAST_FULL_BONUS: c_int = 3;
const ENV_PHENPLAST_UNKNOWN: c_int = -1;

/// Classifies a phenoplastic bonus method string to its ePHENPLAST_BONUS_METHOD enum value.
/// Returns -1 for unknown strings or null input.
#[no_mangle]
pub extern "C" fn avd_env_phenplast_bonus_method(method_str: *const c_char) -> c_int {
    with_cstr(method_str, ENV_PHENPLAST_UNKNOWN, |s| match s.to_bytes() {
        b"default" => ENV_PHENPLAST_DEFAULT,
        b"nobonus" => ENV_PHENPLAST_NO_BONUS,
        b"fracbonus" => ENV_PHENPLAST_FRAC_BONUS,
        b"fullbonus" => ENV_PHENPLAST_FULL_BONUS,
        _ => ENV_PHENPLAST_UNKNOWN,
    })
}

// --- Reaction entry type classification ---
const ENV_ENTRY_TYPE_PROCESS: c_int = 0;
const ENV_ENTRY_TYPE_REQUISITE: c_int = 1;
const ENV_ENTRY_TYPE_CONTEXT_REQUISITE: c_int = 2;
const ENV_ENTRY_TYPE_UNKNOWN: c_int = -1;

/// Classifies a reaction entry type string to a dispatch code.
/// Returns -1 for unknown strings or null input.
#[no_mangle]
pub extern "C" fn avd_env_reaction_entry_type(entry_str: *const c_char) -> c_int {
    with_cstr(entry_str, ENV_ENTRY_TYPE_UNKNOWN, |s| match s.to_bytes() {
        b"process" => ENV_ENTRY_TYPE_PROCESS,
        b"requisite" => ENV_ENTRY_TYPE_REQUISITE,
        b"context_requisite" => ENV_ENTRY_TYPE_CONTEXT_REQUISITE,
        _ => ENV_ENTRY_TYPE_UNKNOWN,
    })
}

// --- Resource geometry string classifier ---
// nGeometry: GLOBAL=0, GRID=1, TORUS=2, CLIQUE=3, HEX=4, PARTIAL=5

const ENV_GEOMETRY_GLOBAL: c_int = 0;
const ENV_GEOMETRY_GRID: c_int = 1;
const ENV_GEOMETRY_TORUS: c_int = 2;
const ENV_GEOMETRY_PARTIAL: c_int = 5;
const ENV_GEOMETRY_UNKNOWN: c_int = -1;

/// Classify a geometry string (case-insensitive via caller ToLower) to nGeometry enum.
/// Returns -1 for unknown.
#[no_mangle]
pub extern "C" fn avd_env_geometry_type(geometry_str: *const c_char) -> c_int {
    with_cstr(geometry_str, ENV_GEOMETRY_UNKNOWN, |s| match s.to_bytes() {
        b"global" => ENV_GEOMETRY_GLOBAL,
        b"grid" => ENV_GEOMETRY_GRID,
        b"torus" => ENV_GEOMETRY_TORUS,
        b"partial" => ENV_GEOMETRY_PARTIAL,
        _ => ENV_GEOMETRY_UNKNOWN,
    })
}

// --- Bool-string parser for resource config ---
const ENV_BOOL_FALSE: c_int = 0;
const ENV_BOOL_TRUE: c_int = 1;
const ENV_BOOL_INVALID: c_int = -1;

/// Parse a boolean string ("true"/"1"/"false"/"0") to 0/1/-1.
/// Input should already be lowercased.
#[no_mangle]
pub extern "C" fn avd_env_parse_bool_string(value_str: *const c_char) -> c_int {
    with_cstr(value_str, ENV_BOOL_INVALID, |s| match s.to_bytes() {
        b"false" | b"0" => ENV_BOOL_FALSE,
        b"true" | b"1" => ENV_BOOL_TRUE,
        _ => ENV_BOOL_INVALID,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr;

    fn cstr(s: &str) -> CString {
        CString::new(s).unwrap()
    }

    // --- Process type tests ---

    #[test]
    fn process_type_known_values() {
        let cases = [
            ("add", ENV_PROCTYPE_ADD),
            ("mult", ENV_PROCTYPE_MULT),
            ("pow", ENV_PROCTYPE_POW),
            ("lin", ENV_PROCTYPE_LIN),
            ("energy", ENV_PROCTYPE_ENERGY),
            ("enzyme", ENV_PROCTYPE_ENZYME),
            ("exp", ENV_PROCTYPE_EXP),
        ];
        for (input, expected) in &cases {
            let cs = cstr(input);
            assert_eq!(
                avd_env_process_type(cs.as_ptr()),
                *expected,
                "process type mismatch for '{input}'"
            );
        }
    }

    #[test]
    fn process_type_unknown_and_null() {
        let unknown = cstr("subtract");
        assert_eq!(avd_env_process_type(unknown.as_ptr()), ENV_PROCTYPE_UNKNOWN);
        let empty = cstr("");
        assert_eq!(avd_env_process_type(empty.as_ptr()), ENV_PROCTYPE_UNKNOWN);
        assert_eq!(avd_env_process_type(ptr::null()), ENV_PROCTYPE_UNKNOWN);
    }

    #[test]
    fn process_type_case_sensitive() {
        let upper = cstr("ADD");
        assert_eq!(avd_env_process_type(upper.as_ptr()), ENV_PROCTYPE_UNKNOWN);
        let mixed = cstr("Mult");
        assert_eq!(avd_env_process_type(mixed.as_ptr()), ENV_PROCTYPE_UNKNOWN);
    }

    // --- PhenPlast bonus method tests ---

    #[test]
    fn phenplast_known_values() {
        let cases = [
            ("default", ENV_PHENPLAST_DEFAULT),
            ("nobonus", ENV_PHENPLAST_NO_BONUS),
            ("fracbonus", ENV_PHENPLAST_FRAC_BONUS),
            ("fullbonus", ENV_PHENPLAST_FULL_BONUS),
        ];
        for (input, expected) in &cases {
            let cs = cstr(input);
            assert_eq!(
                avd_env_phenplast_bonus_method(cs.as_ptr()),
                *expected,
                "phenplast mismatch for '{input}'"
            );
        }
    }

    #[test]
    fn phenplast_unknown_and_null() {
        let unknown = cstr("halfbonus");
        assert_eq!(
            avd_env_phenplast_bonus_method(unknown.as_ptr()),
            ENV_PHENPLAST_UNKNOWN
        );
        let empty = cstr("");
        assert_eq!(
            avd_env_phenplast_bonus_method(empty.as_ptr()),
            ENV_PHENPLAST_UNKNOWN
        );
        assert_eq!(
            avd_env_phenplast_bonus_method(ptr::null()),
            ENV_PHENPLAST_UNKNOWN
        );
    }

    #[test]
    fn phenplast_case_sensitive() {
        let upper = cstr("NOBONUS");
        assert_eq!(
            avd_env_phenplast_bonus_method(upper.as_ptr()),
            ENV_PHENPLAST_UNKNOWN
        );
    }

    // --- Reaction entry type tests ---

    #[test]
    fn entry_type_known_values() {
        let cases = [
            ("process", ENV_ENTRY_TYPE_PROCESS),
            ("requisite", ENV_ENTRY_TYPE_REQUISITE),
            ("context_requisite", ENV_ENTRY_TYPE_CONTEXT_REQUISITE),
        ];
        for (input, expected) in &cases {
            let cs = cstr(input);
            assert_eq!(
                avd_env_reaction_entry_type(cs.as_ptr()),
                *expected,
                "entry type mismatch for '{input}'"
            );
        }
    }

    #[test]
    fn entry_type_unknown_and_null() {
        let unknown = cstr("trigger");
        assert_eq!(
            avd_env_reaction_entry_type(unknown.as_ptr()),
            ENV_ENTRY_TYPE_UNKNOWN
        );
        let empty = cstr("");
        assert_eq!(
            avd_env_reaction_entry_type(empty.as_ptr()),
            ENV_ENTRY_TYPE_UNKNOWN
        );
        assert_eq!(
            avd_env_reaction_entry_type(ptr::null()),
            ENV_ENTRY_TYPE_UNKNOWN
        );
    }

    // --- Geometry type tests ---

    #[test]
    fn geometry_known_values() {
        let cases = [
            ("global", ENV_GEOMETRY_GLOBAL),
            ("grid", ENV_GEOMETRY_GRID),
            ("torus", ENV_GEOMETRY_TORUS),
            ("partial", ENV_GEOMETRY_PARTIAL),
        ];
        for (input, expected) in &cases {
            let cs = cstr(input);
            assert_eq!(
                avd_env_geometry_type(cs.as_ptr()),
                *expected,
                "geometry mismatch for '{input}'"
            );
        }
    }

    #[test]
    fn geometry_unknown_and_null() {
        let unknown = cstr("clique");
        assert_eq!(
            avd_env_geometry_type(unknown.as_ptr()),
            ENV_GEOMETRY_UNKNOWN
        );
        assert_eq!(avd_env_geometry_type(ptr::null()), ENV_GEOMETRY_UNKNOWN);
    }

    // --- Bool-string parser tests ---

    #[test]
    fn parse_bool_string_known_values() {
        let t1 = cstr("true");
        assert_eq!(avd_env_parse_bool_string(t1.as_ptr()), ENV_BOOL_TRUE);
        let t2 = cstr("1");
        assert_eq!(avd_env_parse_bool_string(t2.as_ptr()), ENV_BOOL_TRUE);
        let f1 = cstr("false");
        assert_eq!(avd_env_parse_bool_string(f1.as_ptr()), ENV_BOOL_FALSE);
        let f2 = cstr("0");
        assert_eq!(avd_env_parse_bool_string(f2.as_ptr()), ENV_BOOL_FALSE);
    }

    #[test]
    fn parse_bool_string_invalid() {
        let bad = cstr("yes");
        assert_eq!(avd_env_parse_bool_string(bad.as_ptr()), ENV_BOOL_INVALID);
        assert_eq!(avd_env_parse_bool_string(ptr::null()), ENV_BOOL_INVALID);
    }

    #[test]
    fn entry_type_case_sensitive() {
        let upper = cstr("PROCESS");
        assert_eq!(
            avd_env_reaction_entry_type(upper.as_ptr()),
            ENV_ENTRY_TYPE_UNKNOWN
        );
        let mixed = cstr("Requisite");
        assert_eq!(
            avd_env_reaction_entry_type(mixed.as_ptr()),
            ENV_ENTRY_TYPE_UNKNOWN
        );
    }
}
