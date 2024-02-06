use std::collections::HashMap;

use egui::{Ui};

pub mod line;
pub mod suspension_graph;

pub struct GraphPoint {
    pub x: f64,
    pub y: f64,
}

pub trait ToGraphPoint {
    fn to_graph_point(&self, x: f64) -> GraphPoint;        
}

pub trait ToPlotPoint {
    fn to_plot_point(&self) -> [f64; 2];
}

impl ToPlotPoint for GraphPoint {
    fn to_plot_point(&self) -> [f64; 2] {
        [self.x, self.y]
    }
}

pub fn to_graph_points<T: ToGraphPoint>(data: &Vec<T>) -> Vec<GraphPoint> where T: Sized {
    data.iter().enumerate().map(|(i, d)| {
        d.to_graph_point(i as f64)
    }).collect()
}

pub trait Graph {
    fn init(&mut self);
    fn set_data<T: ToGraphPoint>(&mut self, data: &Vec<T>);
    fn set_metadata(&mut self, metadata: &HashMap<String, f64>);
    fn update(&self, ui: &mut Ui);
}