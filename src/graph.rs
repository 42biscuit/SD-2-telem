use std::collections::HashMap;

use egui::{Context, Ui};
use egui_plot::PlotPoint;

use crate::data::Data;

pub mod suspension_graph;
pub mod line_manager;

/// Convert a value of an arbitrary data type to a PlotPoint
pub trait ToPlotPoint {
    fn to_plot_point(&self) -> PlotPoint;
}

/// Convert a vector of arbitrary data points to a vector of PlotPoints
/// 
/// # Arguments
/// 
/// `data`: A vector containing the data points
/// 
/// # Returns
/// 
/// A vector of PlotPoints
pub fn to_plot_points<T: ToPlotPoint>(data: &Vec<T>) -> Vec<PlotPoint> where T: Sized {
    data.iter().enumerate().map(|(i, d)| {
        d.to_plot_point()
    }).collect()
}

/// Functions which are required by all graphs
pub trait Graph <'a>{
    /// Initialise a new graph
    /// 
    /// # Arguments
    /// 
    /// `data`: The data that this graph will use
    fn init() -> Self where Self: Sized;

    /// Update and render the graph
    /// 
    /// # Arguments
    /// 
    /// `ctx`: The eGui context  
    /// `ui`: The eGui UI instance
    fn draw(&self, data: &Data, ctx: &Context, ui: &mut Ui);
}