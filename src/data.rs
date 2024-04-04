use std::collections::HashMap;
use std::thread::current;
use crate::graph::to_plot_points;
use std::fs::File;
use std::{io, thread};
use std::io::prelude::*;

pub const BUFF_SIZE: usize = 4500;
pub const FREQUENCY: f32 = 250.0;

///The minimum period of a compression + rebound in the data. Used for turning point detection
pub const MIN_PERIOD: f64 = 0.1; 
/// allows more polymorphic approach to storing different data typed to Data
#[allow(dead_code)]
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

    pub fn data_average_raw(&self , data:&Vec<f32>)-> f32{
        let mut average = 0.0;
        for (size, point) in data.iter().enumerate() {
            average += (point - average) / (size as f32+1.0);
        }
        average
    }

    pub fn get_line_manager(&self, field: String) -> &LineManager {
        if let Ok(TelemData::LineManager(res)) = self.get(field) {
            return &res;
        }

        panic!("Field does not exist");
    }

    pub fn get_u32v(&self, field: String) -> &Vec<u32> {
        if let Ok(TelemData::U32V(res)) = self.get(field) {
            return &res;
        }

        panic!("Field does not exist");
    }   
    
    pub fn get_f32v(&self, field: String) -> &Vec<f32> {
        if let Ok(TelemData::F32V(res)) = self.get(field) {
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

    pub fn get_f32_err(&self, field: String) -> Option<f32> {
        if let Ok(TelemData::F32(res)) = self.get(field) {
            return Some(*res);
        }
        None
    }

    pub fn get_f64v(&self, field: String) -> &Vec<f64> {
        if let Ok(TelemData::F64V(res)) = self.get(field) {
            return &res;
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
    pub fn set_turning_points(&mut self, rebounds_field: String, compressions_field: String, turning_point_field: String, data: &Vec<f32>, front:bool) -> Result<(), &str> {
        let mut turning_points = Vec::new();

        let line_choice = data;

        let turning_range = ((FREQUENCY * MIN_PERIOD as f32)/2.0) as usize;
        let mut outer_index = turning_range.clone() ;

        let mut decreasing = false;
        turning_points.push((0.0,line_choice[turning_range]));
        if turning_points[0].1 > line_choice[5] {
            decreasing = true;
        }

        let mut median = line_choice.clone(); 
        for outer in (turning_range/2)..line_choice.len()-(turning_range/2) {
            median[outer] = self.data_average_raw(&data[(outer - turning_range/2)..(outer + turning_range/2)].to_vec())
        }

        for plot_point in &line_choice[turning_range..line_choice.len() - turning_range] {
            let (mut back_average, mut front_average) = (0.0, 0.0);
            let last_point = turning_points.last().unwrap().1;
            for inner_index in 0..turning_range {
                front_average += (plot_point - line_choice[outer_index + inner_index])as f32;
                back_average += (plot_point - line_choice[outer_index - inner_index]) as f32;
            }

            // if decreasing dosent match the direction that the graph is heading in flip it
            if (decreasing == (back_average > front_average +  (if decreasing == true{-15.0 }else{15.0 }))) && (plot_point - last_point).abs() > 1.0{
                decreasing ^= true;
                turning_points.push((outer_index as f32 / FREQUENCY,plot_point.clone()));
            };

            outer_index += 1
        }
 
        //line_choice[last..current].to_vec()
        
        let mut compressions = Vec::new();
        let mut rebounds = Vec::new();
        let mut last = turning_points[0];
        for point in 1..(turning_points.len()-1){   // +ve disp and it is a rebound
            let current_disp = turning_points[point].1 - last.1;
            let mut max_speed = 0.0_f32;
            let mut last_point_y = line_choice[(last.0 * FREQUENCY) as usize];
            for inner in line_choice[1 + (last.0 * FREQUENCY) as usize..(turning_points[point].0 * FREQUENCY) as usize].to_vec(){
                if (inner - last_point_y).abs() > max_speed.abs(){
                    max_speed = (inner - last_point_y).abs()
                }
                last_point_y = inner
            } 
            if current_disp < 0.0{

                compressions.push((current_disp.abs(),max_speed.abs()));
            }else{
                rebounds.push((current_disp.abs(),max_speed.abs()));
            }
            last = turning_points[point];
        }
        
        println!("{}, {}", compressions.len(), rebounds.len());
        //self.set_displacements(displacements_field, &turning_points).unwrap();
        self.set(compressions_field, TelemData::F32PV(compressions)).unwrap();
        self.set(rebounds_field, TelemData::F32PV(rebounds)).unwrap();
        

        self.set(turning_point_field, TelemData::PlotPointV(to_plot_points(&turning_points))) 
    }

    /// Sets the Displacement value for the given data
    ///
    /// # Arguments
    /// * 'save_field' the feild to save the calculated displacements under
    /// * 'data' plot poinnts of the data to be used
    /// # Return 
    ///
    /// result of pushing data to the HashMap
    pub fn set_displacements(&mut self, save_field: String, data:&Vec<(f32,f32)> ) -> Result<(), &str>{
        let mut displacements = Vec::new();
        let mut last = data[0];
        for point in 1..(data.len()-1){
            displacements.push((data[point].1 - last.1)as f64);
            last = data[point];
        }
        /*let temp = data.clone();
        let mut  dataC = temp.clone().iter();
        dataC.next();f
        displacements = data.iter().zip(dataC).map(|(x1,x2)| x1.y - x2.y).collect();*/
        println!("disp: {:?}",displacements);
        self.set(save_field, TelemData::F64V(displacements))
    }



    /// sets sorts data in set Bins
    /// # Returns
    /// sorted data
    pub fn set_count(&mut self, field: String, data: &Vec<f32>, bin_count: usize, max_val: f64, reverse: bool) -> Result<(), &str> {
        let mut data_count = vec![0u32; bin_count];
        for point in data.iter(){
            let mut index = ((*point as f64/max_val) * (bin_count as f64-1.0)).round() as usize;
            index = usize::clamp(index, 0, bin_count - 1);
            if reverse {index = bin_count -1 - index ;}
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

    pub fn remapped_1d_with_clamp_no_offset(&mut self, data: &Vec<f32>, remap_info: &SuspensionRemapInfo, min: f32, max: f32) -> Vec<f32> {
        data.iter().map(|d| {
            let new_val = f32::clamp(remap_info.remap_no_offset(*d), min, max);
            new_val
        }).collect()
    }
    
    pub fn enumerated_with_transform<T: Copy>(&mut self, data: &Vec<T>, scale: f32, offset: f32) -> Vec<(f32, T)> {
        data.iter().enumerate().map(|(i, d)| {
            (i as f32 * scale + offset, *d)
        }).collect()
    }
}



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

impl ToPlotPoint for (f64, f64) {
    fn to_plot_point(&self) -> PlotPoint {
        PlotPoint {
            x: self.0 as f64,
            y: self.1 as f64,
        }
    }
}

impl ToPlotPoint for (&f64, &f64) {
    fn to_plot_point(&self) -> PlotPoint {
        PlotPoint {
            x: *self.0 as f64,
            y: *self.1 as f64,
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
    pub fn to_f32v(&mut self)-> Vec<f32>{
        let mut r = Vec::new();
        for i in &self.data{
            r.push(*i as f32)
        }
        r
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

