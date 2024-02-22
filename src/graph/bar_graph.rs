use crate::data::MAX_DATA_VALUE;
use egui_plot::{Bar, BarChart};
use egui::{Context, Id, Ui, Vec2b, WidgetText};
use egui_plot::{Line, Plot, PlotBounds, PlotMemory, PlotPoint, PlotPoints};

use super::Graph;
pub const NUMBER_BARS:usize = 15;

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

    pub fn update(&mut self, data: Vec<u32>){

        for point in data.iter(){
            let index = ((*point as f64/(1024.0 / MAX_DATA_VALUE)) * (NUMBER_BARS as f64/MAX_DATA_VALUE)).round() as usize;
            self.data[index-1] += 1;
        }
        println!("{:?}",self.data); 
    }
}

impl<'a> Graph<'a> for BarPoints{

    fn draw(&self, data: &crate::data::Data, ctx: &egui::Context, ui: &mut egui::Ui) {
        
        let plot = Plot::new("histogram")
            .id(Id::new("histogram"))
            .view_aspect(2.0)
            .allow_scroll(false)
            .allow_boxed_zoom(false)
            .show_grid(false);
        let histogram = BarChart::new(self.bars.clone());

        plot.show(ui, |plot_ui|{
            plot_ui.bar_chart(histogram)
        });

        
    }

    fn init() -> Self where Self: Sized {
        BarPoints::new()
    }

}
