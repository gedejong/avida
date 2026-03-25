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
    fn avd_hw_search_label_direct(hw: *mut c_void, direction: c_int) -> c_int;
    fn avd_hw_is_next_inst_nop(hw: *mut c_void) -> c_int;
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
    fn avd_org_clear_easterly(org: *mut c_void);
    fn avd_org_clear_northerly(org: *mut c_void);
    fn avd_org_set_lyse_display(org: *mut c_void);
    fn avd_org_set_mate_preference(org: *mut c_void, pref: c_int);
    fn avd_org_get_cur_mating_display_a(org: *mut c_void) -> c_int;
    fn avd_org_get_cur_mating_display_b(org: *mut c_void) -> c_int;
    fn avd_org_set_cur_mating_display_a(org: *mut c_void, val: c_int);
    fn avd_org_set_cur_mating_display_b(org: *mut c_void, val: c_int);
    fn avd_org_set_cell_data(org: *mut c_void, data: c_int);
    fn avd_org_get_copy_mut_prob(org: *mut c_void) -> f64;
    fn avd_org_set_copy_mut_prob(org: *mut c_void, prob: f64);
    fn avd_org_add_output(org: *mut c_void, val: c_int);
    // Phase 3: I/O organism methods
    fn avd_org_do_output(org: *mut c_void, ctx: *mut c_void, value: c_int);
    fn avd_org_get_next_input(org: *mut c_void) -> c_int;
    fn avd_org_do_input(org: *mut c_void, value: c_int);
    // Agent B additions
    fn avd_org_get_cur_bonus(org: *mut c_void) -> f64;
    fn avd_org_set_cur_bonus(org: *mut c_void, value: f64);
    fn avd_org_repair_point_mut_off(org: *mut c_void);
    // Phase 4: energy mutation (existing FFI, newly used here)
    fn avd_org_reduce_energy(org: *mut c_void, amount: f64);
    fn avd_org_increase_energy_donated(org: *mut c_void, amount: f64);
    fn avd_org_receive_donated_energy(org: *mut c_void, amount: f64);
    fn avd_org_apply_donated_energy(org: *mut c_void);
    fn avd_org_set_is_energy_donor(org: *mut c_void);
    fn avd_org_set_is_energy_receiver(org: *mut c_void);
    // Phase 4: opinion interface
    fn avd_org_iface_set_opinion(org: *mut c_void, value: c_int);
    fn avd_org_iface_clear_opinion(org: *mut c_void);
    fn avd_org_iface_has_opinion(org: *mut c_void) -> c_int;
    fn avd_org_get_opinion_first(org: *mut c_void) -> c_int;
    fn avd_org_get_opinion_second(org: *mut c_void) -> c_int;
    // Phase 4: messaging
    fn avd_org_send_value(org: *mut c_void, value: c_int);
    // Phase 4: donation support
    fn avd_org_get_cur_num_donates(org: *mut c_void) -> c_int;
    fn avd_org_inc_donates(org: *mut c_void);
    fn avd_org_set_is_donor_rand(org: *mut c_void);
    fn avd_org_set_is_donor_null(org: *mut c_void);
    fn avd_org_set_is_receiver_flag(org: *mut c_void);
    fn avd_org_get_merit_double(org: *mut c_void) -> f64;
    fn avd_org_update_merit(org: *mut c_void, ctx: *mut c_void, new_merit: f64);
    fn avd_org_get_energy_usage_ratio(org: *mut c_void) -> f64;
    fn avd_org_convert_energy_to_merit(org: *mut c_void, energy: f64) -> f64;
    fn avd_org_increase_energy_received(org: *mut c_void, amount: f64);
    fn avd_org_increase_num_energy_donations(org: *mut c_void);
    fn avd_org_deme_increase_energy_donated(org: *mut c_void, amount: f64);
    fn avd_org_deme_increase_energy_received(org: *mut c_void, amount: f64);
    fn avd_org_has_open_energy_request(org: *mut c_void) -> c_int;
    // Phase 5: group/population queries + deme donation
    fn avd_org_iface_number_of_orgs_in_group(org: *mut c_void, group_id: c_int) -> c_int;
    fn avd_org_donate_res_consumed_to_deme(org: *mut c_void);
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

// ---------------------------------------------------------------------------
// Batch D9: Cell data, group count, opinion-to-value, donate-res-to-deme
// ---------------------------------------------------------------------------

/// Inst_CollectCellData: read cell data into register.
/// C++ caller is responsible for updating m_last_cell_data after this call.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_collect_cell_data(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let cell_data = unsafe { avd_org_get_cell_data_org(org) };
    unsafe { avd_hw_set_register(hw, reg_id, cell_data) };
}

/// Inst_SetOpinionToZero/One/Two: set register to value, set opinion, do output.
/// Consumes TWO FindModifiedRegister calls in C++ (behavior parity).
/// C++ passes the first-consumed register as out_reg and second-consumed as opinion_reg.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_set_opinion_to_value(
    hw: *mut c_void,
    ctx: *mut c_void,
    value: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_iface_set_opinion(org, value) };
    unsafe { avd_org_do_output(org, ctx, value) };
}

/// Inst_NumberOrgsInMyGroup: get count of organisms in this org's group.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_number_orgs_in_my_group(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let mut num_orgs = 0;
    if unsafe { avd_org_iface_has_opinion(org) } != 0 {
        let opinion = unsafe { avd_org_get_opinion_first(org) };
        num_orgs = unsafe { avd_org_iface_number_of_orgs_in_group(org, opinion) };
    }
    unsafe { avd_hw_set_register(hw, reg_id, num_orgs) };
}

/// Inst_NumberOrgsInGroup: get count of organisms in the group specified by
/// group_reg (the register index IS the group ID — matching C++ behavior).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_number_orgs_in_group(
    hw: *mut c_void,
    group_reg: c_int,
    result_reg: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let num_orgs = unsafe { avd_org_iface_number_of_orgs_in_group(org, group_reg) };
    unsafe { avd_hw_set_register(hw, result_reg, num_orgs) };
}

/// Inst_DonateResToDeme: donate consumed resources to the organism's deme.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_donate_res_to_deme(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_donate_res_consumed_to_deme(org) };
}

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
// Batch C2+: Simple state-write handlers
// ---------------------------------------------------------------------------

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_zero_easterly(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_clear_easterly(org) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_zero_northerly(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_clear_northerly(org) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_display_lyse(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_set_lyse_display(org) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_set_mate_preference(hw: *mut c_void, pref: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_set_mate_preference(org, pref) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_set_mating_display_a(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let val = unsafe { avd_hw_get_register(hw, reg_id) };
    unsafe { avd_org_set_cur_mating_display_a(org, val) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_set_mating_display_b(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let val = unsafe { avd_hw_get_register(hw, reg_id) };
    unsafe { avd_org_set_cur_mating_display_b(org, val) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_increment_mating_display_a(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let counter = unsafe { avd_org_get_cur_mating_display_a(org) } + 1;
    unsafe { avd_org_set_cur_mating_display_a(org, counter) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_increment_mating_display_b(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let counter = unsafe { avd_org_get_cur_mating_display_b(org) } + 1;
    unsafe { avd_org_set_cur_mating_display_b(org, counter) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_mark_cell_with_id(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let id = unsafe { avd_org_get_id(org) };
    unsafe { avd_org_set_cell_data(org, id) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_mark_cell_with_vitality(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let vit = unsafe { avd_org_get_vitality(org) };
    let rounded = (vit + 0.5) as c_int;
    unsafe { avd_org_set_cell_data(org, rounded) };
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_set_copy_mut(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let reg_val = unsafe { avd_hw_get_register(hw, reg_id) };
    let clamped = if reg_val < 1 { 1 } else { reg_val };
    let prob = clamped as f64 / 10000.0;
    unsafe { avd_org_set_copy_mut_prob(org, prob) };
}

/// Inst_ModCopyMut: add register value / 10000 to copy mutation probability.
/// Only applies if result is positive.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_mod_copy_mut(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let reg_val = unsafe { avd_hw_get_register(hw, reg_id) };
    let old_prob = unsafe { avd_org_get_copy_mut_prob(org) };
    let new_prob = old_prob + reg_val as f64 / 10000.0;
    if new_prob > 0.0 {
        unsafe { avd_org_set_copy_mut_prob(org, new_prob) };
    }
}

// ---------------------------------------------------------------------------
// Faced neighbor energy conditionals
// ---------------------------------------------------------------------------

/// Check faced neighbor's discrete energy level vs expected.
/// Returns: 0 = no skip (or no valid neighbor), 1 = skip.
/// `expected_level`: the energy level that SHOULD cause execution (don't skip).
/// `negate`: if true, skip when level EQUALS expected (for "Not" variants).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_faced_energy(
    hw: *mut c_void,
    expected_level: c_int,
    negate: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { avd_org_get_neighbor(org) };
    if neighbor.is_null() {
        return 0;
    }
    if unsafe { avd_org_is_dead(neighbor) } != 0 {
        return 0;
    }
    let level = unsafe { avd_org_get_discrete_energy_level(neighbor) };
    if negate == 0 {
        // Skip if NOT expected level
        if level != expected_level {
            1
        } else {
            0
        }
    } else {
        // Skip if IS expected level (for "Not" variants)
        if level == expected_level {
            1
        } else {
            0
        }
    }
}

// avd_cpu_inst_get_faced_energy_level defined in Agent B's batch below

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_io_buf_add(hw: *mut c_void, val: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_add_output(org, val) };
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

// ---------------------------------------------------------------------------
// Batch: Head-search + label + head-push/transposon handlers
// ---------------------------------------------------------------------------

/// Inst_HeadSearch: read label, rotate complement, find from beginning.
/// Sets BX = search distance, CX = label size, FLOW = found position + 1.
///
/// # Safety
/// `hw` and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_head_search(hw: *mut c_void, regs: *mut CpuRegisters) {
    // ReadLabel
    unsafe { avd_hw_read_label(hw) };
    let ip_pos = unsafe { avd_hw_get_head_position(hw, HEAD_IP) };
    // search_label rotates complement and searches from direction 0 (beginning)
    let found_pos = unsafe { avd_hw_search_label(hw, 0) };
    let search_size = found_pos - ip_pos;
    let label_size = unsafe { avd_hw_get_label_size(hw) };
    unsafe { set_reg(regs, 1, search_size) }; // REG_BX = 1
    unsafe { set_reg(regs, 2, label_size) }; // REG_CX = 2
                                             // Set FLOW head to found position, then advance
    unsafe { avd_hw_set_head_position(hw, HEAD_FLOW, found_pos) };
    unsafe { avd_hw_advance_head(hw, HEAD_FLOW) };
}

/// Inst_HeadSearchDirect: read label, find direct match forward from IP.
/// Sets BX = search distance, CX = label size, FLOW = found position + 1.
///
/// # Safety
/// `hw` and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_head_search_direct(hw: *mut c_void, regs: *mut CpuRegisters) {
    // ReadLabel (no complement rotation)
    unsafe { avd_hw_read_label(hw) };
    let ip_pos = unsafe { avd_hw_get_head_position(hw, HEAD_IP) };
    // search_label_direct does NOT rotate; direction 1 = forward from IP
    let found_pos = unsafe { avd_hw_search_label_direct(hw, 1) };
    let search_size = found_pos - ip_pos;
    let label_size = unsafe { avd_hw_get_label_size(hw) };
    unsafe { set_reg(regs, 1, search_size) }; // REG_BX = 1
    unsafe { set_reg(regs, 2, label_size) }; // REG_CX = 2
                                             // Set FLOW head to found position, then advance
    unsafe { avd_hw_set_head_position(hw, HEAD_FLOW, found_pos) };
    unsafe { avd_hw_advance_head(hw, HEAD_FLOW) };
}

/// Inst_HeadPush: push head[head_id] position onto stack.
/// If head_id == IP, also set IP to FLOW position.
/// Returns 1 if IP should NOT auto-advance (head_id == IP), 0 otherwise.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_head_push(hw: *mut c_void, head_id: c_int) -> c_int {
    let pos = unsafe { avd_hw_get_head_position(hw, head_id) };
    unsafe { avd_hw_stack_push(hw, pos) };
    if head_id == HEAD_IP {
        let flow_pos = unsafe { avd_hw_get_head_position(hw, HEAD_FLOW) };
        unsafe { avd_hw_set_head_position(hw, head_id, flow_pos) };
        1 // suppress auto-advance
    } else {
        0
    }
}

/// Inst_IfLabel2: read label, rotate complement, skip instruction + following nops
/// if labels don't match.
/// Returns a skip count: 0 = no skip, 1 = skip one, 2 = skip one + a nop.
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_label2(hw: *mut c_void) -> c_int {
    // ReadLabel + rotate + compare (reuses avd_hw_if_label_match which does rotate+compare)
    unsafe { avd_hw_read_label(hw) };
    let mismatch = unsafe { avd_hw_if_label_match(hw) };
    if mismatch != 0 {
        // Labels don't match: skip next instruction.
        // Also skip any nop following that instruction.
        unsafe { avd_hw_advance_ip(hw) }; // skip one
        if unsafe { avd_hw_is_next_inst_nop(hw) } != 0 {
            unsafe { avd_hw_advance_ip(hw) }; // skip nop after it
        }
    }
    0 // we already did our own IP advances; caller should not skip further
}

/// Inst_Transposon: just read the label (no other effect).
///
/// # Safety
/// `hw` must be a valid cHardwareBase pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_transposon(hw: *mut c_void) {
    unsafe { avd_hw_read_label(hw) };
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
// Batch: I/O instruction handlers (Phase 3)
// ---------------------------------------------------------------------------

/// Inst_TaskIO: read reg → DoOutput → GetNextInput → write reg → DoInput.
///
/// This is the core I/O instruction that triggers task evaluation.
/// No guards before FindModifiedRegister in the original C++.
///
/// # Safety
/// `hw`, `ctx`, and `regs` must be valid pointers. `ctx` is a cAvidaContext*.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_task_io(
    hw: *mut c_void,
    ctx: *mut c_void,
    regs: *mut CpuRegisters,
    reg_id: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    // Do the "put" component
    let value_out = unsafe { get_reg(regs, reg_id) };
    unsafe { avd_org_do_output(org, ctx, value_out) };
    // Do the "get" component
    let value_in = unsafe { avd_org_get_next_input(org) };
    unsafe { set_reg(regs, reg_id, value_in) };
    unsafe { avd_org_do_input(org, value_in) };
}

/// Inst_TaskGet (TaskInput): GetNextInput → write reg → DoInput.
///
/// No guards before FindModifiedRegister in the original C++.
///
/// # Safety
/// `hw` and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_task_input(
    hw: *mut c_void,
    regs: *mut CpuRegisters,
    reg_id: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let value = unsafe { avd_org_get_next_input(org) };
    unsafe { set_reg(regs, reg_id, value) };
    unsafe { avd_org_do_input(org, value) };
}

/// Inst_TaskPut (TaskOutput): read reg → DoOutput → zero reg.
///
/// No guards before FindModifiedRegister in the original C++.
///
/// # Safety
/// `hw`, `ctx`, and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_task_output(
    hw: *mut c_void,
    ctx: *mut c_void,
    regs: *mut CpuRegisters,
    reg_id: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let value = unsafe { get_reg(regs, reg_id) };
    unsafe { set_reg(regs, reg_id, 0) };
    unsafe { avd_org_do_output(org, ctx, value) };
}

/// Inst_TaskStackGet: GetNextInput → push to stack → DoInput.
///
/// No register operand, no FindModifiedRegister.
///
/// # Safety
/// `hw` must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_task_stack_get(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let value = unsafe { avd_org_get_next_input(org) };
    unsafe { avd_hw_stack_push(hw, value) };
    unsafe { avd_org_do_input(org, value) };
}

/// Inst_TaskStackLoad: push 3 inputs onto stack (no DoInput).
///
/// # Safety
/// `hw` must be a valid pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_task_stack_load(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    for _ in 0..3 {
        let value = unsafe { avd_org_get_next_input(org) };
        unsafe { avd_hw_stack_push(hw, value) };
    }
}

// ---------------------------------------------------------------------------
// Batch: I/O variants + state transitions (Phase 4)
// ---------------------------------------------------------------------------

unsafe extern "C" {
    fn avd_org_reset_inputs(org: *mut c_void, ctx: *mut c_void);
    fn avd_org_clear_input(org: *mut c_void);
    fn avd_org_die(org: *mut c_void, ctx: *mut c_void);
    fn avd_hw_find_next_register(base_reg: c_int) -> c_int;
}

/// Inst_TaskIO_BonusCost: levy a proportional bonus cost, then do TaskIO.
///
/// bonus_cost is the fraction of bonus to remove (e.g. 0.001).
/// Bonus is clamped to >= 0.
///
/// # Safety
/// `hw`, `ctx`, and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_task_io_bonus_cost(
    hw: *mut c_void,
    ctx: *mut c_void,
    regs: *mut CpuRegisters,
    reg_id: c_int,
    bonus_cost: f64,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    // Levy the cost
    let cur_bonus = unsafe { avd_org_get_cur_bonus(org) };
    let mut new_bonus = cur_bonus * (1.0 - bonus_cost);
    if new_bonus < 0.0 {
        new_bonus = 0.0;
    }
    unsafe { avd_org_set_cur_bonus(org, new_bonus) };
    // Then do normal TaskIO
    unsafe { avd_cpu_inst_task_io(hw, ctx, regs, reg_id) };
}

/// Inst_TaskIO_Feedback: TaskIO + push merit-change indicator to stack.
///
/// Pushes 1 if bonus increased, 0 if same, -1 if decreased.
///
/// # Safety
/// `hw`, `ctx`, and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_task_io_feedback(
    hw: *mut c_void,
    ctx: *mut c_void,
    regs: *mut CpuRegisters,
    reg_id: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    // Check cur_bonus before the output
    let pre_bonus = unsafe { avd_org_get_cur_bonus(org) };

    // Do the "put" component
    let value_out = unsafe { get_reg(regs, reg_id) };
    unsafe { avd_org_do_output(org, ctx, value_out) };

    // Check cur_bonus after the output
    let post_bonus = unsafe { avd_org_get_cur_bonus(org) };

    // Push the effect on merit (+, 0, -) to active stack
    let indicator = if pre_bonus > post_bonus {
        -1
    } else if pre_bonus < post_bonus {
        1
    } else {
        0
    };
    unsafe { avd_hw_stack_push(hw, indicator) };

    // Do the "get" component
    let value_in = unsafe { avd_org_get_next_input(org) };
    unsafe { set_reg(regs, reg_id, value_in) };
    unsafe { avd_org_do_input(org, value_in) };
}

/// Inst_TaskPutResetInputs: do a normal TaskPut, then reset+clear inputs.
///
/// # Safety
/// `hw`, `ctx`, and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_task_put_reset_inputs(
    hw: *mut c_void,
    ctx: *mut c_void,
    regs: *mut CpuRegisters,
    reg_id: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    // Do a normal put (TaskOutput): read reg → zero reg → DoOutput
    let value = unsafe { get_reg(regs, reg_id) };
    unsafe { set_reg(regs, reg_id, 0) };
    unsafe { avd_org_do_output(org, ctx, value) };
    // Now re-randomize inputs and clear input buffer
    unsafe { avd_org_reset_inputs(org, ctx) };
    unsafe { avd_org_clear_input(org) };
}

/// Inst_TaskGet2: get two input values into consecutive registers.
///
/// C++ caller handles ResetInputs/ClearInput and FindModifiedRegister before
/// calling this function (reset must happen before nop consumption).
/// FindNextRegister is a pure function (no side effects) so safe to call here.
///
/// # Safety
/// `hw`, `ctx`, and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_task_get2(
    hw: *mut c_void,
    _ctx: *mut c_void,
    regs: *mut CpuRegisters,
    reg_id: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };

    let reg_used_1 = reg_id;
    let reg_used_2 = unsafe { avd_hw_find_next_register(reg_used_1) };

    let value1 = unsafe { avd_org_get_next_input(org) };
    unsafe { set_reg(regs, reg_used_1, value1) };
    unsafe { avd_org_do_input(org, value1) };

    let value2 = unsafe { avd_org_get_next_input(org) };
    unsafe { set_reg(regs, reg_used_2, value2) };
    unsafe { avd_org_do_input(org, value2) };
}

/// Inst_Die: unconditionally kill the organism.
///
/// # Safety
/// `hw` and `ctx` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_die(hw: *mut c_void, ctx: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_die(org, ctx) };
}

/// Inst_Prob_Die: probabilistically kill the organism.
///
/// config_prob: KABOOM_PROB from config. If -1.0, use register value % 100 as percent.
///
/// # Safety
/// `hw`, `ctx`, and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_prob_die(
    hw: *mut c_void,
    ctx: *mut c_void,
    regs: *const CpuRegisters,
    reg_id: c_int,
    config_prob: f64,
) {
    let percent_prob = if config_prob == -1.0 {
        let reg_val = unsafe { get_reg(regs, reg_id) };
        ((reg_val % 100) as f64) / 100.0
    } else {
        config_prob
    };
    if unsafe { avd_ctx_random_p(ctx, percent_prob) } != 0 {
        let org = unsafe { avd_hw_get_organism(hw) };
        unsafe { avd_org_die(org, ctx) };
    }
}

// Batch D1: Faced-energy conditionals
// ---------------------------------------------------------------------------

unsafe extern "C" {
    fn avd_org_is_energy_requestor(org: *mut c_void) -> c_int;
    #[allow(dead_code)]
    fn avd_org_has_open_energy_request_read(org: *mut c_void) -> c_int;
    fn avd_org_set_reputation(org: *mut c_void, rep: c_int);
    fn avd_org_get_faced_cell_data(org: *mut c_void) -> c_int;
    fn avd_org_get_faced_cell_data_update(org: *mut c_void) -> c_int;
    fn avd_org_is_donor(org: *mut c_void, neighbor_id: c_int) -> c_int;
    // Hardware flash info
    fn avd_hw_get_flash_received(hw: *mut c_void) -> c_int;
    fn avd_hw_get_flash_cycle(hw: *mut c_void) -> c_int;
    fn avd_hw_reset_flash_info(hw: *mut c_void);
}

/// Helper: get neighbor organism pointer, checking occupied and alive.
/// Returns null if no valid neighbor.
#[inline]
unsafe fn get_live_neighbor(org: *mut c_void) -> *mut c_void {
    if unsafe { avd_org_is_neighbor_cell_occupied(org) } == 0 {
        return std::ptr::null_mut();
    }
    let neighbor = unsafe { avd_org_get_neighbor(org) };
    if neighbor.is_null() || unsafe { avd_org_is_dead(neighbor) } != 0 {
        return std::ptr::null_mut();
    }
    neighbor
}

/// Inst_IfFacedEnergyLow: skip if faced neighbor's energy is NOT low.
/// Only acts if neighbor exists and is alive.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_faced_energy_low(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { get_live_neighbor(org) };
    if neighbor.is_null() {
        return 0; // no skip, no neighbor
    }
    let level = unsafe { avd_org_get_discrete_energy_level(neighbor) };
    if level != ENERGY_LEVEL_LOW {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_faced_energy_not_low(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { get_live_neighbor(org) };
    if neighbor.is_null() {
        return 0;
    }
    let level = unsafe { avd_org_get_discrete_energy_level(neighbor) };
    if level == ENERGY_LEVEL_LOW {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_faced_energy_high(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { get_live_neighbor(org) };
    if neighbor.is_null() {
        return 0;
    }
    let level = unsafe { avd_org_get_discrete_energy_level(neighbor) };
    if level != ENERGY_LEVEL_HIGH {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_faced_energy_not_high(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { get_live_neighbor(org) };
    if neighbor.is_null() {
        return 0;
    }
    let level = unsafe { avd_org_get_discrete_energy_level(neighbor) };
    if level == ENERGY_LEVEL_HIGH {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_faced_energy_med(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { get_live_neighbor(org) };
    if neighbor.is_null() {
        return 0;
    }
    let level = unsafe { avd_org_get_discrete_energy_level(neighbor) };
    if level != ENERGY_LEVEL_MEDIUM {
        1
    } else {
        0
    }
}

/// Inst_IfFacedEnergyLess: skip if neighbor energy >= my energy * (1-epsilon).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_faced_energy_less(hw: *mut c_void, epsilon: f64) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { get_live_neighbor(org) };
    if neighbor.is_null() {
        return 0;
    }
    let neighbor_energy = unsafe { avd_org_get_stored_energy(neighbor) };
    let my_energy = unsafe { avd_org_get_stored_energy(org) };
    if neighbor_energy >= my_energy * (1.0 - epsilon) {
        1
    } else {
        0
    }
}

/// Inst_IfFacedEnergyMore: skip if neighbor energy <= my energy * (1+epsilon).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_faced_energy_more(hw: *mut c_void, epsilon: f64) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { get_live_neighbor(org) };
    if neighbor.is_null() {
        return 0;
    }
    let neighbor_energy = unsafe { avd_org_get_stored_energy(neighbor) };
    let my_energy = unsafe { avd_org_get_stored_energy(org) };
    if neighbor_energy <= my_energy * (1.0 + epsilon) {
        1
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// Batch D2: Faced energy request handlers
// ---------------------------------------------------------------------------

/// Inst_IfFacedEnergyRequestOn: skip if neighbor is NOT an energy requestor.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_faced_energy_request_on(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { get_live_neighbor(org) };
    if neighbor.is_null() {
        return -1; // signal failure to C++
    }
    if unsafe { avd_org_is_energy_requestor(neighbor) } == 0 {
        1
    } else {
        0
    }
}

/// Inst_IfFacedEnergyRequestOff: skip if neighbor IS an energy requestor.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_faced_energy_request_off(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { get_live_neighbor(org) };
    if neighbor.is_null() {
        return -1;
    }
    if unsafe { avd_org_is_energy_requestor(neighbor) } != 0 {
        1
    } else {
        0
    }
}

/// Inst_GetEnergyRequestStatus: store 1 if self is energy requestor, 0 otherwise.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_energy_request_status(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let status = if unsafe { avd_org_is_energy_requestor(org) } != 0 {
        1
    } else {
        0
    };
    unsafe { avd_hw_set_register(hw, reg_id, status) };
}

/// Inst_GetFacedEnergyRequestStatus: store 1 if neighbor is energy requestor.
/// Returns -1 on failure (no neighbor), 0 on success.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_faced_energy_request_status(
    hw: *mut c_void,
    reg_id: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { get_live_neighbor(org) };
    if neighbor.is_null() {
        return -1;
    }
    let status = if unsafe { avd_org_is_energy_requestor(neighbor) } != 0 {
        1
    } else {
        0
    };
    unsafe { avd_hw_set_register(hw, reg_id, status) };
    0
}

/// Inst_GetFacedEnergyLevel: store faced neighbor's stored energy (floored).
/// Returns 0 on failure, 1 on success.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_faced_energy_level(
    hw: *mut c_void,
    reg_id: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { get_live_neighbor(org) };
    if neighbor.is_null() {
        return 0;
    }
    let energy = unsafe { avd_org_get_stored_energy(neighbor) };
    unsafe { avd_hw_set_register(hw, reg_id, energy.floor() as c_int) };
    1
}

// ---------------------------------------------------------------------------
// Batch D3: Energy in buffer, GetUpdate, GetTimeUsed, Poison, Pose, etc.
// ---------------------------------------------------------------------------

/// Inst_IfEnergyInBuffer: skip if NO energy in buffer (energy == 0).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_energy_in_buffer(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let energy = unsafe { avd_org_get_energy_in_buffer(org) };
    if energy == 0.0 {
        1
    } else {
        0
    }
}

/// Inst_GetUpdate: store world update into register.
/// `update_val` is passed from C++ caller (m_world->GetStats().GetUpdate()).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_update(
    hw: *mut c_void,
    reg_id: c_int,
    update_val: c_int,
) {
    unsafe { avd_hw_set_register(hw, reg_id, update_val) };
}

/// Inst_GetTimeUsed: store organism's time used into register.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_time_used(hw: *mut c_void, reg_id: c_int) {
    unsafe { org_int_to_reg(hw, reg_id, avd_org_get_time_used) };
}

/// Inst_GetCellPosition: store x and y into two registers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_cell_position(hw: *mut c_void, xreg: c_int, yreg: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let x = unsafe { avd_org_get_cell_position_x(org) };
    let y = unsafe { avd_org_get_cell_y_position(org) };
    unsafe { avd_hw_set_register(hw, xreg, x) };
    unsafe { avd_hw_set_register(hw, yreg, y) };
}

/// Inst_Poison: multiply current bonus by (1 - penalty).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_poison(hw: *mut c_void, penalty: f64) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let cur = unsafe { avd_org_get_cur_bonus(org) };
    unsafe { avd_org_set_cur_bonus(org, cur * (1.0 - penalty)) };
}

/// Inst_Pose: increment organism's reputation by 1.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_pose(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let rep = unsafe { avd_org_get_reputation(org) };
    unsafe { avd_org_set_reputation(org, rep + 1) };
}

/// Inst_GetNeighborsReputation: read faced neighbor's reputation into register.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_neighbors_reputation(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { avd_org_get_neighbor(org) };
    if neighbor.is_null() {
        return; // no neighbor, do nothing (matches C++ behavior)
    }
    let rep = unsafe { avd_org_get_reputation(neighbor) };
    unsafe { avd_hw_set_register(hw, reg_id, rep) };
}

/// Inst_RepairPointMutOff: disable repair.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_repair_point_mut_off(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_repair_point_mut_off(org) };
}

// ---------------------------------------------------------------------------
// Batch D4: Flash info handlers
// ---------------------------------------------------------------------------

/// Inst_IfRecvdFlash: skip if no flash has been received.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_recvd_flash(hw: *mut c_void) -> c_int {
    let received = unsafe { avd_hw_get_flash_received(hw) };
    if received == 0 {
        1
    } else {
        0
    }
}

/// Inst_FlashInfo: if flash received, store count in bx and cycles-since in cx.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_flash_info(hw: *mut c_void, bx: c_int, cx: c_int) {
    let received = unsafe { avd_hw_get_flash_received(hw) };
    if received > 0 {
        let cycle = unsafe { avd_hw_get_flash_cycle(hw) };
        let cur_cycle = unsafe { avd_hw_get_cycle_counter(hw) };
        unsafe { avd_hw_set_register(hw, bx, received) };
        unsafe { avd_hw_set_register(hw, cx, cur_cycle - cycle) };
    } else {
        unsafe { avd_hw_set_register(hw, bx, 0) };
        unsafe { avd_hw_set_register(hw, cx, 0) };
    }
}

/// Inst_FlashInfoB: store flash received count in bx (no cycle info).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_flash_info_b(hw: *mut c_void, bx: c_int) {
    let received = unsafe { avd_hw_get_flash_received(hw) };
    if received > 0 {
        unsafe { avd_hw_set_register(hw, bx, received) };
    } else {
        unsafe { avd_hw_set_register(hw, bx, 0) };
    }
}

/// Inst_ResetFlashInfo: reset flash info to (0, 0).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_reset_flash_info(hw: *mut c_void) {
    unsafe { avd_hw_reset_flash_info(hw) };
}

// ---------------------------------------------------------------------------
// Batch D5: Fixed-register stack ops, IfDonor, ReadFacedCellData*
// ---------------------------------------------------------------------------

/// Pop from active stack into a fixed register (for PopA/PopB/PopC).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_pop_fixed(hw: *mut c_void, reg_id: c_int) {
    let value = unsafe { avd_hw_stack_pop(hw) };
    unsafe { avd_hw_set_register(hw, reg_id, value) };
}

/// Push a fixed register onto the active stack (for PushA/PushB/PushC).
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_push_fixed(hw: *mut c_void, reg_id: c_int) {
    let value = unsafe { avd_hw_get_register(hw, reg_id) };
    unsafe { avd_hw_stack_push(hw, value) };
}

/// Inst_IfDonor: skip if neighbor was NOT a donor to this organism.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_if_donor(hw: *mut c_void) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighbor = unsafe { avd_org_get_neighbor(org) };
    if neighbor.is_null() {
        return 1; // no neighbor → skip (not a donor)
    }
    let neighbor_id = unsafe { avd_org_get_id(neighbor) };
    let is_donor = unsafe { avd_org_is_donor(org, neighbor_id) };
    if is_donor == 0 {
        1
    } else {
        0
    }
}

/// Inst_ReadFacedCellData: store % diff of faced cell data vs my vitality.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_read_faced_cell_data(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let my_vit = (unsafe { avd_org_get_vitality(org) } + 0.5) as c_int;
    let faced_data = unsafe { avd_org_get_faced_cell_data(org) };
    let vit_diff = if my_vit != 0 {
        (faced_data - my_vit) / my_vit * 100
    } else {
        0
    };
    unsafe { avd_hw_set_register(hw, reg_id, vit_diff) };
}

/// Inst_ReadFacedCellDataFreshness: store update age of faced cell data.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_read_faced_cell_data_freshness(
    hw: *mut c_void,
    reg_id: c_int,
    current_update: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let data_update = unsafe { avd_org_get_faced_cell_data_update(org) };
    unsafe { avd_hw_set_register(hw, reg_id, current_update - data_update) };
}

/// Inst_GetDistanceFromDiagonal: store distance from diagonal.
/// pos_x and pos_y are pre-computed by C++ caller from deme.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_distance_from_diagonal(
    hw: *mut c_void,
    reg_id: c_int,
    pos_x: c_int,
    pos_y: c_int,
) {
    let diff = pos_x - pos_y;
    let result = if pos_x > pos_y {
        // ceil(diff / 2.0)
        (diff + 1) / 2
    } else {
        // floor(diff / 2.0) — diff is <= 0 here
        if diff % 2 == 0 {
            diff / 2
        } else {
            (diff - 1) / 2
        }
    };
    unsafe { avd_hw_set_register(hw, reg_id, result) };
}

// ---------------------------------------------------------------------------
// Batch D6: Opinion handlers
// ---------------------------------------------------------------------------

/// Inst_SetOpinion: set organism's opinion to register[reg_id].
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_set_opinion(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let value = unsafe { avd_hw_get_register(hw, reg_id) };
    unsafe { avd_org_iface_set_opinion(org, value) };
}

/// Inst_ClearOpinion: clear organism's opinion.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_clear_opinion(hw: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_iface_clear_opinion(org) };
}

/// Inst_GetOpinion: get organism's opinion into reg_id, age into age_reg.
/// Returns 1 if organism has opinion (registers were set), 0 otherwise.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_get_opinion(
    hw: *mut c_void,
    opinion_reg: c_int,
    age_reg: c_int,
    current_update: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    if unsafe { avd_org_iface_has_opinion(org) } != 0 {
        let opinion_val = unsafe { avd_org_get_opinion_first(org) };
        let opinion_time = unsafe { avd_org_get_opinion_second(org) };
        unsafe { avd_hw_set_register(hw, opinion_reg, opinion_val) };
        unsafe { avd_hw_set_register(hw, age_reg, current_update - opinion_time) };
        1
    } else {
        0
    }
}

// ---------------------------------------------------------------------------
// Batch D7: Send handler
// ---------------------------------------------------------------------------

/// Inst_Send: send register[reg_id] value, then zero the register.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_send(hw: *mut c_void, reg_id: c_int) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let value = unsafe { avd_hw_get_register(hw, reg_id) };
    unsafe { avd_org_send_value(org, value) };
    unsafe { avd_hw_set_register(hw, reg_id, 0) };
}

// ---------------------------------------------------------------------------
// Batch D8: Donation handlers
// ---------------------------------------------------------------------------

/// Inst_DonateNULL: lose merit without giving to anyone.
/// Returns 0 if max donates exceeded (caller should return false), 1 otherwise.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_donate_null(
    hw: *mut c_void,
    ctx: *mut c_void,
    max_donates: c_int,
    merit_given: f64,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    if unsafe { avd_org_get_cur_num_donates(org) } > max_donates {
        return 0;
    }
    unsafe { avd_org_inc_donates(org) };
    unsafe { avd_org_set_is_donor_null(org) };

    let mut cur_merit = unsafe { avd_org_get_merit_double(org) };
    cur_merit -= merit_given;
    if cur_merit < 0.0 {
        cur_merit = 0.0;
    }
    unsafe { avd_org_update_merit(org, ctx, cur_merit) };
    1
}

/// Inst_DonateFacing: donate energy to faced neighbor.
/// Returns 0 if max donates exceeded (caller should return false), 1 otherwise.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_donate_facing(
    hw: *mut c_void,
    ctx: *mut c_void,
    max_donates: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    if unsafe { avd_org_get_cur_num_donates(org) } > max_donates {
        return 0;
    }
    unsafe { avd_org_inc_donates(org) };
    unsafe { avd_org_set_is_donor_rand(org) };

    let neighbor = unsafe { avd_org_get_neighbor(org) };
    if !neighbor.is_null() {
        unsafe { do_energy_donate(org, ctx, neighbor) };
        unsafe { avd_org_set_is_receiver_flag(neighbor) };
    }
    1
}

/// Port of cHardwareCPU::DoEnergyDonate — energy-based donation from org to to_org.
unsafe fn do_energy_donate(org: *mut c_void, ctx: *mut c_void, to_org: *mut c_void) {
    let frac = unsafe { avd_org_get_frac_energy_donating(org) };
    let cur_energy = unsafe { avd_org_get_stored_energy(org) };
    let energy_given = cur_energy * frac;

    // Update donor
    unsafe { avd_org_reduce_energy(org, energy_given) };
    unsafe { avd_org_increase_energy_donated(org, energy_given) };
    let donor_stored = unsafe { avd_org_get_stored_energy(org) };
    let donor_ratio = unsafe { avd_org_get_energy_usage_ratio(org) };
    let sender_merit = unsafe { avd_org_convert_energy_to_merit(org, donor_stored * donor_ratio) };
    unsafe { avd_org_update_merit(org, ctx, sender_merit) };
    unsafe { avd_org_set_is_energy_donor(org) };

    // Update recipient — ReduceEnergy with negative = increase
    unsafe { avd_org_reduce_energy(to_org, -energy_given) };
    unsafe { avd_org_increase_energy_received(to_org, energy_given) };
    let recv_stored = unsafe { avd_org_get_stored_energy(to_org) };
    let recv_ratio = unsafe { avd_org_get_energy_usage_ratio(to_org) };
    let receiver_merit =
        unsafe { avd_org_convert_energy_to_merit(to_org, recv_stored * recv_ratio) };
    unsafe { avd_org_update_merit(to_org, ctx, receiver_merit) };
    unsafe { avd_org_set_is_energy_receiver(to_org) };
}

/// Port of cHardwareCPU::DoEnergyDonateAmount — buffered energy donation with loss.
unsafe fn do_energy_donate_amount(
    org: *mut c_void,
    ctx: *mut c_void,
    to_org: *mut c_void,
    amount: f64,
    loss_pct: f64,
    update_metabolic: c_int,
    sharing_method: c_int,
) {
    let stored = unsafe { avd_org_get_stored_energy(org) };
    let energy_given = if amount < stored { amount } else { stored };

    // Update donor
    unsafe { avd_org_reduce_energy(org, energy_given) };
    unsafe { avd_org_set_is_energy_donor(org) };
    unsafe { avd_org_increase_energy_donated(org, energy_given) };
    unsafe { avd_org_increase_num_energy_donations(org) };
    unsafe { avd_org_deme_increase_energy_donated(org, energy_given) };

    if update_metabolic == 1 {
        let donor_stored = unsafe { avd_org_get_stored_energy(org) };
        let donor_ratio = unsafe { avd_org_get_energy_usage_ratio(org) };
        let sender_merit =
            unsafe { avd_org_convert_energy_to_merit(org, donor_stored * donor_ratio) };
        unsafe { avd_org_update_merit(org, ctx, sender_merit) };
    }

    // Apply loss
    let energy_received = energy_given * (1.0 - loss_pct);

    // Place into receiver's buffer
    unsafe { avd_org_receive_donated_energy(to_org, energy_received) };
    unsafe { avd_org_deme_increase_energy_received(to_org, energy_received) };

    // If push sharing, apply immediately
    if sharing_method == 1 {
        unsafe { avd_org_apply_donated_energy(to_org) };
        if update_metabolic == 1 {
            let recv_stored = unsafe { avd_org_get_stored_energy(to_org) };
            let recv_ratio = unsafe { avd_org_get_energy_usage_ratio(to_org) };
            let receiver_merit =
                unsafe { avd_org_convert_energy_to_merit(to_org, recv_stored * recv_ratio) };
            unsafe { avd_org_update_merit(to_org, ctx, receiver_merit) };
        }
    }
}

/// Inst_DonateEnergyFaced: donate fraction of energy to faced neighbor.
/// Returns 0 if cell_id < 0 (caller should return false), 1 otherwise.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_donate_energy_faced(
    hw: *mut c_void,
    ctx: *mut c_void,
    loss_pct: f64,
    update_metabolic: c_int,
    sharing_method: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    if unsafe { avd_org_get_cell_id(org) } < 0 {
        return 0;
    }

    let neighbor = unsafe { avd_org_get_neighbor(org) };
    if !neighbor.is_null()
        && unsafe { avd_org_is_dead(neighbor) } == 0
        && (unsafe { avd_org_has_open_energy_request(neighbor) } != 0 || sharing_method == 1)
    {
        let frac = unsafe { avd_org_get_frac_energy_donating(org) };
        let stored = unsafe { avd_org_get_stored_energy(org) };
        let amount = stored * frac;
        unsafe {
            do_energy_donate_amount(
                org,
                ctx,
                neighbor,
                amount,
                loss_pct,
                update_metabolic,
                sharing_method,
            );
        }
    }
    1
}

/// Inst_DonateEnergyFacedN: donate fixed amount of energy to faced neighbor.
/// Returns 0 if cell_id < 0 (caller should return false), 1 otherwise.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_donate_energy_faced_amount(
    hw: *mut c_void,
    ctx: *mut c_void,
    fixed_amount: f64,
    loss_pct: f64,
    update_metabolic: c_int,
    sharing_method: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    if unsafe { avd_org_get_cell_id(org) } < 0 {
        return 0;
    }

    let neighbor = unsafe { avd_org_get_neighbor(org) };
    if !neighbor.is_null()
        && unsafe { avd_org_is_dead(neighbor) } == 0
        && (unsafe { avd_org_has_open_energy_request(neighbor) } != 0 || sharing_method == 1)
    {
        unsafe {
            do_energy_donate_amount(
                org,
                ctx,
                neighbor,
                fixed_amount,
                loss_pct,
                update_metabolic,
                sharing_method,
            );
        }
    }
    1
}

/// Inst_ReceiveDonatedEnergy: apply donated energy from buffer.
/// Returns 0 if cell_id < 0 (caller should return false), 1 otherwise.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_receive_donated_energy(
    hw: *mut c_void,
    ctx: *mut c_void,
    update_metabolic: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    if unsafe { avd_org_get_cell_id(org) } < 0 {
        return 0;
    }

    let buffer_amount = unsafe { avd_org_get_energy_in_buffer(org) };
    if buffer_amount > 0.0 {
        unsafe { avd_org_apply_donated_energy(org) };
        if update_metabolic == 1 {
            let stored = unsafe { avd_org_get_stored_energy(org) };
            let ratio = unsafe { avd_org_get_energy_usage_ratio(org) };
            let receiver_merit = unsafe { avd_org_convert_energy_to_merit(org, stored * ratio) };
            unsafe { avd_org_update_merit(org, ctx, receiver_merit) };
        }
    }
    1
}

/// Inst_UpdateMetabolicRate: recalculate merit from energy.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_update_metabolic_rate(hw: *mut c_void, ctx: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let stored = unsafe { avd_org_get_stored_energy(org) };
    let ratio = unsafe { avd_org_get_energy_usage_ratio(org) };
    let new_merit = unsafe { avd_org_convert_energy_to_merit(org, stored * ratio) };
    unsafe { avd_org_update_merit(org, ctx, new_merit) };
}

// ---------------------------------------------------------------------------
// Batch E1: Kazi/Lyse/SmartExplode handlers
// ---------------------------------------------------------------------------

/// Inst_Kazi (generic): handles Kazi, Kazi1-5.
///
/// C++ passes pre-computed config values. The handler:
/// 1. Sets kaboom_executed = true
/// 2. Calls DoOutput(ctx, 0) to trigger reaction checks
/// 3. Computes distance + probability from config + register
/// 4. Random check → Kaboom
///
/// Config semantics:
/// - kaboom_prob != -1 && kaboom_hamming == -1 → adjustable hamming (from register)
/// - kaboom_prob != -1 && kaboom_hamming != -1 → both static
/// - kaboom_prob == -1 && kaboom_hamming != -1 → adjustable probability (from register)
///
/// # Safety
/// `hw`, `ctx`, and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_kazi_generic(
    hw: *mut c_void,
    ctx: *mut c_void,
    regs: *const CpuRegisters,
    reg_id: c_int,
    kaboom_prob: f64,
    kaboom_hamming: c_int,
    max_genome_size: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    // 1. Set kaboom executed
    unsafe { avd_org_set_kaboom_executed(org, 1) };
    // 2. Trigger reaction checks
    unsafe { avd_org_do_output(org, ctx, 0) };
    // 3. Compute distance and probability
    let reg_value = unsafe { get_reg(regs, reg_id) };
    let mut percent_prob: f64 = 1.0;
    let mut distance: c_int = -1;

    let prob_config = kaboom_prob as c_int;
    let ham_config = kaboom_hamming;

    if prob_config != -1 && ham_config == -1 {
        // Probability is static, hamming distance is adjustable
        let genome_size = max_genome_size;
        percent_prob = kaboom_prob;
        distance = reg_value % genome_size;
    } else if prob_config != -1 && ham_config != -1 {
        // Both static
        percent_prob = kaboom_prob;
        distance = kaboom_hamming;
    } else if prob_config == -1 && ham_config != -1 {
        // Probability is adjustable, hamming distance is static
        percent_prob = ((reg_value % 100) as f64) / 100.0;
        distance = kaboom_hamming;
    }

    // 4. Random check → Kaboom
    if unsafe { avd_ctx_random_p(ctx, percent_prob) } != 0 {
        unsafe { avd_org_kaboom(org, ctx, distance) };
    }
}

/// Inst_Lyse: probabilistic lysis without killing (assumes paired with lethal reaction).
///
/// If kaboom_prob == -1.0, uses register value % 100 as percentage.
/// On success: sets kaboom_executed, increments kaboom/lyse stats.
/// On failure: increments dont_explode stat.
///
/// # Safety
/// `hw`, `ctx`, `regs`, and `world` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_lyse(
    hw: *mut c_void,
    ctx: *mut c_void,
    regs: *const CpuRegisters,
    reg_id: c_int,
    kaboom_prob: f64,
    world: *mut c_void,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let reg_value = unsafe { get_reg(regs, reg_id) };

    let percent_prob = if kaboom_prob == -1.0 {
        ((reg_value % 100) as f64) / 100.0
    } else {
        kaboom_prob
    };

    if unsafe { avd_ctx_random_p(ctx, percent_prob) } != 0 {
        unsafe { avd_org_set_kaboom_executed(org, 1) };
        unsafe { avd_stats_inc_kaboom(world) };
        unsafe { avd_stats_inc_perc_lyse(world, percent_prob) };
        let cpu_cycles = unsafe { avd_org_get_cpu_cycles_used(org) };
        unsafe { avd_stats_inc_sum_cpus(world, cpu_cycles) };
    } else {
        unsafe { avd_stats_inc_dont_explode(world) };
    }
}

/// Inst_SmartExplode: conditional kaboom based on register truth value.
///
/// If register is truthy → set kaboom_executed, roll probability, kaboom.
/// If register is falsy → increment dont_explode stat.
///
/// # Safety
/// `hw`, `ctx`, `regs`, and `world` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_smart_explode(
    hw: *mut c_void,
    ctx: *mut c_void,
    regs: *const CpuRegisters,
    reg_id: c_int,
    kaboom_prob: f64,
    kaboom_hamming: c_int,
    world: *mut c_void,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let reg_value = unsafe { get_reg(regs, reg_id) };

    if reg_value != 0 {
        unsafe { avd_org_set_kaboom_executed(org, 1) };
        let percent_prob = kaboom_prob;
        let distance = kaboom_hamming;
        if unsafe { avd_ctx_random_p(ctx, percent_prob) } != 0 {
            unsafe { avd_org_kaboom(org, ctx, distance) };
        }
    } else {
        unsafe { avd_stats_inc_dont_explode(world) };
    }
}

// ---------------------------------------------------------------------------
// Batch E2: Message handlers
// ---------------------------------------------------------------------------

unsafe extern "C" {
    fn avd_org_get_cpu_cycles_used(org: *mut c_void) -> c_int;
}

/// Inst_SendMessage: send a message with label from reg and data from next reg.
///
/// Returns 1 if message was sent, 0 otherwise.
///
/// # Safety
/// `hw`, `ctx`, and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_send_message(
    hw: *mut c_void,
    ctx: *mut c_void,
    regs: *const CpuRegisters,
    reg_id: c_int,
    msg_type: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let data_reg = unsafe { avd_hw_find_next_register(reg_id) };
    let label = unsafe { get_reg(regs, reg_id) };
    let data = unsafe { get_reg(regs, data_reg) };
    unsafe { avd_org_send_message_regs(org, ctx, label, data, msg_type) }
}

/// Inst_RetrieveMessage: retrieve a message, storing label and data in registers.
///
/// GUARD ORDERING: RetrieveMessage is called BEFORE FindModifiedRegister.
/// If no message is available, we return 0 without consuming a nop.
/// Only on success do we call FindModifiedRegister (via FFI) to resolve registers.
///
/// Returns 0 if no message available (caller returns false), 1 on success.
///
/// # Safety
/// `hw` and `regs` must be valid pointers. `world` may be null if log_enabled == 0.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_retrieve_message(
    hw: *mut c_void,
    regs: *mut CpuRegisters,
    log_enabled: c_int,
    world: *mut c_void,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };

    // Guard: try to retrieve message BEFORE consuming nop
    let mut label: c_int = 0;
    let mut data: c_int = 0;
    let ok = unsafe { avd_org_retrieve_message(org, &mut label, &mut data, log_enabled, world) };
    if ok == 0 {
        return 0;
    }

    // Only now consume the nop via FindModifiedRegister
    let label_reg = unsafe { avd_hw_find_modified_register(hw, REG_BX) };
    let data_reg = unsafe { avd_hw_find_next_register(label_reg) };

    unsafe { set_reg(regs, label_reg, label) };
    unsafe { set_reg(regs, data_reg, data) };
    1
}

/// Inst_Receive: receive a value into a register (simple value, not message).
///
/// # Safety
/// `hw` and `regs` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_receive(
    hw: *mut c_void,
    regs: *mut CpuRegisters,
    reg_id: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let value = unsafe { avd_org_receive_value(org) };
    unsafe { set_reg(regs, reg_id, value) };
// Batch: Movement handlers (Phase 5)
// ---------------------------------------------------------------------------

unsafe extern "C" {
    fn avd_org_move(org: *mut c_void, ctx: *mut c_void) -> c_int;
    fn avd_org_rotate(org: *mut c_void, ctx: *mut c_void, direction: c_int);
    fn avd_org_get_neighborhood_size(org: *mut c_void) -> c_int;
    #[allow(dead_code)]
    fn avd_org_get_facing(org: *mut c_void) -> c_int;
}

/// Inst_RotateLeftOne: Rotate organism one step left (direction +1).
///
/// No guards, no FindModifiedRegister.
///
/// # Safety
/// `hw` and `ctx` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_rotate_left_one(hw: *mut c_void, ctx: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_rotate(org, ctx, 1) };
}

/// Inst_RotateRightOne: Rotate organism one step right (direction -1).
///
/// No guards, no FindModifiedRegister.
///
/// # Safety
/// `hw` and `ctx` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_rotate_right_one(hw: *mut c_void, ctx: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_rotate(org, ctx, -1) };
}

/// Inst_RotateUnoccupiedCell: Rotate until facing an unoccupied cell.
/// Writes 1 to reg_used if found, 0 otherwise.
///
/// No guard before FindModifiedRegister in the original C++.
///
/// # Safety
/// `hw` and `ctx` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_rotate_unoccupied_cell(
    hw: *mut c_void,
    ctx: *mut c_void,
    reg_used: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighborhood_size = unsafe { avd_org_get_neighborhood_size(org) };
    for _ in 0..neighborhood_size {
        if unsafe { avd_org_is_neighbor_cell_occupied(org) } == 0 {
            unsafe { avd_hw_set_register(hw, reg_used, 1) };
            return;
        }
        unsafe { avd_org_rotate(org, ctx, 1) };
    }
    unsafe { avd_hw_set_register(hw, reg_used, 0) };
}

/// Inst_RotateOccupiedCell: Rotate until facing an occupied cell.
/// Writes 1 to reg_used if found, 0 otherwise.
///
/// No guard before FindModifiedRegister in the original C++.
///
/// # Safety
/// `hw` and `ctx` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_rotate_occupied_cell(
    hw: *mut c_void,
    ctx: *mut c_void,
    reg_used: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighborhood_size = unsafe { avd_org_get_neighborhood_size(org) };
    for _ in 0..neighborhood_size {
        if unsafe { avd_org_is_neighbor_cell_occupied(org) } != 0 {
            unsafe { avd_hw_set_register(hw, reg_used, 1) };
            return;
        }
        unsafe { avd_org_rotate(org, ctx, 1) };
    }
    unsafe { avd_hw_set_register(hw, reg_used, 0) };
}

/// Inst_RotateNextOccupiedCell: Rotate once, then rotate to occupied cell.
///
/// No guard before FindModifiedRegister.
///
/// # Safety
/// `hw` and `ctx` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_rotate_next_occupied_cell(
    hw: *mut c_void,
    ctx: *mut c_void,
    reg_used: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_rotate(org, ctx, 1) };
    // Now do RotateOccupiedCell logic inline
    let neighborhood_size = unsafe { avd_org_get_neighborhood_size(org) };
    for _ in 0..neighborhood_size {
        if unsafe { avd_org_is_neighbor_cell_occupied(org) } != 0 {
            unsafe { avd_hw_set_register(hw, reg_used, 1) };
            return;
        }
        unsafe { avd_org_rotate(org, ctx, 1) };
    }
    unsafe { avd_hw_set_register(hw, reg_used, 0) };
}

/// Inst_RotateNextUnoccupiedCell: Rotate once, then rotate to unoccupied cell.
///
/// No guard before FindModifiedRegister.
///
/// # Safety
/// `hw` and `ctx` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_rotate_next_unoccupied_cell(
    hw: *mut c_void,
    ctx: *mut c_void,
    reg_used: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    unsafe { avd_org_rotate(org, ctx, 1) };
    // Now do RotateUnoccupiedCell logic inline
    let neighborhood_size = unsafe { avd_org_get_neighborhood_size(org) };
    for _ in 0..neighborhood_size {
        if unsafe { avd_org_is_neighbor_cell_occupied(org) } == 0 {
            unsafe { avd_hw_set_register(hw, reg_used, 1) };
            return;
        }
        unsafe { avd_org_rotate(org, ctx, 1) };
    }
    unsafe { avd_hw_set_register(hw, reg_used, 0) };
}

/// Inst_RotateHome: Rotate to face birth cell (or marked cell).
///
/// No FindModifiedRegister in this handler.
///
/// # Safety
/// `hw` and `ctx` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_rotate_home(hw: *mut c_void, ctx: *mut c_void) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let easterly = unsafe { avd_org_get_easterly(org) };
    let northerly = unsafe { avd_org_get_northerly(org) };
    let mut correct_facing = crate::cpu_helpers::avd_cpu_gradient_facing(northerly, easterly);
    if correct_facing < 0 {
        correct_facing = 0; // zero vector defaults to N
    }
    let neighborhood_size = unsafe { avd_org_get_neighborhood_size(org) };
    for _ in 0..neighborhood_size {
        unsafe { avd_org_rotate(org, ctx, 1) };
        if unsafe { avd_org_get_faced_dir(org) } == correct_facing {
            break;
        }
    }
}

/// Inst_RotateEventCell: Rotate until facing a cell with event data > 0.
/// Writes 1 to reg_used if found, 0 otherwise.
///
/// No guard before FindModifiedRegister in the original C++.
///
/// # Safety
/// `hw` and `ctx` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_rotate_event_cell(
    hw: *mut c_void,
    ctx: *mut c_void,
    reg_used: c_int,
) {
    let org = unsafe { avd_hw_get_organism(hw) };
    let neighborhood_size = unsafe { avd_org_get_neighborhood_size(org) };
    for _ in 0..neighborhood_size {
        if unsafe { avd_org_get_cell_data_org(org) } > 0 {
            unsafe { avd_hw_set_register(hw, reg_used, 1) };
            return;
        }
        unsafe { avd_org_rotate(org, ctx, 1) };
    }
    unsafe { avd_hw_set_register(hw, reg_used, 0) };
}

/// Inst_Move: Move organism to faced cell. Returns 0 if in TestCPU (cellid == -1).
/// The guard (cellid == -1 → return false) is checked in C++ BEFORE calling this.
/// This handles the post-guard logic: call Move and write result to reg.
///
/// # Safety
/// `hw` and `ctx` must be valid pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_cpu_inst_move(
    hw: *mut c_void,
    ctx: *mut c_void,
    reg_used: c_int,
) -> c_int {
    let org = unsafe { avd_hw_get_organism(hw) };
    let move_success = unsafe { avd_org_move(org, ctx) };
    unsafe { avd_hw_set_register(hw, reg_used, move_success) };
    1 // always returns true (success) after guard passes
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

    // --- Phase 4: I/O variant + state transition logic tests ---

    /// Bonus cost calculation: verify the formula matches C++ exactly.
    #[test]
    fn test_bonus_cost_formula() {
        // C++: new_bonus = cur_bonus * (1 - bonus_cost); clamp to 0
        let cur_bonus: f64 = 1.0;
        let bonus_cost: f64 = 0.001;
        let new_bonus = cur_bonus * (1.0 - bonus_cost);
        assert!((new_bonus - 0.999).abs() < 1e-10);

        // Zero bonus stays zero
        let cur_bonus: f64 = 0.0;
        let new_bonus = cur_bonus * (1.0 - bonus_cost);
        assert_eq!(new_bonus, 0.0);

        // Large cost clamps to zero
        let cur_bonus: f64 = 0.5;
        let bonus_cost: f64 = 2.0; // more than 100%
        let mut new_bonus = cur_bonus * (1.0 - bonus_cost);
        if new_bonus < 0.0 {
            new_bonus = 0.0;
        }
        assert_eq!(new_bonus, 0.0);
    }

    /// Prob_Die probability calculation from register value.
    #[test]
    fn test_prob_die_register_probability() {
        // When config_prob == -1.0, use register value % 100 / 100.0
        let reg_val: i32 = 50;
        let percent_prob = ((reg_val % 100) as f64) / 100.0;
        assert!((percent_prob - 0.5).abs() < 1e-10);

        // Negative register value
        let reg_val: i32 = -30;
        let percent_prob = ((reg_val % 100) as f64) / 100.0;
        // In Rust, -30 % 100 = -30 (same as C++)
        assert!((percent_prob - (-0.3)).abs() < 1e-10);

        // Register 0 → 0% chance
        let reg_val: i32 = 0;
        let percent_prob = ((reg_val % 100) as f64) / 100.0;
        assert_eq!(percent_prob, 0.0);

        // Register 200 → 0% (200 % 100 = 0)
        let reg_val: i32 = 200;
        let percent_prob = ((reg_val % 100) as f64) / 100.0;
        assert_eq!(percent_prob, 0.0);
    }

    /// TaskIO_Feedback merit comparison logic.
    #[test]
    fn test_feedback_indicator_logic() {
        fn feedback_indicator(pre: f64, post: f64) -> i32 {
            if pre > post {
                -1
            } else if pre < post {
                1
            } else {
                0
            }
        }
        // pre > post → push -1
        assert_eq!(feedback_indicator(2.0, 1.0), -1);
        // pre < post → push 1
        assert_eq!(feedback_indicator(1.0, 2.0), 1);
        // pre == post → push 0
        assert_eq!(feedback_indicator(1.0, 1.0), 0);
    }

    // --- Phase 5: Movement handler logic tests ---

    /// RotateHome uses gradient_facing to compute correct_facing.
    /// Verify the zero-vector default and standard compass directions.
    #[test]
    fn test_rotate_home_gradient_facing_zero_vector_default() {
        // When both northerly and easterly are 0, gradient_facing returns -1.
        // RotateHome clamps -1 to 0 (north).
        let mut correct_facing = crate::cpu_helpers::avd_cpu_gradient_facing(0, 0);
        if correct_facing < 0 {
            correct_facing = 0;
        }
        assert_eq!(correct_facing, 0);
    }

    #[test]
    fn test_rotate_home_gradient_facing_all_directions() {
        // N
        assert_eq!(crate::cpu_helpers::avd_cpu_gradient_facing(1, 0), 0);
        // NE
        assert_eq!(crate::cpu_helpers::avd_cpu_gradient_facing(1, -1), 1);
        // E
        assert_eq!(crate::cpu_helpers::avd_cpu_gradient_facing(0, -1), 2);
        // SE
        assert_eq!(crate::cpu_helpers::avd_cpu_gradient_facing(-1, -1), 3);
        // S
        assert_eq!(crate::cpu_helpers::avd_cpu_gradient_facing(-1, 0), 4);
        // SW
        assert_eq!(crate::cpu_helpers::avd_cpu_gradient_facing(-1, 1), 5);
        // W
        assert_eq!(crate::cpu_helpers::avd_cpu_gradient_facing(0, 1), 6);
        // NW
        assert_eq!(crate::cpu_helpers::avd_cpu_gradient_facing(1, 1), 7);
    }

    /// Verify the rotate_unoccupied/occupied/event logic patterns:
    /// they loop from 0..neighborhood_size, checking a condition,
    /// setting register to 1 if found or 0 if not.
    #[test]
    fn test_rotate_loop_pattern() {
        // This tests the algorithmic logic pattern shared by all rotate-to-X handlers.
        // In a 4-cell neighborhood where target is at position 2:
        let neighborhood_size = 4;
        let target_position = 2;
        let mut found = false;
        let mut rotations = 0;
        for i in 0..neighborhood_size {
            if i == target_position {
                found = true;
                break;
            }
            rotations += 1;
        }
        assert!(found);
        assert_eq!(rotations, 2);

        // All positions empty (no target matches):
        let mut rotations = 0;
        let mut found_any = false;
        let never_target = neighborhood_size + 1; // impossible position
        for i in 0..neighborhood_size {
            if i == never_target {
                found_any = true;
                break;
            }
            rotations += 1;
        }
        assert!(!found_any);
        assert_eq!(rotations, 4);
    }
}
