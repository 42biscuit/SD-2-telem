use egui_plot::{PlotPoint, PlotPoints};

use crate::graph::{GraphPoint, ToPlotPoint};

pub struct Line {
    pub plot_points: Vec<PlotPoint>
}

impl Line {
    pub fn new() -> Line {
        Line {
            plot_points: Vec::new()
        }
    }

    pub fn generate(points: &Vec<GraphPoint>) -> Line {
        let mut plot_points = Vec::<PlotPoint>::with_capacity(points.len());
        let _: Vec<_> = points.iter().enumerate().map(|(i, p)| {
            plot_points.push(p.to_plot_point().into());
        }).collect();

        Line {
            plot_points
        }
    }
}