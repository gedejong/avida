use crate::common::{set_out, with_cstr};
use std::ffi::{c_char, c_int};

const TRIGGER_UPDATE: c_int = 0;
const TRIGGER_GENERATION: c_int = 1;
const TRIGGER_IMMEDIATE: c_int = 2;
const TRIGGER_BIRTHS: c_int = 3;
const TRIGGER_BIRTHS_INTERRUPT: c_int = 5;
const TRIGGER_INVALID: c_int = -1;

const TRIGGER_BEGIN: f64 = f64::MIN_POSITIVE;
const TRIGGER_END: f64 = f64::MAX;
const TRIGGER_ALL: f64 = 0.0;
const TRIGGER_ONCE: f64 = f64::MAX;

fn parse_number_strict(token: &str) -> Option<f64> {
    token.parse::<f64>().ok()
}

fn parse_number_legacy(token: &str) -> f64 {
    token.parse::<f64>().unwrap_or(0.0)
}

#[no_mangle]
pub extern "C" fn avd_event_parse_trigger(token: *const c_char) -> c_int {
    with_cstr(token, TRIGGER_INVALID, |c| {
        match c.to_string_lossy().as_ref() {
            "i" | "immediate" => TRIGGER_IMMEDIATE,
            "u" | "update" => TRIGGER_UPDATE,
            "g" | "generation" => TRIGGER_GENERATION,
            "b" | "births" => TRIGGER_BIRTHS,
            "o" | "org_id" => TRIGGER_BIRTHS_INTERRUPT,
            _ => TRIGGER_INVALID,
        }
    })
}

#[no_mangle]
pub extern "C" fn avd_event_parse_timing(
    timing: *const c_char,
    out_start: *mut f64,
    out_interval: *mut f64,
    out_stop: *mut f64,
) -> c_int {
    if out_start.is_null() || out_interval.is_null() || out_stop.is_null() {
        return 0;
    }
    let parsed = with_cstr(timing, None::<(f64, f64, f64)>, |c| {
        let raw = c.to_string_lossy();
        let mut parts = raw.split(':');

        let first = parts.next()?;
        let start = if first == "begin" {
            TRIGGER_BEGIN
        } else {
            parse_number_strict(first)?
        };

        let second = parts.next();
        let third = parts.next();
        if parts.next().is_some() {
            return None;
        }

        let interval = match second {
            None => TRIGGER_ONCE,
            Some("all") => TRIGGER_ALL,
            Some("once") => TRIGGER_ONCE,
            Some(value) => parse_number_legacy(value),
        };

        let stop = match third {
            None => TRIGGER_END,
            Some("end") => TRIGGER_END,
            Some(value) => parse_number_legacy(value),
        };

        Some((start, interval, stop))
    });

    let Some((start, interval, stop)) = parsed else {
        return 0;
    };

    set_out(out_start, start);
    set_out(out_interval, interval);
    set_out(out_stop, stop);
    1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn parse_trigger_alias_matrix() {
        let immediate = CString::new("immediate").expect("literal has no NUL");
        let update = CString::new("u").expect("literal has no NUL");
        let generation = CString::new("generation").expect("literal has no NUL");
        let births = CString::new("b").expect("literal has no NUL");
        let org_id = CString::new("org_id").expect("literal has no NUL");
        let bad = CString::new("???").expect("literal has no NUL");

        assert_eq!(
            avd_event_parse_trigger(immediate.as_ptr()),
            TRIGGER_IMMEDIATE
        );
        assert_eq!(avd_event_parse_trigger(update.as_ptr()), TRIGGER_UPDATE);
        assert_eq!(
            avd_event_parse_trigger(generation.as_ptr()),
            TRIGGER_GENERATION
        );
        assert_eq!(avd_event_parse_trigger(births.as_ptr()), TRIGGER_BIRTHS);
        assert_eq!(
            avd_event_parse_trigger(org_id.as_ptr()),
            TRIGGER_BIRTHS_INTERRUPT
        );
        assert_eq!(avd_event_parse_trigger(bad.as_ptr()), TRIGGER_INVALID);
        assert_eq!(avd_event_parse_trigger(std::ptr::null()), TRIGGER_INVALID);
    }

    #[test]
    fn parse_timing_matrix() {
        let mut start = 0.0;
        let mut interval = 0.0;
        let mut stop = 0.0;

        let begin = CString::new("begin").expect("literal has no NUL");
        assert_eq!(
            avd_event_parse_timing(begin.as_ptr(), &mut start, &mut interval, &mut stop),
            1
        );
        assert_eq!(start, TRIGGER_BEGIN);
        assert_eq!(interval, TRIGGER_ONCE);
        assert_eq!(stop, TRIGGER_END);

        let start_interval = CString::new("10:all").expect("literal has no NUL");
        assert_eq!(
            avd_event_parse_timing(
                start_interval.as_ptr(),
                &mut start,
                &mut interval,
                &mut stop
            ),
            1
        );
        assert_eq!(start, 10.0);
        assert_eq!(interval, TRIGGER_ALL);
        assert_eq!(stop, TRIGGER_END);

        let full = CString::new("10:2:20").expect("literal has no NUL");
        assert_eq!(
            avd_event_parse_timing(full.as_ptr(), &mut start, &mut interval, &mut stop),
            1
        );
        assert_eq!(start, 10.0);
        assert_eq!(interval, 2.0);
        assert_eq!(stop, 20.0);
    }

    #[test]
    fn parse_timing_rejects_invalid_first_token_and_null_outputs() {
        let mut start = 0.0;
        let mut interval = 0.0;
        let mut stop = 0.0;
        let invalid = CString::new("bad:1:2").expect("literal has no NUL");
        assert_eq!(
            avd_event_parse_timing(invalid.as_ptr(), &mut start, &mut interval, &mut stop),
            0
        );
        assert_eq!(
            avd_event_parse_timing(
                invalid.as_ptr(),
                std::ptr::null_mut(),
                &mut interval,
                &mut stop
            ),
            0
        );

        let legacy = CString::new("10:not_a_number:end").expect("literal has no NUL");
        assert_eq!(
            avd_event_parse_timing(legacy.as_ptr(), &mut start, &mut interval, &mut stop),
            1
        );
        assert_eq!(interval, 0.0);
        assert_eq!(stop, TRIGGER_END);
    }
}
