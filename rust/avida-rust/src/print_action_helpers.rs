use std::ffi::c_int;

const PRINTACTION_FILENAME_MODE_DEFAULT_PLAIN: c_int = 0;
const PRINTACTION_FILENAME_MODE_KEEP_PROVIDED: c_int = 1;
const PRINTACTION_FILENAME_MODE_FORMAT_WITH_INSTSET: c_int = 2;
const PRINTACTION_OUTPUT_SINK_STATS: c_int = 0;
const PRINTACTION_OUTPUT_SINK_RECORDER: c_int = 1;

fn printaction_instruction_filename_mode(
    has_filename_token: c_int,
    filename_empty: c_int,
) -> c_int {
    if has_filename_token == 0 {
        PRINTACTION_FILENAME_MODE_DEFAULT_PLAIN
    } else if filename_empty != 0 {
        PRINTACTION_FILENAME_MODE_FORMAT_WITH_INSTSET
    } else {
        PRINTACTION_FILENAME_MODE_KEEP_PROVIDED
    }
}

fn printaction_instruction_output_sink_kind(action_kind: c_int) -> c_int {
    match action_kind {
        0 | 1 => PRINTACTION_OUTPUT_SINK_RECORDER,
        2..=8 => PRINTACTION_OUTPUT_SINK_STATS,
        _ => -1,
    }
}

#[no_mangle]
pub extern "C" fn avd_printaction_instruction_filename_mode(
    has_filename_token: c_int,
    filename_empty: c_int,
) -> c_int {
    printaction_instruction_filename_mode(has_filename_token, filename_empty)
}

#[no_mangle]
pub extern "C" fn avd_printaction_instruction_output_sink_kind(action_kind: c_int) -> c_int {
    printaction_instruction_output_sink_kind(action_kind)
}

#[cfg(test)]
mod tests {
    use super::{
        avd_printaction_instruction_filename_mode, avd_printaction_instruction_output_sink_kind,
        PRINTACTION_FILENAME_MODE_DEFAULT_PLAIN, PRINTACTION_FILENAME_MODE_FORMAT_WITH_INSTSET,
        PRINTACTION_FILENAME_MODE_KEEP_PROVIDED, PRINTACTION_OUTPUT_SINK_RECORDER,
        PRINTACTION_OUTPUT_SINK_STATS,
    };

    #[test]
    fn printaction_instruction_filename_mode_policy() {
        assert_eq!(
            avd_printaction_instruction_filename_mode(0, 0),
            PRINTACTION_FILENAME_MODE_DEFAULT_PLAIN
        );
        assert_eq!(
            avd_printaction_instruction_filename_mode(1, 0),
            PRINTACTION_FILENAME_MODE_KEEP_PROVIDED
        );
        assert_eq!(
            avd_printaction_instruction_filename_mode(1, 1),
            PRINTACTION_FILENAME_MODE_FORMAT_WITH_INSTSET
        );
        assert_eq!(
            avd_printaction_instruction_filename_mode(0, 1),
            PRINTACTION_FILENAME_MODE_DEFAULT_PLAIN
        );
        assert_eq!(
            avd_printaction_instruction_filename_mode(2, 0),
            PRINTACTION_FILENAME_MODE_KEEP_PROVIDED
        );
    }

    #[test]
    fn printaction_instruction_output_sink_kind_policy() {
        assert_eq!(
            avd_printaction_instruction_output_sink_kind(0),
            PRINTACTION_OUTPUT_SINK_RECORDER
        );
        assert_eq!(
            avd_printaction_instruction_output_sink_kind(1),
            PRINTACTION_OUTPUT_SINK_RECORDER
        );
        assert_eq!(
            avd_printaction_instruction_output_sink_kind(2),
            PRINTACTION_OUTPUT_SINK_STATS
        );
        assert_eq!(
            avd_printaction_instruction_output_sink_kind(7),
            PRINTACTION_OUTPUT_SINK_STATS
        );
        assert_eq!(
            avd_printaction_instruction_output_sink_kind(8),
            PRINTACTION_OUTPUT_SINK_STATS
        );
        assert_eq!(avd_printaction_instruction_output_sink_kind(-1), -1);
        assert_eq!(avd_printaction_instruction_output_sink_kind(9), -1);
    }
}
