use std::collections::HashMap;

use egui::{Context, Ui};
use egui_plot::PlotPoint;

pub mod suspension_graph;
pub mod line_manager;

pub trait ToPlotPoint {
    fn to_plot_point(&self) -> PlotPoint;
}

pub fn to_plot_points<T: ToPlotPoint>(data: &Vec<T>) -> Vec<PlotPoint> where T: Sized {
    data.iter().enumerate().map(|(i, d)| {
        d.to_plot_point()
    }).collect()
}

pub trait Graph {
    fn init(&mut self);
    fn set_data<T: ToPlotPoint>(&mut self, data: &Vec<T>);
    fn set_metadata(&mut self, metadata: &HashMap<String, f64>);
    fn update(&mut self, ctx: &Context, ui: &mut Ui);
}