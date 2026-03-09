use std::ffi::{c_double, c_int};

use crate::{
    common::{with_wi_mut, with_wi_ref},
    AvidaWeightedIndexHandle,
};

impl AvidaWeightedIndexHandle {
    fn new(size: usize) -> Self {
        Self {
            size,
            item_weight: vec![0.0; size],
            subtree_weight: vec![0.0; size],
        }
    }

    fn set_weight(&mut self, id: usize, in_weight: f64) {
        if id >= self.size {
            return;
        }
        self.item_weight[id] = in_weight;
        let mut cur_id = id;
        loop {
            let left_id = 2 * cur_id + 1;
            let right_id = 2 * cur_id + 2;
            let left_subtree = if left_id >= self.size {
                0.0
            } else {
                self.subtree_weight[left_id]
            };
            let right_subtree = if right_id >= self.size {
                0.0
            } else {
                self.subtree_weight[right_id]
            };
            self.subtree_weight[cur_id] = self.item_weight[cur_id] + left_subtree + right_subtree;
            if cur_id == 0 {
                break;
            }
            cur_id = (cur_id - 1) / 2;
        }
    }

    fn find_position(&self, mut position: f64, mut root_id: usize) -> i32 {
        if self.size == 0 || root_id >= self.size || position >= self.subtree_weight[root_id] {
            return -1;
        }
        loop {
            if position < self.item_weight[root_id] {
                return root_id as i32;
            }
            position -= self.item_weight[root_id];
            let left_id = 2 * root_id + 1;
            if left_id >= self.size {
                return -1;
            }
            if position < self.subtree_weight[left_id] {
                root_id = left_id;
                continue;
            }
            position -= self.subtree_weight[left_id];
            let right_id = 2 * root_id + 2;
            if right_id >= self.size || position >= self.subtree_weight[right_id] {
                return -1;
            }
            root_id = right_id;
        }
    }
}

#[no_mangle]
pub extern "C" fn avd_wi_new(size: c_int) -> *mut AvidaWeightedIndexHandle {
    if size <= 0 {
        return std::ptr::null_mut();
    }
    let valid_size = match usize::try_from(size) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(AvidaWeightedIndexHandle::new(valid_size)))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_wi_clone(
    other: *const AvidaWeightedIndexHandle,
) -> *mut AvidaWeightedIndexHandle {
    if other.is_null() {
        return std::ptr::null_mut();
    }
    // SAFETY: pointer was checked for null and is only read.
    let other_ref = unsafe { &*other };
    Box::into_raw(Box::new(AvidaWeightedIndexHandle {
        size: other_ref.size,
        item_weight: other_ref.item_weight.clone(),
        subtree_weight: other_ref.subtree_weight.clone(),
    }))
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_wi_free(handle: *mut AvidaWeightedIndexHandle) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer came from Box::into_raw in this crate and is freed exactly once here.
    unsafe {
        drop(Box::from_raw(handle));
    }
}

#[no_mangle]
pub extern "C" fn avd_wi_set_weight(
    handle: *mut AvidaWeightedIndexHandle,
    id: c_int,
    weight: c_double,
) {
    if id < 0 {
        return;
    }
    with_wi_mut(handle, |h| h.set_weight(id as usize, weight));
}

#[no_mangle]
pub extern "C" fn avd_wi_get_weight(
    handle: *const AvidaWeightedIndexHandle,
    id: c_int,
) -> c_double {
    if id < 0 {
        return 0.0;
    }
    with_wi_ref(handle, 0.0, |h| {
        let idx = id as usize;
        if idx >= h.size {
            0.0
        } else {
            h.item_weight[idx]
        }
    })
}

#[no_mangle]
pub extern "C" fn avd_wi_get_total_weight(handle: *const AvidaWeightedIndexHandle) -> c_double {
    with_wi_ref(handle, 0.0, |h| {
        if h.size == 0 {
            0.0
        } else {
            h.subtree_weight[0]
        }
    })
}

#[no_mangle]
pub extern "C" fn avd_wi_get_size(handle: *const AvidaWeightedIndexHandle) -> c_int {
    with_wi_ref(handle, 0, |h| h.size as c_int)
}

#[no_mangle]
pub extern "C" fn avd_wi_find_position(
    handle: *const AvidaWeightedIndexHandle,
    position: c_double,
    root_id: c_int,
) -> c_int {
    if root_id < 0 {
        return -1;
    }
    with_wi_ref(handle, -1, |h| h.find_position(position, root_id as usize))
}
