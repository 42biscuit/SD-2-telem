
use crate::config_info::{self, ConfigInfo};
use crate::data::{Data, TelemData, FREQUENCY};
use crate::graph::bar_graph::BarPoints;
use crate::graph::line_manager::LineManager;
use crate::graph::suspension_graph::SuspensionGraph;
use crate::graph::{to_plot_points, Graph};
use crate::view::View;
use crate::Buff;

use rfd::FileDialog;

use std::env;
use std::path::PathBuf;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TelemApp<'a> {
    // Example stuff:
    path: String,
    time: u64,
    #[serde(skip)]
    data: Buff,
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    value: f64,
    #[serde(skip)]
    stroke_len_str: String,
    #[serde(skip)]
    low_adj_str: String,
    #[serde(skip)]
    high_adj_str: String,
    bottom_out_threshold: f64,
    bottom_outs: u32,
    #[serde(skip)]
    telem_data: Data,
    #[serde(skip)]
    sus_view: View<'a>,
    #[serde(skip)]
    config: ConfigInfo,
    #[serde(skip)]
    show_unmapped_data: bool,
}

impl<'a> Default for TelemApp<'a> {
    fn default() -> Self {
        let data = Buff::new();

        let mut config = ConfigInfo::load();
        let curr_remap_info = config.current_sus_remap_info();

        Self {
            // Example stuff:
            time: 0,
            path: "Hello World!".to_owned(),
            data,
            value: 1.0,
            stroke_len_str: curr_remap_info.stroke_len.to_string(),
            low_adj_str: curr_remap_info.inverse_without_stroke_len_scale(config_info::DEFAULT_SUS_MIN).to_string(),
            high_adj_str: curr_remap_info.inverse_without_stroke_len_scale(config_info::DEFAULT_SUS_MAX).to_string(),
            bottom_out_threshold: 0.0,
            bottom_outs: 0,
            telem_data: Data::new(),
            sus_view: View::new(),
            config,
            show_unmapped_data: false,
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

    pub fn reset_data(&mut self) {
        if self.data.data.is_empty() {
            return;
        }

        self.telem_data.clear();

        let mut sus_data_f32: Vec<f32> = self.data.data.iter().map(|d| { *d as f32 }).collect();

        if !self.show_unmapped_data {
            let cur_sus_remap_info = self.config.current_sus_remap_info();

            sus_data_f32 = self.telem_data.remapped_1d_with_clamp(&sus_data_f32, cur_sus_remap_info, 0.0, cur_sus_remap_info.stroke_len);
            self.telem_data.set_count("suspension_counts".to_string(), &sus_data_f32, 15, cur_sus_remap_info.stroke_len as f64, true).unwrap();
            self.telem_data.set("stroke_len".to_string(), TelemData::F32(cur_sus_remap_info.stroke_len)).unwrap();
        } else {
            self.telem_data.set("stroke_len".to_string(), TelemData::F32(config_info::DEFAULT_SUS_MAX - config_info::DEFAULT_SUS_MAX)).unwrap();
            self.telem_data.set_count("suspension_counts".to_string(), &sus_data_f32, 15, config_info::DEFAULT_SUS_MAX as f64, true).unwrap();
        }

        let sus_data_f32_enum = self.telem_data.enumerated_with_transform(&sus_data_f32, 1.0 / FREQUENCY as f32, 0.0);
        let line_manager = LineManager::new(to_plot_points(&sus_data_f32_enum));
        
        self.telem_data.set_turning_points("FDispl".to_string(), "FSpeeds".to_string(), "FTurning".to_string(), &self.data.to_f32v()).unwrap();

        self.telem_data.set("suspension_line".to_string(), TelemData::LineManager(line_manager)).unwrap();



        let suspension_graph = SuspensionGraph::init();
        let histogram = BarPoints::init();


        let suspension_graph_box = Box::new(suspension_graph);
        let histogram_box = Box::new(histogram);

        self.sus_view = View::new();
        self.sus_view.add_graph(suspension_graph_box);
        self.sus_view.add_graph(histogram_box);
        
        self.count_bottom_outs();
    }
}

impl<'a> eframe::App for TelemApp<'a> {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
        self.config.save();
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        let mut updated_data = false;

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

            ui.horizontal(|ui| {
                ui.label("path to data.");
                ui.text_edit_singleline(&mut self.path);
            });
            ui.heading("File Select");
            
            ui.label(self.path.clone());
            ui.horizontal(|ui| {
                if ui.button("Select File").clicked() {
                    let mut res_dir = env::current_dir().unwrap_or(PathBuf::default());

                    res_dir.push("resources");

                    let file = FileDialog::new()
                        .add_filter("Run Data", &["txt"])
                        .set_directory(res_dir)
                        .pick_file();

                    if let Some(file_path) = file {
                        self.path = file_path.to_str().unwrap().to_string();
                    }
                }
                if ui.button("Load").clicked() {
                    self.data.load(self.path.to_string());
                    updated_data = true;
                }
            });

            ui.separator();


            ui.heading("Data From Run");
            ui.horizontal(|ui| {
                ui.label("Bottom outs: ");
                ui.label(self.bottom_outs.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("Bottom threshhold: ");
                ui.add(
                    egui::Slider::new(&mut self.bottom_out_threshold, 0.0..=60.0).text("Threshold"),
                );
            });
            if ui.button("Recalculate").clicked() {
                self.count_bottom_outs();
            }

            ui.separator();
            ui.heading("Config");

            let mut stroke_len = 0.0;
            let mut low_adj = 0.0;
            let mut high_adj = 0.0;

            let curr_sus_remap_info = self.config.current_sus_remap_info();

            ui.horizontal(|ui| {
                ui.label("Stroke length: ");
                ui.text_edit_singleline(&mut self.stroke_len_str);
            });

            ui.horizontal(|ui| {
                ui.label("Low threshold: ");
                ui.text_edit_singleline(&mut self.low_adj_str);
            });

            ui.horizontal(|ui| {
                ui.label("High threshold: ");
                ui.text_edit_singleline(&mut self.high_adj_str);
            });

            ui.horizontal(|ui| {
                if ui.button("Recalculate").clicked() {
                    let mut incorrect_formatting = false;
                    
                    if let Ok(stroke_len_res) = self.stroke_len_str.parse::<f32>() {
                        stroke_len = stroke_len_res;
                    } else {
                        incorrect_formatting = true;
                    }
                    
                    if let Ok(low_res) = self.low_adj_str.parse::<f32>() {
                        low_adj = low_res;
                    } else {
                        incorrect_formatting = true;
                    }
                    
                    if let Ok(high_res) = self.high_adj_str.parse::<f32>() {
                        high_adj = high_res;
                    } else {
                        incorrect_formatting = true;
                    }
    
                    if incorrect_formatting {
                        ui.label("Error: Incorrect Formatting");
                    } else {
                        curr_sus_remap_info.stroke_len = stroke_len;
                        curr_sus_remap_info.calc_vals_from_min_and_max(low_adj, high_adj);
                        updated_data = true;
                    }
                }

                if !self.show_unmapped_data {
                    if ui.button("Show data without mapping").clicked() {
                        updated_data = true;
                        self.show_unmapped_data = true;
                    }
                } else {
                    if ui.button("Show data with mapping").clicked() {
                        updated_data = true;
                        self.show_unmapped_data = false;
                    }
                }
            });

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

        if updated_data {
            self.reset_data();
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
