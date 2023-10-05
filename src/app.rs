use crate::{Buff, BUFF_SIZE};
use egui::plot::{Line, Plot, PlotPoints};
use egui::widgets::plot;
use std::fs;

const TIME_STEP: f64 = 0.1;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    path: String,
    time:u64,
    #[serde(skip)]
    data: Buff,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f64,

}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            time: 0,
            path: "Hello World!".to_owned(),
            data: Buff::new(),
            value: 1.0,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { path, data, value,time } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.close();
                    }
                    if ui.button("more").clicked() {
                        println!("button pressed");
                    }         
                });
            });
        });
        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("path to data.");
                ui.text_edit_singleline(path);
            });
            if ui.button("Load").clicked() {
                data.load(path.to_string());
                println!("{}",&path);
            }
            ui.add(egui::Slider::new(value, 0.0..=100.0).text("value"));
            ui.add(egui::Slider::new(time, 1..=10).text("time for loading"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }


            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            let width = ctx.available_rect().width().floor();
            let mut points:PlotPoints = self
                .data.clone()
                .data
                .iter()
                .enumerate()
                .map(|i| {
                    let x = self.time as f64 * i.0 as f64 * TIME_STEP * self.value;
                    [x, *i.1 as f64]
                })
                .collect();
            let line = Line::new(points);

            let window_info = frame.info().window_info.clone();
            Plot::new("my_plot")
                .view_aspect(1.0)
                .show(ui, |plot_ui| plot_ui.line(line));
            ui.heading("eframe template");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally choose either panels OR windows.");
            });
        }
    }
}

/*impl FromIterator<[f32;2]> for egui::plot::PlotPoints{
    fn from_iter<T: IntoIterator<Item = [f32; 2]>>(iter: T) -> Self {
        Self::Owned(iter.into_iter().map(|point| point.into()).collect())
    }
}*/
