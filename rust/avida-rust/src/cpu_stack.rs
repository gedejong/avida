use std::ffi::c_int;

const STACK_SIZE: usize = 10;

/// Rust-native CPU stack replacing cCPUStack.
/// Fixed-size circular stack of 10 integers.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CpuStack {
    stack: [c_int; STACK_SIZE],
    stack_pointer: u8,
}

impl Default for CpuStack {
    fn default() -> Self {
        CpuStack {
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
        }
    }
}

impl CpuStack {
    pub fn push(&mut self, value: c_int) {
        if self.stack_pointer == 0 {
            self.stack_pointer = (STACK_SIZE - 1) as u8;
        } else {
            self.stack_pointer -= 1;
        }
        self.stack[self.stack_pointer as usize] = value;
    }

    pub fn pop(&mut self) -> c_int {
        let value = self.stack[self.stack_pointer as usize];
        self.stack[self.stack_pointer as usize] = 0;
        self.stack_pointer += 1;
        if self.stack_pointer as usize == STACK_SIZE {
            self.stack_pointer = 0;
        }
        value
    }

    pub fn peek(&self) -> c_int {
        self.stack[self.stack_pointer as usize]
    }

    pub fn get(&self, depth: c_int) -> c_int {
        let mut array_pos = depth as usize + self.stack_pointer as usize;
        if array_pos >= STACK_SIZE {
            array_pos -= STACK_SIZE;
        }
        self.stack[array_pos]
    }

    pub fn top(&self) -> c_int {
        self.stack[self.stack_pointer as usize]
    }

    pub fn clear(&mut self) {
        self.stack = [0; STACK_SIZE];
        self.stack_pointer = 0;
    }

    pub fn flip(&mut self) {
        let mut temp = [0i32; STACK_SIZE];
        for item in temp.iter_mut() {
            *item = self.pop();
        }
        for &item in temp.iter() {
            self.push(item);
        }
    }
}

// --- FFI interface ---

#[no_mangle]
pub extern "C" fn avd_cpu_stack_default() -> CpuStack {
    CpuStack::default()
}

#[no_mangle]
pub extern "C" fn avd_cpu_stack_push(s: &mut CpuStack, value: c_int) {
    s.push(value);
}

#[no_mangle]
pub extern "C" fn avd_cpu_stack_pop(s: &mut CpuStack) -> c_int {
    s.pop()
}

#[no_mangle]
pub extern "C" fn avd_cpu_stack_peek(s: &CpuStack) -> c_int {
    s.peek()
}

#[no_mangle]
pub extern "C" fn avd_cpu_stack_get(s: &CpuStack, depth: c_int) -> c_int {
    s.get(depth)
}

#[no_mangle]
pub extern "C" fn avd_cpu_stack_top(s: &CpuStack) -> c_int {
    s.top()
}

#[no_mangle]
pub extern "C" fn avd_cpu_stack_clear(s: &mut CpuStack) {
    s.clear();
}

#[no_mangle]
pub extern "C" fn avd_cpu_stack_flip(s: &mut CpuStack) {
    s.flip();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stack_default_is_zero() {
        let s = CpuStack::default();
        assert_eq!(s.peek(), 0);
        assert_eq!(s.stack_pointer, 0);
        for i in 0..STACK_SIZE {
            assert_eq!(s.stack[i], 0);
        }
    }

    #[test]
    fn stack_push_pop_basic() {
        let mut s = CpuStack::default();
        s.push(42);
        assert_eq!(s.peek(), 42);
        assert_eq!(s.pop(), 42);
        assert_eq!(s.peek(), 0);
    }

    #[test]
    fn stack_push_pop_multiple() {
        let mut s = CpuStack::default();
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), 3);
        assert_eq!(s.pop(), 2);
        assert_eq!(s.pop(), 1);
    }

    #[test]
    fn stack_wraps_around() {
        let mut s = CpuStack::default();
        // Push more than STACK_SIZE elements to test wrap
        for i in 0..12 {
            s.push(i);
        }
        // Should still work, wrapping around
        assert_eq!(s.peek(), 11);
    }

    #[test]
    fn stack_get_depth() {
        let mut s = CpuStack::default();
        s.push(10);
        s.push(20);
        s.push(30);
        assert_eq!(s.get(0), 30); // top
        assert_eq!(s.get(1), 20);
        assert_eq!(s.get(2), 10);
    }

    #[test]
    fn stack_flip() {
        // Fill the entire stack to avoid zero-padding artifacts
        let mut s = CpuStack::default();
        for i in 0..10 {
            s.push(i + 1);
        }
        // Stack is [10,9,8,7,6,5,4,3,2,1] (10 on top)
        assert_eq!(s.peek(), 10);
        s.flip();
        // After flip: reversed → 1 on top
        assert_eq!(s.pop(), 1);
        assert_eq!(s.pop(), 2);
        assert_eq!(s.pop(), 3);
    }

    #[test]
    fn stack_clear() {
        let mut s = CpuStack::default();
        s.push(1);
        s.push(2);
        s.clear();
        assert_eq!(s.peek(), 0);
        assert_eq!(s.stack_pointer, 0);
    }

    #[test]
    fn stack_ffi_roundtrip() {
        let mut s = avd_cpu_stack_default();
        avd_cpu_stack_push(&mut s, 77);
        assert_eq!(avd_cpu_stack_peek(&s), 77);
        assert_eq!(avd_cpu_stack_pop(&mut s), 77);
        assert_eq!(avd_cpu_stack_top(&s), 0);
    }
}
