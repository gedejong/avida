//! Avida Viewer — egui/eframe-based simulation visualization.
//!
//! This is the scaffold for the modern replacement of the ncurses TUI viewer.
//! It will eventually display population maps, stats dashboards, organism
//! inspectors, and configuration panels.

mod sim_bridge;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Snapshot of simulation state, produced by sim thread, consumed by UI.
#[derive(Clone, Debug)]
pub struct SimSnapshot {
    pub update: u64,
    pub num_organisms: u32,
    pub avg_fitness: f64,
    pub avg_merit: f64,
    pub avg_gestation: f64,
    pub avg_genome_length: f64,
}

impl Default for SimSnapshot {
    fn default() -> Self {
        SimSnapshot {
            update: 0,
            num_organisms: 0,
            avg_fitness: 0.0,
            avg_merit: 0.0,
            avg_gestation: 0.0,
            avg_genome_length: 0.0,
        }
    }
}

/// Time-series history for charting.
pub struct SimHistory {
    pub snapshots: VecDeque<SimSnapshot>,
    pub max_len: usize,
}

impl SimHistory {
    fn new(max_len: usize) -> Self {
        SimHistory {
            snapshots: VecDeque::with_capacity(max_len),
            max_len,
        }
    }

    fn push(&mut self, snap: SimSnapshot) {
        if self.snapshots.len() >= self.max_len {
            self.snapshots.pop_front();
        }
        self.snapshots.push_back(snap);
    }
}

/// Shared state between simulation and UI threads.
pub struct SharedState {
    pub current: SimSnapshot,
    pub history: SimHistory,
    pub running: bool,
}

/// Main application state.
struct AvidaViewerApp {
    shared: Arc<Mutex<SharedState>>,
    _bridge: Option<sim_bridge::SimulationBridge>,
}

impl AvidaViewerApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let shared = Arc::new(Mutex::new(SharedState {
            current: SimSnapshot::default(),
            history: SimHistory::new(1000),
            running: false,
        }));

        // Try to connect to a simulation if a config dir is provided via CLI
        let bridge = std::env::args().nth(1).map(|config_dir| {
            sim_bridge::SimulationBridge::new(config_dir, Arc::clone(&shared))
        });

        AvidaViewerApp {
            shared,
            _bridge: bridge,
        }
    }
}

impl eframe::App for AvidaViewerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let state = match self.shared.lock() {
            Ok(s) => s.current.clone(),
            Err(_) => SimSnapshot::default(),
        };

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Avida Viewer");
                ui.separator();
                ui.label(format!("Update: {}", state.update));
                ui.label(format!("Organisms: {}", state.num_organisms));
                ui.label(format!("Avg Fitness: {:.4}", state.avg_fitness));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Population Map");
            ui.label("(placeholder — will show grid visualization)");

            ui.separator();

            ui.heading("Stats");
            ui.label(format!("Average Merit: {:.2}", state.avg_merit));
            ui.label(format!("Average Gestation: {:.1}", state.avg_gestation));
            ui.label(format!("Average Genome Length: {:.1}", state.avg_genome_length));

            ui.separator();

            ui.heading("Status");
            let running = match self.shared.lock() {
                Ok(s) => s.running,
                Err(_) => false,
            };
            if running {
                ui.label("Simulation running — receiving live data");
            } else if self._bridge.is_some() {
                ui.label("Simulation bridge created — waiting for data...");
            } else {
                ui.label("No simulation connected. Run with: avida-viewer-egui <config-dir>");
            }
        });

        // Request repaint for live updates
        ctx.request_repaint();
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1024.0, 768.0])
            .with_title("Avida Viewer"),
        ..Default::default()
    };

    eframe::run_native(
        "Avida Viewer",
        options,
        Box::new(|cc| Ok(Box::new(AvidaViewerApp::new(cc)))),
    )
}
