
use crate::config_info::{self, ConfigInfo, SuspensionRemapInfo};
use crate::config_window::ConfigWindow;
use crate::data::{Data, TelemData};
use crate::graph::bar_graph::BarPoints;
use crate::graph::disp_vel_graph::DispVelGraph;
use crate::graph::line_manager::LineManager;
use crate::graph::suspension_graph::SuspensionGraph;
use crate::graph::to_plot_points;
use crate::loader::Loader;
use crate::view::View;
use crate::Buff;

use std::collections::HashMap;

use egui::Color32;
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
    #[serde(skip)]
    bottom_out_threshold: f64,
    bottom_outs: u32,
    #[serde(skip)]
    telem_data: Data,
    #[serde(skip)]
    loader: Loader ,
    #[serde(skip)]
    sus_view: View<'a>,
    #[serde(skip)]
    config: ConfigInfo,
    show_unmapped_data: bool,
    #[serde(skip)]
    config_window: ConfigWindow,
    #[serde(skip)]
    current_remap_info: SuspensionRemapInfo,
    #[serde(skip)]
    current_remap_info_ref: String,

}

impl<'a> Default for TelemApp<'a> {
    fn default() -> Self {
        let data = Buff::new();

        let curr_remap_info = SuspensionRemapInfo::default();

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
            loader: Loader::new(),
            telem_data: Data::new(),
            sus_view: View::new(),
            config: ConfigInfo::load(),
            show_unmapped_data: false,
            config_window: ConfigWindow::new(),
            current_remap_info: SuspensionRemapInfo::default(),
            current_remap_info_ref: "Pick a remap reference".to_string(),
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

    pub fn new_add_config(cc:&eframe::CreationContext<'_>,config:ConfigInfo) -> Self {

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let mut a =  Self::default();
        a.config = config.clone();
        
        a
    }


    pub fn count_bottom_outs(&mut self) {
        self.bottom_outs = 0;
        // let mut above_threshold = self.data.data[0] as f64 > self.bottom_out_threshold;
        // for reading in &self.data.data {
        //     let reading_as_f64 = *reading as f64;
        //     if reading_as_f64 > self.bottom_out_threshold {
        //         if !above_threshold {
        //             self.bottom_outs += 1;
        //         }
        //         above_threshold = true;
        //     } else {
        //         above_threshold = false;
        //     }
        // }
    }

    pub fn reset_data(&mut self) {
        self.telem_data.clear();

        let rs_pot_data = self.loader.get_raw_pot_data("RS".to_string());
        let fs_pot_data = self.loader.get_raw_pot_data("FS".to_string());
        println!("{:?}, {:?}", rs_pot_data.remap_ref,fs_pot_data.remap_ref);
        let rb_pot_data = self.loader.get_raw_pot_data("RB".to_string());
        let fb_pot_data = self.loader.get_raw_pot_data("FB".to_string());

        let mut rear_sus_data_f32: Vec<f32> = rs_pot_data.data.iter().map(|d| { *d as f32 }).collect();
        let mut front_sus_data_f32: Vec<f32> = fs_pot_data.data.iter().map(|d| { *d as f32 }).collect();


        if !self.show_unmapped_data {
            let rs_remap_info = self.config.get_sus_remap_info(rs_pot_data.remap_ref.clone()).expect("Error: Suspension remap info not found");
            let fs_remap_info = self.config.get_sus_remap_info(fs_pot_data.remap_ref.clone()).expect("Error: Suspension remap info not found");

            rear_sus_data_f32 = self.telem_data.remapped_1d_with_clamp(&rear_sus_data_f32, &rs_remap_info, 0.0, 100.0);
            front_sus_data_f32 = self.telem_data.remapped_1d_with_clamp(&front_sus_data_f32, &fs_remap_info, 0.0, 100.0);

            self.telem_data.set_count("rear_suspension_counts".to_string(), &rear_sus_data_f32, 26, 100.0, false).unwrap();
            self.telem_data.set_count("front_suspension_counts".to_string(), &front_sus_data_f32, 26, 100.0, false).unwrap();
        } else {
            self.telem_data.set("stroke_len".to_string(), TelemData::F32(config_info::DEFAULT_SUS_MAX - config_info::DEFAULT_SUS_MAX)).unwrap();
            self.telem_data.set_count("rear_suspension_counts".to_string(), &rear_sus_data_f32, 26, config_info::DEFAULT_SUS_MAX as f64, false).unwrap();
            self.telem_data.set_count("front_suspension_counts".to_string(), &front_sus_data_f32, 26, config_info::DEFAULT_SUS_MAX as f64, false).unwrap();
        }
        self.telem_data.set_turning_points("rear_rebound".to_string(), "rear_compression".to_string(), "rear_turning".to_string(), &rear_sus_data_f32, false).unwrap();
        self.telem_data.set_turning_points("front_rebound".to_string(), "front_compression".to_string(), "front_turning".to_string(), &front_sus_data_f32, true).unwrap();
        
        let rear_sus_data_f32_enum = self.telem_data.enumerated_with_transform(&rear_sus_data_f32, 1.0 / rs_pot_data.polling_rate as f32, 0.0);
        let front_sus_data_f32_enum = self.telem_data.enumerated_with_transform(&front_sus_data_f32, 1.0 / fs_pot_data.polling_rate as f32, 0.0);

        let rear_line_manager = LineManager::new(to_plot_points(&rear_sus_data_f32_enum), rs_pot_data.polling_rate as f64);
        let front_line_manager = LineManager::new(to_plot_points(&front_sus_data_f32_enum), fs_pot_data.polling_rate as f64);
        
        self.telem_data.set("rear_suspension_line".to_string(), TelemData::LineManager(rear_line_manager)).unwrap();
        self.telem_data.set("front_suspension_line".to_string(), TelemData::LineManager(front_line_manager)).unwrap();


        let suspension_graph = SuspensionGraph::new("rear_suspension_line".to_string(), "front_suspension_line".to_string());
        let mut rear_histogram = BarPoints::new("rear_suspension_counts".to_string(),Color32::RED);
        let mut front_histogram = BarPoints::new("front_suspension_counts".to_string(),Color32::LIGHT_BLUE);
        rear_histogram.set_dims(500.0, 500.0);
        front_histogram.set_dims(500.0, 500.0);

        let disp_vel_rebound = DispVelGraph::new("rebound".to_string(),"front_rebound".to_string(),"rear_rebound".to_string());
        let disp_vel_compression = DispVelGraph::new("compression".to_string(),"front_compression".to_string(),"rear_compression".to_string());

        self.sus_view = View::new();
        self.sus_view.add_graph(1, Box::new(suspension_graph));
        self.sus_view.add_graph(2, Box::new(rear_histogram));
        self.sus_view.add_graph(2, Box::new(front_histogram));
        self.sus_view.add_graph(3,Box::new(disp_vel_rebound));
        self.sus_view.add_graph(4,Box::new(disp_vel_compression));


        self.telem_data.set("front_dyn_sag".to_string(), TelemData::F32(self.telem_data.data_average_raw(&front_sus_data_f32))).unwrap();
        self.telem_data.set("rear_dyn_sag".to_string(), TelemData::F32(self.telem_data.data_average_raw(&rear_sus_data_f32))).unwrap();
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
                    if ui.button("Config").clicked() {
                        self.config_window.open = true;
                    }
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

            // ui.horizontal(|ui| {
            //     ui.label("path to data.");
            //     ui.text_edit_singleline(&mut self.path);
            // });
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
                    self.loader.load(self.path.to_string());
                    
                    updated_data = true;
                }
            });


            ui.separator();

            ui.heading("Config");

            egui::ComboBox::new("config_selector", "Select Config Line")
                .selected_text(self.current_remap_info_ref.clone())
                .show_ui(ui, |ui| {
                    for (config_key, _) in &self.config.sus_remap_info {
                        ui.selectable_value(&mut self.current_remap_info_ref, config_key.to_string(), config_key);
                    }
                });

            let mut stroke_len = 0.0;
            let mut low_adj = 0.0;
            let mut high_adj = 0.0;
            
            //let curr_sus_remap_info = self.config.get_sus_remap_info();
            let mut remap_info_selected = false;
            let mut curr_sus_remap_info = SuspensionRemapInfo::default();
            if let Some(csri) = self.config.get_sus_remap_info(self.current_remap_info_ref.clone()) {
                remap_info_selected = true;
                curr_sus_remap_info = csri
            }

            ui.horizontal(|ui| {


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



            ui.heading("Suspension information");
            ui.heading("Suspension Data");
            ui.label("dynamic sag");

            let (f_average,r_average);

            match  self.telem_data.get_f32_err("front_dyn_sag".to_string()){
                Some(val) => f_average = val,
                None => f_average = 1000.0,
            }  
            match  self.telem_data.get_f32_err("rear_dyn_sag".to_string()){
                Some(val) => r_average = val,
                None => r_average = 1000.0,
            }

            ui.horizontal(|ui|{
                ui.label("front: ");
                ui.label(f_average.to_string());
                ui.label("    rear: ");
                ui.label(r_average.to_string());
            });


        });

        if updated_data {
            self.reset_data();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            //let mut metadata = HashMap::<String, f64>::new();
            //metadata.insert("bottom_out_threshold".to_string(), self.bottom_out_threshold);
            egui::ScrollArea::vertical().show(ui, |ui| {
                
                self.sus_view.draw(&self.telem_data, ctx, ui);
            });

            // ui.heading("eframe template");
            // ui.hyperlink("https://github.com/emilk/eframe_template");
            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/master/",
            //     "Source code."
            // ));
            // egui::warn_if_debug_build(ui);
        });

        self.config_window.update(ctx);

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
