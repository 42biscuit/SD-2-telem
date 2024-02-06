use std::collections::HashMap;

use egui::{Ui, WidgetText};
use egui_plot::{Line, Plot, AxisBools, PlotPoints};

use crate::graph::{ToPlotPoint, Graph, ToGraphPoint, to_graph_points, GraphPoint, self, line};

pub struct SuspensionGraph {
    points: Vec<GraphPoint>,
    travel_line: graph::line::Line,
    bottom_out_threshold: f64,
}

impl SuspensionGraph {
    pub fn blank() -> Self {
        SuspensionGraph {
            points: Vec::new(),
            travel_line: graph::line::Line::new(),
            bottom_out_threshold: 0.0,
        }
    }
}

impl Graph for SuspensionGraph {
    fn init(&mut self) {}

    fn set_data<T: ToGraphPoint>(&mut self, data: &Vec<T>) {
        self.points = to_graph_points(data);
        self.travel_line = line::Line::generate(&self.points);
    }

    fn set_metadata(&mut self, metadata: &HashMap<String, f64>) {
        self.bottom_out_threshold = *metadata.get("bottom_out_threshold").expect("Metadata is missing field bottom_out_threshold");
    }

    fn update(&self, ui: &mut Ui) {
        let bottom_out_points: PlotPoints = (0..2).map(|i| {
                [(i * self.points.len()) as f64, self.bottom_out_threshold]
            }).collect();

        let cloned_points = self.travel_line.plot_points.clone();
        let plot_points: PlotPoints = egui_plot::PlotPoints::Owned(cloned_points);

        let suspension_line = Line::new(plot_points);
        let bottom_out_line = Line::new(bottom_out_points);

        let y_disabled_axis_bools = AxisBools::new(true, false);

        Plot::new("suspension")
            .view_aspect(5.0)
            .allow_zoom(y_disabled_axis_bools)
            .allow_drag(y_disabled_axis_bools)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .show(ui, |plot_ui| {
                plot_ui.line(suspension_line);
                plot_ui.line(bottom_out_line);
            });
    }
}