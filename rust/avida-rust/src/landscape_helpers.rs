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
}
