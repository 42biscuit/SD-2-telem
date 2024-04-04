use egui::{Color32, Id, Vec2b};
use egui_plot::{Plot, PlotPoint, PlotPoints};

use crate::data::TelemData;

use super::{to_plot_points, Graph};


pub struct DispVelGraph{
    plot_id:String,
    front : String,
    rear : String,

}

impl DispVelGraph {
    pub fn new(plot_id:String,front_field:String,  rear_field:String) -> DispVelGraph {
        DispVelGraph { 
            plot_id:plot_id,
            front: front_field,
            rear: rear_field,
        }
    }



}   

impl <'a> Graph<'a> for DispVelGraph{
    fn draw(&self, data: &crate::data::Data, ctx: &egui::Context, ui: &mut egui::Ui) {


        let front_res = data.get(self.front.clone());
        let front_data;
        if let Ok(TelemData::F32PV(counts)) = front_res {
            front_data = counts.to_vec();
        } else {
            return;
        }

        let rear_res = data.get(self.rear.clone());
        let rear_data;
        if let Ok(TelemData::F32PV(counts)) = rear_res {
            rear_data = counts.to_vec();
        } else {
            return;
        }



        let _axis_bools_auto_zoom = Vec2b::new(false, false);



        let rebound_plot = Plot::new("suspension43")
            .id(Id::new(&self.plot_id))
            .view_aspect(3.0)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .show_grid(true);
        ui.horizontal(|ui|{

            rebound_plot.show(ui, |plot_ui| {
                plot_ui.points(egui_plot::Points::new( PlotPoints::Owned( to_plot_points(&rear_data))).radius(4.0).color(Color32::RED));
                plot_ui.points(egui_plot::Points::new( PlotPoints::Owned( to_plot_points(&front_data))).radius(4.0).color(Color32::BLUE));
            });

        });
    }

}