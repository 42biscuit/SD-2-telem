use egui::{Id, Vec2b};
use egui_plot::{Plot, PlotPoint, PlotPoints};

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
    fn draw(&self, data: &crate::data::Data, ctx: &egui::Context, ui: &mut egui::Ui) {
        let x_disp = data.get_f64v("front_disp".to_string());
        let y_vel = data.get_f64v("front_speed".to_string());


        let _axis_bools_auto_zoom = Vec2b::new(false, false);

        let plot = Plot::new("suspension")
            .id(Id::new("dist_vel"))
            .view_aspect(3.0)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .show_grid(true);

        let a: Vec<(&f64, &f64)> = x_disp.iter().zip(y_vel.iter()).filter(|x| *x.0 > 0.0).collect();

        plot.show(ui, |plot_ui| {
            plot_ui.points(egui_plot::Points::new( PlotPoints::Owned( to_plot_points(&a))).radius(2.0));
        });
    }

}