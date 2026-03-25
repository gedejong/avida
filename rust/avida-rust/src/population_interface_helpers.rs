//! Deterministic policy helpers extracted from `cPopulationInterface.cc`.
//!
//! Pure-computation functions for avatar geometry, message drop policy,
//! and rotation normalization. C++ retains ownership and side effects.

use std::ffi::{c_int, c_void};

// FFI declaration for RNG access (defined in cHardwareFFI.cc).
// In test builds, provide a stub to avoid linker errors.
#[cfg(not(test))]
unsafe extern "C" {
    fn avd_ctx_random_get_int(ctx: *mut c_void, max: c_int) -> c_int;
}

#[cfg(test)]
unsafe extern "C" fn avd_ctx_random_get_int(_ctx: *mut c_void, _max: c_int) -> c_int {
    0 // Stub: tests should not reach RNG-dependent code paths
}

// ---------------------------------------------------------------------------
// Avatar geometry: SetAVFacedCellID
// ---------------------------------------------------------------------------

/// World geometry constants matching Avida's WORLD_GEOMETRY config.
const GEOM_BOUNDED: c_int = 1;
const GEOM_TORUS: c_int = 2;

/// Compute the faced cell ID for an avatar given its current position and facing.
///
/// Handles bounded grid edge/corner cases (with random tiebreaking) and
/// toroidal wrapping. Returns the new cell ID.
///
/// `ctx`: cAvidaContext pointer for RNG access (only used for edge tiebreaking).
///
/// # Safety
/// `ctx` must be a valid cAvidaContext pointer.
#[no_mangle]
pub unsafe extern "C" fn avd_popif_av_faced_cell(
    old_cell_id: c_int,
    facing: c_int,
    x_size: c_int,
    y_size: c_int,
    num_demes: c_int,
    world_geometry: c_int,
    ctx: *mut c_void,
) -> c_int {
    let deme_y_size = y_size / num_demes.max(1);
    let deme_size = x_size * deme_y_size;
    if deme_size <= 0 {
        return old_cell_id;
    }

    // Single-cell deme: face self
    if deme_size == 1 {
        return old_cell_id;
    }

    let deme_id = old_cell_id / deme_size;
    let old_deme_cell = old_cell_id % deme_size;
    let mut x = old_deme_cell % x_size;
    let mut y = old_deme_cell / x_size;

    let mut off_edge = false;

    // Bounded geometry edge handling
    if world_geometry == GEOM_BOUNDED {
        // Single column
        if x_size == 1 {
            if y == 0 {
                y += 1;
                off_edge = true;
            } else if y == deme_y_size - 1 {
                y -= 1;
                off_edge = true;
            }
        // Single row
        } else if deme_y_size == 1 {
            if x == 0 {
                x += 1;
                off_edge = true;
            } else if x == x_size - 1 {
                x -= 1;
                off_edge = true;
            }
        }

        if !off_edge {
            // SAFETY: ctx is a valid cAvidaContext pointer per caller contract.
            off_edge =
                unsafe { bounded_edge_adjust(&mut x, &mut y, x_size, deme_y_size, facing, ctx) };
        }
    }

    // Torus geometry or non-edge bounded cell: apply facing delta
    if !off_edge || world_geometry == GEOM_TORUS {
        // North component
        if facing == 0 || facing == 1 || facing == 7 {
            y = (y - 1 + deme_y_size) % deme_y_size;
        }
        // South component
        if facing == 3 || facing == 4 || facing == 5 {
            y = (y + 1) % deme_y_size;
        }
        // East component
        if facing == 1 || facing == 2 || facing == 3 {
            x = (x + 1) % x_size;
        }
        // West component
        if facing == 5 || facing == 6 || facing == 7 {
            x = (x - 1 + x_size) % x_size;
        }
    }

    let new_deme_cell = y * x_size + x;
    deme_id * deme_size + new_deme_cell
}

/// Handle bounded grid edge/corner cases.
/// Returns true if facing was adjusted (off-the-edge), false otherwise.
///
/// # Safety
/// `ctx` must be a valid cAvidaContext pointer.
unsafe fn bounded_edge_adjust(
    x: &mut c_int,
    y: &mut c_int,
    x_size: c_int,
    y_size: c_int,
    facing: c_int,
    ctx: *mut c_void,
) -> bool {
    // Lazy RNG: only call when needed for a tiebreak
    let rb = || -> bool {
        // SAFETY: ctx is valid per caller contract.
        unsafe { avd_ctx_random_get_int(ctx, 2) != 0 }
    };

    // West edge
    if *x == 0 {
        // Northwest corner
        if *y == 0 {
            if facing == 0 || facing == 7 || facing == 6 {
                if rb() {
                    *x += 1;
                } else {
                    *y += 1;
                }
                return true;
            } else if facing == 5 {
                *y += 1;
                return true;
            } else if facing == 1 {
                *x += 1;
                return true;
            }
        }
        // Southwest corner
        else if *y == y_size - 1 {
            if facing == 4 || facing == 5 || facing == 6 {
                if rb() {
                    *x += 1;
                } else {
                    *y -= 1;
                }
                return true;
            } else if facing == 7 {
                *x += 1;
                return true;
            } else if facing == 3 {
                *y -= 1;
                return true;
            }
        }
        // West edge (not corner)
        if facing == 5 {
            *y -= 1;
            return true;
        } else if facing == 6 {
            if rb() {
                *y += 1;
            } else {
                *y -= 1;
            }
            return true;
        } else if facing == 7 {
            *y += 1;
            return true;
        }
    }
    // East edge
    else if *x == x_size - 1 {
        // Northeast corner
        if *y == 0 {
            if facing == 0 || facing == 1 || facing == 2 {
                if rb() {
                    *x -= 1;
                } else {
                    *y += 1;
                }
                return true;
            }
            if facing == 3 {
                *y += 1;
                return true;
            }
            if facing == 7 {
                *x -= 1;
                return true;
            }
        }
        // Southeast corner
        else if *y == y_size - 1 {
            if facing == 2 || facing == 3 || facing == 4 {
                if rb() {
                    *x -= 1;
                } else {
                    *y -= 1;
                }
                return true;
            } else if facing == 1 {
                *y -= 1;
                return true;
            } else if facing == 5 {
                *x -= 1;
                return true;
            }
        }
        // East edge (not corner)
        if facing == 1 {
            *y -= 1;
            return true;
        } else if facing == 2 {
            if rb() {
                *y += 1;
            } else {
                *y -= 1;
            }
            return true;
        } else if facing == 3 {
            *y += 1;
            return true;
        }
    }
    // North edge (not corner — corners handled above)
    else if *y == 0 {
        if facing == 7 {
            *x -= 1;
            return true;
        } else if facing == 0 {
            if rb() {
                *x += 1;
            } else {
                *x -= 1;
            }
            return true;
        } else if facing == 1 {
            *x += 1;
            return true;
        }
    }
    // South edge (not corner)
    else if *y == y_size - 1 {
        if facing == 3 {
            *x += 1;
            return true;
        } else if facing == 4 {
            if rb() {
                *x += 1;
            } else {
                *x -= 1;
            }
            return true;
        } else if facing == 5 {
            *x -= 1;
            return true;
        }
    }

    false
}

// ---------------------------------------------------------------------------
// GetAVNumNeighbors: bounded grid neighbor count
// ---------------------------------------------------------------------------

/// Compute the number of neighbors for a cell in a bounded or toroidal grid.
/// Returns 3 (corner), 5 (edge), or 8 (interior/torus).
///
/// `world_geometry`: 1=bounded, 2=torus.
#[no_mangle]
pub extern "C" fn avd_popif_av_num_neighbors(
    cell_id: c_int,
    x_size: c_int,
    y_size: c_int,
    num_demes: c_int,
    world_geometry: c_int,
) -> c_int {
    // Torus: always 8 neighbors
    if world_geometry == GEOM_TORUS {
        return 8;
    }

    let deme_y_size = y_size / num_demes.max(1);
    let deme_size = x_size * deme_y_size;
    if deme_size <= 0 {
        return 0;
    }

    let deme_cell = cell_id % deme_size;
    let x = deme_cell % x_size;
    let y = deme_cell / x_size;

    let on_x_edge = x == 0 || x == x_size - 1;
    let on_y_edge = y == 0 || y == deme_y_size - 1;

    if on_x_edge && on_y_edge {
        3 // corner
    } else if on_x_edge || on_y_edge {
        5 // edge
    } else {
        8 // interior
    }
}

// ---------------------------------------------------------------------------
// RotateAV increment normalization
// ---------------------------------------------------------------------------

/// Normalize a rotation increment to [-7, 7] range and compute new facing.
/// Returns the new facing direction (0-7).
#[no_mangle]
pub extern "C" fn avd_popif_rotate_av(current_facing: c_int, increment: c_int) -> c_int {
    let norm = if increment >= 0 {
        increment % 8
    } else {
        -((-increment) % 8)
    };
    (current_facing + norm + 8) % 8
}

// ---------------------------------------------------------------------------
// Message send policy: should_drop / should_lose
// ---------------------------------------------------------------------------

/// Evaluate message send policy: returns a bitmask.
/// Bit 0 (1): message dropped (random drop probability hit)
/// Bit 1 (2): message lost (no valid recipient)
///
/// `drop_hit`: 1 if the random drop trial succeeded (caller does ctx.GetRandom().P(drop_prob))
/// `cell_occupied`: 1 if the target cell has an organism
/// `neural_networking`: 1 if NEURAL_NETWORKING config is on
/// `use_avatars_2`: 1 if USE_AVATARS == 2
/// `num_av_inputs`: number of input avatars in target cell
/// `self_communication`: 1 if SELF_COMMUNICATION config is on
/// `has_other_input_av`: 1 if target cell has at least one input avatar from a different organism
#[no_mangle]
pub extern "C" fn avd_popif_message_send_policy(
    drop_hit: c_int,
    cell_occupied: c_int,
    neural_networking: c_int,
    use_avatars_2: c_int,
    num_av_inputs: c_int,
    self_communication: c_int,
    has_other_input_av: c_int,
) -> c_int {
    let mut result = 0;

    // Dropped?
    if drop_hit != 0 {
        result |= 1;
    }

    // Lost (no valid recipient)?
    if neural_networking == 0 || use_avatars_2 == 0 {
        // Standard mode: lost if cell not occupied
        if cell_occupied == 0 {
            result |= 2;
        }
    } else {
        // Neural networking with avatars: lost if no input avatars
        if num_av_inputs == 0 {
            result |= 2;
        } else if self_communication == 0 && has_other_input_av == 0 {
            // No self-communication allowed and no other organism's avatars
            result |= 2;
        }
    }

    result
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torus_facing_north() {
        // 10x10 world, 1 deme, cell at (5,5)=55, facing north (0)
        // SAFETY: torus geometry never calls RNG, so null ctx is safe.
        let result =
            unsafe { avd_popif_av_faced_cell(55, 0, 10, 10, 1, GEOM_TORUS, std::ptr::null_mut()) };
        assert_eq!(result, 45); // (5, 4) = 45
    }

    #[test]
    fn test_torus_facing_south() {
        // SAFETY: torus geometry never calls RNG.
        let result =
            unsafe { avd_popif_av_faced_cell(55, 4, 10, 10, 1, GEOM_TORUS, std::ptr::null_mut()) };
        assert_eq!(result, 65); // (5, 6) = 65
    }

    #[test]
    fn test_torus_wrap_north() {
        // Cell at (5,0)=5, facing north → wraps to (5,9)=95
        // SAFETY: torus geometry never calls RNG.
        let result =
            unsafe { avd_popif_av_faced_cell(5, 0, 10, 10, 1, GEOM_TORUS, std::ptr::null_mut()) };
        assert_eq!(result, 95);
    }

    #[test]
    fn test_torus_wrap_west() {
        // Cell at (0,5)=50, facing west (6) → wraps to (9,5)=59
        // SAFETY: torus geometry never calls RNG.
        let result =
            unsafe { avd_popif_av_faced_cell(50, 6, 10, 10, 1, GEOM_TORUS, std::ptr::null_mut()) };
        assert_eq!(result, 59);
    }

    // Note: bounded edge tests with RNG are validated by C++ integration tests
    // (avatars, avatars-pred_look). Cannot unit-test here without a real ctx.

    #[test]
    fn test_single_cell_deme() {
        // SAFETY: single-cell deme returns immediately, no RNG.
        let result =
            unsafe { avd_popif_av_faced_cell(0, 3, 1, 1, 1, GEOM_TORUS, std::ptr::null_mut()) };
        assert_eq!(result, 0);
    }

    #[test]
    fn test_rotate_av_positive() {
        assert_eq!(avd_popif_rotate_av(3, 2), 5);
    }

    #[test]
    fn test_rotate_av_negative() {
        assert_eq!(avd_popif_rotate_av(3, -2), 1);
    }

    #[test]
    fn test_rotate_av_wrap() {
        assert_eq!(avd_popif_rotate_av(6, 5), 3);
    }

    #[test]
    fn test_rotate_av_large_increment() {
        assert_eq!(avd_popif_rotate_av(0, 17), (17 % 8));
    }

    #[test]
    fn test_message_policy_no_drop_occupied() {
        assert_eq!(avd_popif_message_send_policy(0, 1, 0, 0, 0, 0, 0), 0); // no drop, occupied → success
    }

    #[test]
    fn test_message_policy_dropped() {
        assert_eq!(avd_popif_message_send_policy(1, 1, 0, 0, 0, 0, 0), 1); // dropped
    }

    #[test]
    fn test_message_policy_lost_unoccupied() {
        assert_eq!(avd_popif_message_send_policy(0, 0, 0, 0, 0, 0, 0), 2); // lost (unoccupied)
    }

    #[test]
    fn test_message_policy_neural_no_inputs() {
        assert_eq!(avd_popif_message_send_policy(0, 1, 1, 1, 0, 0, 0), 2); // neural networking, no input AVs → lost
    }

    #[test]
    fn test_message_policy_neural_self_only() {
        assert_eq!(avd_popif_message_send_policy(0, 1, 1, 1, 1, 0, 0), 2); // neural, has inputs but all from self, no self-comm → lost
    }

    #[test]
    fn test_message_policy_neural_other_av() {
        assert_eq!(avd_popif_message_send_policy(0, 1, 1, 1, 1, 0, 1), 0); // neural, has other organism input AV → success
    }

    // --- GetAVNumNeighbors tests ---

    #[test]
    fn test_num_neighbors_torus() {
        assert_eq!(avd_popif_av_num_neighbors(0, 10, 10, 1, GEOM_TORUS), 8);
        assert_eq!(avd_popif_av_num_neighbors(55, 10, 10, 1, GEOM_TORUS), 8);
    }

    #[test]
    fn test_num_neighbors_bounded_corner() {
        assert_eq!(avd_popif_av_num_neighbors(0, 10, 10, 1, GEOM_BOUNDED), 3); // (0,0)
        assert_eq!(avd_popif_av_num_neighbors(9, 10, 10, 1, GEOM_BOUNDED), 3); // (9,0)
        assert_eq!(avd_popif_av_num_neighbors(90, 10, 10, 1, GEOM_BOUNDED), 3); // (0,9)
        assert_eq!(avd_popif_av_num_neighbors(99, 10, 10, 1, GEOM_BOUNDED), 3); // (9,9)
    }

    #[test]
    fn test_num_neighbors_bounded_edge() {
        assert_eq!(avd_popif_av_num_neighbors(5, 10, 10, 1, GEOM_BOUNDED), 5); // (5,0) top
        assert_eq!(avd_popif_av_num_neighbors(50, 10, 10, 1, GEOM_BOUNDED), 5); // (0,5) left
    }

    #[test]
    fn test_num_neighbors_bounded_interior() {
        assert_eq!(avd_popif_av_num_neighbors(55, 10, 10, 1, GEOM_BOUNDED), 8); // (5,5)
    }
}
