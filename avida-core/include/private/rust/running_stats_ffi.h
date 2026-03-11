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
double avd_rc_accumulate_update_time(double current, double delta);
int avd_rc_num_steps(double update_time, double update_step);
int avd_rc_num_spatial_updates(int current_update, int previous_update);
double avd_rc_remainder_update_time(double update_time, double update_step, int num_steps);
double avd_rc_apply_nonspatial_steps(double current, const double* decay_precalc, const double* inflow_precalc, int precalc_distance, int num_steps);
int avd_src_normalize_span(int start, int end, int bound, int* out_start, int* out_end);
double avd_src_compute_flow_scalar(double elem1_amount, double elem2_amount, double inxdiffuse, double inydiffuse, double inxgravity, double inygravity, int xdist, int ydist, double dist);
double avd_src_source_per_cell(double amount, int x1, int x2, int y1, int y2);
double avd_src_sink_delta(double current_amount, double decay);
double avd_src_cell_outflow_delta(double current_amount, double outflow);
int avd_src_wrapped_elem_index(int x, int y, int world_x, int world_y);
int avd_rh_select_entry_index(const int* updates, int count, int update, int exact);
double avd_rh_value_at_or_zero(const double* values, int count, int index);
int avd_event_parse_trigger(const char* token);
int avd_event_parse_timing(const char* timing, double* out_start, double* out_interval, double* out_stop);

#ifdef __cplusplus
}
#endif

#endif
