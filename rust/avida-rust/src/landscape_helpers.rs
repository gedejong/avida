use std::ffi::c_int;

// --- Fitness mutation category ---
const LANDSCAPE_DEAD: c_int = 0;
const LANDSCAPE_NEGATIVE: c_int = 1;
const LANDSCAPE_NEUTRAL: c_int = 2;
const LANDSCAPE_POSITIVE: c_int = 3;

/// Classify a mutation's fitness effect.
/// Returns: 0=dead, 1=negative, 2=neutral, 3=positive.
#[no_mangle]
pub extern "C" fn avd_landscape_fitness_category(
    fitness: f64,
    neut_min: f64,
    neut_max: f64,
) -> c_int {
    if fitness == 0.0 {
        LANDSCAPE_DEAD
    } else if fitness < neut_min {
        LANDSCAPE_NEGATIVE
    } else if fitness <= neut_max {
        LANDSCAPE_NEUTRAL
    } else {
        LANDSCAPE_POSITIVE
    }
}

// --- Epistasis category ---
const LANDSCAPE_EPI_DEAD: c_int = 0;
const LANDSCAPE_EPI_NEGATIVE: c_int = 1;
const LANDSCAPE_EPI_POSITIVE: c_int = 2;
const LANDSCAPE_EPI_NONE: c_int = 3;

/// Classify epistasis type from single and combo fitness values.
/// Returns: 0=dead, 1=antagonistic, 2=synergistic, 3=no epistasis.
#[no_mangle]
pub extern "C" fn avd_landscape_epistasis_category(
    mut1_fitness: f64,
    mut2_fitness: f64,
    combo_fitness: f64,
) -> c_int {
    let mult_combo = mut1_fitness * mut2_fitness;
    if (mut1_fitness == 0.0 || mut2_fitness == 0.0) && combo_fitness == 0.0 {
        LANDSCAPE_EPI_DEAD
    } else if combo_fitness < mult_combo {
        LANDSCAPE_EPI_NEGATIVE
    } else if combo_fitness > mult_combo {
        LANDSCAPE_EPI_POSITIVE
    } else {
        LANDSCAPE_EPI_NONE
    }
}

// ---------------------------------------------------------------------------
// Entropy / complexity computation
// ---------------------------------------------------------------------------

/// Compute total entropy and complexity from site_count array.
///
/// `total_entropy = sum(log(site_count[i] + 1) / max_entropy)` for i in 0..genome_size.
/// `complexity = genome_size - total_entropy`.
///
/// Returns: (total_entropy, complexity) written to the output pointers.
///
/// # Safety
/// `site_count` must point to at least `genome_size` valid `c_int` values.
/// `out_entropy` and `out_complexity` must be valid writable pointers.
#[no_mangle]
pub unsafe extern "C" fn avd_landscape_entropy_complexity(
    site_count: *const c_int,
    genome_size: c_int,
    inst_set_size: c_int,
    out_entropy: *mut f64,
    out_complexity: *mut f64,
) {
    if site_count.is_null() || out_entropy.is_null() || out_complexity.is_null() {
        return;
    }
    if genome_size <= 0 || inst_set_size <= 0 {
        // SAFETY: pointers verified non-null above.
        unsafe {
            *out_entropy = 0.0;
            *out_complexity = genome_size as f64;
        }
        return;
    }

    let max_ent = (inst_set_size as f64).ln();
    if max_ent <= 0.0 {
        // SAFETY: pointers verified non-null above.
        unsafe {
            *out_entropy = 0.0;
            *out_complexity = genome_size as f64;
        }
        return;
    }

    // SAFETY: caller guarantees site_count points to genome_size valid ints.
    let counts = unsafe { std::slice::from_raw_parts(site_count, genome_size as usize) };
    let mut entropy = 0.0f64;
    for &count in counts {
        entropy += ((count as f64 + 1.0).ln()) / max_ent;
    }

    // SAFETY: pointers verified non-null above.
    unsafe {
        *out_entropy = entropy;
        *out_complexity = genome_size as f64 - entropy;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fitness_category_classification() {
        assert_eq!(
            avd_landscape_fitness_category(0.0, 0.9, 1.1),
            LANDSCAPE_DEAD
        );
        assert_eq!(
            avd_landscape_fitness_category(0.5, 0.9, 1.1),
            LANDSCAPE_NEGATIVE
        );
        assert_eq!(
            avd_landscape_fitness_category(1.0, 0.9, 1.1),
            LANDSCAPE_NEUTRAL
        );
        assert_eq!(
            avd_landscape_fitness_category(2.0, 0.9, 1.1),
            LANDSCAPE_POSITIVE
        );
        // boundary: exactly neut_min is neutral
        assert_eq!(
            avd_landscape_fitness_category(0.9, 0.9, 1.1),
            LANDSCAPE_NEUTRAL
        );
        // boundary: exactly neut_max is neutral
        assert_eq!(
            avd_landscape_fitness_category(1.1, 0.9, 1.1),
            LANDSCAPE_NEUTRAL
        );
    }

    #[test]
    fn epistasis_category_classification() {
        // Both dead → dead epistasis
        assert_eq!(
            avd_landscape_epistasis_category(0.0, 1.0, 0.0),
            LANDSCAPE_EPI_DEAD
        );
        assert_eq!(
            avd_landscape_epistasis_category(1.0, 0.0, 0.0),
            LANDSCAPE_EPI_DEAD
        );
        // combo < mult → antagonistic
        assert_eq!(
            avd_landscape_epistasis_category(0.8, 0.9, 0.5),
            LANDSCAPE_EPI_NEGATIVE
        );
        // combo > mult → synergistic
        assert_eq!(
            avd_landscape_epistasis_category(0.8, 0.9, 0.9),
            LANDSCAPE_EPI_POSITIVE
        );
        // combo == mult → no epistasis
        let m1 = 0.8;
        let m2 = 0.5;
        assert_eq!(
            avd_landscape_epistasis_category(m1, m2, m1 * m2),
            LANDSCAPE_EPI_NONE
        );
    }

    #[test]
    fn entropy_basic() {
        // 3 sites, all with count 0 → log(1)/log(26) = 0 each → entropy 0
        let counts = [0i32, 0, 0];
        let mut entropy = 0.0f64;
        let mut complexity = 0.0f64;
        // SAFETY: all pointers are valid stack-allocated variables.
        unsafe {
            avd_landscape_entropy_complexity(counts.as_ptr(), 3, 26, &mut entropy, &mut complexity);
        }
        assert!((entropy - 0.0).abs() < 1e-10);
        assert!((complexity - 3.0).abs() < 1e-10);
    }

    #[test]
    fn entropy_all_sites_max() {
        // 2 sites, each with count = inst_set_size - 1 (25 for inst_set=26)
        // Each: log(26) / log(26) = 1.0
        let counts = [25i32, 25];
        let mut entropy = 0.0f64;
        let mut complexity = 0.0f64;
        // SAFETY: all pointers are valid stack-allocated variables.
        unsafe {
            avd_landscape_entropy_complexity(counts.as_ptr(), 2, 26, &mut entropy, &mut complexity);
        }
        assert!((entropy - 2.0).abs() < 1e-10);
        assert!((complexity - 0.0).abs() < 1e-10);
    }
}
