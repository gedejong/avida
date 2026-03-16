#ifndef AVIDA_RUST_RUNNING_STATS_FFI_H
#define AVIDA_RUST_RUNNING_STATS_FFI_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct AvidaRunningStatsHandle AvidaRunningStatsHandle;
typedef struct AvidaRunningAverageHandle AvidaRunningAverageHandle;
typedef struct AvidaDoubleSumHandle AvidaDoubleSumHandle;
typedef struct AvidaWeightedIndexHandle AvidaWeightedIndexHandle;
typedef struct AvidaOrderedWeightedIndexHandle AvidaOrderedWeightedIndexHandle;
typedef struct AvidaHistogramHandle AvidaHistogramHandle;
typedef struct AvidaRawBitArrayHandle AvidaRawBitArrayHandle;
typedef struct AvidaTimeSeriesHandle AvidaTimeSeriesHandle;

enum {
  AVD_RC_DISPATCH_NONE = 0,
  AVD_RC_DISPATCH_NONSPATIAL = 1,
  AVD_RC_DISPATCH_SPATIAL = 2,
  AVD_RC_WRAPPER_GLOBAL_ONLY = 0,
  AVD_RC_WRAPPER_RANDOM = 1,
  AVD_RC_WRAPPER_FULL = 2,
  AVD_RC_READ_PATH_GLOBAL = 0,
  AVD_RC_READ_PATH_SPATIAL = 1,
  AVD_RC_SETCELL_GLOBAL_NOOP = 0,
  AVD_RC_SETCELL_SPATIAL_WRITE = 1,
  AVD_RC_SETUP_PATH_GLOBAL = 0,
  AVD_RC_SETUP_PATH_PARTIAL = 1,
  AVD_RC_SETUP_PATH_SPATIAL = 2,
  AVD_RC_GRAD_SETTER_PEAK_X = 0,
  AVD_RC_GRAD_SETTER_PEAK_Y = 1,
  AVD_RC_GRAD_SETTER_HEIGHT = 2,
  AVD_RC_GRAD_SETTER_SPREAD = 3,
  AVD_RC_GRAD_SETTER_PLATEAU = 4,
  AVD_RC_GRAD_SETTER_INITIAL_PLAT = 5,
  AVD_RC_GRAD_SETTER_DECAY = 6,
  AVD_RC_GRAD_SETTER_MAX_X = 7,
  AVD_RC_GRAD_SETTER_MAX_Y = 8,
  AVD_RC_GRAD_SETTER_MIN_X = 9,
  AVD_RC_GRAD_SETTER_MIN_Y = 10,
  AVD_RC_GRAD_SETTER_MOVE_SCALER = 11,
  AVD_RC_GRAD_SETTER_UPDATE_STEP = 12,
  AVD_RC_GRAD_SETTER_IS_HALO = 13,
  AVD_RC_GRAD_SETTER_HALO_INNER_RADIUS = 14,
  AVD_RC_GRAD_SETTER_HALO_WIDTH = 15,
  AVD_RC_GRAD_SETTER_HALO_ANCHOR_X = 16,
  AVD_RC_GRAD_SETTER_HALO_ANCHOR_Y = 17,
  AVD_RC_GRAD_SETTER_MOVE_SPEED = 18,
  AVD_RC_GRAD_SETTER_MOVE_RESISTANCE = 19,
  AVD_RC_GRAD_SETTER_PLATEAU_INFLOW = 20,
  AVD_RC_GRAD_SETTER_PLATEAU_OUTFLOW = 21,
  AVD_RC_GRAD_SETTER_CONE_INFLOW = 22,
  AVD_RC_GRAD_SETTER_CONE_OUTFLOW = 23,
  AVD_RC_GRAD_SETTER_GRADIENT_INFLOW = 24,
  AVD_RC_GRAD_SETTER_PLATEAU_COMMON = 25,
  AVD_RC_GRAD_SETTER_FLOOR = 26,
  AVD_RC_GRAD_SETTER_HABITAT = 27,
  AVD_RC_GRAD_SETTER_MIN_SIZE = 28,
  AVD_RC_GRAD_SETTER_MAX_SIZE = 29,
  AVD_RC_GRAD_SETTER_CONFIG = 30,
  AVD_RC_GRAD_SETTER_COUNT = 31,
  AVD_RC_GRAD_SETTER_RESISTANCE = 32,
  AVD_RC_GRAD_SETTER_DAMAGE = 33,
  AVD_RC_GRAD_SETTER_THRESHOLD = 34,
  AVD_RC_GRAD_SETTER_REFUGE = 35,
  AVD_RC_GRAD_SETTER_DEATH_ODDS = 36,
  AVD_RC_GRAD_SETTER_INVALID = -1
};

AvidaRunningStatsHandle* avd_rs_new(void);
AvidaRunningStatsHandle* avd_rs_clone(const AvidaRunningStatsHandle* other);
void avd_rs_free(AvidaRunningStatsHandle* handle);

void avd_rs_clear(AvidaRunningStatsHandle* handle);
void avd_rs_push(AvidaRunningStatsHandle* handle, double x);

double avd_rs_n(const AvidaRunningStatsHandle* handle);
double avd_rs_mean(const AvidaRunningStatsHandle* handle);
double avd_rs_variance(const AvidaRunningStatsHandle* handle);
double avd_rs_std_deviation(const AvidaRunningStatsHandle* handle);
double avd_rs_std_error(const AvidaRunningStatsHandle* handle);
double avd_rs_skewness(const AvidaRunningStatsHandle* handle);
double avd_rs_kurtosis(const AvidaRunningStatsHandle* handle);

AvidaRunningAverageHandle* avd_ra_new(int window_size);
void avd_ra_free(AvidaRunningAverageHandle* handle);

void avd_ra_clear(AvidaRunningAverageHandle* handle);
void avd_ra_add(AvidaRunningAverageHandle* handle, double value);

double avd_ra_sum(const AvidaRunningAverageHandle* handle);
double avd_ra_sum_of_squares(const AvidaRunningAverageHandle* handle);
double avd_ra_average(const AvidaRunningAverageHandle* handle);
double avd_ra_variance(const AvidaRunningAverageHandle* handle);
double avd_ra_std_deviation(const AvidaRunningAverageHandle* handle);
double avd_ra_std_error(const AvidaRunningAverageHandle* handle);

AvidaDoubleSumHandle* avd_ds_new(void);
AvidaDoubleSumHandle* avd_ds_clone(const AvidaDoubleSumHandle* other);
void avd_ds_free(AvidaDoubleSumHandle* handle);

void avd_ds_clear(AvidaDoubleSumHandle* handle);
void avd_ds_add(AvidaDoubleSumHandle* handle, double value, double weight);
void avd_ds_subtract(AvidaDoubleSumHandle* handle, double value, double weight);

double avd_ds_count(const AvidaDoubleSumHandle* handle);
double avd_ds_sum(const AvidaDoubleSumHandle* handle);
double avd_ds_max(const AvidaDoubleSumHandle* handle);
double avd_ds_average(const AvidaDoubleSumHandle* handle);
double avd_ds_variance(const AvidaDoubleSumHandle* handle);
double avd_ds_std_deviation(const AvidaDoubleSumHandle* handle);
double avd_ds_std_error(const AvidaDoubleSumHandle* handle);

AvidaWeightedIndexHandle* avd_wi_new(int size);
AvidaWeightedIndexHandle* avd_wi_clone(const AvidaWeightedIndexHandle* other);
void avd_wi_free(AvidaWeightedIndexHandle* handle);
void avd_wi_set_weight(AvidaWeightedIndexHandle* handle, int id, double weight);
double avd_wi_get_weight(const AvidaWeightedIndexHandle* handle, int id);
double avd_wi_get_total_weight(const AvidaWeightedIndexHandle* handle);
int avd_wi_get_size(const AvidaWeightedIndexHandle* handle);
int avd_wi_find_position(const AvidaWeightedIndexHandle* handle, double position, int root_id);

AvidaOrderedWeightedIndexHandle* avd_owi_new(void);
AvidaOrderedWeightedIndexHandle* avd_owi_clone(const AvidaOrderedWeightedIndexHandle* other);
void avd_owi_free(AvidaOrderedWeightedIndexHandle* handle);
void avd_owi_set_weight(AvidaOrderedWeightedIndexHandle* handle, int value, double weight);
double avd_owi_get_weight(const AvidaOrderedWeightedIndexHandle* handle, int id);
int avd_owi_get_value(const AvidaOrderedWeightedIndexHandle* handle, int id);
double avd_owi_get_total_weight(const AvidaOrderedWeightedIndexHandle* handle);
int avd_owi_get_size(const AvidaOrderedWeightedIndexHandle* handle);
int avd_owi_find_position(const AvidaOrderedWeightedIndexHandle* handle, double position);

AvidaHistogramHandle* avd_hist_new(int max_bin, int min_bin);
void avd_hist_free(AvidaHistogramHandle* handle);
void avd_hist_resize(AvidaHistogramHandle* handle, int new_max, int new_min);
void avd_hist_clear(AvidaHistogramHandle* handle);
void avd_hist_insert(AvidaHistogramHandle* handle, int value, int count);
void avd_hist_remove(AvidaHistogramHandle* handle, int value);
void avd_hist_remove_bin(AvidaHistogramHandle* handle, int value);

double avd_hist_get_average(const AvidaHistogramHandle* handle);
double avd_hist_get_count_average(const AvidaHistogramHandle* handle);
int avd_hist_get_mode(const AvidaHistogramHandle* handle);
double avd_hist_get_variance(const AvidaHistogramHandle* handle);
double avd_hist_get_count_variance(const AvidaHistogramHandle* handle);
double avd_hist_get_std_dev(const AvidaHistogramHandle* handle);
double avd_hist_get_count_std_dev(const AvidaHistogramHandle* handle);
double avd_hist_get_entropy(const AvidaHistogramHandle* handle);
double avd_hist_get_norm_entropy(const AvidaHistogramHandle* handle);

int avd_hist_get_count(const AvidaHistogramHandle* handle);
int avd_hist_get_count_for_value(const AvidaHistogramHandle* handle, int value);
int avd_hist_get_total(const AvidaHistogramHandle* handle);
int avd_hist_get_min_bin(const AvidaHistogramHandle* handle);
int avd_hist_get_max_bin(const AvidaHistogramHandle* handle);
int avd_hist_get_num_bins(const AvidaHistogramHandle* handle);

AvidaRawBitArrayHandle* avd_rba_new(int num_bits);
AvidaRawBitArrayHandle* avd_rba_clone(const AvidaRawBitArrayHandle* other);
void avd_rba_free(AvidaRawBitArrayHandle* handle);
void avd_rba_resize(AvidaRawBitArrayHandle* handle, int old_bits, int new_bits);
void avd_rba_zero(AvidaRawBitArrayHandle* handle, int num_bits);
void avd_rba_ones(AvidaRawBitArrayHandle* handle, int num_bits);
int avd_rba_get_bit(const AvidaRawBitArrayHandle* handle, int index);
void avd_rba_set_bit(AvidaRawBitArrayHandle* handle, int index, int value);
int avd_rba_is_equal(const AvidaRawBitArrayHandle* left, const AvidaRawBitArrayHandle* right, int num_bits);
int avd_rba_count_bits(const AvidaRawBitArrayHandle* handle, int num_bits);
int avd_rba_count_bits2(const AvidaRawBitArrayHandle* handle, int num_bits);
int avd_rba_find_bit1(const AvidaRawBitArrayHandle* handle, int num_bits, int start_pos);
void avd_rba_not(AvidaRawBitArrayHandle* handle, int num_bits);
void avd_rba_and(AvidaRawBitArrayHandle* handle, const AvidaRawBitArrayHandle* other, int num_bits);
void avd_rba_or(AvidaRawBitArrayHandle* handle, const AvidaRawBitArrayHandle* other, int num_bits);
void avd_rba_nand(AvidaRawBitArrayHandle* handle, const AvidaRawBitArrayHandle* other, int num_bits);
void avd_rba_nor(AvidaRawBitArrayHandle* handle, const AvidaRawBitArrayHandle* other, int num_bits);
void avd_rba_xor(AvidaRawBitArrayHandle* handle, const AvidaRawBitArrayHandle* other, int num_bits);
void avd_rba_equ(AvidaRawBitArrayHandle* handle, const AvidaRawBitArrayHandle* other, int num_bits);
void avd_rba_shift(AvidaRawBitArrayHandle* handle, int num_bits, int shift_size);
void avd_rba_increment(AvidaRawBitArrayHandle* handle, int num_bits);

int avd_pkg_array_bool_value(int count);
int avd_pkg_array_int_value(int count);
double avd_pkg_array_double_value(void);
int avd_pkg_str_as_bool(const char* value);
int avd_pkg_str_as_int(const char* value);
double avd_pkg_str_as_double(const char* value);
char* avd_pkg_bool_to_string(int value);
char* avd_pkg_int_to_string(int value);
char* avd_pkg_double_to_string(double value);
char* avd_pkg_array_descriptor(int count);
char* avd_pkg_array_string_value(const char* const* entries, int count);
void avd_pkg_string_free(char* value);

AvidaTimeSeriesHandle* avd_tsr_new(void);
AvidaTimeSeriesHandle* avd_tsr_from_string(const char* serialized);
void avd_tsr_free(AvidaTimeSeriesHandle* handle);
int avd_tsr_len(const AvidaTimeSeriesHandle* handle);
int avd_tsr_update_at(const AvidaTimeSeriesHandle* handle, int index);
char* avd_tsr_value_as_cstr(const AvidaTimeSeriesHandle* handle, int index);
int avd_tsr_value_as_bool(const AvidaTimeSeriesHandle* handle, int index, int* out_value);
int avd_tsr_value_as_int(const AvidaTimeSeriesHandle* handle, int index, int* out_value);
int avd_tsr_value_as_double(const AvidaTimeSeriesHandle* handle, int index, double* out_value);
void avd_tsr_push_bool(AvidaTimeSeriesHandle* handle, int update, int value);
void avd_tsr_push_int(AvidaTimeSeriesHandle* handle, int update, int value);
void avd_tsr_push_double(AvidaTimeSeriesHandle* handle, int update, double value);
void avd_tsr_push_string(AvidaTimeSeriesHandle* handle, int update, const char* value);
char* avd_tsr_as_string(const AvidaTimeSeriesHandle* handle);
void avd_tsr_string_free(char* value);

int avd_provider_is_standard_id(const char* data_id);
int avd_provider_is_argumented_id(const char* data_id);
int avd_provider_split_argumented_id(const char* data_id, char** out_raw_id, char** out_argument);
int avd_provider_classify_id(const char* data_id, char** out_raw_id, char** out_argument);
void avd_provider_string_free(char* value);
int avd_rc_lookup_resource_index(const char* const* names, int count, const char* query);
double avd_rc_step_inflow(double inflow, double update_step);
double avd_rc_step_decay(double decay_rate, double update_step);
double avd_rc_inflow_precalc_next(double previous, double step_decay, double step_inflow);
double avd_rc_decay_precalc_next(double previous, double step_decay);
void avd_rc_fill_precalc_tables(double decay_rate, double inflow, double update_step, int precalc_distance, double* out_decay, double* out_inflow);
void avd_rc_fill_inflow_precalc_table(double decay_rate, double inflow, double update_step, int precalc_distance, double* out_inflow);
void avd_rc_fill_decay_precalc_table(double decay_rate, double update_step, int precalc_distance, double* out_decay);
double avd_rc_accumulate_update_time(double current, double delta);
double avd_rc_update_time_delta(double in_time);
int avd_rc_wrapper_global_only_flag(int wrapper_mode);
int avd_rc_num_steps(double update_time, double update_step);
int avd_rc_num_spatial_updates(int current_update, int previous_update);
double avd_rc_remainder_update_time(double update_time, double update_step, int num_steps);
double avd_rc_apply_nonspatial_steps(double current, const double* decay_precalc, const double* inflow_precalc, int precalc_distance, int num_steps);
int avd_rc_spatial_step_iterations(int num_updates);
int avd_rc_use_cell_list_branch(int cell_list_size);
int avd_rc_is_spatial_geometry(int geometry);
int avd_rc_dispatch_action(int is_spatial, int global_only);
int avd_rc_should_advance_last_updated(int global_only);
int avd_rc_read_path_kind(int geometry);
int avd_rc_setcell_write_path_kind(int geometry);
int avd_rc_setup_path_kind(int geometry);
int avd_rc_should_log_spatial_rectangles(int geometry);
int avd_rc_resize_cell_count(int world_x, int world_y);
int avd_rc_gradient_setter_count(void);
int avd_rc_gradient_setter_opcode(int index);
int avd_src_normalize_span(int start, int end, int bound, int* out_start, int* out_end);
double avd_src_compute_flow_scalar(double elem1_amount, double elem2_amount, double inxdiffuse, double inydiffuse, double inxgravity, double inygravity, int xdist, int ydist, double dist);
int avd_src_compute_flow_pair_deltas(double elem1_amount, double elem2_amount, double inxdiffuse, double inydiffuse, double inxgravity, double inygravity, int xdist, int ydist, double dist, double* out_elem1_delta, double* out_elem2_delta);
double avd_src_source_per_cell(double amount, int x1, int x2, int y1, int y2);
double avd_src_sink_delta(double current_amount, double decay);
double avd_src_cell_outflow_delta(double current_amount, double outflow);
int avd_src_wrapped_elem_index(int x, int y, int world_x, int world_y);
int avd_src_cell_id_in_bounds_strict(int cell_id, int grid_size);
int avd_src_cell_id_in_bounds_legacy_setcell(int cell_id, int grid_size);
int avd_src_setpointer_entry(int cell_id, int world_x, int world_y, int geometry, int slot, int* out_elempt, int* out_xdist, int* out_ydist, double* out_dist);
int avd_src_state_fold(double amount, double delta, double* out_amount, double* out_delta);
double avd_src_sum_amounts(const double* values, int count);
int avd_src_rate_next_delta(double current_delta, double rate_in, double* out_delta);
int avd_src_reset_amount(double res_initial, double cell_initial, double* out_amount);
int avd_src_setcell_apply_initial(double amount, double delta, double cell_initial, double* out_amount, double* out_delta);
int avd_rh_select_entry_index(const int* updates, int count, int update, int exact);
double avd_rh_value_at_or_zero(const double* values, int count, int index);
int avd_event_parse_trigger(const char* token);
int avd_event_parse_timing(const char* timing, double* out_start, double* out_interval, double* out_stop);

#ifdef __cplusplus
}
#endif

#endif
