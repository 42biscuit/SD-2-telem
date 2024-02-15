use std::collections::HashMap;
use crate::graph::bar_graph;
use egui::{Context, Id, Ui, Vec2b, WidgetText};
use egui_plot::{Line, Plot, PlotBounds, PlotMemory, PlotPoint, PlotPoints};

use crate::graph::{self, line_manager::LineManager, to_plot_points, Graph, ToPlotPoint};

pub struct SuspensionGraph {
    points: Vec<PlotPoint>,
    line_manager: LineManager,
    travel_line: Line,
    bottom_out_threshold: f64,
}

impl SuspensionGraph {
    pub fn blank() -> Self {
        SuspensionGraph {
            points: Vec::new(),
            line_manager: LineManager::new(&Vec::new()),
            travel_line: Line::new(Vec::new()),
            bottom_out_threshold: 0.0,
        }
    }
}

impl Graph for SuspensionGraph {
    fn init(&mut self) {}

    fn set_data<T: ToPlotPoint>(&mut self, data: &Vec<T>) {
        println!("Data len: {}", data.len());
        self.points = to_plot_points(data);
        self.travel_line = Line::new(Vec::new());
        //self.travel_line = Line::new(PlotPoints::Owned(self.points));
        self.line_manager = LineManager::new(&self.points);
    }

    fn set_metadata(&mut self, metadata: &HashMap<String, f64>) {
        self.bottom_out_threshold = *metadata.get("bottom_out_threshold").expect("Metadata is missing field bottom_out_threshold");
    }

    fn update(&mut self, ctx: &Context, ui: &mut Ui) {
        let bottom_out_points: PlotPoints = (0..2).map(|i| {
                [(i * self.points.len()) as f64, self.bottom_out_threshold]
            }).collect();

        let bottom_out_line = Line::new(bottom_out_points);

        let axis_bools_drag = Vec2b::new(true, false);
        let axis_bools_auto_zoom = Vec2b::new(false,false);

        let plot = Plot::new("suspension")
            .id(Id::new("suspension"))
            .view_aspect(5.0)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .allow_drag(axis_bools_drag)
            .auto_bounds(axis_bools_auto_zoom);

        let mut extremes = [0.0, 0.0];
        
        if let Some(state) = PlotMemory::load(ctx, Id::new("suspension")) {
            let transform = state.transform();
            let bounds = transform.bounds();
            extremes = [bounds.min()[0], bounds.max()[0]];
        }

        let temp_travel_line = self.line_manager.gen_line(extremes[0], extremes[1]);

        plot.show(ui, |plot_ui| {
            if let Some(travel_line) = temp_travel_line {
                plot_ui.line(travel_line);
            }
            //plot_ui.line(temp_travel_line);
            plot_ui.line(bottom_out_line);  
        });
    }
}