//! CPU instruction handlers ported to Rust.
//!
//! Instruction evaluators that operate on CpuRegisters (pure computation) and
//! instruction handlers that call back into C++ hardware FFI (stack/head ops).
//! The C++ caller handles `FindModifiedRegister` and passes resolved operands.
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::undocumented_unsafe_blocks)]

use std::ffi::{c_int, c_void};

use crate::cpu_registers::CpuRegisters;

#[cfg(test)]
const REG_AX: c_int = 0;
#[cfg(test)]
const REG_BX: c_int = 1;

// FFI declarations for hardware accessors (defined in cHardwareFFI.cc).
// Circular link dependency resolved by listing avida-rust both before and after
// avida-core in the link order (CMakeLists.txt).
unsafe extern "C" {
    fn avd_hw_stack_push(hw: *mut c_void, value: c_int);
    fn avd_hw_stack_pop(hw: *mut c_void) -> c_int;
    fn avd_hw_switch_stack(hw: *mut c_void);
    fn avd_hw_stack_flip(hw: *mut c_void);
    fn avd_hw_get_head_position(hw: *mut c_void, head_id: c_int) -> c_int;
    fn avd_hw_set_head_position(hw: *mut c_void, head_id: c_int, pos: c_int);
    fn avd_hw_advance_ip(hw: *mut c_void);
    fn avd_hw_get_memory_size(hw: *mut c_void) -> c_int;
    #[allow(dead_code)]
    fn avd_hw_get_memory_inst(hw: *mut c_void, idx: c_int) -> c_int;
    #[allow(dead_code)]
    fn avd_hw_set_memory_inst(hw: *mut c_void, idx: c_int, inst_id: c_int);
    fn avd_hw_advance_head(hw: *mut c_void, head_id: c_int);
    fn avd_hw_set_cur_head(hw: *mut c_void, head_id: c_int);
    fn avd_hw_get_thread_id(hw: *mut c_void) -> c_int;
    fn avd_hw_get_cycle_counter(hw: *mut c_void) -> c_int;
    #[allow(dead_code)]
    fn avd_hw_get_register(hw: *mut c_void, reg_id: c_int) -> c_int;
    fn avd_hw_set_register(hw: *mut c_void, reg_id: c_int, value: c_int);
    fn avd_hw_get_inst_at_raw_pos(hw: *mut c_void, raw_pos: c_int) -> c_int;
    fn avd_hw_read_label(hw: *mut c_void);
    fn avd_hw_get_label_as_int(hw: *mut c_void, mode: c_int) -> c_int;
    fn avd_hw_if_label_match(hw: *mut c_void) -> c_int;
    fn avd_hw_if_label_direct_match(hw: *mut c_void) -> c_int;
    fn avd_hw_search_label(hw: *mut c_void, direction: c_int) -> c_int;
    fn avd_hw_get_label_size(hw: *mut c_void) -> c_int;
    // cAvidaContext RNG
    fn avd_ctx_random_p(ctx: *mut c_void, prob: f64) -> c_int;
    // Organism accessors (organism pointer from avd_hw_get_organism)
    fn avd_hw_get_organism(hw: *mut c_void) -> *mut c_void;
    fn avd_org_get_id(org: *mut c_void) -> c_int;
    fn avd_org_is_germline(org: *mut c_void) -> c_int;
    fn avd_org_is_fertile(org: *mut c_void) -> c_int;
    fn avd_org_get_stored_energy(org: *mut c_void) -> f64;
    fn avd_org_get_cell_id(org: *mut c_void) -> c_int;
    #[allow(dead_code)]
    fn avd_org_get_time_used(org: *mut c_void) -> c_int;
    fn avd_org_get_num_divides(org: *mut c_void) -> c_int;
    fn avd_org_get_reputation(org: *mut c_void) -> c_int;
    fn avd_org_get_faced_dir(org: *mut c_void) -> c_int;
    fn avd_org_get_northerly(org: *mut c_void) -> c_int;
    fn avd_org_get_easterly(org: *mut c_void) -> c_int;
    fn avd_org_get_cell_data_org(org: *mut c_void) -> c_int;
    fn avd_org_get_neighbor_cell_contents(org: *mut c_void) -> c_int;
    fn avd_org_get_faced_cell_data_org_id(org: *mut c_void) -> c_int;
    fn avd_org_get_number_strings_on_hand(org: *mut c_void, type_: c_int) -> c_int;
    fn avd_org_get_mating_type(org: *mut c_void) -> c_int;
    fn avd_org_get_energy_in_buffer(org: *mut c_void) -> f64;
    fn avd_org_has_opinion(org: *mut c_void) -> c_int;
    fn avd_org_get_cell_y_position(org: *mut c_void) -> c_int;
    #[allow(dead_code)]
    fn avd_org_get_kaboom_executed(org: *mut c_void) -> c_int;
    fn avd_org_get_neighbor(org: *mut c_void) -> *mut c_void;
    fn avd_org_is_neighbor_cell_occupied(org: *mut c_void) -> c_int;
    fn avd_org_is_dead(org: *mut c_void) -> c_int;
    fn avd_org_get_vitality(org: *mut c_void) -> f64;
    fn avd_org_get_discrete_energy_level(org: *mut c_void) -> c_int;
    fn avd_org_get_cell_position_x(org: *mut c_void) -> c_int;
    // Phase 2: mutable organism state
    fn avd_org_set_fertile(org: *mut c_void, value: c_int);
    fn avd_org_set_pheromone(org: *mut c_void, value: c_int);
    fn avd_org_set_is_energy_requestor(org: *mut c_void);
    fn avd_org_increase_num_energy_requests(org: *mut c_void);
    fn avd_org_set_has_open_energy_request(org: *mut c_void);
    fn avd_org_clear_has_open_energy_request(org: *mut c_void);
    fn avd_org_get_frac_energy_donating(org: *mut c_void) -> f64;
    fn avd_org_set_frac_energy_donating(org: *mut c_void, frac: f64);
    fn avd_org_get_opinion_only(org: *mut c_void) -> c_int;
    fn avd_org_join_germline(org: *mut c_void);
    fn avd_org_exit_germline(org: *mut c_void);
    fn avd_org_toggle_pheromone(org: *mut c_void);
    fn avd_org_repair_point_mut_on(org: *mut c_void);
    fn avd_org_get_rbin(org: *mut c_void, index: c_int) -> f64;
}

// Head IDs matching nHardware::tHeads
const HEAD_IP: c_int = 0;
#[allow(dead_code)]
const HEAD_READ: c_int = 1;
#[allow(dead_code)]
const HEAD_WRITE: c_int = 2;
const HEAD_FLOW: c_int = 3;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

#[inline]
fn valid_reg(idx: c_int) -> bool {
    idx >= 0 && (idx as usize) < 3
}

/// Read a register value safely.
///
/// # Safety
/// `regs` must be a valid pointer.
#[inline]
unsafe fn get_reg(regs: *const CpuRegisters, idx: c_int) -> c_int {
    if regs.is_null() || !valid_reg(idx) {
        return 0;
    }
    // SAFETY: caller guarantees valid pointer, idx is bounds-checked.
    unsafe { (*regs).reg[idx as usize] }
}

/// Write a register value safely.
///
/// # Safety
/// `regs` must be a valid pointer.
#[inline]
unsafe fn set_reg(regs: *mut CpuRegisters, idx: c_int, value: c_int) {
    if regs.is_null() || !valid_reg(idx) {
        return;
    }
    // SAFETY: caller guarantees valid pointer, idx is bounds-checked.
    unsafe { (*regs).reg[idx as usize] = value };
}

// ---------------------------------------------------------------------------
// Batch 1: Stack instruction handlers
// ---------------------------------------------------------------------------

/// Inst_Push: push register[reg_id] onto the active stack.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer, `regs` a valid CpuRegisters pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_push(
    hw: *mut c_void,
    regs: *const CpuRegisters,
    reg_id: c_int,
) {
    // SAFETY: caller guarantees valid pointers; get_reg null-checks internally.
    let value = unsafe { get_reg(regs, reg_id) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_stack_push(hw, value) };
}

/// Inst_Pop: pop from active stack into register[reg_id].
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer, `regs` a valid CpuRegisters pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_pop(hw: *mut c_void, regs: *mut CpuRegisters, reg_id: c_int) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let value = unsafe { avd_hw_stack_pop(hw) };
    // SAFETY: set_reg null-checks internally.
    unsafe { set_reg(regs, reg_id, value) };
}

/// Inst_SwitchStack: toggle active stack (local <-> global).
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_switch_stack(hw: *mut c_void) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_switch_stack(hw) };
}

/// Inst_FlipStack: flip the active stack.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_flip_stack(hw: *mut c_void) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_stack_flip(hw) };
}

/// Inst_HeadPop: pop value from active stack, set head[head_id] to that position.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_head_pop(hw: *mut c_void, head_id: c_int) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let value = unsafe { avd_hw_stack_pop(hw) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_head_position(hw, head_id, value) };
}

// ---------------------------------------------------------------------------
// Batch 2: Flow control instruction handlers
// ---------------------------------------------------------------------------

/// Inst_MovHead: set head[head_id] to head[FLOW] position.
/// Returns 1 if head_id == IP (caller should disable auto-advance), 0 otherwise.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_mov_head(hw: *mut c_void, head_id: c_int) -> c_int {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let flow_pos = unsafe { avd_hw_get_head_position(hw, HEAD_FLOW) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_head_position(hw, head_id, flow_pos) };
    if head_id == HEAD_IP {
        1
    } else {
        0
    }
}

/// Inst_JmpHead: jump head[head_id] by register[reg_id] positions.
/// Returns the new head position.
///
/// # Safety
/// `hw` and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_jmp_head(
    hw: *mut c_void,
    regs: *const CpuRegisters,
    head_id: c_int,
    reg_id: c_int,
) -> c_int {
    // SAFETY: caller guarantees valid pointers.
    let jump = unsafe { get_reg(regs, reg_id) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let cur_pos = unsafe { avd_hw_get_head_position(hw, head_id) };
    let new_pos = cur_pos + jump;
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_head_position(hw, head_id, new_pos) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_get_head_position(hw, head_id) }
}

/// Inst_GetHead: store head[head_id] position into register[reg_id].
///
/// # Safety
/// `hw` and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_head(
    hw: *mut c_void,
    regs: *mut CpuRegisters,
    head_id: c_int,
    reg_id: c_int,
) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let pos = unsafe { avd_hw_get_head_position(hw, head_id) };
    // SAFETY: set_reg null-checks internally.
    unsafe { set_reg(regs, reg_id, pos) };
}

/// Inst_SetFlow: set FLOW head to register[reg_id] position.
///
/// # Safety
/// `hw` and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_set_flow(
    hw: *mut c_void,
    regs: *const CpuRegisters,
    reg_id: c_int,
) {
    // SAFETY: caller guarantees valid pointers.
    let pos = unsafe { get_reg(regs, reg_id) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_head_position(hw, HEAD_FLOW, pos) };
}

// ---------------------------------------------------------------------------
// Batch: Simple instruction handlers using hardware FFI
// ---------------------------------------------------------------------------

/// Inst_Return: pop stack, set IP to that position.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_return(hw: *mut c_void) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let pos = unsafe { avd_hw_stack_pop(hw) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_head_position(hw, HEAD_IP, pos) };
}

/// Inst_Skip: advance IP by 1 (skip next instruction).
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_skip(hw: *mut c_void) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_advance_ip(hw) };
}

/// Inst_AdvanceHead: advance head[head_id] by 1 position.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_advance_head(hw: *mut c_void, head_id: c_int) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_advance_head(hw, head_id) };
}

/// Inst_SetHead: set cur_head to head_id.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_set_head(hw: *mut c_void, head_id: c_int) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_cur_head(hw, head_id) };
}

/// Inst_MemSize: store memory size into register[reg_id].
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_mem_size(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let size = unsafe { avd_hw_get_memory_size(hw) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_register(hw, reg_id, size) };
}

/// Inst_ThreadID: store current thread ID into register[reg_id].
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_thread_id(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let id = unsafe { avd_hw_get_thread_id(hw) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_register(hw, reg_id, id) };
}

/// Inst_GetCycles: store cycle counter into register[reg_id].
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_cycles(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let cycles = unsafe { avd_hw_get_cycle_counter(hw) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_register(hw, reg_id, cycles) };
}

// ---------------------------------------------------------------------------
// RNG-based instruction handlers
// ---------------------------------------------------------------------------

/// Inst_IfP*: skip next instruction with probability (1 - exec_prob).
/// Returns 1 if should skip (random trial succeeds at complement prob), 0 otherwise.
///
/// # Safety
/// `ctx` must be a valid cAvidaContext pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_p(ctx: *mut c_void, skip_prob: f64) -> c_int {
    // SAFETY: ctx is a valid cAvidaContext pointer per caller contract.
    unsafe { avd_ctx_random_p(ctx, skip_prob) }
}

// ---------------------------------------------------------------------------
// Organism-reading instruction handlers
// ---------------------------------------------------------------------------

/// Inst_IfGerm: skip if organism is NOT germline.
/// Returns 1 if should skip, 0 otherwise.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_germ(hw: *mut c_void) -> c_int {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let org = unsafe { avd_hw_get_organism(hw) };
    // SAFETY: org pointer from avd_hw_get_organism is valid if hw is valid.
    let is_germ = unsafe { avd_org_is_germline(org) };
    if is_germ == 0 {
        1
    } else {
        0
    } // skip if NOT germline
}

/// Inst_IfSoma: skip if organism IS germline.
/// Returns 1 if should skip, 0 otherwise.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_soma(hw: *mut c_void) -> c_int {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let org = unsafe { avd_hw_get_organism(hw) };
    // SAFETY: org pointer from avd_hw_get_organism is valid if hw is valid.
    let is_germ = unsafe { avd_org_is_germline(org) };
    if is_germ != 0 {
        1
    } else {
        0
    } // skip if IS germline
}

/// Inst_GetID: store organism ID into register[reg_id].
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_id(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let org = unsafe { avd_hw_get_organism(hw) };
    // SAFETY: org pointer from avd_hw_get_organism is valid if hw is valid.
    let id = unsafe { avd_org_get_id(org) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_register(hw, reg_id, id) };
}

/// Inst_IfFertile: skip if NOT fertile.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_fertile(hw: *mut c_void) -> c_int {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let org = unsafe { avd_hw_get_organism(hw) };
    // SAFETY: org pointer valid.
    let fertile = unsafe { avd_org_is_fertile(org) };
    if fertile == 0 {
        1
    } else {
        0
    }
}

/// Inst_IfNotFertile: skip if IS fertile.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_not_fertile(hw: *mut c_void) -> c_int {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let org = unsafe { avd_hw_get_organism(hw) };
    // SAFETY: org pointer valid.
    let fertile = unsafe { avd_org_is_fertile(org) };
    if fertile != 0 {
        1
    } else {
        0
    }
}

/// Helper: read an int property from the organism and store in a register.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[inline]
unsafe fn org_int_to_reg(
    hw: *mut c_void,
    reg_id: c_int,
    getter: unsafe extern "C" fn(*mut c_void) -> c_int,
) {
    // SAFETY: hw valid per caller contract.
    let org = unsafe { avd_hw_get_organism(hw) };
    // SAFETY: org valid if hw is valid.
    let val = unsafe { getter(org) };
    // SAFETY: hw valid per caller contract.
    unsafe { avd_hw_set_register(hw, reg_id, val) };
}

/// Helper: read an int property from organism and skip if condition met.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[inline]
unsafe fn org_int_cond(
    hw: *mut c_void,
    getter: unsafe extern "C" fn(*mut c_void) -> c_int,
    skip_when: impl Fn(c_int) -> bool,
) -> c_int {
    // SAFETY: hw valid per caller contract.
    let org = unsafe { avd_hw_get_organism(hw) };
    // SAFETY: org valid if hw is valid.
    let val = unsafe { getter(org) };
    if skip_when(val) {
        1
    } else {
        0
    }
}

// --- Batch B1: Organism property reads into registers ---

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_cell_id(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_to_reg(hw, reg_id, avd_org_get_cell_id) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_energy_level(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw valid per caller contract.
    let org = unsafe { avd_hw_get_organism(hw) };
    // SAFETY: org valid.
    let energy = unsafe { avd_org_get_stored_energy(org) };
    // SAFETY: hw valid.
    unsafe { avd_hw_set_register(hw, reg_id, energy.floor() as c_int) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_reputation(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_to_reg(hw, reg_id, avd_org_get_reputation) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_direction_off_north(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_to_reg(hw, reg_id, avd_org_get_faced_dir) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_northerly(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_to_reg(hw, reg_id, avd_org_get_northerly) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_easterly(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_to_reg(hw, reg_id, avd_org_get_easterly) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_read_cell_data(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_to_reg(hw, reg_id, avd_org_get_cell_data_org) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_read_faced_cell_data_org_id(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_to_reg(hw, reg_id, avd_org_get_faced_cell_data_org_id) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_raw_materials(
    hw: *mut c_void,
    reg_id: c_int,
    material_type: c_int,
) {
    // SAFETY: hw valid per caller contract.
    let org = unsafe { avd_hw_get_organism(hw) };
    // SAFETY: org valid.
    let count = unsafe { avd_org_get_number_strings_on_hand(org, material_type) };
    // SAFETY: hw valid.
    unsafe { avd_hw_set_register(hw, reg_id, count) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_cell_position_y(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_to_reg(hw, reg_id, avd_org_get_cell_y_position) };
}

// --- Batch B2: Organism conditionals ---

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_opinion_set(hw: *mut c_void) -> c_int {
    // skip if opinion IS set (execute next only if NOT set)
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_cond(hw, avd_org_has_opinion, |v| v == 0) }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_opinion_not_set(hw: *mut c_void) -> c_int {
    // skip if opinion is NOT set (execute next only if IS set)
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_cond(hw, avd_org_has_opinion, |v| v != 0) }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_event_in_cell(hw: *mut c_void) -> c_int {
    // skip if cell_data <= 0
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_cond(hw, avd_org_get_cell_data_org, |v| v <= 0) }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_facing_event_cell(hw: *mut c_void) -> c_int {
    // skip if neighbor cell contents <= 0
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_cond(hw, avd_org_get_neighbor_cell_contents, |v| v <= 0) }
}

const MATING_TYPE_JUVENILE: c_int = -1;
const MATING_TYPE_FEMALE: c_int = 0;
const MATING_TYPE_MALE: c_int = 1;

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_mating_type_female(hw: *mut c_void) -> c_int {
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_cond(hw, avd_org_get_mating_type, |v| v != MATING_TYPE_FEMALE) }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_mating_type_male(hw: *mut c_void) -> c_int {
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_cond(hw, avd_org_get_mating_type, |v| v != MATING_TYPE_MALE) }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_mating_type_juvenile(hw: *mut c_void) -> c_int {
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_cond(hw, avd_org_get_mating_type, |v| v != MATING_TYPE_JUVENILE) }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_energy_not_in_buffer(hw: *mut c_void) -> c_int {
    // skip if energy in buffer > 0 (execute next only if no energy in buffer)
    // SAFETY: hw valid per caller contract.
    let org = unsafe { avd_hw_get_organism(hw) };
    // SAFETY: org valid.
    let energy = unsafe { avd_org_get_energy_in_buffer(org) };
    if energy > 0.0 {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_nop_pre(hw: *mut c_void) -> c_int {
    // skip if num_divides > 0 (execute only before first divide)
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_cond(hw, avd_org_get_num_divides, |v| v > 0) }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_nop_post(hw: *mut c_void) -> c_int {
    // skip if num_divides == 0 (execute only after first divide)
    // SAFETY: hw valid per caller contract.
    unsafe { org_int_cond(hw, avd_org_get_num_divides, |v| v == 0) }
}

// --- Batch B3: Neighbor reads ---

// Energy level constants matching cPhenotype::energy_levels
const ENERGY_LEVEL_LOW: c_int = 0;
#[allow(dead_code)]
const ENERGY_LEVEL_MEDIUM: c_int = 1;
const ENERGY_LEVEL_HIGH: c_int = 2;

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_faced_org_id(hw: *mut c_void, reg_id: c_int) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    if unsafe { avd_org_is_neighbor_cell_occupied(org) } == 0 {
        return 0;
    }
    let neighbor = unsafe { avd_org_get_neighbor(org) };
    if unsafe { avd_org_is_dead(neighbor) } != 0 {
        return 0;
    }
    let id = unsafe { avd_org_get_id(neighbor) };
    unsafe { avd_hw_set_register(hw, reg_id, id) };
    1 // success
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_faced_vitality_diff(
    hw: *mut c_void,
    reg_id: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    if unsafe { avd_org_is_neighbor_cell_occupied(org) } == 0 {
        return 0;
    }
    let neighbor = unsafe { avd_org_get_neighbor(org) };
    if unsafe { avd_org_is_dead(neighbor) } != 0 {
        return 0;
    }
    let my_vit = unsafe { avd_org_get_vitality(org) };
    let neigh_vit = unsafe { avd_org_get_vitality(neighbor) };
    let diff = if my_vit != 0.0 {
        ((neigh_vit - my_vit) / my_vit * 100.0 + 0.5) as c_int
    } else {
        0
    };
    unsafe { avd_hw_set_register(hw, reg_id, diff) };
    1
}

// TODO: avd_cpu_inst_read_faced_cell_data needs GetFacedCellData() FFI — skipped

// --- Batch B3/B4: Energy conditionals ---
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_energy_low(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let level = unsafe { avd_org_get_discrete_energy_level(org) };
    if level != ENERGY_LEVEL_LOW {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_energy_not_low(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let level = unsafe { avd_org_get_discrete_energy_level(org) };
    if level == ENERGY_LEVEL_LOW {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_energy_high(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let level = unsafe { avd_org_get_discrete_energy_level(org) };
    if level != ENERGY_LEVEL_HIGH {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_energy_not_high(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let level = unsafe { avd_org_get_discrete_energy_level(org) };
    if level == ENERGY_LEVEL_HIGH {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_energy_med(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let level = unsafe { avd_org_get_discrete_energy_level(org) };
    if level != ENERGY_LEVEL_MEDIUM {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_energy_level_guarded(
    hw: *mut c_void,
    reg_id: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let cell_id = unsafe { avd_org_get_cell_id(org) };
    if cell_id < 0 {
        return 0;
    } // test CPU guard
    let energy = unsafe { avd_org_get_stored_energy(org) };
    unsafe { avd_hw_set_register(hw, reg_id, energy.floor() as c_int) };
    1
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_cell_position_x(hw: *mut c_void, reg_id: c_int) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let x = unsafe { avd_org_get_cell_position_x(org) };
    if x == -1 {
        return 0;
    } // test CPU guard
    unsafe { avd_hw_set_register(hw, reg_id, x) };
    1
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_cell_position_y_guarded(
    hw: *mut c_void,
    reg_id: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let y = unsafe { avd_org_get_cell_y_position(org) };
    if y == -1 {
        return 0;
    } // test CPU guard
    unsafe { avd_hw_set_register(hw, reg_id, y) };
    1
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_opinion_only(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let val = unsafe { avd_org_get_opinion_only(org) };
    unsafe { avd_hw_set_register(hw, reg_id, val) };
}

// TODO: avd_cpu_inst_number_orgs_in_group needs population-level FFI query — skipped

// ---------------------------------------------------------------------------
// Phase 2: Mutable organism state handlers (Batch C1-C2)
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_sterilize(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_set_fertile(org, 0) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_phero_on(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_set_pheromone(org, 1) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_phero_off(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_set_pheromone(org, 0) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_request_energy_flag_on(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_set_is_energy_requestor(org) };
    unsafe { avd_org_increase_num_energy_requests(org) };
    unsafe { avd_org_set_has_open_energy_request(org) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_request_energy_flag_off(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_clear_has_open_energy_request(org) };
}

/// Inst_IncreaseEnergyDonation: increase donation fraction by config increment.
/// `increment`: ENERGY_SHARING_INCREMENT config value (passed by C++ caller).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_increase_energy_donation(hw: *mut c_void, increment: f64) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let curr = unsafe { avd_org_get_frac_energy_donating(org) };
    let new_val = if curr + increment > 1.0 {
        1.0
    } else {
        curr + increment
    };
    unsafe { avd_org_set_frac_energy_donating(org, new_val) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_decrease_energy_donation(hw: *mut c_void, increment: f64) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let curr = unsafe { avd_org_get_frac_energy_donating(org) };
    let new_val = if curr - increment < 0.0 {
        0.0
    } else {
        curr - increment
    };
    unsafe { avd_org_set_frac_energy_donating(org, new_val) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_join_germline(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_join_germline(org) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_exit_germline(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_exit_germline(org) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_phero_toggle(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_toggle_pheromone(org) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_repair_point_mut_on(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_repair_point_mut_on(org) };
}

/// Inst_GetResStored: read resource bin value into register.
/// `resource_id`: COLLECT_SPECIFIC_RESOURCE config value (from C++ caller).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_res_stored(
    hw: *mut c_void,
    reg_id: c_int,
    resource_id: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let res = unsafe { avd_org_get_rbin(org, resource_id) };
    let res_stored = (res * 100.0 - 0.5) as c_int;
    unsafe { avd_hw_set_register(hw, reg_id, res_stored) };
}

// ---------------------------------------------------------------------------
// Label-based instruction handlers
// ---------------------------------------------------------------------------

// Label mode constants for avd_hw_get_label_as_int
#[allow(dead_code)]
const LABEL_AS_INT: c_int = 0;
#[allow(dead_code)]
const LABEL_AS_GREY: c_int = 1;
#[allow(dead_code)]
const LABEL_AS_DIRECT: c_int = 2;
#[allow(dead_code)]
const LABEL_AS_ADD_POLY: c_int = 3;
#[allow(dead_code)]
const LABEL_AS_FIB: c_int = 4;
#[allow(dead_code)]
const LABEL_AS_POLY_COEFF: c_int = 5;

/// Inst_SetNum: read label, set BX = label.AsInt(NUM_NOPS).
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_set_num(hw: *mut c_void, reg_id: c_int) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_read_label(hw) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let val = unsafe { avd_hw_get_label_as_int(hw, LABEL_AS_INT) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_register(hw, reg_id, val) };
}

/// Inst_Val*: read label, set BX = label.AsInt*(NUM_NOPS).
/// `mode`: 0=Int, 1=Grey, 2=Direct, 3=AddPoly, 4=Fib, 5=PolyCoeff
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_val_label(hw: *mut c_void, reg_id: c_int, mode: c_int) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_read_label(hw) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let val = unsafe { avd_hw_get_label_as_int(hw, mode) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_set_register(hw, reg_id, val) };
}

/// Inst_IfLabel: read label, rotate complement, skip if no match.
/// Returns 1 if IP should advance (skip next), 0 otherwise.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_label(hw: *mut c_void) -> c_int {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_read_label(hw) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_if_label_match(hw) }
}

/// Inst_IfLabelDirect: read label, skip if no direct match (no complement rotation).
/// Returns 1 if IP should advance (skip next), 0 otherwise.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_label_direct(hw: *mut c_void) -> c_int {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_read_label(hw) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_if_label_direct_match(hw) }
}

/// Inst_SearchF: search forward for complement label.
/// Sets BX = search distance, CX = label size.
///
/// # Safety
/// `hw` and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_search_f(hw: *mut c_void, regs: *mut CpuRegisters) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_read_label(hw) };
    // Rotate label complement (done inside avd_hw_if_label_match via FFI_IfLabelMatch)
    // But here we need the rotated label for search, not for matching.
    // The C++ ReadLabel + Rotate + FindLabel sequence is encapsulated in the FFI.
    // We use avd_hw_search_label which calls FFI_SearchLabel internally.
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let ip_pos = unsafe { avd_hw_get_head_position(hw, HEAD_IP) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let found_pos = unsafe { avd_hw_search_label(hw, 1) }; // 1 = forward
    let search_size = found_pos - ip_pos;
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let label_size = unsafe { avd_hw_get_label_size(hw) };
    // SAFETY: set_reg null-checks internally.
    unsafe { set_reg(regs, 1, search_size) }; // REG_BX = 1
                                              // SAFETY: set_reg null-checks internally.
    unsafe { set_reg(regs, 2, label_size) }; // REG_CX = 2
}

/// Inst_SearchB: search backward for complement label.
/// Sets BX = search distance, CX = label size.
///
/// # Safety
/// `hw` and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_search_b(hw: *mut c_void, regs: *mut CpuRegisters) {
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_read_label(hw) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let ip_pos = unsafe { avd_hw_get_head_position(hw, HEAD_IP) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let found_pos = unsafe { avd_hw_search_label(hw, -1) };
    let search_size = ip_pos - found_pos;
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let label_size = unsafe { avd_hw_get_label_size(hw) };
    // SAFETY: set_reg null-checks internally.
    unsafe { set_reg(regs, 1, search_size) }; // REG_BX = 1
                                              // SAFETY: set_reg null-checks internally.
    unsafe { set_reg(regs, 2, label_size) }; // REG_CX = 2
}

/// Inst_ReadInst: read instruction opcode at position reg[src] into reg[dst].
///
/// # Safety
/// `hw` and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_read_inst(
    hw: *mut c_void,
    regs: *mut CpuRegisters,
    dst: c_int,
    src: c_int,
) {
    // SAFETY: caller guarantees valid pointers.
    let pos = unsafe { get_reg(regs as *const CpuRegisters, src) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let opcode = unsafe { avd_hw_get_inst_at_raw_pos(hw, pos) };
    // SAFETY: set_reg null-checks internally.
    unsafe { set_reg(regs, dst, opcode) };
}

/// Inst_StackReadInst: read instruction opcode at position reg[src], push onto stack.
///
/// # Safety
/// `hw` and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_stack_read_inst(
    hw: *mut c_void,
    regs: *const CpuRegisters,
    src: c_int,
) {
    // SAFETY: caller guarantees valid pointers.
    let pos = unsafe { get_reg(regs, src) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    let opcode = unsafe { avd_hw_get_inst_at_raw_pos(hw, pos) };
    // SAFETY: hw is a valid cHardwareBase pointer per caller contract.
    unsafe { avd_hw_stack_push(hw, opcode) };
}

// ---------------------------------------------------------------------------
// Conditional instruction evaluators
// ---------------------------------------------------------------------------
// Each returns 1 if the condition says to SKIP the next instruction, 0 otherwise.
// The C++ caller advances IP when the return value is 1.

/// Inst_IfNEqu: skip if reg[op1] == reg[op2] (execute next only if NOT equal).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_n_equ(
    regs: *const CpuRegisters,
    op1: c_int,
    op2: c_int,
) -> c_int {
    if regs.is_null() || !valid_reg(op1) || !valid_reg(op2) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op1 as usize] == r[op2 as usize] {
        1
    } else {
        0
    }
}

/// Inst_IfLess: skip if reg[op1] >= reg[op2] (execute next only if less).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_less(regs: *const CpuRegisters, op1: c_int, op2: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op1) || !valid_reg(op2) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op1 as usize] >= r[op2 as usize] {
        1
    } else {
        0
    }
}

/// Inst_IfGr: skip if reg[op1] <= reg[op2] (execute next only if greater).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_grtr(regs: *const CpuRegisters, op1: c_int, op2: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op1) || !valid_reg(op2) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op1 as usize] <= r[op2 as usize] {
        1
    } else {
        0
    }
}

/// Inst_IfLessEqu: skip if reg[op1] > reg[op2] (execute next only if <=).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_less_equ(
    regs: *const CpuRegisters,
    op1: c_int,
    op2: c_int,
) -> c_int {
    if regs.is_null() || !valid_reg(op1) || !valid_reg(op2) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op1 as usize] > r[op2 as usize] {
        1
    } else {
        0
    }
}

/// Inst_IfGr0: skip if reg[op] <= 0 (execute next only if > 0).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_gr0(regs: *const CpuRegisters, op: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op as usize] <= 0 {
        1
    } else {
        0
    }
}

/// Inst_IfGrEqu0: skip if reg[op] < 0 (execute next only if >= 0).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_gr_equ0(regs: *const CpuRegisters, op: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op as usize] < 0 {
        1
    } else {
        0
    }
}

/// Inst_IfLess0: skip if reg[op] >= 0 (execute next only if < 0).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_less0(regs: *const CpuRegisters, op: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op as usize] >= 0 {
        1
    } else {
        0
    }
}

/// Inst_IfLsEqu0: skip if reg[op] > 0 (execute next only if <= 0).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_ls_equ0(regs: *const CpuRegisters, op: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op as usize] > 0 {
        1
    } else {
        0
    }
}

/// Inst_IfGrEqu: skip if reg[op1] < reg[op2] (execute next only if >=).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_gr_equ(
    regs: *const CpuRegisters,
    op1: c_int,
    op2: c_int,
) -> c_int {
    if regs.is_null() || !valid_reg(op1) || !valid_reg(op2) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op1 as usize] < r[op2 as usize] {
        1
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// Fixed-register conditional evaluators (no FindModifiedRegister)
// ---------------------------------------------------------------------------

/// Inst_IfANotEqB: skip if AX == BX (execute next only if AX != BX).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_a_not_eq_b(regs: *const CpuRegisters) -> c_int {
    if regs.is_null() {
        return 0;
    }
    // SAFETY: null check above.
    let r = unsafe { &(*regs).reg };
    if r[0] == r[1] {
        1
    } else {
        0
    }
}

/// Inst_IfBNotEqC: skip if BX == CX.
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_b_not_eq_c(regs: *const CpuRegisters) -> c_int {
    if regs.is_null() {
        return 0;
    }
    // SAFETY: null check above.
    let r = unsafe { &(*regs).reg };
    if r[1] == r[2] {
        1
    } else {
        0
    }
}

/// Inst_IfANotEqC: skip if AX == CX.
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_a_not_eq_c(regs: *const CpuRegisters) -> c_int {
    if regs.is_null() {
        return 0;
    }
    // SAFETY: null check above.
    let r = unsafe { &(*regs).reg };
    if r[0] == r[2] {
        1
    } else {
        0
    }
}

/// Inst_IfBit1: skip if lowest bit of reg[op] is 0 (execute next only if bit 1 set).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_bit1(regs: *const CpuRegisters, op: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if (r[op as usize] & 1) == 0 {
        1
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// Consensus conditional evaluators (bit-count based)
// ---------------------------------------------------------------------------

/// Popcount (Hamming weight) for a 32-bit value.
#[inline]
fn bit_count(value: u32) -> u32 {
    value.count_ones()
}

const CONSENSUS: u32 = (std::mem::size_of::<c_int>() as u32) * 8 / 2; // 16 for 32-bit int
const CONSENSUS24: u32 = 12;
const MASK24: u32 = 0x00FF_FFFF;

/// Inst_IfConsensus: skip if popcount(reg[op]) < CONSENSUS (16).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_consensus(regs: *const CpuRegisters, op: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if bit_count(r[op as usize] as u32) < CONSENSUS {
        1
    } else {
        0
    }
}

/// Inst_IfConsensus24: skip if popcount(reg[op] & 0xFFFFFF) < 12.
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_consensus24(regs: *const CpuRegisters, op: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if bit_count((r[op as usize] as u32) & MASK24) < CONSENSUS24 {
        1
    } else {
        0
    }
}

/// Inst_IfLessConsensus: skip if popcount(reg[op1]) >= popcount(reg[op2]).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_less_consensus(
    regs: *const CpuRegisters,
    op1: c_int,
    op2: c_int,
) -> c_int {
    if regs.is_null() || !valid_reg(op1) || !valid_reg(op2) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if bit_count(r[op1 as usize] as u32) >= bit_count(r[op2 as usize] as u32) {
        1
    } else {
        0
    }
}

/// Inst_IfLessConsensus24: skip if popcount(reg[op1] & MASK24) >= popcount(reg[op2] & MASK24).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_less_consensus24(
    regs: *const CpuRegisters,
    op1: c_int,
    op2: c_int,
) -> c_int {
    if regs.is_null() || !valid_reg(op1) || !valid_reg(op2) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if bit_count((r[op1 as usize] as u32) & MASK24) >= bit_count((r[op2 as usize] as u32) & MASK24)
    {
        1
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// Additional conditional evaluators
// ---------------------------------------------------------------------------

/// Inst_If0: skip if reg[op] != 0 (execute next only if == 0).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_0(regs: *const CpuRegisters, op: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op as usize] != 0 {
        1
    } else {
        0
    }
}

/// Inst_IfNot0: skip if reg[op] == 0 (execute next only if != 0).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_not_0(regs: *const CpuRegisters, op: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op as usize] == 0 {
        1
    } else {
        0
    }
}

/// Inst_IfEqu: skip if reg[op1] != reg[op2] (execute next only if equal).
#[no_mangle]
pub extern "C" fn avd_cpu_inst_if_equ(regs: *const CpuRegisters, op1: c_int, op2: c_int) -> c_int {
    if regs.is_null() || !valid_reg(op1) || !valid_reg(op2) {
        return 0;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &(*regs).reg };
    if r[op1 as usize] != r[op2 as usize] {
        1
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// Register mask + NOR operations
// ---------------------------------------------------------------------------

/// Apply a bitmask to register[reg_id]: reg = reg & mask.
#[no_mangle]
pub extern "C" fn avd_cpu_reg_mask(regs: *mut CpuRegisters, reg_id: c_int, mask: c_int) {
    if regs.is_null() || !valid_reg(reg_id) {
        return;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &mut (*regs).reg };
    r[reg_id as usize] &= mask;
}

/// Inst_Nor: reg[dst] = ~(reg[BX] | reg[CX]).
#[no_mangle]
pub extern "C" fn avd_cpu_reg_nor(regs: *mut CpuRegisters, dst: c_int, op1: c_int, op2: c_int) {
    if regs.is_null() || !valid_reg(dst) || !valid_reg(op1) || !valid_reg(op2) {
        return;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &mut (*regs).reg };
    r[dst as usize] = !(r[op1 as usize] | r[op2 as usize]);
}

// ---------------------------------------------------------------------------
// BitConsensus value-producing instructions
// ---------------------------------------------------------------------------

/// Inst_BitConsensus: reg[dst] = (popcount(reg[src]) >= CONSENSUS) ? 1 : 0.
#[no_mangle]
pub extern "C" fn avd_cpu_inst_bit_consensus(regs: *mut CpuRegisters, dst: c_int, src: c_int) {
    if regs.is_null() || !valid_reg(dst) || !valid_reg(src) {
        return;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &mut (*regs).reg };
    r[dst as usize] = if bit_count(r[src as usize] as u32) >= CONSENSUS {
        1
    } else {
        0
    };
}

/// Inst_BitConsensus24: reg[dst] = (popcount(reg[src] & MASK24) >= CONSENSUS24) ? 1 : 0.
#[no_mangle]
pub extern "C" fn avd_cpu_inst_bit_consensus24(regs: *mut CpuRegisters, dst: c_int, src: c_int) {
    if regs.is_null() || !valid_reg(dst) || !valid_reg(src) {
        return;
    }
    // SAFETY: null check and bounds check above.
    let r = unsafe { &mut (*regs).reg };
    r[dst as usize] = if bit_count((r[src as usize] as u32) & MASK24) >= CONSENSUS24 {
        1
    } else {
        0
    };
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu_registers::CpuRegisters;

    #[test]
    fn test_if_n_equ_equal() {
        let regs = CpuRegisters { reg: [5, 5, 0] };
        assert_eq!(avd_cpu_inst_if_n_equ(&regs, REG_AX, REG_BX), 1);
    }

    #[test]
    fn test_if_n_equ_not_equal() {
        let regs = CpuRegisters { reg: [5, 3, 0] };
        assert_eq!(avd_cpu_inst_if_n_equ(&regs, REG_AX, REG_BX), 0);
    }

    #[test]
    fn test_if_less_true() {
        let regs = CpuRegisters { reg: [3, 5, 0] };
        assert_eq!(avd_cpu_inst_if_less(&regs, REG_AX, REG_BX), 0); // 3 < 5, don't skip
    }

    #[test]
    fn test_if_less_false() {
        let regs = CpuRegisters { reg: [5, 3, 0] };
        assert_eq!(avd_cpu_inst_if_less(&regs, REG_AX, REG_BX), 1); // 5 >= 3, skip
    }

    #[test]
    fn test_if_grtr_true() {
        let regs = CpuRegisters { reg: [5, 3, 0] };
        assert_eq!(avd_cpu_inst_if_grtr(&regs, REG_AX, REG_BX), 0); // 5 > 3, don't skip
    }

    #[test]
    fn test_if_grtr_false() {
        let regs = CpuRegisters { reg: [3, 5, 0] };
        assert_eq!(avd_cpu_inst_if_grtr(&regs, REG_AX, REG_BX), 1); // 3 <= 5, skip
    }

    #[test]
    fn test_if_less_equ_true() {
        let regs = CpuRegisters { reg: [3, 5, 0] };
        assert_eq!(avd_cpu_inst_if_less_equ(&regs, REG_AX, REG_BX), 0); // 3 <= 5, don't skip
    }

    #[test]
    fn test_if_less_equ_skip() {
        let regs = CpuRegisters { reg: [5, 3, 0] };
        assert_eq!(avd_cpu_inst_if_less_equ(&regs, REG_AX, REG_BX), 1); // 5 > 3, skip
    }

    #[test]
    fn test_if_less_equ_equal() {
        let regs = CpuRegisters { reg: [5, 5, 0] };
        assert_eq!(avd_cpu_inst_if_less_equ(&regs, REG_AX, REG_BX), 0); // 5 <= 5, don't skip
    }

    // Unary conditionals
    #[test]
    fn test_if_gr0() {
        let regs = CpuRegisters { reg: [5, 0, -1] };
        assert_eq!(avd_cpu_inst_if_gr0(&regs, 0), 0); // 5 > 0, don't skip
        assert_eq!(avd_cpu_inst_if_gr0(&regs, 1), 1); // 0 <= 0, skip
        assert_eq!(avd_cpu_inst_if_gr0(&regs, 2), 1); // -1 <= 0, skip
    }

    #[test]
    fn test_if_less0() {
        let regs = CpuRegisters { reg: [-1, 0, 5] };
        assert_eq!(avd_cpu_inst_if_less0(&regs, 0), 0); // -1 < 0, don't skip
        assert_eq!(avd_cpu_inst_if_less0(&regs, 1), 1); // 0 >= 0, skip
        assert_eq!(avd_cpu_inst_if_less0(&regs, 2), 1); // 5 >= 0, skip
    }

    #[test]
    fn test_null_regs() {
        assert_eq!(avd_cpu_inst_if_n_equ(std::ptr::null(), 0, 1), 0);
        assert_eq!(avd_cpu_inst_if_less(std::ptr::null(), 0, 1), 0);
        assert_eq!(avd_cpu_inst_if_grtr(std::ptr::null(), 0, 1), 0);
        assert_eq!(avd_cpu_inst_if_less_equ(std::ptr::null(), 0, 1), 0);
        assert_eq!(avd_cpu_inst_if_gr0(std::ptr::null(), 0), 0);
        assert_eq!(avd_cpu_inst_if_less0(std::ptr::null(), 0), 0);
    }

    // --- Fixed-register conditionals ---

    #[test]
    fn test_if_a_not_eq_b() {
        let regs = CpuRegisters { reg: [5, 5, 0] };
        assert_eq!(avd_cpu_inst_if_a_not_eq_b(&regs), 1); // equal → skip
        let regs = CpuRegisters { reg: [5, 3, 0] };
        assert_eq!(avd_cpu_inst_if_a_not_eq_b(&regs), 0); // not equal → don't skip
    }

    #[test]
    fn test_if_b_not_eq_c() {
        let regs = CpuRegisters { reg: [0, 7, 7] };
        assert_eq!(avd_cpu_inst_if_b_not_eq_c(&regs), 1); // equal → skip
        let regs = CpuRegisters { reg: [0, 7, 3] };
        assert_eq!(avd_cpu_inst_if_b_not_eq_c(&regs), 0);
    }

    #[test]
    fn test_if_a_not_eq_c() {
        let regs = CpuRegisters { reg: [7, 0, 7] };
        assert_eq!(avd_cpu_inst_if_a_not_eq_c(&regs), 1);
        let regs = CpuRegisters { reg: [7, 0, 3] };
        assert_eq!(avd_cpu_inst_if_a_not_eq_c(&regs), 0);
    }

    #[test]
    fn test_if_bit1() {
        let regs = CpuRegisters { reg: [3, 0, 0] }; // 3 = 0b11, bit 0 is 1
        assert_eq!(avd_cpu_inst_if_bit1(&regs, REG_AX), 0); // bit set → don't skip
        let regs = CpuRegisters { reg: [4, 0, 0] }; // 4 = 0b100, bit 0 is 0
        assert_eq!(avd_cpu_inst_if_bit1(&regs, REG_AX), 1); // bit not set → skip
    }

    // --- Consensus conditionals ---

    #[test]
    fn test_if_consensus() {
        // All bits set → 32 >= 16 → don't skip
        let regs = CpuRegisters {
            reg: [-1, 0, 0], // -1 = 0xFFFFFFFF = 32 bits set
        };
        assert_eq!(avd_cpu_inst_if_consensus(&regs, REG_AX), 0);
        // Zero → 0 < 16 → skip
        let regs = CpuRegisters { reg: [0, 0, 0] };
        assert_eq!(avd_cpu_inst_if_consensus(&regs, REG_AX), 1);
    }

    #[test]
    fn test_if_consensus24() {
        // 0xFFFFFF = 24 bits → popcount = 24 >= 12 → don't skip
        let regs = CpuRegisters {
            reg: [0x00FFFFFF_u32 as i32, 0, 0],
        };
        assert_eq!(avd_cpu_inst_if_consensus24(&regs, REG_AX), 0);
        // 0 → skip
        let regs = CpuRegisters { reg: [0, 0, 0] };
        assert_eq!(avd_cpu_inst_if_consensus24(&regs, REG_AX), 1);
    }

    #[test]
    fn test_if_less_consensus() {
        // popcount(0xFF) = 8, popcount(0xFFFF) = 16 → 8 < 16 → don't skip
        let regs = CpuRegisters {
            reg: [0xFF, 0xFFFF, 0],
        };
        assert_eq!(avd_cpu_inst_if_less_consensus(&regs, REG_AX, REG_BX), 0);
        // popcount(0xFFFF) = 16 >= popcount(0xFF) = 8 → skip
        let regs = CpuRegisters {
            reg: [0xFFFF, 0xFF, 0],
        };
        assert_eq!(avd_cpu_inst_if_less_consensus(&regs, REG_AX, REG_BX), 1);
    }

    // --- If0 / IfNot0 / IfEqu ---

    #[test]
    fn test_if_0() {
        let regs = CpuRegisters { reg: [0, 5, 0] };
        assert_eq!(avd_cpu_inst_if_0(&regs, REG_AX), 0); // == 0, don't skip
        assert_eq!(avd_cpu_inst_if_0(&regs, REG_BX), 1); // != 0, skip
    }

    #[test]
    fn test_if_not_0() {
        let regs = CpuRegisters { reg: [0, 5, 0] };
        assert_eq!(avd_cpu_inst_if_not_0(&regs, REG_AX), 1); // == 0, skip
        assert_eq!(avd_cpu_inst_if_not_0(&regs, REG_BX), 0); // != 0, don't skip
    }

    #[test]
    fn test_if_equ() {
        let regs = CpuRegisters { reg: [5, 5, 3] };
        assert_eq!(avd_cpu_inst_if_equ(&regs, REG_AX, REG_BX), 0); // equal, don't skip
        assert_eq!(avd_cpu_inst_if_equ(&regs, REG_AX, 2), 1); // not equal, skip
    }

    // --- Nor / Mask ---

    #[test]
    fn test_nor() {
        let mut regs = CpuRegisters {
            reg: [0, 0xFF, 0xFF00],
        };
        avd_cpu_reg_nor(&mut regs, REG_AX, REG_BX, 2);
        assert_eq!(regs.reg[0], !(0xFF | 0xFF00));
    }

    #[test]
    fn test_mask() {
        let mut regs = CpuRegisters {
            reg: [0xFFFF_FFFFu32 as i32, 0, 0],
        };
        avd_cpu_reg_mask(&mut regs, REG_AX, 0xFFFF_0000u32 as i32);
        assert_eq!(regs.reg[0], 0xFFFF_0000u32 as i32);
    }

    // --- BitConsensus value producers ---

    #[test]
    fn test_bit_consensus() {
        let mut regs = CpuRegisters { reg: [-1, 0, 0] }; // all bits → consensus
        avd_cpu_inst_bit_consensus(&mut regs, REG_BX, REG_AX);
        assert_eq!(regs.reg[1], 1);

        let mut regs = CpuRegisters { reg: [1, 0, 0] }; // 1 bit < 16 → no consensus
        avd_cpu_inst_bit_consensus(&mut regs, REG_BX, REG_AX);
        assert_eq!(regs.reg[1], 0);
    }

    #[test]
    fn test_bit_consensus24() {
        let mut regs = CpuRegisters {
            reg: [0x00FFFFFF_u32 as i32, 0, 0],
        };
        avd_cpu_inst_bit_consensus24(&mut regs, REG_BX, REG_AX);
        assert_eq!(regs.reg[1], 1);

        let mut regs = CpuRegisters { reg: [0xFF, 0, 0] }; // 8 bits < 12 → no
        avd_cpu_inst_bit_consensus24(&mut regs, REG_BX, REG_AX);
        assert_eq!(regs.reg[1], 0);
    }
}
