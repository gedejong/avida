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
