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
  AVD_RC_GRAD_SETTER_INVALID = -1,
  AVD_RC_GRAD_SCALAR_SETTER_PLATEAU_INFLOW = 0,
  AVD_RC_GRAD_SCALAR_SETTER_PLATEAU_OUTFLOW = 1,
  AVD_RC_GRAD_SCALAR_SETTER_CONE_INFLOW = 2,
  AVD_RC_GRAD_SCALAR_SETTER_CONE_OUTFLOW = 3,
  AVD_RC_GRAD_SCALAR_SETTER_GRADIENT_INFLOW = 4,
  AVD_RC_GRAD_SCALAR_SETTER_INVALID = -1,
  AVD_RC_GRAD_VAR_INFLOW_SETTER_PLAT_VAR_INFLOW = 0,
  AVD_RC_GRAD_VAR_INFLOW_SETTER_INVALID = -1,
  AVD_RC_PREDATORY_SETTER_SET_PREDATORY_RESOURCE = 0,
  AVD_RC_PREDATORY_SETTER_INVALID = -1,
  AVD_RC_PROBABILISTIC_SETTER_SET_PROBABILISTIC_RESOURCE = 0,
  AVD_RC_PROBABILISTIC_SETTER_INVALID = -1,
  AVD_RC_PEAK_GETTER_CURR_X = 0,
  AVD_RC_PEAK_GETTER_CURR_Y = 1,
  AVD_RC_PEAK_GETTER_FROZEN_X = 2,
  AVD_RC_PEAK_GETTER_FROZEN_Y = 3,
  AVD_RC_PEAK_GETTER_INVALID = -1,
  AVD_TASKLIB_UNARY_OP_LOG = 0,
  AVD_TASKLIB_UNARY_OP_LOG2 = 1,
  AVD_TASKLIB_UNARY_OP_LOG10 = 2,
  AVD_TASKLIB_UNARY_OP_SQRT = 3,
  AVD_TASKLIB_UNARY_OP_SINE = 4,
  AVD_TASKLIB_UNARY_OP_COSINE = 5,
  AVD_TASKLIB_UNARY_OP_INVALID = -1,
  AVD_TASKLIB_BINARY_OP_MULT = 0,
  AVD_TASKLIB_BINARY_OP_DIV = 1,
  AVD_TASKLIB_BINARY_OP_INVALID = -1,
  AVD_CPU_DISPATCH_FAMILY_INVALID = -1,
  AVD_CPU_DISPATCH_FAMILY_NOP = 0,
  AVD_CPU_DISPATCH_FAMILY_LABEL = 1,
  AVD_CPU_DISPATCH_FAMILY_PROMOTER = 2,
  AVD_CPU_DISPATCH_FAMILY_STALL = 3,
  AVD_CPU_DISPATCH_FAMILY_DEFAULT = 4,
  AVD_POPACTION_SEED_ACTION_PROCEED = 0,
  AVD_POPACTION_SEED_ACTION_SKIP_AND_COUNT = 1,
  AVD_POPACTION_PARASITE_WARNING_ACTION_INJECT_PARASITE = 0,
  AVD_POPACTION_PARASITE_WARNING_ACTION_INJECT_PARASITE_SEQUENCE = 1,
  AVD_POPACTION_PARASITE_WARNING_ACTION_INJECT_PARASITE_PAIR = 2,
  AVD_POPACTION_PARASITE_WARNING_ACTION_INVALID = -1,
  AVD_POPACTION_PARASITE_WARNING_KIND_INJECT_PARASITE = 0,
  AVD_POPACTION_PARASITE_WARNING_KIND_INJECT_PARASITE_PAIR = 1,
  AVD_POPACTION_PARASITE_WARNING_KIND_INVALID = -1,
  AVD_POPACTION_PARASITE_MISSING_TOKEN_FILENAME = 0,
  AVD_POPACTION_PARASITE_MISSING_TOKEN_LABEL = 1,
  AVD_POPACTION_PARASITE_MISSING_TOKEN_SEQUENCE = 2,
  AVD_POPACTION_PARASITE_MISSING_TOKEN_INVALID = -1,
  AVD_POPACTION_PARASITE_MISSING_ACTION_INJECT_PARASITE = 0,
  AVD_POPACTION_PARASITE_MISSING_ACTION_INJECT_PARASITE_SEQUENCE = 1,
  AVD_POPACTION_PARASITE_MISSING_ACTION_INJECT_PARASITE_PAIR = 2,
  AVD_POPACTION_PARASITE_MISSING_ACTION_INVALID = -1,
  AVD_POPACTION_PARASITE_ERROR_KIND_ORGANISM_FILE = 0,
  AVD_POPACTION_PARASITE_ERROR_KIND_LABEL = 1,
  AVD_POPACTION_PARASITE_ERROR_KIND_SEQUENCE = 2,
  AVD_POPACTION_PARASITE_ERROR_KIND_INVALID = -1,
  AVD_PRINTACTION_FILENAME_MODE_DEFAULT_PLAIN = 0,
  AVD_PRINTACTION_FILENAME_MODE_KEEP_PROVIDED = 1,
  AVD_PRINTACTION_FILENAME_MODE_FORMAT_WITH_INSTSET = 2,
  AVD_PRINTACTION_OUTPUT_SINK_STATS = 0,
  AVD_PRINTACTION_OUTPUT_SINK_RECORDER = 1,
  AVD_PRINTACTION_OUTPUT_SINK_INVALID = -1,
  AVD_CPOP_DEME_BLOCK_SKIP = 0,
  AVD_CPOP_DEME_BLOCK_RUN = 1,
  AVD_CPOP_ROUTING_MODE_PROCESS_STEP = 0,
  AVD_CPOP_ROUTING_MODE_SPECULATIVE_STEP = 1,
  AVD_CPOP_ROUTING_MODE_INVALID = -1,
  AVD_ANALYZE_OUTPUT_FILE_TYPE_KIND_KEEP_CURRENT = 0,
  AVD_ANALYZE_OUTPUT_FILE_TYPE_KIND_HTML = 1,
  AVD_ANALYZE_OUTPUT_SINK_KIND_FILE = 0,
  AVD_ANALYZE_OUTPUT_SINK_KIND_COUT = 1,
  AVD_ANALYZE_OUTPUT_SINK_KIND_INVALID = -1,
  AVD_ANALYZE_OUTPUT_HANDLE_MODE_CREATE = 0,
  AVD_ANALYZE_OUTPUT_HANDLE_MODE_STATIC = 1,
  AVD_ANALYZE_OUTPUT_HANDLE_MODE_INVALID = -1,
  AVD_ANALYZE_OUTPUT_HANDLE_ACTION_DETAIL = 0,
  AVD_ANALYZE_OUTPUT_HANDLE_ACTION_DETAIL_TIMELINE = 1,
  AVD_ANALYZE_OUTPUT_HANDLE_ACTION_HISTOGRAM = 2,
  AVD_ANALYZE_OUTPUT_HANDLE_ACTION_INVALID = -1,
  AVD_ANALYZE_OUTPUT_TOKEN_ABSENT = 0,
  AVD_ANALYZE_OUTPUT_TOKEN_PRESENT = 1,
  AVD_ANALYZE_FILE_TYPE_TOKEN_KIND_INVALID = -1,
  AVD_ANALYZE_FILE_TYPE_TOKEN_KIND_TEXT = 0,
  AVD_ANALYZE_FILE_TYPE_TOKEN_KIND_HTML = 1,
  AVD_ANALYZE_REL_MASK_LESS = 1,
  AVD_ANALYZE_REL_MASK_EQUAL = 2,
  AVD_ANALYZE_REL_MASK_GREATER = 4,
  AVD_ENV_PROCTYPE_ADD = 0,
  AVD_ENV_PROCTYPE_MULT = 1,
  AVD_ENV_PROCTYPE_POW = 2,
  AVD_ENV_PROCTYPE_LIN = 3,
  AVD_ENV_PROCTYPE_ENERGY = 4,
  AVD_ENV_PROCTYPE_ENZYME = 5,
  AVD_ENV_PROCTYPE_EXP = 6,
  AVD_ENV_PROCTYPE_UNKNOWN = -1,
  AVD_ENV_PHENPLAST_DEFAULT = 0,
  AVD_ENV_PHENPLAST_NO_BONUS = 1,
  AVD_ENV_PHENPLAST_FRAC_BONUS = 2,
  AVD_ENV_PHENPLAST_FULL_BONUS = 3,
  AVD_ENV_PHENPLAST_UNKNOWN = -1,
  AVD_ENV_ENTRY_TYPE_PROCESS = 0,
  AVD_ENV_ENTRY_TYPE_REQUISITE = 1,
  AVD_ENV_ENTRY_TYPE_CONTEXT_REQUISITE = 2,
  AVD_ENV_ENTRY_TYPE_UNKNOWN = -1,
  AVD_CPU_THREAD_CHANGE_NONE = 0,
  AVD_CPU_THREAD_CHANGE_KILLED_ONE = 1,
  AVD_CPU_THREAD_CHANGE_DIVIDE = 2,
  AVD_CPU_THREAD_CHANGE_ERROR = 3,
  AVD_CPOP_FORAGER_TYPE_PREY = 0,
  AVD_CPOP_FORAGER_TYPE_TOP_PRED = 1,
  AVD_CPOP_FORAGER_TYPE_PRED = 2,
  AVD_CPU_ALLOC_OK = 0,
  AVD_CPU_ALLOC_TOO_SMALL = 1,
  AVD_CPU_ALLOC_OUT_OF_RANGE = 2,
  AVD_CPU_ALLOC_TOO_LARGE = 3,
  AVD_CPU_ALLOC_PARENT_TOO_LARGE = 4,
  AVD_CPU_MATH_COMPUTE = 0,
  AVD_CPU_MATH_NOOP = 1,
  AVD_CPU_MATH_FAULT_NEGATIVE = 2,
  AVD_CPU_DIV_OK = 0,
  AVD_CPU_DIV_ZERO = 1,
  AVD_CPU_DIV_OVERFLOW = 2,
  AVD_CPOP_DEME_RESET_BOTH = 0,
  AVD_CPOP_DEME_RESET_TARGET_ONLY = 1,
  AVD_CPOP_DEME_RESET_NEITHER = 2,
  AVD_CPOP_DEME_RESET_INVALID = -1,
  AVD_ENV_GEOMETRY_GLOBAL = 0,
  AVD_ENV_GEOMETRY_GRID = 1,
  AVD_ENV_GEOMETRY_TORUS = 2,
  AVD_ENV_GEOMETRY_PARTIAL = 5,
  AVD_ENV_GEOMETRY_UNKNOWN = -1,
  AVD_ENV_BOOL_FALSE = 0,
  AVD_ENV_BOOL_TRUE = 1,
  AVD_ENV_BOOL_INVALID = -1,
  AVD_ENV_GRADIENT_ACTION_BARRIER = 0,
  AVD_ENV_GRADIENT_ACTION_HILLS = 1,
  AVD_ENV_GRADIENT_ACTION_PROBABILISTIC = 2,
  AVD_ENV_GRADIENT_ACTION_PEAK = 3,
  AVD_ENV_CELLBOX_OK = 0,
  AVD_ENV_CELLBOX_BAD_X = 1,
  AVD_ENV_CELLBOX_BAD_Y = 2,
  AVD_ENV_CELLBOX_BAD_WIDTH = 3,
  AVD_ENV_CELLBOX_BAD_HEIGHT = 4,
  AVD_ENV_REQUISITE_REACTION = 0,
  AVD_ENV_REQUISITE_NOREACTION = 1,
  AVD_ENV_REQUISITE_MIN_COUNT = 2,
  AVD_ENV_REQUISITE_MAX_COUNT = 3,
  AVD_ENV_REQUISITE_REACTION_MIN_COUNT = 4,
  AVD_ENV_REQUISITE_REACTION_MAX_COUNT = 5,
  AVD_ENV_REQUISITE_DIVIDE_ONLY = 6,
  AVD_ENV_REQUISITE_MIN_TOT_COUNT = 7,
  AVD_ENV_REQUISITE_MAX_TOT_COUNT = 8,
  AVD_ENV_REQUISITE_PARASITE_ONLY = 9,
  AVD_ENV_REQUISITE_CELLBOX = 10,
  AVD_ENV_REQUISITE_UNKNOWN = -1,
  AVD_ENV_PROCESS_RESOURCE = 0,
  AVD_ENV_PROCESS_VALUE = 1,
  AVD_ENV_PROCESS_TYPE = 2,
  AVD_ENV_PROCESS_MAX = 3,
  AVD_ENV_PROCESS_MIN = 4,
  AVD_ENV_PROCESS_FRAC = 5,
  AVD_ENV_PROCESS_KSUBM = 6,
  AVD_ENV_PROCESS_PRODUCT = 7,
  AVD_ENV_PROCESS_CONVERSION = 8,
  AVD_ENV_PROCESS_INST = 9,
  AVD_ENV_PROCESS_RANDOM = 10,
  AVD_ENV_PROCESS_LETHAL = 11,
  AVD_ENV_PROCESS_STERILIZE = 12,
  AVD_ENV_PROCESS_DEME = 13,
  AVD_ENV_PROCESS_GERMLINE = 14,
  AVD_ENV_PROCESS_DETECT = 15,
  AVD_ENV_PROCESS_THRESHOLD = 16,
  AVD_ENV_PROCESS_DETECTIONERROR = 17,
  AVD_ENV_PROCESS_STRING = 18,
  AVD_ENV_PROCESS_DEPLETABLE = 19,
  AVD_ENV_PROCESS_PHENPLASTBONUS = 20,
  AVD_ENV_PROCESS_INTERNAL = 21,
  AVD_ENV_PROCESS_UNKNOWN = -1,
  AVD_ENV_RES_INFLOW = 0,
  AVD_ENV_RES_OUTFLOW = 1,
  AVD_ENV_RES_INITIAL = 2,
  AVD_ENV_RES_GEOMETRY = 3,
  AVD_ENV_RES_CELLS = 4,
  AVD_ENV_RES_INFLOWX1 = 5,
  AVD_ENV_RES_INFLOWX2 = 6,
  AVD_ENV_RES_INFLOWY1 = 7,
  AVD_ENV_RES_INFLOWY2 = 8,
  AVD_ENV_RES_OUTFLOWX1 = 9,
  AVD_ENV_RES_OUTFLOWX2 = 10,
  AVD_ENV_RES_OUTFLOWY1 = 11,
  AVD_ENV_RES_OUTFLOWY2 = 12,
  AVD_ENV_RES_XDIFFUSE = 13,
  AVD_ENV_RES_XGRAVITY = 14,
  AVD_ENV_RES_YDIFFUSE = 15,
  AVD_ENV_RES_YGRAVITY = 16,
  AVD_ENV_RES_DEME = 17,
  AVD_ENV_RES_COLLECTABLE = 18,
  AVD_ENV_RES_ENERGY = 19,
  AVD_ENV_RES_HGT = 20,
  AVD_ENV_RES_UNKNOWN = -1,
  AVD_ENV_GRAD_PEAKX = 0,
  AVD_ENV_GRAD_PEAKY = 1,
  AVD_ENV_GRAD_HEIGHT = 2,
  AVD_ENV_GRAD_SPREAD = 3,
  AVD_ENV_GRAD_PLATEAU = 4,
  AVD_ENV_GRAD_DECAY = 5,
  AVD_ENV_GRAD_MAX_X = 6,
  AVD_ENV_GRAD_MAX_Y = 7,
  AVD_ENV_GRAD_MIN_X = 8,
  AVD_ENV_GRAD_MIN_Y = 9,
  AVD_ENV_GRAD_MOVE_A_SCALER = 10,
  AVD_ENV_GRAD_UPDATESTEP = 11,
  AVD_ENV_GRAD_HALO = 12,
  AVD_ENV_GRAD_HALO_INNER_RADIUS = 13,
  AVD_ENV_GRAD_HALO_ANCHOR_X = 14,
  AVD_ENV_GRAD_HALO_ANCHOR_Y = 15,
  AVD_ENV_GRAD_MOVE_SPEED = 16,
  AVD_ENV_GRAD_MOVE_RESISTANCE = 17,
  AVD_ENV_GRAD_HALO_WIDTH = 18,
  AVD_ENV_GRAD_PLATEAU_INFLOW = 19,
  AVD_ENV_GRAD_PLATEAU_OUTFLOW = 20,
  AVD_ENV_GRAD_CONE_INFLOW = 21,
  AVD_ENV_GRAD_CONE_OUTFLOW = 22,
  AVD_ENV_GRAD_GRADIENT_INFLOW = 23,
  AVD_ENV_GRAD_INITIAL = 24,
  AVD_ENV_GRAD_COMMON = 25,
  AVD_ENV_GRAD_FLOOR = 26,
  AVD_ENV_GRAD_HABITAT = 27,
  AVD_ENV_GRAD_MIN_SIZE = 28,
  AVD_ENV_GRAD_MAX_SIZE = 29,
  AVD_ENV_GRAD_CONFIG = 30,
  AVD_ENV_GRAD_COUNT = 31,
  AVD_ENV_GRAD_RESISTANCE = 32,
  AVD_ENV_GRAD_DAMAGE = 33,
  AVD_ENV_GRAD_DEADLY = 34,
  AVD_ENV_GRAD_PATH = 35,
  AVD_ENV_GRAD_HAMMER = 36,
  AVD_ENV_GRAD_THRESHOLD = 37,
  AVD_ENV_GRAD_REFUGE = 38,
  AVD_ENV_GRAD_UNKNOWN = -1,
  AVD_CPOP_FT_TRANSITION_NONE = 0,
  AVD_CPOP_FT_TRANSITION_PREY_TO_PRED = 1,
  AVD_CPOP_FT_TRANSITION_TOP_PRED_TO_PRED = 2,
  AVD_CPOP_FT_TRANSITION_PREY_TO_TOP_PRED = 3,
  AVD_CPOP_FT_TRANSITION_PRED_TO_TOP_PRED = 4,
  AVD_CPOP_FT_TRANSITION_PRED_TO_PREY = 5,
  AVD_CPOP_FT_TRANSITION_TOP_PRED_TO_PREY = 6,
  AVD_CPOP_MSG_BUFFER_DROP_OLDEST = 0,
  AVD_CPOP_MSG_BUFFER_DROP_NEW = 1,
  AVD_CPOP_MSG_BUFFER_INVALID = -1
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
double avd_tasklib_fractional_reward_bits(unsigned int supplied, unsigned int correct);
int avd_tasklib_is_logic3_or_math1_name(const char* task_name);
int avd_tasklib_is_math2_or_math3_name(const char* task_name);
int avd_tasklib_is_fibonacci_name(const char* task_name);
int avd_tasklib_is_matching_sequence_name(const char* task_name);
int avd_tasklib_is_load_based_name(const char* task_name);
double avd_tasklib_threshold_halflife_quality(long long diff, int threshold, double halflife_arg);
long long avd_tasklib_diff_scan_init(void);
long long avd_tasklib_diff_scan_update(long long current_min, long long candidate);
long long avd_tasklib_unary_math_input_diff(int input_value, long long test_output, int op_kind, double cast_precision);
long long avd_tasklib_binary_pair_input_diff(int lhs_value, int rhs_value, long long test_output, int op_kind);
double avd_tasklib_fib_check(int test_output, int fib_index);
int avd_tasklib_is_basic_name(const char* task_name);
int avd_tasklib_is_comm_name(const char* task_name);
int avd_tasklib_is_movement_name(const char* task_name);
int avd_tasklib_is_event_name(const char* task_name);
int avd_tasklib_is_altruism_name(const char* task_name);
int avd_cpu_dispatch_family(int is_nop, int is_label, int is_promoter, int should_stall);
int avd_cpu_dispatch_counted_opcode(int opcode, int dispatch_family);
int avd_cpu_thread_change_kind(int num_threads_before, int thread_size_after);
int avd_cpu_should_die_max_executed(int max_executed, int time_used, int to_die);
int avd_cpu_should_suppress_no_promoter(int promoters_enabled, int no_active_promoter_effect, int promoter_index);
int avd_cpu_should_terminate_promoter(int promoter_inst_max, int promoter_inst_executed);
int avd_cpu_task_switch_penalty(int penalty_type, int num_new_unique_reactions, int penalty_per_switch);
int avd_cpu_gradient_facing(int northerly, int easterly);
int avd_cpu_alloc_validity(int allocated_size, int old_size, int min_genome, int max_genome, int max_alloc_size, int max_old_size);
int avd_cpu_next_register(int default_register, int num_registers);
int avd_cpu_prev_register(int default_register, int num_registers);
int avd_cpu_unary_math_domain(int value, int threshold);
int avd_cpu_div_guard(int op1, int op2, int int_min);
int avd_popaction_deme_loop_start_index(int energy_enabled);
int avd_popaction_seed_deme_action(int energy_enabled, int injected_count);
int avd_popaction_normalize_cell_end(int cell_start, int cell_end);
int avd_popaction_is_valid_cell_range(int cell_start, int cell_end, int population_size);
int avd_popaction_is_valid_cell_range_with_stride(int cell_start, int cell_end, int population_size, int cell_stride);
int avd_popaction_is_missing_filename_token(int filename_size);
int avd_popaction_is_valid_well_mixed_cell_count(int cell_count, int population_size);
int avd_popaction_is_valid_single_cell_id(int cell_id, int population_size);
int avd_popaction_is_valid_group_cell_id(int cell_id, int population_size);
int avd_popaction_should_skip_parasite_injection(int only_if_parasites_extinct, int num_parasites);
int avd_popaction_is_missing_parasite_filename_token(int filename_size);
int avd_popaction_has_missing_parasite_pair_filenames(int genome_filename_size, int parasite_filename_size);
int avd_popaction_is_missing_parasite_label_token(int label_size);
int avd_popaction_is_missing_parasite_sequence_token(int sequence_size);
int avd_popaction_parasite_invalid_range_warning_kind(int action_kind);
int avd_popaction_parasite_warning_short_circuit_kind(int action_kind, int is_invalid_range);
int avd_popaction_parasite_missing_token_short_circuit_kind(int action_kind, int missing_filename, int missing_label, int missing_sequence);
int avd_popaction_parasite_missing_token_error_kind(int missing_token_kind);
int avd_printaction_instruction_filename_mode(int has_filename_token, int filename_empty);
int avd_printaction_instruction_output_sink_kind(int action_kind);
int avd_cpop_should_check_implicit_deme_repro(int num_demes);
int avd_cpop_should_run_speculative_deme_block(int num_demes);
int avd_cpop_should_update_deme_counters(int num_demes);
int avd_cpop_should_run_multi_deme_block(int num_demes);
int avd_cpop_deme_routing_short_circuit_kind(int routing_mode, int num_demes);
int avd_cpop_is_pred_prey_tracking_active(int pred_prey_switch);
int avd_cpop_forager_type_kind(int is_prey_ft, int is_top_pred_ft);
int avd_cpop_is_deadly_boundary(int deadly_boundaries, int geometry, int dest_x, int dest_y, int world_x, int world_y);
int avd_cpop_is_valid_prey_target(int forage_target, int parent_ft);
int avd_cpop_is_merit_bonus_enabled(int rewarded_instruction);
int avd_cpop_deme_reset_resources_kind(int config_value);
int avd_cpop_should_kill_rand_prey(int max_prey, int num_prey, int is_prey_ft);
int avd_cpop_should_kill_test_birth(int birth_method, int is_inject);
int avd_cpop_forage_target_transition(int new_ft, int old_ft);
int avd_cpop_is_birth_method_eldest(int birth_method);
int avd_cpop_is_divide_method_split(int divide_method);
int avd_cpop_is_generation_inc_both(int gen_inc_method);
int avd_cpop_is_divide_method_split_or_birth(int divide_method);
int avd_cpop_should_copy_parent_ft(int pred_prey_switch, int parent_ft, int forage_target);
int avd_cpop_should_kill_rand_pred(int parent_ft, int max_pred, int num_total_pred);
int avd_cpop_msg_buffer_overflow_action(int behavior);
int avd_cpop_is_msg_buffer_full(int buffer_size, int current_count);
int avd_analyze_relation_mask(const char* relation);
int avd_analyze_is_html_extension(const char* extension);
int avd_analyze_is_html_filename_token(const char* filename_token);
int avd_analyze_output_file_type_short_circuit_kind(int has_html_extension);
int avd_analyze_output_sink_short_circuit_kind(int is_cout_filename);
int avd_analyze_output_file_handle_mode_short_circuit_kind(int action_kind);
int avd_analyze_output_token_presence_short_circuit_kind(int remaining_arg_size);
int avd_analyze_file_type_token_short_circuit_kind(int has_text_token, int has_html_token);
int avd_analyze_apply_file_type_token_policy(int has_text_token, int has_html_token, int current_file_type, int text_file_type, int html_file_type);
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
int avd_rc_gradient_scalar_setter_count(void);
int avd_rc_gradient_scalar_setter_opcode(int index);
int avd_rc_gradient_var_inflow_setter_count(void);
int avd_rc_gradient_var_inflow_setter_opcode(int index);
int avd_rc_predatory_setter_count(void);
int avd_rc_predatory_setter_opcode(int index);
int avd_rc_probabilistic_setter_count(void);
int avd_rc_probabilistic_setter_opcode(int index);
int avd_rc_peak_getter_count(void);
int avd_rc_peak_getter_opcode(int index);
int avd_rc_peak_getter_requires_update(int opcode);
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
int avd_stats_is_dual_task_filename(const char* filename);
int avd_stats_is_dual_internal_task_filename(const char* filename);
int avd_stats_is_spatial_resource(int geometry);
double avd_stats_task_quality_average(double quality, int count);
int avd_stats_is_wall_gradient(int is_gradient, int habitat);
int avd_stats_is_den_habitat(int habitat);

int avd_env_process_type(const char* type_str);
int avd_env_phenplast_bonus_method(const char* method_str);
int avd_env_reaction_entry_type(const char* entry_str);
double avd_deme_base_merit(int method, double const_merit);
int avd_deme_should_join_germline_first(int selection_method);
double avd_deme_reaction_weight(double slope, int index);

int avd_env_gradient_var_kind(const char* var_name);
int avd_env_resource_var_kind(const char* var_name);
int avd_env_process_var_kind(const char* var_name);
int avd_env_cellbox_validate(int xx, int yy, int width, int height, int world_x, int world_y);
int avd_env_requisite_var_kind(const char* var_name);

int avd_env_gradient_update_action(int habitat, int is_probabilistic);
int avd_env_gradient_temp_height(double plateau, int height);
int avd_env_gradient_should_fillin(double move_a_scaler, double plateau_inflow, double plateau_outflow, double cone_inflow, double cone_outflow, double gradient_inflow, int just_reset);
int avd_env_geometry_type(const char* geometry_str);
int avd_sensor_normalize_search_type(int habitat_used, int search_type, int pred_experiment, int is_predator);
int avd_sensor_clamp_distance(int distance_sought, int max_dist);
int avd_sensor_max_distance(int look_dist_config, int world_x, int world_y);
int avd_sensor_clamp_id_sought(int id_sought);
int avd_env_parse_bool_string(const char* value_str);

int avd_event_parse_trigger(const char* token);
int avd_event_parse_timing(const char* timing, double* out_start, double* out_interval, double* out_stop);

#ifdef __cplusplus
}
#endif

#endif
