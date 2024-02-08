use egui_plot::{Line, PlotPoint, PlotPoints};
use tracing_subscriber::filter::combinator::Or;

const MAX_POINTS: usize = 1000;

pub struct LineInstance {
    pub period: u32,
    pub data: Vec<PlotPoint>,
}

pub struct LineManager {
    instances: Vec<LineInstance>,
}

impl LineInstance {
    pub fn get_points_in_range(&self, min: f64, max: f64) -> (usize, usize) {
        if self.data.len() == 0 {
            return (0, 0);
        }

        let step = 2usize.pow(self.period);
    
        let min_i = f64::max(min, 0.0) as usize;
        let max_i = f64::max(max, 0.0) as usize;
        
        let mut low_i = min_i / step;
        let mut high_i = max_i / step + 1;

        low_i = low_i.min(self.data.len() - 1);
        high_i = high_i.min(self.data.len() - 1);

        if low_i > high_i {
            return (0, self.data.len() - 1);
        }

        (low_i, high_i)
    }
}

impl LineManager {
    pub fn new(data: &Vec<PlotPoint>) -> LineManager {
        let max_period_f = (data.len() as f64 / MAX_POINTS as f64).log2();
        let max_period = u32::max(max_period_f as u32 + 1, 1);

        let mut instances = Vec::<LineInstance>::new();
        
        for i in 0..max_period {
            instances.push(LineInstance {
                period: i,
                data: Vec::with_capacity(2usize.pow(i) * MAX_POINTS),
            })
        }

        for (i, point) in data.iter().enumerate() {
            for j in 0..max_period {
                if i % 2usize.pow(j) == 0 {
                    instances[j as usize].data.push(*point);
                }
            }
        }

        LineManager {
            instances
        }
    }

    pub fn gen_line(&self, min: f64, max: f64) -> Option<Line> {
        for i in &self.instances {
            let indices = i.get_points_in_range(min, max);
            let line_len = indices.1 - indices.0;

            if indices.1 - indices.0 > MAX_POINTS {
                continue;
            }

            let mut line_points = vec![PlotPoint { x: 0.0, y: 0.0 }; line_len];

            line_points.clone_from_slice(&i.data[indices.0..indices.1]);
            println!("{}", line_len);

            return Some(Line::new(PlotPoints::Owned(line_points)));
        }

        None
    }
}