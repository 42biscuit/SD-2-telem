#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use egui::{Style, Visuals};

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    // Log to stdout (if you run with `RUST_LOG=debug`).

    use sd2_telem::ConfigInfo;

    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();

    let style = Style {
        visuals: Visuals::dark(),
        ..Style::default()
    };
    let style2s = Style {
        visuals: Visuals::dark(),
        ..Style::default()
    };


    let mut initial_config = ConfigInfo::load_blank();
    
    eframe::run_native(
        "SD2 Telemetry Config Wondow",
        native_options.clone(), 
        Box::new(|cc|{
            cc.egui_ctx.set_style(style);
            Box::new(sd2_telem::InitialConfig::new_with_mut_config(cc ))
        })
    ).unwrap();
    
    
    eframe::run_native(
        "SD2 Telemetry Software",
        native_options,
        Box::new(|cd| {
            cd.egui_ctx.set_style(style2s);
            Box::new(sd2_telem::TelemApp::new_add_config(cd, initial_config))
        }),
    )
}
