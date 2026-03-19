use crate::common::{boxed_free, boxed_new};
use std::ffi::c_int;

/// Rust-native CodeLabel replacing cCodeLabel.
pub struct CodeLabel {
    nops: Vec<u8>,
    max_length: usize,
}

const DEFAULT_MAX_LENGTH: usize = 20;

impl Default for CodeLabel {
    fn default() -> Self {
        Self::new()
    }
}

impl CodeLabel {
    pub fn new() -> Self {
        CodeLabel {
            nops: Vec::with_capacity(DEFAULT_MAX_LENGTH),
            max_length: DEFAULT_MAX_LENGTH,
        }
    }

    pub fn add_nop(&mut self, nop_num: u8) {
        if self.nops.len() < self.max_length {
            self.nops.push(nop_num);
        }
    }

    pub fn get_size(&self) -> c_int {
        self.nops.len() as c_int
    }

    pub fn get(&self, index: c_int) -> u8 {
        self.nops[index as usize]
    }

    pub fn clear(&mut self) {
        self.nops.clear();
    }

    pub fn rotate(&mut self, rot: c_int, base: c_int) {
        for nop in self.nops.iter_mut() {
            let mut val = *nop as c_int + rot;
            if val >= base {
                val -= base;
            }
            *nop = val as u8;
        }
    }

    pub fn find_sublabel(&self, sub: &CodeLabel) -> c_int {
        if sub.nops.is_empty() || sub.nops.len() > self.nops.len() {
            return -1;
        }
        for i in 0..=(self.nops.len() - sub.nops.len()) {
            if self.nops[i..i + sub.nops.len()] == sub.nops[..] {
                return i as c_int;
            }
        }
        -1
    }

    pub fn as_int(&self, base: c_int) -> c_int {
        let mut value: c_int = 0;
        for &nop in &self.nops {
            value = value * base + nop as c_int;
        }
        value
    }
}

impl PartialEq for CodeLabel {
    fn eq(&self, other: &Self) -> bool {
        self.nops == other.nops
    }
}

impl Clone for CodeLabel {
    fn clone(&self) -> Self {
        CodeLabel {
            nops: self.nops.clone(),
            max_length: self.max_length,
        }
    }
}

// --- Handle helpers (same pattern as TimeSeriesRecorder) ---

fn with_label_ref<T>(handle: *const CodeLabel, default: T, f: impl FnOnce(&CodeLabel) -> T) -> T {
    if handle.is_null() {
        return default;
    }
    // SAFETY: pointer was checked for null and is only read.
    let h = unsafe { &*handle };
    f(h)
}

fn with_label_mut(handle: *mut CodeLabel, f: impl FnOnce(&mut CodeLabel)) {
    if handle.is_null() {
        return;
    }
    // SAFETY: pointer was checked for null and is only mutably borrowed for this call.
    let h = unsafe { &mut *handle };
    f(h);
}

// --- FFI ---

#[no_mangle]
pub extern "C" fn avd_code_label_new() -> *mut CodeLabel {
    boxed_new(CodeLabel::new())
}

#[no_mangle]
pub extern "C" fn avd_code_label_free(label: *mut CodeLabel) {
    boxed_free(label);
}

#[no_mangle]
pub extern "C" fn avd_code_label_clone(label: *const CodeLabel) -> *mut CodeLabel {
    with_label_ref(label, std::ptr::null_mut(), |l| boxed_new(l.clone()))
}

#[no_mangle]
pub extern "C" fn avd_code_label_add_nop(label: *mut CodeLabel, nop_num: c_int) {
    with_label_mut(label, |l| l.add_nop(nop_num as u8));
}

#[no_mangle]
pub extern "C" fn avd_code_label_get_size(label: *const CodeLabel) -> c_int {
    with_label_ref(label, 0, |l| l.get_size())
}

#[no_mangle]
pub extern "C" fn avd_code_label_get(label: *const CodeLabel, index: c_int) -> c_int {
    with_label_ref(label, 0, |l| l.get(index) as c_int)
}

#[no_mangle]
pub extern "C" fn avd_code_label_clear(label: *mut CodeLabel) {
    with_label_mut(label, |l| l.clear());
}

#[no_mangle]
pub extern "C" fn avd_code_label_rotate(label: *mut CodeLabel, rot: c_int, base: c_int) {
    with_label_mut(label, |l| l.rotate(rot, base));
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_code_label_eq(a: *const CodeLabel, b: *const CodeLabel) -> c_int {
    if a.is_null() || b.is_null() {
        return (a.is_null() && b.is_null()) as c_int;
    }
    // SAFETY: both checked for null above
    unsafe { ((*a) == (*b)) as c_int }
}

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
pub extern "C" fn avd_code_label_find_sublabel(
    label: *const CodeLabel,
    sub: *const CodeLabel,
) -> c_int {
    if label.is_null() || sub.is_null() {
        return -1;
    }
    // SAFETY: both checked for null above
    unsafe { (*label).find_sublabel(&*sub) }
}

#[no_mangle]
pub extern "C" fn avd_code_label_as_int(label: *const CodeLabel, base: c_int) -> c_int {
    with_label_ref(label, 0, |l| l.as_int(base))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn label_basic() {
        let mut l = CodeLabel::new();
        assert_eq!(l.get_size(), 0);
        l.add_nop(0);
        l.add_nop(1);
        l.add_nop(2);
        assert_eq!(l.get_size(), 3);
        assert_eq!(l.get(0), 0);
        assert_eq!(l.get(1), 1);
        assert_eq!(l.get(2), 2);
    }

    #[test]
    fn label_rotate() {
        let mut l = CodeLabel::new();
        l.add_nop(0);
        l.add_nop(1);
        l.add_nop(2);
        l.rotate(1, 3);
        assert_eq!(l.get(0), 1);
        assert_eq!(l.get(1), 2);
        assert_eq!(l.get(2), 0);
    }

    #[test]
    fn label_equality() {
        let mut a = CodeLabel::new();
        let mut b = CodeLabel::new();
        a.add_nop(1);
        a.add_nop(2);
        b.add_nop(1);
        b.add_nop(2);
        assert!(a == b);
        b.add_nop(3);
        assert!(a != b);
    }

    #[test]
    fn label_sublabel() {
        let mut main_label = CodeLabel::new();
        main_label.add_nop(0);
        main_label.add_nop(1);
        main_label.add_nop(2);
        main_label.add_nop(3);

        let mut sub = CodeLabel::new();
        sub.add_nop(1);
        sub.add_nop(2);
        assert_eq!(main_label.find_sublabel(&sub), 1);
    }

    #[test]
    fn label_as_int() {
        let mut l = CodeLabel::new();
        l.add_nop(1);
        l.add_nop(0);
        l.add_nop(1);
        // base-3: 1*9 + 0*3 + 1 = 10
        assert_eq!(l.as_int(3), 10);
    }

    #[test]
    fn label_ffi_roundtrip() {
        let ptr = avd_code_label_new();
        assert!(!ptr.is_null());
        avd_code_label_add_nop(ptr, 2);
        avd_code_label_add_nop(ptr, 1);
        assert_eq!(avd_code_label_get_size(ptr), 2);
        assert_eq!(avd_code_label_get(ptr, 0), 2);

        let clone = avd_code_label_clone(ptr);
        assert_eq!(avd_code_label_eq(ptr, clone), 1);

        avd_code_label_free(clone);
        avd_code_label_free(ptr);
    }

    #[test]
    fn label_max_length() {
        let mut l = CodeLabel::new();
        for i in 0..25 {
            l.add_nop(i);
        }
        assert_eq!(l.get_size(), DEFAULT_MAX_LENGTH as c_int);
    }
}
