use std::ffi::{c_double, c_int};

use crate::{
    common::{with_owi_mut, with_owi_ref},
    AvidaOrderedWeightedIndexHandle,
};

impl AvidaOrderedWeightedIndexHandle {
    fn new() -> Self {
        Self {
            item_weight: Vec::new(),
            cum_weight: Vec::new(),
            item_value: Vec::new(),
        }
    }

    fn set_weight(&mut self, value: i32, in_weight: f64) {
        self.item_value.push(value);
        self.item_weight.push(in_weight);
        let next = match self.cum_weight.last() {
            Some(prev) => *prev + in_weight,
            None => in_weight,
        };
        self.cum_weight.push(next);
    }

    fn find_position(&self, position: f64) -> i32 {
        if self.item_weight.is_empty() {
            return -1;
        }
        self.lookup(position, 0, self.item_weight.len() - 1)
    }

    fn lookup(&self, weight: f64, ndx_a: usize, ndx_e: usize) -> i32 {
        if ndx_a > ndx_e {
            return -1;
        }
        let mid = ndx_a + (ndx_e - ndx_a) / 2;
        if self.cum_weight[mid] - self.item_weight[mid] <= weight && self.cum_weight[mid] > weight {
            return self.item_value[mid];
        }
        if self.cum_weight[mid] > weight {
            if mid == 0 {
                return -1;
            }
            self.lookup(weight, ndx_a, mid - 1)
        } else {
            self.lookup(weight, mid + 1, ndx_e)
        }
    }
}

#[no_mangle]
pub extern "C" fn avd_owi_new() -> *mut AvidaOrderedWeightedIndexHandle {
    Box::into_raw(Box::new(AvidaOrderedWeightedIndexHandle::new()))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_owi_clone(
    other: *const AvidaOrderedWeightedIndexHandle,
) -> *mut AvidaOrderedWeightedIndexHandle {
    if other.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: pointer was checked for null and is only read.
    let other_ref = unsafe { &*other };
    Box::into_raw(Box::new(AvidaOrderedWeightedIndexHandle {
        item_weight: other_ref.item_weight.clone(),
        cum_weight: other_ref.cum_weight.clone(),
        item_value: other_ref.item_value.clone(),
    }))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_owi_free(handle: *mut AvidaOrderedWeightedIndexHandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer came from Box::into_raw in this crate and is freed exactly once here.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub extern "C" fn avd_owi_set_weight(
    handle: *mut AvidaOrderedWeightedIndexHandle,
    value: c_int,
    weight: c_double,
) {
    with_owi_mut(handle, |h| h.set_weight(value, weight));
}

#[no_mangle]
pub extern "C" fn avd_owi_get_weight(
    handle: *const AvidaOrderedWeightedIndexHandle,
    id: c_int,
) -> c_double {
    if id < 0 {
        return 0.0;
    }
    with_owi_ref(handle, 0.0, |h| {
        let idx = id as usize;
        if idx >= h.item_weight.len() {
            0.0
        } else {
            h.item_weight[idx]
        }
    })
}

#[no_mangle]
pub extern "C" fn avd_owi_get_value(
    handle: *const AvidaOrderedWeightedIndexHandle,
    id: c_int,
) -> c_int {
    if id < 0 {
        return -1;
    }
    with_owi_ref(handle, -1, |h| {
        let idx = id as usize;
        if idx >= h.item_value.len() {
            -1
        } else {
            h.item_value[idx]
        }
    })
}

#[no_mangle]
pub extern "C" fn avd_owi_get_total_weight(
    handle: *const AvidaOrderedWeightedIndexHandle,
) -> c_double {
    with_owi_ref(handle, 0.0, |h| *h.cum_weight.last().unwrap_or(&0.0))
}

#[no_mangle]
pub extern "C" fn avd_owi_get_size(handle: *const AvidaOrderedWeightedIndexHandle) -> c_int {
    with_owi_ref(handle, 0, |h| h.item_weight.len() as c_int)
}

#[no_mangle]
pub extern "C" fn avd_owi_find_position(
    handle: *const AvidaOrderedWeightedIndexHandle,
    position: c_double,
) -> c_int {
    with_owi_ref(handle, -1, |h| h.find_position(position))
}
