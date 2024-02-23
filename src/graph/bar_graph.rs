use crate::data::{TelemData, MAX_DATA_VALUE};
use egui_plot::{Bar, BarChart, PlotItem};
use egui::{Context, Id, Ui, Vec2b, WidgetText};
use egui_plot::{Line, Plot, PlotBounds, PlotMemory, PlotPoint, PlotPoints};

use super::Graph;
pub const NUMBER_BARS:usize = 15;
const BAR_WIDTH: f64 = 1.0;

pub struct BarPoints{
    pub data:[u32; NUMBER_BARS],
    bars: Vec<Bar>,
}

impl BarPoints{
    pub fn new() -> BarPoints{
        BarPoints{
            data: [0; NUMBER_BARS],
            bars: Vec::new(),
        }
    }

    pub fn bars(&self) -> Vec<Bar>{
        self.bars.clone()
    }
}

impl<'a> Graph<'a> for BarPoints{
    fn draw(&self, data: &crate::data::Data, ctx: &egui::Context, ui: &mut egui::Ui) {
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
            .show_grid(false);

        plot.show(ui, |plot_ui|{
            plot_ui.bar_chart(histogram)
        });
    }

    fn init() -> Self where Self: Sized {
        BarPoints::new()
    }
}
