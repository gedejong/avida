//! Simulation bridge — spawns Avida simulation thread and extracts live data.

use std::ffi::{c_char, c_double, c_int, CString};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::{SharedState, SimSnapshot};

// Opaque handle for C++ Avida::Viewer::Driver
#[repr(C)]
pub struct AvidaViewerDriver {
    _private: [u8; 0],
}

// FFI declarations for viewer_ffi.cc
unsafe extern "C" {
    fn avd_viewer_create(config_dir: *const c_char) -> *mut AvidaViewerDriver;
    fn avd_viewer_start(driver: *mut AvidaViewerDriver);
    fn avd_viewer_pause(driver: *mut AvidaViewerDriver);
    fn avd_viewer_resume(driver: *mut AvidaViewerDriver);
    fn avd_viewer_finish(driver: *mut AvidaViewerDriver);
    fn avd_viewer_is_paused(driver: *mut AvidaViewerDriver) -> c_int;
    fn avd_viewer_has_finished(driver: *mut AvidaViewerDriver) -> c_int;
    fn avd_viewer_get_update(driver: *mut AvidaViewerDriver) -> c_int;
    fn avd_viewer_get_num_organisms(driver: *mut AvidaViewerDriver) -> c_int;
    fn avd_viewer_get_avg_fitness(driver: *mut AvidaViewerDriver) -> c_double;
    fn avd_viewer_get_avg_merit(driver: *mut AvidaViewerDriver) -> c_double;
    fn avd_viewer_get_avg_gestation(driver: *mut AvidaViewerDriver) -> c_double;
    fn avd_viewer_get_avg_genome_length(driver: *mut AvidaViewerDriver) -> c_double;
    fn avd_viewer_free(driver: *mut AvidaViewerDriver);
}

/// Manages the simulation thread and extracts snapshots for the UI.
pub struct SimulationBridge {
    shared: Arc<Mutex<SharedState>>,
    _sim_thread: Option<thread::JoinHandle<()>>,
}

impl SimulationBridge {
    /// Create a new bridge. Spawns a background thread that:
    /// 1. Creates an Avida simulation from the config directory
    /// 2. Starts it
    /// 3. Periodically extracts snapshots into SharedState
    pub fn new(config_dir: String, shared: Arc<Mutex<SharedState>>) -> Self {
        let shared_clone = Arc::clone(&shared);

        let handle = thread::spawn(move || {
            let c_dir = match CString::new(config_dir) {
                Ok(s) => s,
                Err(_) => return,
            };

            // SAFETY: avd_viewer_create returns a valid handle or null.
            let driver = unsafe { avd_viewer_create(c_dir.as_ptr()) };
            if driver.is_null() {
                return;
            }

            // Start the simulation
            // SAFETY: driver is valid, non-null.
            unsafe { avd_viewer_start(driver) };

            // Poll loop — extract snapshots every 50ms
            loop {
                thread::sleep(std::time::Duration::from_millis(50));

                // SAFETY: driver is valid throughout this loop.
                let finished = unsafe { avd_viewer_has_finished(driver) } != 0;
                if finished {
                    break;
                }

                let snap = SimSnapshot {
                    // SAFETY: driver is valid.
                    update: unsafe { avd_viewer_get_update(driver) } as u64,
                    num_organisms: unsafe { avd_viewer_get_num_organisms(driver) } as u32,
                    avg_fitness: unsafe { avd_viewer_get_avg_fitness(driver) },
                    avg_merit: unsafe { avd_viewer_get_avg_merit(driver) },
                    avg_gestation: unsafe { avd_viewer_get_avg_gestation(driver) },
                    avg_genome_length: unsafe { avd_viewer_get_avg_genome_length(driver) },
                };

                if let Ok(mut state) = shared_clone.lock() {
                    state.running = true;
                    state.history.push(snap.clone());
                    state.current = snap;
                }
            }

            if let Ok(mut state) = shared_clone.lock() {
                state.running = false;
            }

            // SAFETY: driver is valid, we're done with it.
            unsafe { avd_viewer_free(driver) };
        });

        SimulationBridge {
            shared,
            _sim_thread: Some(handle),
        }
    }
}
