use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Lines;
use std::io::prelude::*;
use std::io::SeekFrom;
pub const BUFF_SIZE: usize = 4500;
pub const frequency: u16 = 40; 

#[derive(Clone)]
pub struct Buff{
    pub data:Vec<u16>,
    stackBuff:[u16;BUFF_SIZE],
}

impl ToGraphPoint for u16 {
    fn to_graph_point(&self, x: f64) -> GraphPoint {
        GraphPoint {
            x,
            y: (*self as f64) / (1024.0 / 60.0),
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
                Some(i) => self.data.push(lineHolder.slice(0 as usize..i).parse::<u16>().unwrap()),
                    //self.data.push(lineHolder.slice(0 as usize..lineHolder.find(".").unwrap()).parse::<u16>().unwrap());
                None => break,
            }
        }
    }
    pub fn updateStackBuff(&mut self){

    }
}




use std::ops::{Bound, RangeBounds};

use crate::graph::GraphPoint;
use crate::graph::ToGraphPoint;

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
