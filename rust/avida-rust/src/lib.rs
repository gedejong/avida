#![deny(unsafe_op_in_unsafe_fn)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![cfg_attr(
    not(test),
    deny(
        clippy::dbg_macro,
        clippy::expect_used,
        clippy::panic,
        clippy::todo,
        clippy::unimplemented,
        clippy::unwrap_used
    )
)]
#![cfg_attr(test, allow(clippy::expect_used, clippy::panic, clippy::unwrap_used))]

pub mod bit_array;
pub mod common;
pub mod double_sum;
pub mod histogram;
pub mod ordered_weighted_index;
pub mod package;
pub mod provider_helpers;
pub mod resource_count_helpers;
pub mod resource_history_helpers;
pub mod running_average;
pub mod running_stats;
pub mod time_series_recorder;
pub mod weighted_index;

pub use package::*;
pub use provider_helpers::*;

#[repr(C)]
pub struct AvidaRunningStatsHandle {
    pub(crate) n: f64,
    pub(crate) m1: f64,
    pub(crate) m2: f64,
    pub(crate) m3: f64,
    pub(crate) m4: f64,
}

#[repr(C)]
pub struct AvidaRunningAverageHandle {
    pub(crate) values: Vec<f64>,
    pub(crate) s1: f64,
    pub(crate) s2: f64,
    pub(crate) window_size: usize,
    pub(crate) pointer: usize,
    pub(crate) n: usize,
}

#[repr(C)]
pub struct AvidaDoubleSumHandle {
    pub(crate) s1: f64,
    pub(crate) s2: f64,
    pub(crate) n: f64,
    pub(crate) max: f64,
}

#[repr(C)]
pub struct AvidaWeightedIndexHandle {
    pub(crate) size: usize,
    pub(crate) item_weight: Vec<f64>,
    pub(crate) subtree_weight: Vec<f64>,
}

#[repr(C)]
pub struct AvidaOrderedWeightedIndexHandle {
    pub(crate) item_weight: Vec<f64>,
    pub(crate) cum_weight: Vec<f64>,
    pub(crate) item_value: Vec<i32>,
}

#[repr(C)]
pub struct AvidaHistogramHandle {
    pub(crate) bins: Vec<i32>,
    pub(crate) min_bin: i32,
    pub(crate) max_bin: i32,
    pub(crate) entry_count: i32,
    pub(crate) entry_total: i32,
}

#[repr(C)]
pub struct AvidaRawBitArrayHandle {
    pub(crate) bit_fields: Vec<u32>,
}
