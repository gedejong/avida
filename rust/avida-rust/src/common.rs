use crate::{
    AvidaDoubleSumHandle, AvidaHistogramHandle, AvidaOrderedWeightedIndexHandle,
    AvidaRawBitArrayHandle, AvidaRunningAverageHandle, AvidaRunningStatsHandle,
    AvidaWeightedIndexHandle,
};

pub(crate) fn with_ref<T>(
    handle: *const AvidaRunningStatsHandle,
    default: T,
    f: impl FnOnce(&AvidaRunningStatsHandle) -> T,
) -> T {
    if handle.is_null() {
        return default;
    }
    // SAFETY: pointer was checked for null and is only read.
    let h = unsafe { &*handle };
    f(h)
}

pub(crate) fn with_mut(
    handle: *mut AvidaRunningStatsHandle,
    f: impl FnOnce(&mut AvidaRunningStatsHandle),
) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer was checked for null and is only mutably borrowed for this call.
    let h = unsafe { &mut *handle };
    f(h);
}

pub(crate) fn with_ra_ref<T>(
    handle: *const AvidaRunningAverageHandle,
    default: T,
    f: impl FnOnce(&AvidaRunningAverageHandle) -> T,
) -> T {
    if handle.is_null() {
        return default;
    }
    // SAFETY: pointer was checked for null and is only read.
    let h = unsafe { &*handle };
    f(h)
}

pub(crate) fn with_ra_mut(
    handle: *mut AvidaRunningAverageHandle,
    f: impl FnOnce(&mut AvidaRunningAverageHandle),
) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer was checked for null and is only mutably borrowed for this call.
    let h = unsafe { &mut *handle };
    f(h);
}

pub(crate) fn with_ds_ref<T>(
    handle: *const AvidaDoubleSumHandle,
    default: T,
    f: impl FnOnce(&AvidaDoubleSumHandle) -> T,
) -> T {
    if handle.is_null() {
        return default;
    }
    // SAFETY: pointer was checked for null and is only read.
    let h = unsafe { &*handle };
    f(h)
}

pub(crate) fn with_ds_mut(
    handle: *mut AvidaDoubleSumHandle,
    f: impl FnOnce(&mut AvidaDoubleSumHandle),
) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer was checked for null and is only mutably borrowed for this call.
    let h = unsafe { &mut *handle };
    f(h);
}

pub(crate) fn with_wi_ref<T>(
    handle: *const AvidaWeightedIndexHandle,
    default: T,
    f: impl FnOnce(&AvidaWeightedIndexHandle) -> T,
) -> T {
    if handle.is_null() {
        return default;
    }
    // SAFETY: pointer was checked for null and is only read.
    let h = unsafe { &*handle };
    f(h)
}

pub(crate) fn with_wi_mut(
    handle: *mut AvidaWeightedIndexHandle,
    f: impl FnOnce(&mut AvidaWeightedIndexHandle),
) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer was checked for null and is only mutably borrowed for this call.
    let h = unsafe { &mut *handle };
    f(h);
}

pub(crate) fn with_owi_ref<T>(
    handle: *const AvidaOrderedWeightedIndexHandle,
    default: T,
    f: impl FnOnce(&AvidaOrderedWeightedIndexHandle) -> T,
) -> T {
    if handle.is_null() {
        return default;
    }
    // SAFETY: pointer was checked for null and is only read.
    let h = unsafe { &*handle };
    f(h)
}

pub(crate) fn with_owi_mut(
    handle: *mut AvidaOrderedWeightedIndexHandle,
    f: impl FnOnce(&mut AvidaOrderedWeightedIndexHandle),
) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer was checked for null and is only mutably borrowed for this call.
    let h = unsafe { &mut *handle };
    f(h);
}

pub(crate) fn with_hist_ref<T>(
    handle: *const AvidaHistogramHandle,
    default: T,
    f: impl FnOnce(&AvidaHistogramHandle) -> T,
) -> T {
    if handle.is_null() {
        return default;
    }
    // SAFETY: pointer was checked for null and is only read.
    let h = unsafe { &*handle };
    f(h)
}

pub(crate) fn with_hist_mut(
    handle: *mut AvidaHistogramHandle,
    f: impl FnOnce(&mut AvidaHistogramHandle),
) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer was checked for null and is only mutably borrowed for this call.
    let h = unsafe { &mut *handle };
    f(h);
}

pub(crate) fn with_rba_ref<T>(
    handle: *const AvidaRawBitArrayHandle,
    default: T,
    f: impl FnOnce(&AvidaRawBitArrayHandle) -> T,
) -> T {
    if handle.is_null() {
        return default;
    }
    // SAFETY: pointer was checked for null and is only read.
    let h = unsafe { &*handle };
    f(h)
}

pub(crate) fn with_rba_mut(
    handle: *mut AvidaRawBitArrayHandle,
    f: impl FnOnce(&mut AvidaRawBitArrayHandle),
) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer was checked for null and is only mutably borrowed for this call.
    let h = unsafe { &mut *handle };
    f(h);
}
