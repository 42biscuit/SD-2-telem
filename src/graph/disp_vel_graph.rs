use egui::{Id, Vec2b};
use egui_plot::{Plot, PlotPoint, PlotPoints};

use crate::data::TelemData;

use super::{to_plot_points, Graph};


pub struct DispVelGraph{
    front_compression : String,
    rear_compression : String,
    front_rebound : String,
    rear_rebound : String,

}

impl DispVelGraph {
    pub fn new(front_rebounds_field:String, front_compressions_field:String, rear_rebounds_field:String, rear_compressions_field:String) -> DispVelGraph {
        DispVelGraph { 
            front_compression: front_compressions_field,
            rear_compression: rear_compressions_field,
            front_rebound: front_rebounds_field,
            rear_rebound: rear_rebounds_field,
        }
    }



}   

impl <'a> Graph<'a> for DispVelGraph{
    fn draw(&self, data: &crate::data::Data, ctx: &egui::Context, ui: &mut egui::Ui) {

        println!("draw called");

        let rear_compression_res = data.get(self.rear_compression.clone());
        let rear_compressions;
        if let Ok(TelemData::F32PV(counts)) = rear_compression_res {
            rear_compressions = counts.to_vec();
        } else {
            return;
        }

        let rear_rebounds_res = data.get(self.rear_rebound.clone());
        let rear_rebounds;
        if let Ok(TelemData::F32PV(counts)) = rear_rebounds_res {
            rear_rebounds = counts.to_vec();
        } else {
            return;
        }



        let _axis_bools_auto_zoom = Vec2b::new(false, false);

        let compression_plot = Plot::new("suspension34")
            .id(Id::new("dist_vel"))
            .view_aspect(2.0)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .show_grid(true);

        let rebound_plot = Plot::new("suspension43")
            .id(Id::new("dist_vel2"))
            .view_aspect(2.0)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .show_grid(true);
        ui.horizontal(|ui|{
            compression_plot.show(ui, |plot_ui| {
                plot_ui.points(egui_plot::Points::new( PlotPoints::Owned( to_plot_points(&rear_compressions))).radius(5.0));
            });
    
            rebound_plot.show(ui, |plot_ui| {
                plot_ui.points(egui_plot::Points::new( PlotPoints::Owned( to_plot_points(&rear_rebounds))).radius(2.0));
            });

        });
        println!("graphs showin1");
    }

}