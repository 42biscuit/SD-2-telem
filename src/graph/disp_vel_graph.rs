use crate::data::TelemData;
use egui::{Id, Vec2b};
use egui_plot::{Plot, PlotPoint, PlotPoints};
use egui_plot::{Bar, BarChart};

use super::{to_plot_points, Graph};


pub struct DispVelGraph{
    data : Vec<PlotPoint>,
    displacements: Vec<f32>,
    velocities: Vec<f32>,
}

impl DispVelGraph {
    pub fn new() -> DispVelGraph {
        DispVelGraph { 
            data: Vec::new(),
            displacements: Vec::new(),
            velocities: Vec::new(),
        }
    }

    pub fn displacements(&self) -> &Vec<f32> {
        &self.displacements
    }

    pub fn velocities(&self) -> &Vec<f32>{
        &self.velocities
    }

}   

impl <'a> Graph<'a> for DispVelGraph{

    fn init() -> Self
        where
            Self: Sized {
        DispVelGraph::new()
    }
    fn draw(&self, data: &crate::data::Data, ctx: &egui::Context, ui: &mut egui::Ui) {
        let x_disp = data.get_f64v("field".to_string());
        let y_vel = data.get_f64v("field".to_string());


        let axis_bools_drag = Vec2b::new(true, false);
        let _axis_bools_auto_zoom = Vec2b::new(false, false);

        let plot = Plot::new("suspension")
            .id(Id::new("suspension"))
            .view_aspect(2.0)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .show_grid(false);

        let a: Vec<(&f64, &f64)> = x_disp.iter().zip(y_vel.iter()).collect();

            
        plot.show(ui, |plot_ui| {
            plot_ui.points(egui_plot::Points::new(PlotPoints::Owned(to_plot_points(&a))));
            //plot_ui.line(bottom_out_line);  
        });
    }

}