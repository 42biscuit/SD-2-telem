use egui_plot::{Line, PlotPoint};

const MAX_POINTS: usize = 1000;

pub struct LineInstance {
    pub period: u32,
    pub data: Vec<PlotPoint>,
}

pub struct LineManager {
    instances: Vec<LineInstance>,
}

fn find_point_in_vec(data: &Vec<PlotPoint>, x_val: f64) -> Option<usize> {
    const MAX_ITERATIONS: u32 = 50;
    const COMPARE_MAX: f64 = 0.0005;

    if data.len() == 0 {
        return None;
    }

    let mut iterations = 0;
    let mut final_i: Option<usize> = None;

    let mut low_i: usize = 0;
    let mut high_i: usize = data.len();
    while iterations < MAX_ITERATIONS {
        iterations += 1;

        let mid_i = (low_i + high_i) / 2;
        let mid_val = data[mid_i].x;

        if x_val - mid_val < COMPARE_MAX {
            final_i = Some(mid_i);

            break;
        } else if x_val > data[mid_i].x {
            low_i = mid_i;
        } else {
            high_i = mid_i;
        }
    }

    final_i
}

impl LineInstance {
    pub fn get_points_in_range(&self, min: f64, max: f64) -> (usize, usize) {
        if self.data.len() == 0 {
            return (0, 0);
        }

        let low_i = find_point_in_vec(&self.data, min).unwrap_or(0);
        let high_i = find_point_in_vec(&self.data, max).unwrap_or(self.data.len() - 1);

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
                period: i + 1,
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

    pub fn gen_line(&self, min: f64, max: f64) {
        for i in &self.instances {
            i.get_points_in_range(min, max);
        }
    }
}