use std::ffi::c_int;

// ASType_t enum values
const AS_TYPE_ARRAY: c_int = 0;
const AS_TYPE_BOOL: c_int = 1;
const AS_TYPE_CHAR: c_int = 2;
const AS_TYPE_DICT: c_int = 3;
const AS_TYPE_FLOAT: c_int = 4;
const AS_TYPE_INT: c_int = 5;
const AS_TYPE_OBJECT_REF: c_int = 6;
const AS_TYPE_MATRIX: c_int = 7;
const AS_TYPE_STRING: c_int = 8;
const AS_TYPE_INVALID: c_int = 12;

/// Determine the runtime type for a binary operation between two typed values.
/// Implements the type-promotion/coercion table from cDirectInterpretASTVisitor::getRuntimeType.
#[no_mangle]
pub extern "C" fn avd_script_get_runtime_type(
    ltype: c_int,
    rtype: c_int,
    allow_str: c_int,
) -> c_int {
    match ltype {
        AS_TYPE_ARRAY => AS_TYPE_ARRAY,
        AS_TYPE_BOOL => match rtype {
            AS_TYPE_ARRAY | AS_TYPE_BOOL | AS_TYPE_CHAR | AS_TYPE_FLOAT | AS_TYPE_INT
            | AS_TYPE_MATRIX | AS_TYPE_OBJECT_REF | AS_TYPE_STRING => AS_TYPE_BOOL,
            _ => AS_TYPE_INVALID,
        },
        AS_TYPE_CHAR => match rtype {
            AS_TYPE_ARRAY => AS_TYPE_ARRAY,
            AS_TYPE_BOOL | AS_TYPE_CHAR => AS_TYPE_CHAR,
            AS_TYPE_FLOAT => AS_TYPE_FLOAT,
            AS_TYPE_INT => AS_TYPE_INT,
            AS_TYPE_MATRIX => AS_TYPE_MATRIX,
            AS_TYPE_STRING if allow_str != 0 => AS_TYPE_STRING,
            _ => AS_TYPE_INVALID,
        },
        AS_TYPE_DICT => AS_TYPE_DICT,
        AS_TYPE_FLOAT => match rtype {
            AS_TYPE_ARRAY => AS_TYPE_ARRAY,
            AS_TYPE_BOOL | AS_TYPE_CHAR | AS_TYPE_FLOAT | AS_TYPE_INT => AS_TYPE_FLOAT,
            AS_TYPE_MATRIX => AS_TYPE_MATRIX,
            AS_TYPE_STRING if allow_str != 0 => AS_TYPE_FLOAT,
            _ => AS_TYPE_INVALID,
        },
        AS_TYPE_INT => match rtype {
            AS_TYPE_ARRAY => AS_TYPE_ARRAY,
            AS_TYPE_BOOL | AS_TYPE_CHAR | AS_TYPE_INT => AS_TYPE_INT,
            AS_TYPE_FLOAT => AS_TYPE_FLOAT,
            AS_TYPE_MATRIX => AS_TYPE_MATRIX,
            AS_TYPE_STRING if allow_str != 0 => AS_TYPE_INT,
            _ => AS_TYPE_INVALID,
        },
        AS_TYPE_MATRIX => AS_TYPE_MATRIX,
        AS_TYPE_STRING if allow_str != 0 => AS_TYPE_STRING,
        _ => AS_TYPE_INVALID,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_type_array_always_wins() {
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_ARRAY, AS_TYPE_INT, 0),
            AS_TYPE_ARRAY
        );
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_INT, AS_TYPE_ARRAY, 0),
            AS_TYPE_ARRAY
        );
    }

    #[test]
    fn runtime_type_bool_coercion() {
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_BOOL, AS_TYPE_INT, 0),
            AS_TYPE_BOOL
        );
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_BOOL, AS_TYPE_FLOAT, 0),
            AS_TYPE_BOOL
        );
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_BOOL, AS_TYPE_STRING, 0),
            AS_TYPE_BOOL
        );
    }

    #[test]
    fn runtime_type_numeric_promotion() {
        // int + float → float
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_INT, AS_TYPE_FLOAT, 0),
            AS_TYPE_FLOAT
        );
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_FLOAT, AS_TYPE_INT, 0),
            AS_TYPE_FLOAT
        );
        // int + int → int
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_INT, AS_TYPE_INT, 0),
            AS_TYPE_INT
        );
        // char + int → int
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_CHAR, AS_TYPE_INT, 0),
            AS_TYPE_INT
        );
    }

    #[test]
    fn runtime_type_string_gated() {
        // string without allow_str → invalid
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_STRING, AS_TYPE_INT, 0),
            AS_TYPE_INVALID
        );
        // string with allow_str → string
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_STRING, AS_TYPE_INT, 1),
            AS_TYPE_STRING
        );
        // int + string without allow_str → invalid
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_INT, AS_TYPE_STRING, 0),
            AS_TYPE_INVALID
        );
        // int + string with allow_str → int
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_INT, AS_TYPE_STRING, 1),
            AS_TYPE_INT
        );
    }

    #[test]
    fn runtime_type_matrix_promotion() {
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_MATRIX, AS_TYPE_INT, 0),
            AS_TYPE_MATRIX
        );
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_INT, AS_TYPE_MATRIX, 0),
            AS_TYPE_MATRIX
        );
    }

    #[test]
    fn runtime_type_invalid_combos() {
        assert_eq!(
            avd_script_get_runtime_type(AS_TYPE_INVALID, AS_TYPE_INT, 0),
            AS_TYPE_INVALID
        );
        assert_eq!(avd_script_get_runtime_type(99, 0, 0), AS_TYPE_INVALID);
    }
}
