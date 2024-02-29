use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub const BUFF_SIZE: usize = 4500;
pub const FREQUENCY: f64 = 40.0;

/// allows more polymorphic approach to storing different data typed to Data
pub enum TelemData {
    U32(u32),
    U32V(Vec<u32>),
    U32P((u32, u32)),
    U32PV(Vec<(u32, u32)>),
    F32(f32),
    F32V(Vec<f32>),
    F32PV(Vec<(f32, f32)>),
    F64(f64),
    F64V(Vec<f64>),
    PlotPointV(Vec<PlotPoint>),
    LineManager(LineManager),
}
/// Hash map containing multipil data entries
pub struct Data {
    /// fields, hashmap holding all telem data
    pub fields: HashMap<String, TelemData>,
}

impl Data {
    ///new empty Data
    pub fn new() -> Data {
        Data {
            fields: HashMap::new(),
        }
    }

    pub fn set(&mut self, field: String, value: TelemData) -> Result<(), &str> {
        if self.fields.get(&field).is_some() {
            return Err("Field already exists");
        }

        self.fields.insert(field, value);

        Ok(())
    }

    pub fn get(&self, field: String) -> Result<&TelemData, &str> {
        if let Some(field_boxed) = self.fields.get(&field) {
            return Ok(field_boxed);
        }

        Err("Field does not exist")
    }

    pub fn clear(&mut self) {
        self.fields.clear();
    }


    ///  returns average value of all points in the given data field
    ///  # Arguments
    /// * `field` - the string data field to calculate the average value for
    pub fn data_average(&self, field: String) -> f32 {
        let mut average = 0.0;
        let data_res = self.get(field);

        let mut data = None;

        if let Ok(TelemData::U32V(t_data)) = data_res {
            data = Some(t_data);
        }

        for (size, point) in data.unwrap().iter().enumerate() {
            average += (*point as f32 - average) / size as f32;
        }

        average
    }

    pub fn get_u32v(&self, field: String) -> &Vec<u32> {
        if let Ok(TelemData::U32V(res)) = self.get(field) {
            return &res;
        }

        panic!("Field does not exist");
    }

    pub fn get_f32(&self, field: String) -> f32 {
        if let Ok(TelemData::F32(res)) = self.get(field) {
            return *res;
        }

        panic!("Field does not exist");
    }

    pub fn get_f64pv(&self, field: String) -> &Vec<(f32, f32)> {
        if let Ok(TelemData::F32PV(res)) = self.get(field) {
            return &res;
        }

        panic!("Field does not exist");
    }


    /// generates a list of the turning points for a graph
    ///
    /// # Arguments
    ///
    /// * `field` - the lable of where the data will be stored in Data
    /// * `data` - the LineManager to take the line data from
    ///
    /// # Return
    /// result of adding the data generated to self
    pub fn set_turning_points(&mut self, field: String, data: LineManager) -> Result<(), &str> {
        let mut turning_points = Vec::new();

        let line_choice = data;
        let max_line_points =
            (line_choice.get_line_instance(0).unwrap().data.len() as f64 / FREQUENCY) * 20.0;
        let mut index = 1;
        while line_choice.get_line_instance(index).unwrap().data.len() as f64 >= max_line_points {
            index += 1;
        }
        let line_choice = line_choice.get_line_instance(index).unwrap().data.clone();
        let mut decreasing = false;
        turning_points.push(line_choice[0]);
        if turning_points[0].y > line_choice[1].y {
            decreasing = true;
        }
        let mut outer_index = 5;
        for plot_point in &line_choice[5..line_choice.len() - 5] {
            let (mut back_average, mut front_average) = (0.0, 0.0);

            for inner_index in 0..5 {
                front_average += plot_point.y - line_choice[outer_index + inner_index].y;
                back_average += plot_point.y - line_choice[outer_index - inner_index].y;
            }

            // if decreasing dosent match the direction that the graph is heading in flip it
            if decreasing == (back_average > front_average) {
                decreasing ^= true;
                turning_points.push(plot_point.clone());
            };

            outer_index += 1
        }
        self.set(field, TelemData::PlotPointV(turning_points))
    }

    /// sets sorts data in set Bins
    /// # Returns
    /// sorted data
    pub fn set_count(&mut self, field: String, data: &Vec<f32>, bin_count: usize, max_val: f64) -> Result<(), &str> {
        let mut data_count = vec![0u32; bin_count];
        for point in data.iter(){
            let mut index = ((*point as f64/max_val) * (bin_count as f64)).round() as usize;
            index = usize::clamp(index, 0, bin_count - 1);
            data_count[index] += 1;
        }

        self.set(field, TelemData::U32V(data_count))
    }

    pub fn remapped_1d(&mut self, data: &Vec<f32>, remap_info: &SuspensionRemapInfo) -> Vec<f32> {
        data.iter().map(|d| {
            let new_val = remap_info.remap(*d);
            new_val
        }).collect()
    }
    
    pub fn remapped_1d_with_clamp(&mut self, data: &Vec<f32>, remap_info: &SuspensionRemapInfo, min: f32, max: f32) -> Vec<f32> {
        data.iter().map(|d| {
            let new_val = f32::clamp(remap_info.remap(*d), min, max);
            new_val
        }).collect()
    }
    
    pub fn enumerated_with_transform<T: Copy>(&mut self, data: &Vec<T>, scale: f32, offset: f32) -> Vec<(f32, T)> {
        data.iter().enumerate().map(|(i, d)| {
            (i as f32 * scale + offset, *d)
        }).collect()
    }
}


pub const MAX_DATA_VALUE: f64 = 60.0;

#[derive(Clone)]

pub struct Buff {
    pub data: Vec<u32>,
    stackBuff: [u16; BUFF_SIZE],
}

impl ToPlotPoint for (u32, u32) {
    fn to_plot_point(&self) -> PlotPoint {
        PlotPoint {
            x: self.0 as f64,
            y: self.1 as f64,
        }
    }
}

impl ToPlotPoint for (f32, f32) {
    fn to_plot_point(&self) -> PlotPoint {
        PlotPoint {
            x: self.0 as f64,
            y: self.1 as f64,
        }
    }
}

impl Buff {
    /// constructs new Buff size BUFF_SIZE
    pub fn new() -> Self {
        Buff {
            data: Vec::new(),
            stackBuff: [0 as u16; BUFF_SIZE],
        }
    }

    pub fn getstackBuff(&self) -> [u16; BUFF_SIZE] {
        self.stackBuff
    }

    /// takes a String path [path] and returns instace of bufReader
    /// - [x] Load all data
    /// - [ ]  Save time data to allow easier referencing
    /// - [ ]  Implement rolling loading 
    pub fn load(&mut self, path : String){
        //does not filter out 
        self.data.clear();
        let file = File::open(path.trim()).unwrap();
        for line in io::BufReader::new(&file).lines() {
            let lineHolder = line.unwrap();
            match lineHolder.find(".") {
                Some(i) => self
                    .data
                    .push(lineHolder.slice(0 as usize..i).parse::<u32>().unwrap()),
                //self.data.push(lineHolder.slice(0 as usize..lineHolder.find(".").unwrap()).parse::<u16>().unwrap());
                None => break,
            }
        }
    }
}

use std::ops::{Add, Bound, Mul, RangeBounds};

use egui_plot::{Line, PlotPoint};

use crate::config_info::SuspensionRemapInfo;
use crate::graph::line_manager::LineManager;
use crate::graph::ToPlotPoint;

trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> &str;
    fn slice(&self, range: impl RangeBounds<usize>) -> &str;
}

impl StringUtils for str {
    fn substring(&self, start: usize, len: usize) -> &str {
        let mut char_pos = 0;
        let mut byte_start = 0;
        let mut it = self.chars();
        loop {
            if char_pos == start {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            } else {
                break;
            }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len {
                break;
            }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            } else {
                break;
            }
        }
        &self[byte_start..byte_end]
    }
    fn slice(&self, range: impl RangeBounds<usize>) -> &str {
        let start = match range.start_bound() {
            Bound::Included(bound) | Bound::Excluded(bound) => *bound,
            Bound::Unbounded => 0,
        };
        let len = match range.end_bound() {
            Bound::Included(bound) => *bound + 1,
            Bound::Excluded(bound) => *bound,
            Bound::Unbounded => self.len(),
        } - start;
        self.substring(start, len)
    }
}

