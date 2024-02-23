use eframe::glow::PROGRAM_POINT_SIZE;
use egui_plot::{Line, PlotPoint, PlotPoints};

const MAX_POINTS: usize = 1024;

/// Represents a single level of detail, and should only be used as part of a LineManager. 
/// It stores a sampled copy of the original data at a lower resolution.
pub struct LineInstance {
    /// How often the instance has sampled from the original data, as an index of 2, (e.g. a period of 3 
    /// means it has taken every 8th point, since 2^3=8)
    pub period: u32,
    /// The sampled data
    pub data: Vec<PlotPoint>,
}

/// Stores a list of data points at multiple resolutions
pub struct LineManager {
    /// A vector containing the same line at different levels of detail. 
    /// Index 0 is original resolution, index 1 is half resolution (every 2nd point), etc.
    instances: Vec<LineInstance>,
}

impl LineInstance {
    /// Find the start and end indices that fill a data range
    /// 
    /// # Arguments
    /// 
    /// `min`: The low end of the range  
    /// `max`: The high end of the range
    /// 
    /// # Returns
    /// 
    /// A 2-tuple containing the start and end indices
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
    /// Create a new LineManager
    /// 
    /// # Arguments
    /// 
    /// `data`: A vector containing the points to be plotted
    /// 
    /// # Returns
    /// 
    /// A new LineManager
    pub fn new(data: Vec<PlotPoint>) -> LineManager {
        let data_len = data.len();
        let max_period_f = (data_len as f64 / MAX_POINTS as f64).log2();
        let max_period = u32::max(max_period_f as u32 + 2, 1);

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
        println!("data len {}", data.len());
        for i in instances.iter() {
            println!("{:?}", i.data.len());
        }

        LineManager {
            instances
        }
    }

    /// Generate a line at the appropriate resolution
    /// 
    /// # Arguments
    /// 
    /// `min`: The lowest visible value  
    /// `max`: The highest visible value
    /// 
    /// # Returns
    /// 
    /// Some(Line) if a line could be created  
    /// None otherwise
    pub fn gen_line(&self, min: f64, max: f64) -> Option<Line> {
        for i in &self.instances {
            let indices = i.get_points_in_range(min, max);
            let line_len = indices.1 - indices.0;

            if indices.1 - indices.0 > MAX_POINTS {
                continue;
            }

            let mut line_points = vec![PlotPoint { x: 0.0, y: 0.0 }; line_len];

            line_points.clone_from_slice(&i.data[indices.0..indices.1]);
            
            //println!("LOD: {}, Points: {}", i.period, line_len);

            return Some(Line::new(PlotPoints::Owned(line_points)));
        }

        None
    }

    /// Get the maximum X co-ordinate of a line
    pub fn max_x(&self) -> f64 {
        if self.instances.len() == 0 {
            return 0.0;
        }

        if let Some(last_point) = self.instances[0].data.last() {
            return last_point.x;
        }

        0.0
    }
}