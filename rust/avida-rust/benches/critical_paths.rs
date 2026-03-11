use avida_rust::resource_count_helpers::{
    avd_rc_accumulate_update_time, avd_rc_num_steps, avd_rc_remainder_update_time,
};
use avida_rust::{
    avd_pkg_double_to_string, avd_pkg_str_as_bool, avd_pkg_str_as_double, avd_pkg_str_as_int,
    avd_pkg_string_free, avd_provider_classify_id, avd_provider_string_free,
};
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

criterion_group!(
    benches,
    bench_resource_scheduling,
    bench_provider_classification,
    bench_package_parsing_and_formatting
);
criterion_main!(benches);
