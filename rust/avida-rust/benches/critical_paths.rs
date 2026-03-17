use avida_rust::bit_array::{
    avd_rba_count_bits, avd_rba_free, avd_rba_increment, avd_rba_new, avd_rba_set_bit,
    avd_rba_shift,
};
use avida_rust::cpu_helpers::{avd_cpu_dispatch_counted_opcode, avd_cpu_dispatch_family};
use avida_rust::resource_count_helpers::{
    avd_rc_accumulate_update_time, avd_rc_apply_nonspatial_steps, avd_rc_dispatch_action,
    avd_rc_is_spatial_geometry, avd_rc_num_spatial_updates, avd_rc_num_steps,
    avd_rc_remainder_update_time, avd_rc_spatial_step_iterations, avd_rc_use_cell_list_branch,
};
use avida_rust::{
    avd_pkg_double_to_string, avd_pkg_str_as_bool, avd_pkg_str_as_double, avd_pkg_str_as_int,
    avd_pkg_string_free, avd_provider_classify_id, avd_provider_string_free,
};
use bitvec::prelude::{BitVec, Lsb0};
use criterion::{criterion_group, criterion_main, Criterion};
use std::ffi::{c_char, CString};
use std::hint::black_box;

fn make_cstring(text: &str) -> CString {
    match CString::new(text) {
        Ok(s) => s,
        Err(_) => {
            let sanitized: Vec<u8> = text.bytes().filter(|b| *b != 0).collect();
            match CString::new(sanitized) {
                Ok(s) => s,
                // SAFETY: vec contains a single trailing NUL byte.
                Err(_) => unsafe { CString::from_vec_unchecked(vec![0]) },
            }
        }
    }
}

fn bench_resource_scheduling(c: &mut Criterion) {
    let mut group = c.benchmark_group("resource_scheduling_helpers");
    let step = 1.0 / 10000.0;
    group.bench_function("num_steps+remainder_pipeline", |b| {
        b.iter(|| {
            let mut update_time = black_box(0.0_f64);
            let mut checksum = black_box(0.0_f64);
            for _ in 0..4096 {
                update_time = avd_rc_accumulate_update_time(update_time, 0.37 * step);
                let steps = avd_rc_num_steps(update_time, step);
                let rem = avd_rc_remainder_update_time(update_time, step, steps);
                update_time = rem;
                checksum += rem + f64::from(steps);
            }
            black_box(checksum)
        })
    });
    group.finish();
}

fn bench_resource_update_dispatch(c: &mut Criterion) {
    let mut group = c.benchmark_group("resource_update_dispatch_helpers");
    const RES_COUNT: usize = 128;
    const PRECALC_DISTANCE: i32 = 100;
    let geometries: [i32; RES_COUNT] = {
        let mut values = [0_i32; RES_COUNT];
        let mut i = 0;
        while i < RES_COUNT {
            values[i] = match i % 3 {
                0 => 0, // global
                1 => 5, // partial
                _ => 1, // grid
            };
            i += 1;
        }
        values
    };
    let mut decay_table = [0.0_f64; PRECALC_DISTANCE as usize];
    let mut inflow_table = [0.0_f64; PRECALC_DISTANCE as usize];
    decay_table[0] = 1.0;
    inflow_table[0] = 0.0;
    for i in 1..PRECALC_DISTANCE as usize {
        decay_table[i] = decay_table[i - 1] * 0.995;
        inflow_table[i] = inflow_table[i - 1] * 0.995 + 0.0001;
    }

    group.bench_function("mixed_geometry_dispatch_pipeline", |b| {
        b.iter(|| {
            let mut checksum = black_box(0.0_f64);
            let mut update_time = black_box(0.73_f64);
            let num_steps = avd_rc_num_steps(update_time, 1.0 / 10000.0);
            update_time = avd_rc_remainder_update_time(update_time, 1.0 / 10000.0, num_steps);
            let num_spatial_updates = avd_rc_num_spatial_updates(black_box(3101), black_box(3092));
            for geometry in geometries {
                let is_spatial = avd_rc_is_spatial_geometry(black_box(geometry));
                let action = avd_rc_dispatch_action(is_spatial, black_box(0));
                if action == 1 {
                    checksum += avd_rc_apply_nonspatial_steps(
                        black_box(1.0),
                        decay_table.as_ptr(),
                        inflow_table.as_ptr(),
                        PRECALC_DISTANCE,
                        num_steps,
                    );
                } else if action == 2 {
                    let iterations = avd_rc_spatial_step_iterations(num_spatial_updates);
                    let use_cell_branch = avd_rc_use_cell_list_branch(black_box(12));
                    checksum += f64::from(iterations + use_cell_branch);
                }
            }
            checksum += update_time;
            black_box(checksum)
        })
    });

    group.finish();
}

fn bench_provider_classification(c: &mut Criterion) {
    let mut group = c.benchmark_group("provider_id_helpers");
    let standard = make_cstring("core.demo");
    let argumented = make_cstring("core.demo[value]");
    let malformed = make_cstring("demo]");

    group.bench_function("classify_standard", |b| {
        b.iter(|| {
            let kind = avd_provider_classify_id(
                black_box(standard.as_ptr()),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            black_box(kind)
        })
    });

    group.bench_function("classify_argumented_with_outputs", |b| {
        b.iter(|| {
            let mut raw: *mut c_char = std::ptr::null_mut();
            let mut arg: *mut c_char = std::ptr::null_mut();
            let kind = avd_provider_classify_id(black_box(argumented.as_ptr()), &mut raw, &mut arg);
            if !raw.is_null() {
                avd_provider_string_free(raw);
            }
            if !arg.is_null() {
                avd_provider_string_free(arg);
            }
            black_box(kind)
        })
    });

    group.bench_function("classify_malformed", |b| {
        b.iter(|| {
            let mut raw: *mut c_char = std::ptr::null_mut();
            let mut arg: *mut c_char = std::ptr::null_mut();
            let kind = avd_provider_classify_id(black_box(malformed.as_ptr()), &mut raw, &mut arg);
            black_box(kind)
        })
    });

    group.finish();
}

fn bench_cpu_dispatch_classification(c: &mut Criterion) {
    let mut group = c.benchmark_group("cpu_dispatch_helpers");

    group.bench_function("dispatch_family_and_counted_opcode", |b| {
        b.iter(|| {
            let mut checksum = 0_i32;
            for i in 0..4096_i32 {
                let is_nop = if i % 11 == 0 { 1 } else { 0 };
                let is_label = if i % 13 == 0 { 1 } else { 0 };
                let is_promoter = if i % 17 == 0 { 1 } else { 0 };
                let should_stall = if i % 19 == 0 { 1 } else { 0 };
                let family = avd_cpu_dispatch_family(
                    black_box(is_nop),
                    black_box(is_label),
                    black_box(is_promoter),
                    black_box(should_stall),
                );
                checksum += avd_cpu_dispatch_counted_opcode(black_box(i & 255), black_box(family));
            }
            black_box(checksum)
        })
    });

    group.finish();
}

fn bench_package_parsing_and_formatting(c: &mut Criterion) {
    let mut group = c.benchmark_group("package_helpers");
    let bool_true = make_cstring("true");
    let int_hex = make_cstring("0x10");
    let double_value = make_cstring("12345.6789");

    group.bench_function("str_as_bool", |b| {
        b.iter(|| black_box(avd_pkg_str_as_bool(black_box(bool_true.as_ptr()))))
    });
    group.bench_function("str_as_int", |b| {
        b.iter(|| black_box(avd_pkg_str_as_int(black_box(int_hex.as_ptr()))))
    });
    group.bench_function("str_as_double", |b| {
        b.iter(|| black_box(avd_pkg_str_as_double(black_box(double_value.as_ptr()))))
    });
    group.bench_function("double_to_string", |b| {
        b.iter(|| {
            let ptr = avd_pkg_double_to_string(black_box(12345.6789));
            if !ptr.is_null() {
                avd_pkg_string_free(ptr);
            }
        })
    });

    group.finish();
}

fn bench_bit_array_shift_increment_count(c: &mut Criterion) {
    let mut group = c.benchmark_group("bit_array_helpers");
    let sizes = [128_i32, 1024_i32, 4096_i32];

    for num_bits in sizes {
        group.bench_function(format!("ffi_shift_increment_count_{num_bits}b"), |b| {
            b.iter(|| {
                let handle = avd_rba_new(num_bits);
                if handle.is_null() {
                    return;
                }
                for i in (0..num_bits).step_by(17) {
                    avd_rba_set_bit(handle, i, 1);
                }
                for shift in [1, 7, -3, 13, -17] {
                    avd_rba_shift(handle, num_bits, black_box(shift));
                    avd_rba_increment(handle, num_bits);
                }
                let count = avd_rba_count_bits(handle, num_bits);
                avd_rba_free(handle);
                black_box(count);
            })
        });

        group.bench_function(format!("bitvec_reference_{num_bits}b"), |b| {
            b.iter(|| {
                let mut bits = BitVec::<u32, Lsb0>::repeat(false, num_bits as usize);
                for i in (0..num_bits).step_by(17) {
                    bits.set(i as usize, true);
                }
                for shift in [1_i32, 7, -3, 13, -17] {
                    let mut shifted = BitVec::<u32, Lsb0>::repeat(false, num_bits as usize);
                    if shift > 0 {
                        for i in (0..num_bits).rev() {
                            let src = i - shift;
                            if src >= 0 {
                                shifted.set(i as usize, bits[src as usize]);
                            }
                        }
                    } else if shift < 0 {
                        let right = -shift;
                        for i in 0..num_bits {
                            let src = i + right;
                            if src < num_bits {
                                shifted.set(i as usize, bits[src as usize]);
                            }
                        }
                    } else {
                        shifted.clone_from(&bits);
                    }
                    bits = shifted;

                    let mut carry = true;
                    for i in 0..bits.len() {
                        if !carry {
                            break;
                        }
                        let next = !bits[i];
                        bits.set(i, next);
                        carry = !next;
                    }
                }
                black_box(bits.count_ones())
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_resource_scheduling,
    bench_resource_update_dispatch,
    bench_cpu_dispatch_classification,
    bench_provider_classification,
    bench_package_parsing_and_formatting,
    bench_bit_array_shift_increment_count
);
criterion_main!(benches);
