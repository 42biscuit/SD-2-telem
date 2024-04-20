use crate::data::Data;
use egui::{Context, Ui};
use egui_plot::PlotPoint;

pub mod bar_graph;
pub mod line_manager;
pub mod suspension_graph;
pub mod disp_vel_graph;
pub mod wave_gen;

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
pub fn to_plot_points<T: ToPlotPoint>(data: &Vec<T>) -> Vec<PlotPoint>
where
    T: Sized,
{
    data.iter()
        .enumerate()
        .map(|(_i, d)| d.to_plot_point())
        .collect()
}

/// Functions which are required by all graphs
pub trait Graph<'a> {
    //fn pre_draw(&self, data: &Data, ctx: &Context, ui: &mut Ui);

    /// Update and render the graph
    ///
    /// # Arguments
    ///
    /// `ctx`: The eGui context  
    /// `ui`: The eGui UI instance
    fn draw(&self, data: &Data, ctx: &Context, ui: &mut Ui);
}
