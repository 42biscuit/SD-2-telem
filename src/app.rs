
use crate::data::{Data, TelemData};
use crate::graph::line_manager::LineManager;
use crate::graph::{line_manager, to_plot_points, Graph};
use crate::graph::suspension_graph::{SuspensionGraph, self};
use crate::view::View;
use crate::{data, Buff, BUFF_SIZE};
use egui_plot::{Line, Plot, PlotPoints, PlotPoint};
use rfd::FileDialog;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};

const TIME_STEP: f64 = 0.1;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TelemApp<'a> {
    // Example stuff:
    path: String,
    time:u64,
    #[serde(skip)]
    data: Buff,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f64,
    bottom_out_threshold: f64,
    bottom_outs: u32,
    #[serde(skip)]
    telem_data: Data,
    #[serde(skip)]
    sus_view: View<'a>,
}

impl<'a> Default for TelemApp<'a> {
    fn default() -> Self {
        let data = Buff::new();

        Self {
            // Example stuff:
            time: 0,
            path: "Hello World!".to_owned(),
            data,
            value: 1.0,
            bottom_out_threshold: 0.0,
            bottom_outs: 0,
            telem_data: Data::new(),
            sus_view: View::new(),
        }
    }
}

impl<'a> TelemApp<'a> {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn count_bottom_outs(&mut self) {
        self.bottom_outs = 0;
        let mut above_threshold = self.data.data[0] as f64 > self.bottom_out_threshold;
        for reading in &self.data.data {
            let reading_as_f64 = *reading as f64;
            if reading_as_f64 > self.bottom_out_threshold {
                if !above_threshold {
                    self.bottom_outs += 1;
                }
                above_threshold = true;
            } else {
                above_threshold = false;
            }
        }
    }
}

impl<'a> eframe::App for TelemApp<'a> {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        let mut data_tuple: Option<Vec<(u32, u32)>> = None;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        //frame.close();
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
                ui.text_edit_singleline(&mut self.path);
            });
            if ui.button("Load").clicked() {
                // self.data.load(self.path.to_string());
                // self.suspension_graph.set_data(&self.data.data);
                // self.count_bottom_outs();
                // println!("{}", &self.path);
            }
            
            ui.horizontal(|ui| {
                if ui.button("Select File").clicked() {
                    let mut res_dir = env::current_dir()
                        .unwrap_or(PathBuf::default());

                    res_dir.push("resources");

                    let file = FileDialog::new()
                        .add_filter("Run Data", &["txt"])
                        .set_directory(res_dir)
                        .pick_file();

                    if let Some(file_path) = file {
                        self.path = file_path.to_str().unwrap().to_string();
                        self.data.load(self.path.to_string());
                        
                        data_tuple = Some(self.data.data.iter().enumerate().map(|(i, d)| {
                            (i as u32, *d)
                        }).collect());

                        //println!("{}, {}", self.data.data.len(), file_path.to_str().unwrap());

                        //println!("{}, {}", self.data.data.len(), file_path.to_str().unwrap());
                    }

                    //println!("{}", self.path);
                }
                ui.label(self.path.clone());
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=100.0).text("value"));
            ui.add(egui::Slider::new(&mut self.time, 1..=10).text("time for loading"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.heading("Data From Run");
            ui.horizontal(|ui| {
                ui.label("Bottom outs: ");
                ui.label(self.bottom_outs.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("Bottom threshhold: ");
                ui.add(egui::Slider::new(&mut self.bottom_out_threshold, 0.0..=60.0).text("Threshold"));
            });
            if (ui.button("Recalculate").clicked()) {
                self.count_bottom_outs();
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

        if let Some(sus_data) = data_tuple {
            let line_manager = LineManager::new(to_plot_points(&sus_data));
            self.telem_data.set("suspension_line".to_string(), TelemData::LineManager(line_manager));
            self.sus_view = View::new();
            let suspension_graph = SuspensionGraph::init();
            let suspension_graph_box = Box::new(suspension_graph);
            self.sus_view.add_graph(suspension_graph_box);
            //self.count_bottom_outs();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            //let mut metadata = HashMap::<String, f64>::new();
            //metadata.insert("bottom_out_threshold".to_string(), self.bottom_out_threshold);

            self.sus_view.draw(&self.telem_data, ctx, ui);

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
