use egui::{Context, Id, Ui, Vec2b};
use egui_plot::{Legend, Line, Plot, PlotBounds, PlotMemory, PlotPoints, Points};

use crate::{
    data::{Data, TelemData},
    graph::Graph,
};

/// A graph that can be used to visualise suspension data
pub struct SuspensionGraph {
    rear_sus_str: String,
    front_sus_str: String,
}

impl SuspensionGraph {
    pub fn new(rear_sus_str: String, front_sus_str: String) -> SuspensionGraph {
        SuspensionGraph {
            rear_sus_str,
            front_sus_str,
        }
    }
}

impl<'a> Graph<'a> for SuspensionGraph {
    fn draw(&self, data: &Data, ctx: &Context, ui: &mut Ui) {
        let rear_line_manager_res = data.get(self.rear_sus_str.clone());
        let front_line_manager_res = data.get(self.front_sus_str.clone());
        //let bottom_out_threshold_res = data.get("bottom_out_threshold".to_string());
        let line_manager_res = data.get("suspension_line".to_string());
        let bottom_out_threshold_res = data.get("bottom_out_threshold".to_string());
        let turning_points_res = data.get("rear_turning".to_string());

        let mut rear_line_manager = None;
        if let Ok(TelemData::LineManager(lm)) = rear_line_manager_res {
            rear_line_manager = Some(lm);
        }
        let mut front_line_manager = None;
        if let Ok(TelemData::LineManager(lm)) = front_line_manager_res {
            front_line_manager = Some(lm);
        }

        // let mut bottom_out_threshold = 0.0;
        // if let Ok(TelemData::F64(bot)) = bottom_out_threshold_res {
        //     bottom_out_threshold = *bot;
        // }

        let mut turning_points = None;
        if let Ok(TelemData::PlotPointV(pts)) = turning_points_res {
            turning_points = Some(pts);
        }

        let axis_bools_drag = Vec2b::new(true, false);
        let _axis_bools_auto_zoom = Vec2b::new(false, false);

        let legend = Legend::default();

        let plot = Plot::new("suspension")
            .id(Id::new("suspension"))
            .view_aspect(5.0)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .allow_drag(axis_bools_drag)
            .allow_zoom(axis_bools_drag)
            .show_grid(false)
            .include_y(0.0)
            .include_y(100.0)
            .legend(legend);
            //.include_y(data.get_f32("suspension_min".to_string()))
            //.include_y(data.get_f32("suspension_max".to_string()));

        let mut extremes = [0.0, 0.0];

        if let Some(state) = PlotMemory::load(ctx, Id::new("suspension")) {
            let transform = state.transform();
            let bounds = transform.bounds();
            extremes = [bounds.min()[0], bounds.max()[0]];
        }

        // let bottom_out_points: PlotPoints = (0..2)
        //     .map(|i| [i as f64 * extremes[1], bottom_out_threshold])
        //     .collect();

        // let bottom_out_line = Line::new(bottom_out_points);

        let mut rear_travel_line = None;
        let mut front_travel_line = None;
        
        if let Some(lm) = rear_line_manager {
            rear_travel_line = lm.gen_line(
                extremes[0],
                extremes[1],
            );
        }
        if let Some(lm) = front_line_manager {
            front_travel_line = lm.gen_line(
                extremes[0],
                extremes[1],
            );
        }

        plot.show(ui, |plot_ui| {
            if let Some(travel_line_u) = rear_travel_line {
                plot_ui.line(travel_line_u.name("Rear Suspension"));
            }
            if let Some(travel_line_u) = front_travel_line {
                plot_ui.line(travel_line_u.name("Front Suspension"));
            }
            if let Some(turning_points_u) = turning_points {
                plot_ui.points(Points::new(PlotPoints::Owned(turning_points_u.clone())).radius(3.0));
            }
            //plot_ui.line(bottom_out_line);  
        });
    }
}
