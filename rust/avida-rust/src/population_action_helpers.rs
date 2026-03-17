use std::ffi::c_int;

const POPACTION_SEED_ACTION_PROCEED: c_int = 0;
const POPACTION_SEED_ACTION_SKIP_AND_COUNT: c_int = 1;

fn popaction_deme_loop_start_index(energy_enabled: c_int) -> c_int {
    if energy_enabled == 1 {
        1
    } else {
        0
    }
}

fn popaction_seed_deme_action(energy_enabled: c_int, injected_count: c_int) -> c_int {
    if energy_enabled == 1 && injected_count == 0 {
        POPACTION_SEED_ACTION_SKIP_AND_COUNT
    } else {
        POPACTION_SEED_ACTION_PROCEED
    }
}

fn popaction_normalize_cell_end(cell_start: c_int, cell_end: c_int) -> c_int {
    if cell_end == -1 {
        cell_start.saturating_add(1)
    } else {
        cell_end
    }
}

fn popaction_is_valid_cell_range(
    cell_start: c_int,
    cell_end: c_int,
    population_size: c_int,
) -> c_int {
    if cell_start < 0 || cell_end > population_size || cell_start >= cell_end {
        0
    } else {
        1
    }
}

fn popaction_is_valid_cell_range_with_stride(
    cell_start: c_int,
    cell_end: c_int,
    population_size: c_int,
    cell_stride: c_int,
) -> c_int {
    if popaction_is_valid_cell_range(cell_start, cell_end, population_size) == 0 || cell_stride <= 0
    {
        0
    } else {
        1
    }
}

fn popaction_is_missing_filename_token(filename_size: c_int) -> c_int {
    if filename_size == 0 {
        1
    } else {
        0
    }
}

fn popaction_is_valid_well_mixed_cell_count(cell_count: c_int, population_size: c_int) -> c_int {
    if cell_count < 0 || cell_count > population_size {
        0
    } else {
        1
    }
}

fn popaction_is_valid_single_cell_id(cell_id: c_int, population_size: c_int) -> c_int {
    if cell_id < 0 || cell_id >= population_size {
        0
    } else {
        1
    }
}

fn popaction_is_valid_group_cell_id(cell_id: c_int, population_size: c_int) -> c_int {
    popaction_is_valid_single_cell_id(cell_id, population_size)
}

fn popaction_should_skip_parasite_injection(
    only_if_parasites_extinct: c_int,
    num_parasites: c_int,
) -> c_int {
    if only_if_parasites_extinct != 0 && num_parasites != 0 {
        1
    } else {
        0
    }
}

fn popaction_is_missing_parasite_filename_token(filename_size: c_int) -> c_int {
    if filename_size == 0 {
        1
    } else {
        0
    }
}

fn popaction_has_missing_parasite_pair_filenames(
    genome_filename_size: c_int,
    parasite_filename_size: c_int,
) -> c_int {
    if popaction_is_missing_parasite_filename_token(genome_filename_size) != 0
        || popaction_is_missing_parasite_filename_token(parasite_filename_size) != 0
    {
        1
    } else {
        0
    }
}

fn popaction_is_missing_parasite_label_token(label_size: c_int) -> c_int {
    if label_size == 0 {
        1
    } else {
        0
    }
}

fn popaction_is_missing_parasite_sequence_token(sequence_size: c_int) -> c_int {
    if sequence_size == 0 {
        1
    } else {
        0
    }
}

fn popaction_parasite_invalid_range_warning_kind(action_kind: c_int) -> c_int {
    match action_kind {
        0 | 1 => 0,
        2 => 1,
        _ => -1,
    }
}

fn popaction_parasite_warning_short_circuit_kind(
    action_kind: c_int,
    is_invalid_range: c_int,
) -> c_int {
    if is_invalid_range == 0 {
        -1
    } else {
        popaction_parasite_invalid_range_warning_kind(action_kind)
    }
}

fn popaction_parasite_missing_token_short_circuit_kind(
    action_kind: c_int,
    missing_filename: c_int,
    missing_label: c_int,
    missing_sequence: c_int,
) -> c_int {
    match action_kind {
        0 | 2 => {
            if missing_filename != 0 {
                0
            } else if missing_label != 0 {
                1
            } else {
                -1
            }
        }
        1 => {
            if missing_sequence != 0 {
                2
            } else if missing_label != 0 {
                1
            } else {
                -1
            }
        }
        _ => -1,
    }
}

fn popaction_parasite_missing_token_error_kind(missing_token_kind: c_int) -> c_int {
    match missing_token_kind {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => -1,
    }
}

#[no_mangle]
pub extern "C" fn avd_popaction_deme_loop_start_index(energy_enabled: c_int) -> c_int {
    popaction_deme_loop_start_index(energy_enabled)
}

#[no_mangle]
pub extern "C" fn avd_popaction_seed_deme_action(
    energy_enabled: c_int,
    injected_count: c_int,
) -> c_int {
    popaction_seed_deme_action(energy_enabled, injected_count)
}

#[no_mangle]
pub extern "C" fn avd_popaction_normalize_cell_end(cell_start: c_int, cell_end: c_int) -> c_int {
    popaction_normalize_cell_end(cell_start, cell_end)
}

#[no_mangle]
pub extern "C" fn avd_popaction_is_valid_cell_range(
    cell_start: c_int,
    cell_end: c_int,
    population_size: c_int,
) -> c_int {
    popaction_is_valid_cell_range(cell_start, cell_end, population_size)
}

#[no_mangle]
pub extern "C" fn avd_popaction_is_valid_cell_range_with_stride(
    cell_start: c_int,
    cell_end: c_int,
    population_size: c_int,
    cell_stride: c_int,
) -> c_int {
    popaction_is_valid_cell_range_with_stride(cell_start, cell_end, population_size, cell_stride)
}

#[no_mangle]
pub extern "C" fn avd_popaction_is_missing_filename_token(filename_size: c_int) -> c_int {
    popaction_is_missing_filename_token(filename_size)
}

#[no_mangle]
pub extern "C" fn avd_popaction_is_valid_well_mixed_cell_count(
    cell_count: c_int,
    population_size: c_int,
) -> c_int {
    popaction_is_valid_well_mixed_cell_count(cell_count, population_size)
}

#[no_mangle]
pub extern "C" fn avd_popaction_is_valid_single_cell_id(
    cell_id: c_int,
    population_size: c_int,
) -> c_int {
    popaction_is_valid_single_cell_id(cell_id, population_size)
}

#[no_mangle]
pub extern "C" fn avd_popaction_is_valid_group_cell_id(
    cell_id: c_int,
    population_size: c_int,
) -> c_int {
    popaction_is_valid_group_cell_id(cell_id, population_size)
}

#[no_mangle]
pub extern "C" fn avd_popaction_should_skip_parasite_injection(
    only_if_parasites_extinct: c_int,
    num_parasites: c_int,
) -> c_int {
    popaction_should_skip_parasite_injection(only_if_parasites_extinct, num_parasites)
}

#[no_mangle]
pub extern "C" fn avd_popaction_is_missing_parasite_filename_token(filename_size: c_int) -> c_int {
    popaction_is_missing_parasite_filename_token(filename_size)
}

#[no_mangle]
pub extern "C" fn avd_popaction_has_missing_parasite_pair_filenames(
    genome_filename_size: c_int,
    parasite_filename_size: c_int,
) -> c_int {
    popaction_has_missing_parasite_pair_filenames(genome_filename_size, parasite_filename_size)
}

#[no_mangle]
pub extern "C" fn avd_popaction_is_missing_parasite_label_token(label_size: c_int) -> c_int {
    popaction_is_missing_parasite_label_token(label_size)
}

#[no_mangle]
pub extern "C" fn avd_popaction_is_missing_parasite_sequence_token(sequence_size: c_int) -> c_int {
    popaction_is_missing_parasite_sequence_token(sequence_size)
}

#[no_mangle]
pub extern "C" fn avd_popaction_parasite_invalid_range_warning_kind(action_kind: c_int) -> c_int {
    popaction_parasite_invalid_range_warning_kind(action_kind)
}

#[no_mangle]
pub extern "C" fn avd_popaction_parasite_warning_short_circuit_kind(
    action_kind: c_int,
    is_invalid_range: c_int,
) -> c_int {
    popaction_parasite_warning_short_circuit_kind(action_kind, is_invalid_range)
}

#[no_mangle]
pub extern "C" fn avd_popaction_parasite_missing_token_short_circuit_kind(
    action_kind: c_int,
    missing_filename: c_int,
    missing_label: c_int,
    missing_sequence: c_int,
) -> c_int {
    popaction_parasite_missing_token_short_circuit_kind(
        action_kind,
        missing_filename,
        missing_label,
        missing_sequence,
    )
}

#[no_mangle]
pub extern "C" fn avd_popaction_parasite_missing_token_error_kind(
    missing_token_kind: c_int,
) -> c_int {
    popaction_parasite_missing_token_error_kind(missing_token_kind)
}

#[cfg(test)]
mod tests {
    use super::{
        avd_popaction_deme_loop_start_index, avd_popaction_has_missing_parasite_pair_filenames,
        avd_popaction_is_missing_filename_token, avd_popaction_is_missing_parasite_filename_token,
        avd_popaction_is_missing_parasite_label_token,
        avd_popaction_is_missing_parasite_sequence_token, avd_popaction_is_valid_cell_range,
        avd_popaction_is_valid_cell_range_with_stride, avd_popaction_is_valid_group_cell_id,
        avd_popaction_is_valid_single_cell_id, avd_popaction_is_valid_well_mixed_cell_count,
        avd_popaction_normalize_cell_end, avd_popaction_parasite_invalid_range_warning_kind,
        avd_popaction_parasite_missing_token_error_kind,
        avd_popaction_parasite_missing_token_short_circuit_kind,
        avd_popaction_parasite_warning_short_circuit_kind, avd_popaction_seed_deme_action,
        avd_popaction_should_skip_parasite_injection, POPACTION_SEED_ACTION_PROCEED,
        POPACTION_SEED_ACTION_SKIP_AND_COUNT,
    };

    #[test]
    fn popaction_deme_loop_start_index_policy() {
        assert_eq!(avd_popaction_deme_loop_start_index(1), 1);
        assert_eq!(avd_popaction_deme_loop_start_index(0), 0);
        assert_eq!(avd_popaction_deme_loop_start_index(2), 0);
    }

    #[test]
    fn popaction_seed_action_policy() {
        assert_eq!(
            avd_popaction_seed_deme_action(1, 0),
            POPACTION_SEED_ACTION_SKIP_AND_COUNT
        );
        assert_eq!(
            avd_popaction_seed_deme_action(1, 1),
            POPACTION_SEED_ACTION_PROCEED
        );
        assert_eq!(
            avd_popaction_seed_deme_action(0, 0),
            POPACTION_SEED_ACTION_PROCEED
        );
    }

    #[test]
    fn popaction_normalize_cell_end_policy() {
        assert_eq!(avd_popaction_normalize_cell_end(0, -1), 1);
        assert_eq!(avd_popaction_normalize_cell_end(5, -1), 6);
        assert_eq!(avd_popaction_normalize_cell_end(5, 9), 9);
    }

    #[test]
    fn popaction_is_valid_cell_range_policy() {
        assert_eq!(avd_popaction_is_valid_cell_range(0, 1, 10), 1);
        assert_eq!(avd_popaction_is_valid_cell_range(-1, 1, 10), 0);
        assert_eq!(avd_popaction_is_valid_cell_range(2, 2, 10), 0);
        assert_eq!(avd_popaction_is_valid_cell_range(0, 11, 10), 0);
    }

    #[test]
    fn popaction_is_valid_cell_range_with_stride_policy() {
        assert_eq!(
            avd_popaction_is_valid_cell_range_with_stride(0, 5, 10, 1),
            1
        );
        assert_eq!(
            avd_popaction_is_valid_cell_range_with_stride(0, 5, 10, 0),
            0
        );
        assert_eq!(
            avd_popaction_is_valid_cell_range_with_stride(0, 5, 10, -1),
            0
        );
        assert_eq!(
            avd_popaction_is_valid_cell_range_with_stride(5, 5, 10, 1),
            0
        );
    }

    #[test]
    fn popaction_is_missing_filename_token_policy() {
        assert_eq!(avd_popaction_is_missing_filename_token(0), 1);
        assert_eq!(avd_popaction_is_missing_filename_token(1), 0);
        assert_eq!(avd_popaction_is_missing_filename_token(12), 0);
        assert_eq!(avd_popaction_is_missing_filename_token(-1), 0);
    }

    #[test]
    fn popaction_is_valid_well_mixed_cell_count_policy() {
        assert_eq!(avd_popaction_is_valid_well_mixed_cell_count(0, 10), 1);
        assert_eq!(avd_popaction_is_valid_well_mixed_cell_count(10, 10), 1);
        assert_eq!(avd_popaction_is_valid_well_mixed_cell_count(-1, 10), 0);
        assert_eq!(avd_popaction_is_valid_well_mixed_cell_count(11, 10), 0);
    }

    #[test]
    fn popaction_is_valid_group_cell_id_policy() {
        assert_eq!(avd_popaction_is_valid_group_cell_id(0, 10), 1);
        assert_eq!(avd_popaction_is_valid_group_cell_id(9, 10), 1);
        assert_eq!(avd_popaction_is_valid_group_cell_id(-1, 10), 0);
        assert_eq!(avd_popaction_is_valid_group_cell_id(10, 10), 0);
        assert_eq!(avd_popaction_is_valid_group_cell_id(0, 0), 0);
        assert_eq!(
            avd_popaction_is_valid_group_cell_id(7, 10),
            avd_popaction_is_valid_single_cell_id(7, 10)
        );
        assert_eq!(
            avd_popaction_is_valid_group_cell_id(10, 10),
            avd_popaction_is_valid_single_cell_id(10, 10)
        );
    }

    #[test]
    fn popaction_is_valid_single_cell_id_policy() {
        assert_eq!(avd_popaction_is_valid_single_cell_id(0, 10), 1);
        assert_eq!(avd_popaction_is_valid_single_cell_id(9, 10), 1);
        assert_eq!(avd_popaction_is_valid_single_cell_id(-1, 10), 0);
        assert_eq!(avd_popaction_is_valid_single_cell_id(10, 10), 0);
        assert_eq!(avd_popaction_is_valid_single_cell_id(0, 0), 0);
    }

    #[test]
    fn popaction_should_skip_parasite_injection_policy() {
        assert_eq!(avd_popaction_should_skip_parasite_injection(1, 0), 0);
        assert_eq!(avd_popaction_should_skip_parasite_injection(1, 1), 1);
        assert_eq!(avd_popaction_should_skip_parasite_injection(0, 1), 0);
        assert_eq!(avd_popaction_should_skip_parasite_injection(2, 3), 1);
        assert_eq!(avd_popaction_should_skip_parasite_injection(1, -1), 1);
    }

    #[test]
    fn popaction_is_missing_parasite_filename_token_policy() {
        assert_eq!(avd_popaction_is_missing_parasite_filename_token(0), 1);
        assert_eq!(avd_popaction_is_missing_parasite_filename_token(1), 0);
        assert_eq!(avd_popaction_is_missing_parasite_filename_token(7), 0);
        assert_eq!(avd_popaction_is_missing_parasite_filename_token(-1), 0);
    }

    #[test]
    fn popaction_has_missing_parasite_pair_filenames_policy() {
        assert_eq!(avd_popaction_has_missing_parasite_pair_filenames(0, 2), 1);
        assert_eq!(avd_popaction_has_missing_parasite_pair_filenames(2, 0), 1);
        assert_eq!(avd_popaction_has_missing_parasite_pair_filenames(0, 0), 1);
        assert_eq!(avd_popaction_has_missing_parasite_pair_filenames(2, 3), 0);
    }

    #[test]
    fn popaction_is_missing_parasite_label_token_policy() {
        assert_eq!(avd_popaction_is_missing_parasite_label_token(0), 1);
        assert_eq!(avd_popaction_is_missing_parasite_label_token(1), 0);
        assert_eq!(avd_popaction_is_missing_parasite_label_token(5), 0);
        assert_eq!(avd_popaction_is_missing_parasite_label_token(-1), 0);
    }

    #[test]
    fn popaction_is_missing_parasite_sequence_token_policy() {
        assert_eq!(avd_popaction_is_missing_parasite_sequence_token(0), 1);
        assert_eq!(avd_popaction_is_missing_parasite_sequence_token(1), 0);
        assert_eq!(avd_popaction_is_missing_parasite_sequence_token(8), 0);
        assert_eq!(avd_popaction_is_missing_parasite_sequence_token(-1), 0);
    }

    #[test]
    fn popaction_parasite_invalid_range_warning_kind_policy() {
        assert_eq!(avd_popaction_parasite_invalid_range_warning_kind(0), 0);
        assert_eq!(avd_popaction_parasite_invalid_range_warning_kind(1), 0);
        assert_eq!(avd_popaction_parasite_invalid_range_warning_kind(2), 1);
        assert_eq!(avd_popaction_parasite_invalid_range_warning_kind(-1), -1);
        assert_eq!(avd_popaction_parasite_invalid_range_warning_kind(3), -1);
    }

    #[test]
    fn popaction_parasite_warning_short_circuit_kind_policy() {
        assert_eq!(avd_popaction_parasite_warning_short_circuit_kind(0, 1), 0);
        assert_eq!(avd_popaction_parasite_warning_short_circuit_kind(1, 1), 0);
        assert_eq!(avd_popaction_parasite_warning_short_circuit_kind(2, 1), 1);
        assert_eq!(avd_popaction_parasite_warning_short_circuit_kind(-1, 1), -1);
        assert_eq!(avd_popaction_parasite_warning_short_circuit_kind(0, 0), -1);
        assert_eq!(avd_popaction_parasite_warning_short_circuit_kind(2, 0), -1);
    }

    #[test]
    fn popaction_parasite_missing_token_short_circuit_kind_policy() {
        assert_eq!(
            avd_popaction_parasite_missing_token_short_circuit_kind(0, 1, 1, 1),
            0
        );
        assert_eq!(
            avd_popaction_parasite_missing_token_short_circuit_kind(0, 0, 1, 0),
            1
        );
        assert_eq!(
            avd_popaction_parasite_missing_token_short_circuit_kind(1, 0, 1, 1),
            2
        );
        assert_eq!(
            avd_popaction_parasite_missing_token_short_circuit_kind(1, 1, 1, 0),
            1
        );
        assert_eq!(
            avd_popaction_parasite_missing_token_short_circuit_kind(2, 1, 0, 0),
            0
        );
        assert_eq!(
            avd_popaction_parasite_missing_token_short_circuit_kind(2, 0, 1, 1),
            1
        );
        assert_eq!(
            avd_popaction_parasite_missing_token_short_circuit_kind(0, 0, 0, 0),
            -1
        );
        assert_eq!(
            avd_popaction_parasite_missing_token_short_circuit_kind(-1, 1, 1, 1),
            -1
        );
    }

    #[test]
    fn popaction_parasite_missing_token_error_kind_policy() {
        assert_eq!(avd_popaction_parasite_missing_token_error_kind(0), 0);
        assert_eq!(avd_popaction_parasite_missing_token_error_kind(1), 1);
        assert_eq!(avd_popaction_parasite_missing_token_error_kind(2), 2);
        assert_eq!(avd_popaction_parasite_missing_token_error_kind(-1), -1);
        assert_eq!(avd_popaction_parasite_missing_token_error_kind(3), -1);
    }
}
