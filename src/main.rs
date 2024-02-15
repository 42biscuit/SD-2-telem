#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::{Visuals, Style};

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();

    let style = Style {
        visuals: Visuals::dark(),
        ..Style::default()
    };

    eframe::run_native(
        "SD2 Telemetry Software",
        native_options,
        Box::new(|cc| {
            cc.egui_ctx.set_style(style);
            Box::new(sd2_telem::TelemApp::new(cc))
        }),
    )
}
