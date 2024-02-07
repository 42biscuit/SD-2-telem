use std::collections::HashMap;

use egui::{Ui, WidgetText};
use egui_plot::{AxisBools, Line, Plot, PlotPoint, PlotPoints};

use crate::graph::{self, to_plot_points, Graph, ToPlotPoint};

pub struct SuspensionGraph {
    points: Vec<PlotPoint>,
    travel_line: Line,
    bottom_out_threshold: f64,
}

impl SuspensionGraph {
    pub fn blank() -> Self {
        SuspensionGraph {
            points: Vec::new(),
            travel_line: Line::new(Vec::new()),
            bottom_out_threshold: 0.0,
        }
    }
}

impl Graph for SuspensionGraph {
    fn init(&mut self) {}

    fn set_data<T: ToPlotPoint>(&mut self, data: &Vec<T>) {
        self.points = to_plot_points(data);
        self.travel_line = Line::new(Vec::new());
        //self.travel_line = Line::new(PlotPoints::Owned(self.points));
    }

    fn set_metadata(&mut self, metadata: &HashMap<String, f64>) {
        self.bottom_out_threshold = *metadata.get("bottom_out_threshold").expect("Metadata is missing field bottom_out_threshold");
    }

    fn update(&mut self, ui: &mut Ui) {
        let bottom_out_points: PlotPoints = (0..2).map(|i| {
                [(i * self.points.len()) as f64, self.bottom_out_threshold]
            }).collect();

        let bottom_out_line = Line::new(bottom_out_points);

        let mut temp_travel_line = Line::new(Vec::new());

        let y_disabled_axis_bools = AxisBools::new(true, false);

        Plot::new("suspension")
            .view_aspect(5.0)
            .allow_zoom(y_disabled_axis_bools)
            .allow_drag(y_disabled_axis_bools)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .show(ui, |plot_ui| {
                plot_ui.line(temp_travel_line);
                plot_ui.line(bottom_out_line);
            });
    }
}