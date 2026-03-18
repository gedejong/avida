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

// --- Resource var_name classification ---
const ENV_RES_INFLOW: c_int = 0;
const ENV_RES_OUTFLOW: c_int = 1;
const ENV_RES_INITIAL: c_int = 2;
const ENV_RES_GEOMETRY: c_int = 3;
const ENV_RES_CELLS: c_int = 4;
const ENV_RES_INFLOWX1: c_int = 5;
const ENV_RES_INFLOWX2: c_int = 6;
const ENV_RES_INFLOWY1: c_int = 7;
const ENV_RES_INFLOWY2: c_int = 8;
const ENV_RES_OUTFLOWX1: c_int = 9;
const ENV_RES_OUTFLOWX2: c_int = 10;
const ENV_RES_OUTFLOWY1: c_int = 11;
const ENV_RES_OUTFLOWY2: c_int = 12;
const ENV_RES_XDIFFUSE: c_int = 13;
const ENV_RES_XGRAVITY: c_int = 14;
const ENV_RES_YDIFFUSE: c_int = 15;
const ENV_RES_YGRAVITY: c_int = 16;
const ENV_RES_DEME: c_int = 17;
const ENV_RES_COLLECTABLE: c_int = 18;
const ENV_RES_ENERGY: c_int = 19;
const ENV_RES_HGT: c_int = 20;
const ENV_RES_UNKNOWN: c_int = -1;

/// Classify a resource variable name to an opcode. Handles aliases (inflowx→inflowx1, etc).
#[no_mangle]
pub extern "C" fn avd_env_resource_var_kind(var_name: *const c_char) -> c_int {
    with_cstr(var_name, ENV_RES_UNKNOWN, |s| match s.to_bytes() {
        b"inflow" => ENV_RES_INFLOW,
        b"outflow" => ENV_RES_OUTFLOW,
        b"initial" => ENV_RES_INITIAL,
        b"geometry" => ENV_RES_GEOMETRY,
        b"cells" => ENV_RES_CELLS,
        b"inflowx1" | b"inflowx" => ENV_RES_INFLOWX1,
        b"inflowx2" => ENV_RES_INFLOWX2,
        b"inflowy1" | b"inflowy" => ENV_RES_INFLOWY1,
        b"inflowy2" => ENV_RES_INFLOWY2,
        b"outflowx1" | b"outflowx" => ENV_RES_OUTFLOWX1,
        b"outflowx2" => ENV_RES_OUTFLOWX2,
        b"outflowy1" | b"outflowy" => ENV_RES_OUTFLOWY1,
        b"outflowy2" => ENV_RES_OUTFLOWY2,
        b"xdiffuse" => ENV_RES_XDIFFUSE,
        b"xgravity" => ENV_RES_XGRAVITY,
        b"ydiffuse" => ENV_RES_YDIFFUSE,
        b"ygravity" => ENV_RES_YGRAVITY,
        b"deme" => ENV_RES_DEME,
        b"collectable" => ENV_RES_COLLECTABLE,
        b"energy" => ENV_RES_ENERGY,
        b"hgt" => ENV_RES_HGT,
        _ => ENV_RES_UNKNOWN,
    })
}

// --- Process var_name classification ---
const ENV_PROCESS_RESOURCE: c_int = 0;
const ENV_PROCESS_VALUE: c_int = 1;
const ENV_PROCESS_TYPE: c_int = 2;
const ENV_PROCESS_MAX: c_int = 3;
const ENV_PROCESS_MIN: c_int = 4;
const ENV_PROCESS_FRAC: c_int = 5;
const ENV_PROCESS_KSUBM: c_int = 6;
const ENV_PROCESS_PRODUCT: c_int = 7;
const ENV_PROCESS_CONVERSION: c_int = 8;
const ENV_PROCESS_INST: c_int = 9;
const ENV_PROCESS_RANDOM: c_int = 10;
const ENV_PROCESS_LETHAL: c_int = 11;
const ENV_PROCESS_STERILIZE: c_int = 12;
const ENV_PROCESS_DEME: c_int = 13;
const ENV_PROCESS_GERMLINE: c_int = 14;
const ENV_PROCESS_DETECT: c_int = 15;
const ENV_PROCESS_THRESHOLD: c_int = 16;
const ENV_PROCESS_DETECTIONERROR: c_int = 17;
const ENV_PROCESS_STRING: c_int = 18;
const ENV_PROCESS_DEPLETABLE: c_int = 19;
const ENV_PROCESS_PHENPLASTBONUS: c_int = 20;
const ENV_PROCESS_INTERNAL: c_int = 21;
const ENV_PROCESS_UNKNOWN: c_int = -1;

/// Classify a reaction process variable name to an opcode.
#[no_mangle]
pub extern "C" fn avd_env_process_var_kind(var_name: *const c_char) -> c_int {
    with_cstr(var_name, ENV_PROCESS_UNKNOWN, |s| match s.to_bytes() {
        b"resource" => ENV_PROCESS_RESOURCE,
        b"value" => ENV_PROCESS_VALUE,
        b"type" => ENV_PROCESS_TYPE,
        b"max" => ENV_PROCESS_MAX,
        b"min" => ENV_PROCESS_MIN,
        b"frac" => ENV_PROCESS_FRAC,
        b"ksubm" => ENV_PROCESS_KSUBM,
        b"product" => ENV_PROCESS_PRODUCT,
        b"conversion" => ENV_PROCESS_CONVERSION,
        b"inst" => ENV_PROCESS_INST,
        b"random" => ENV_PROCESS_RANDOM,
        b"lethal" => ENV_PROCESS_LETHAL,
        b"sterilize" => ENV_PROCESS_STERILIZE,
        b"deme" => ENV_PROCESS_DEME,
        b"germline" => ENV_PROCESS_GERMLINE,
        b"detect" => ENV_PROCESS_DETECT,
        b"threshold" => ENV_PROCESS_THRESHOLD,
        b"detectionerror" => ENV_PROCESS_DETECTIONERROR,
        b"string" => ENV_PROCESS_STRING,
        b"depletable" => ENV_PROCESS_DEPLETABLE,
        b"phenplastbonus" => ENV_PROCESS_PHENPLASTBONUS,
        b"internal" => ENV_PROCESS_INTERNAL,
        _ => ENV_PROCESS_UNKNOWN,
    })
}

// --- Cellbox bounds validation ---
const ENV_CELLBOX_OK: c_int = 0;
const ENV_CELLBOX_BAD_X: c_int = 1;
const ENV_CELLBOX_BAD_Y: c_int = 2;
const ENV_CELLBOX_BAD_WIDTH: c_int = 3;
const ENV_CELLBOX_BAD_HEIGHT: c_int = 4;

/// Validate cellbox requisite bounds against world dimensions.
/// Returns 0 if OK, or 1-4 for the specific invalid parameter.
#[no_mangle]
pub extern "C" fn avd_env_cellbox_validate(
    xx: c_int,
    yy: c_int,
    width: c_int,
    height: c_int,
    world_x: c_int,
    world_y: c_int,
) -> c_int {
    if xx < 0 || xx >= world_x {
        ENV_CELLBOX_BAD_X
    } else if yy < 0 || yy >= world_y {
        ENV_CELLBOX_BAD_Y
    } else if width <= 0 || width + xx >= world_x {
        ENV_CELLBOX_BAD_WIDTH
    } else if height <= 0 || height + yy >= world_y {
        ENV_CELLBOX_BAD_HEIGHT
    } else {
        ENV_CELLBOX_OK
    }
}

// --- Requisite var_name classification ---
const ENV_REQUISITE_REACTION: c_int = 0;
const ENV_REQUISITE_NOREACTION: c_int = 1;
const ENV_REQUISITE_MIN_COUNT: c_int = 2;
const ENV_REQUISITE_MAX_COUNT: c_int = 3;
const ENV_REQUISITE_REACTION_MIN_COUNT: c_int = 4;
const ENV_REQUISITE_REACTION_MAX_COUNT: c_int = 5;
const ENV_REQUISITE_DIVIDE_ONLY: c_int = 6;
const ENV_REQUISITE_MIN_TOT_COUNT: c_int = 7;
const ENV_REQUISITE_MAX_TOT_COUNT: c_int = 8;
const ENV_REQUISITE_PARASITE_ONLY: c_int = 9;
const ENV_REQUISITE_CELLBOX: c_int = 10;
const ENV_REQUISITE_UNKNOWN: c_int = -1;

/// Classify a requisite variable name to an opcode.
#[no_mangle]
pub extern "C" fn avd_env_requisite_var_kind(var_name: *const c_char) -> c_int {
    with_cstr(var_name, ENV_REQUISITE_UNKNOWN, |s| match s.to_bytes() {
        b"reaction" => ENV_REQUISITE_REACTION,
        b"noreaction" => ENV_REQUISITE_NOREACTION,
        b"min_count" => ENV_REQUISITE_MIN_COUNT,
        b"max_count" => ENV_REQUISITE_MAX_COUNT,
        b"reaction_min_count" => ENV_REQUISITE_REACTION_MIN_COUNT,
        b"reaction_max_count" => ENV_REQUISITE_REACTION_MAX_COUNT,
        b"divide_only" => ENV_REQUISITE_DIVIDE_ONLY,
        b"min_tot_count" => ENV_REQUISITE_MIN_TOT_COUNT,
        b"max_tot_count" => ENV_REQUISITE_MAX_TOT_COUNT,
        b"parasite_only" => ENV_REQUISITE_PARASITE_ONLY,
        b"cellbox" => ENV_REQUISITE_CELLBOX,
        _ => ENV_REQUISITE_UNKNOWN,
    })
}

// --- Gradient resource update-action classifier ---
const ENV_GRADIENT_ACTION_BARRIER: c_int = 0;
const ENV_GRADIENT_ACTION_HILLS: c_int = 1;
const ENV_GRADIENT_ACTION_PROBABILISTIC: c_int = 2;
const ENV_GRADIENT_ACTION_PEAK: c_int = 3;

/// Classify gradient resource update action based on habitat and probabilistic flag.
/// habitat==2 → barrier, habitat==1 → hills, probabilistic → probabilistic, else → peak
#[no_mangle]
pub extern "C" fn avd_env_gradient_update_action(habitat: c_int, is_probabilistic: c_int) -> c_int {
    if habitat == 2 {
        ENV_GRADIENT_ACTION_BARRIER
    } else if habitat == 1 {
        ENV_GRADIENT_ACTION_HILLS
    } else if is_probabilistic != 0 {
        ENV_GRADIENT_ACTION_PROBABILISTIC
    } else {
        ENV_GRADIENT_ACTION_PEAK
    }
}

// --- Gradient temp height computation ---

/// Compute effective height for gradient boundary calculations.
/// If plateau < 0, returns 1; otherwise returns height.
#[no_mangle]
pub extern "C" fn avd_env_gradient_temp_height(plateau: f64, height: c_int) -> c_int {
    if plateau < 0.0 {
        1
    } else {
        height
    }
}

// --- Should-fillin-resource-values gate ---

/// Returns 1 if gradient resource values need updating this tick.
/// Active when: move_a_scaler > 1, or any inflow/outflow is nonzero,
/// or (non-moving resource that was just reset).
#[no_mangle]
pub extern "C" fn avd_env_gradient_should_fillin(
    move_a_scaler: f64,
    plateau_inflow: f64,
    plateau_outflow: f64,
    cone_inflow: f64,
    cone_outflow: f64,
    gradient_inflow: f64,
    just_reset: c_int,
) -> c_int {
    if move_a_scaler > 1.0
        || plateau_inflow != 0.0
        || plateau_outflow != 0.0
        || cone_inflow != 0.0
        || cone_outflow != 0.0
        || gradient_inflow != 0.0
        || (move_a_scaler == 1.0 && just_reset != 0)
    {
        1
    } else {
        0
    }
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

    // --- Resource var_name classification tests ---

    #[test]
    fn resource_var_kind_known_values() {
        let cases = [
            ("inflow", ENV_RES_INFLOW),
            ("outflow", ENV_RES_OUTFLOW),
            ("initial", ENV_RES_INITIAL),
            ("geometry", ENV_RES_GEOMETRY),
            ("cells", ENV_RES_CELLS),
            ("inflowx1", ENV_RES_INFLOWX1),
            ("inflowx", ENV_RES_INFLOWX1), // alias
            ("inflowy1", ENV_RES_INFLOWY1),
            ("inflowy", ENV_RES_INFLOWY1), // alias
            ("outflowx1", ENV_RES_OUTFLOWX1),
            ("outflowx", ENV_RES_OUTFLOWX1), // alias
            ("outflowy1", ENV_RES_OUTFLOWY1),
            ("outflowy", ENV_RES_OUTFLOWY1), // alias
            ("deme", ENV_RES_DEME),
            ("energy", ENV_RES_ENERGY),
            ("hgt", ENV_RES_HGT),
        ];
        for (input, expected) in &cases {
            let cs = cstr(input);
            assert_eq!(
                avd_env_resource_var_kind(cs.as_ptr()),
                *expected,
                "resource var_kind mismatch for '{input}'"
            );
        }
    }

    #[test]
    fn resource_var_kind_unknown() {
        let bad = cstr("bogus");
        assert_eq!(avd_env_resource_var_kind(bad.as_ptr()), ENV_RES_UNKNOWN);
        assert_eq!(avd_env_resource_var_kind(ptr::null()), ENV_RES_UNKNOWN);
    }

    // --- Process var_name classification tests ---

    #[test]
    fn process_var_kind_known_values() {
        let cases = [
            ("resource", ENV_PROCESS_RESOURCE),
            ("value", ENV_PROCESS_VALUE),
            ("type", ENV_PROCESS_TYPE),
            ("max", ENV_PROCESS_MAX),
            ("min", ENV_PROCESS_MIN),
            ("frac", ENV_PROCESS_FRAC),
            ("ksubm", ENV_PROCESS_KSUBM),
            ("product", ENV_PROCESS_PRODUCT),
            ("conversion", ENV_PROCESS_CONVERSION),
            ("inst", ENV_PROCESS_INST),
            ("random", ENV_PROCESS_RANDOM),
            ("lethal", ENV_PROCESS_LETHAL),
            ("sterilize", ENV_PROCESS_STERILIZE),
            ("deme", ENV_PROCESS_DEME),
            ("germline", ENV_PROCESS_GERMLINE),
            ("detect", ENV_PROCESS_DETECT),
            ("threshold", ENV_PROCESS_THRESHOLD),
            ("detectionerror", ENV_PROCESS_DETECTIONERROR),
            ("string", ENV_PROCESS_STRING),
            ("depletable", ENV_PROCESS_DEPLETABLE),
            ("phenplastbonus", ENV_PROCESS_PHENPLASTBONUS),
            ("internal", ENV_PROCESS_INTERNAL),
        ];
        for (input, expected) in &cases {
            let cs = cstr(input);
            assert_eq!(
                avd_env_process_var_kind(cs.as_ptr()),
                *expected,
                "process var_kind mismatch for '{input}'"
            );
        }
    }

    #[test]
    fn process_var_kind_unknown() {
        let bad = cstr("bogus");
        assert_eq!(avd_env_process_var_kind(bad.as_ptr()), ENV_PROCESS_UNKNOWN);
        assert_eq!(avd_env_process_var_kind(ptr::null()), ENV_PROCESS_UNKNOWN);
    }

    // --- Cellbox validation tests ---

    #[test]
    fn cellbox_validate_ok() {
        assert_eq!(avd_env_cellbox_validate(5, 5, 3, 3, 20, 20), ENV_CELLBOX_OK);
    }

    #[test]
    fn cellbox_validate_failures() {
        assert_eq!(
            avd_env_cellbox_validate(-1, 5, 3, 3, 20, 20),
            ENV_CELLBOX_BAD_X
        );
        assert_eq!(
            avd_env_cellbox_validate(20, 5, 3, 3, 20, 20),
            ENV_CELLBOX_BAD_X
        );
        assert_eq!(
            avd_env_cellbox_validate(5, -1, 3, 3, 20, 20),
            ENV_CELLBOX_BAD_Y
        );
        assert_eq!(
            avd_env_cellbox_validate(5, 5, 0, 3, 20, 20),
            ENV_CELLBOX_BAD_WIDTH
        );
        assert_eq!(
            avd_env_cellbox_validate(5, 5, 15, 3, 20, 20),
            ENV_CELLBOX_BAD_WIDTH
        );
        assert_eq!(
            avd_env_cellbox_validate(5, 5, 3, 0, 20, 20),
            ENV_CELLBOX_BAD_HEIGHT
        );
        assert_eq!(
            avd_env_cellbox_validate(5, 5, 3, 15, 20, 20),
            ENV_CELLBOX_BAD_HEIGHT
        );
    }

    // --- Requisite var_name classification tests ---

    #[test]
    fn requisite_var_kind_known_values() {
        let cases = [
            ("reaction", ENV_REQUISITE_REACTION),
            ("noreaction", ENV_REQUISITE_NOREACTION),
            ("min_count", ENV_REQUISITE_MIN_COUNT),
            ("max_count", ENV_REQUISITE_MAX_COUNT),
            ("reaction_min_count", ENV_REQUISITE_REACTION_MIN_COUNT),
            ("reaction_max_count", ENV_REQUISITE_REACTION_MAX_COUNT),
            ("divide_only", ENV_REQUISITE_DIVIDE_ONLY),
            ("min_tot_count", ENV_REQUISITE_MIN_TOT_COUNT),
            ("max_tot_count", ENV_REQUISITE_MAX_TOT_COUNT),
            ("parasite_only", ENV_REQUISITE_PARASITE_ONLY),
            ("cellbox", ENV_REQUISITE_CELLBOX),
        ];
        for (input, expected) in &cases {
            let cs = cstr(input);
            assert_eq!(
                avd_env_requisite_var_kind(cs.as_ptr()),
                *expected,
                "requisite var_kind mismatch for '{input}'"
            );
        }
    }

    #[test]
    fn requisite_var_kind_unknown() {
        let bad = cstr("unknown_field");
        assert_eq!(
            avd_env_requisite_var_kind(bad.as_ptr()),
            ENV_REQUISITE_UNKNOWN
        );
        assert_eq!(
            avd_env_requisite_var_kind(ptr::null()),
            ENV_REQUISITE_UNKNOWN
        );
    }

    // --- Gradient temp height tests ---

    #[test]
    fn gradient_temp_height_policy() {
        assert_eq!(avd_env_gradient_temp_height(-1.0, 5), 1);
        assert_eq!(avd_env_gradient_temp_height(-0.5, 5), 1);
        assert_eq!(avd_env_gradient_temp_height(0.0, 5), 5);
        assert_eq!(avd_env_gradient_temp_height(1.0, 10), 10);
    }

    // --- Should-fillin gate tests ---

    #[test]
    fn gradient_should_fillin_policy() {
        // moving resource
        assert_eq!(
            avd_env_gradient_should_fillin(2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0),
            1
        );
        // has inflow
        assert_eq!(
            avd_env_gradient_should_fillin(1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0),
            1
        );
        // non-moving just reset
        assert_eq!(
            avd_env_gradient_should_fillin(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1),
            1
        );
        // nothing active
        assert_eq!(
            avd_env_gradient_should_fillin(1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0),
            0
        );
    }

    // --- Gradient update action tests ---

    #[test]
    fn gradient_update_action_policy() {
        assert_eq!(
            avd_env_gradient_update_action(2, 0),
            ENV_GRADIENT_ACTION_BARRIER
        );
        assert_eq!(
            avd_env_gradient_update_action(1, 0),
            ENV_GRADIENT_ACTION_HILLS
        );
        assert_eq!(
            avd_env_gradient_update_action(0, 1),
            ENV_GRADIENT_ACTION_PROBABILISTIC
        );
        assert_eq!(
            avd_env_gradient_update_action(0, 0),
            ENV_GRADIENT_ACTION_PEAK
        );
        // habitat takes precedence over probabilistic
        assert_eq!(
            avd_env_gradient_update_action(2, 1),
            ENV_GRADIENT_ACTION_BARRIER
        );
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
