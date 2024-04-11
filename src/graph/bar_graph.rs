use crate::data::TelemData;
use egui::{Color32, Id};
use egui_plot::{uniform_grid_spacer, Axis, Plot};
use egui_plot::{Bar, BarChart};

use super::Graph;
const BAR_WIDTH: f64 = 1.0;

pub struct BarPoints {
    sus_count_str: String,
    dims: Option<(f32, f32)>,
    colour: Color32
}

impl BarPoints {
    pub fn new(sus_count_str: String, colour:Color32) -> BarPoints {
        BarPoints {
            sus_count_str,
            dims: None,
            colour: colour,
        }
    }

    pub fn set_dims(&mut self, width: f32, height: f32) {
        self.dims = Some((width, height));
    }
}

impl<'a> Graph<'a> for BarPoints {
    fn draw(&self, data: &crate::data::Data, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let data_count_res = data.get(self.sus_count_str.clone());
        let data_count;
        if let Ok(TelemData::U32V(counts)) = data_count_res {
            data_count = counts.to_vec();
        } else {
            return;
        }

        let mut bars = Vec::new();
        for (i, v) in data_count.iter().enumerate() {
            let bar = Bar::new(i as f64 * BAR_WIDTH, *v as f64).width(BAR_WIDTH).fill(self.colour);

            bars.push(bar);
        }

        let histogram = BarChart::new(bars);

        let mut plot = Plot::new("histogram")
            .id(Id::new(self.sus_count_str.clone()))
            .view_aspect(2.0)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .allow_zoom(false)
            .allow_drag(false)
            .show_grid(false);
            

        if let Some((width, height)) = self.dims {
            plot = plot
                .width(width)
                .height(height);
        }

        plot.show(ui, |plot_ui| plot_ui.bar_chart(histogram));
    }
}
