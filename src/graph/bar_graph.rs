use crate::data::TelemData;
use egui::Id;
use egui_plot::Plot;
use egui_plot::{Bar, BarChart};

use super::Graph;
pub const NUMBER_BARS: usize = 15;
const BAR_WIDTH: f64 = 1.0;

pub struct BarPoints {
    data: [u32; NUMBER_BARS],
    bars: Vec<Bar>,
}

impl BarPoints {
    pub fn new() -> BarPoints {
        BarPoints {
            data: [0; NUMBER_BARS],
            bars: Vec::new(),
        }
    }

    pub fn bars(&self) -> Vec<Bar> {
        self.bars.clone()
    }
}

impl<'a> Graph<'a> for BarPoints {
    fn draw(&self, data: &crate::data::Data, _ctx: &egui::Context, ui: &mut egui::Ui) {
        let data_count_res = data.get("suspension_counts".to_string());
        let data_count;
        if let Ok(TelemData::U32V(counts)) = data_count_res {
            data_count = counts.to_vec();
        } else {
            return;
        }

        let mut bars = Vec::new();
        for (i, v) in data_count.iter().enumerate() {
            let bar = Bar::new(i as f64 * BAR_WIDTH, *v as f64).width(BAR_WIDTH);

            bars.push(bar);
        }

        let histogram = BarChart::new(bars);

        let plot = Plot::new("histogram")
            .id(Id::new("histogram"))
            .view_aspect(2.0)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .allow_zoom(false)
            .allow_drag(false)
            .show_grid(false);

        plot.show(ui, |plot_ui| plot_ui.bar_chart(histogram));
    }
 
    fn init() -> Self where Self: Sized {
        BarPoints::new()
    }
}
