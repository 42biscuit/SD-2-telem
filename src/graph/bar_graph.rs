use crate::data::MAX_DATA_VALUE;
use egui_plot::Bar;
pub const NUMBER_BARS:usize = 15;

pub struct BarPoints{
    data:[u32; NUMBER_BARS],
    bars: Vec<Bar>,
}

impl BarPoints{
    pub fn new(data: Vec<u32>) -> BarPoints{
        let mut chart = BarPoints{
            data: [0; NUMBER_BARS],
            bars:Vec::new(),
        };
        for point in data.iter(){
            let index = ((*point as f64/(1024.0 / MAX_DATA_VALUE)) * (NUMBER_BARS as f64/MAX_DATA_VALUE)).round() as usize;
            chart.data[index-1] += 1;
        }
        println!("{:?}",chart.data);
        chart
    }
}