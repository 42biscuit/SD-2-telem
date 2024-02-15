use std::fs::File;
use std::io;
use std::io::prelude::*;
use crate::graph::bar_graph;
pub const BUFF_SIZE: usize = 4500;
pub const frequency: u16 = 40; 
pub const MAX_DATA_VALUE:f64 = 60.0;


#[derive(Clone)]
pub struct Buff{
    pub data:Vec<u32>,
    stackBuff:[u16;BUFF_SIZE],
}

impl ToPlotPoint for (u32, u32) {
    fn to_plot_point(&self) -> PlotPoint {
        PlotPoint {
            x: self.0 as f64,
            y: (self.1 as f64) / (1024.0 / MAX_DATA_VALUE),
        }
    }
}

impl Buff{
    /// constructs new Buff size BUFF_SIZE
    pub fn new()->Self{
        Buff{
            data:Vec::new(),
            stackBuff: [0 as u16;BUFF_SIZE],
        }
    }

    pub fn getstackBuff(&self) -> [u16;BUFF_SIZE]{
        self.stackBuff
    }

    /// takes a String path [path] and returns instace of bufReader
    /// - [x] Load all data
    /// - [ ]  Save time data to allow easier referencing
    /// - [ ]  Implement rolling loading 
    pub fn load(&mut self, path : String){
        //does not filter out 
        let file = File::open(path.trim()).unwrap();
        for line in io::BufReader::new(&file).lines(){
            let lineHolder = line.unwrap();
            match lineHolder.find("."){
                Some(i) => self.data.push(lineHolder.slice(0 as usize..i).parse::<u32>().unwrap()),
                    //self.data.push(lineHolder.slice(0 as usize..lineHolder.find(".").unwrap()).parse::<u16>().unwrap());
                None => break,
            }
        }
        let _ = bar_graph::BarPoints::new(self.data.clone());
    }
}

use std::ops::{Bound, RangeBounds};

use egui_plot::PlotPoint;

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
            if char_pos == start { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_start += c.len_utf8();
            }
            else { break; }
        }
        char_pos = 0;
        let mut byte_end = byte_start;
        loop {
            if char_pos == len { break; }
            if let Some(c) = it.next() {
                char_pos += 1;
                byte_end += c.len_utf8();
            }
            else { break; }
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
