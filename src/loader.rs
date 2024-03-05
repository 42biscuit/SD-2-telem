use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::ops::{Bound, RangeBounds};

pub struct Loader {
    pub polling_rate: u32,
    pub front_sus_data: Vec<u32>,
    pub rear_sus_data: Vec<u32>,
    pub front_brake_data: Option<Vec<u32>>,
    pub rear_brake_data: Option<Vec<u32>>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader {
            polling_rate: 0,
            front_sus_data: Vec::new(),
            rear_sus_data: Vec::new(),
            front_brake_data: None,
            rear_brake_data: None,
        }
    }
    /// takes a String path [path] and returns instace of bufReader
    /// - [x] Load all data
    /// - [ ]  Save time data to allow easier referencing
    /// - [ ]  Implement rolling loading 
    pub fn load(&mut self, path: String) {
        let file_for_count = File::open(path.trim()).unwrap();
        let line_count = io::BufReader::new(&file_for_count).lines().count();

        let file = File::open(path.trim()).unwrap();
        let mut lines = io::BufReader::new(&file).lines();
        let first_line = lines.next().unwrap().unwrap();

        let mut metadata_iter = first_line.split(',');
        self.polling_rate = metadata_iter.next().unwrap().parse::<u32>().unwrap();
        let contains_brake_data = metadata_iter.next().unwrap().parse::<u32>().unwrap() == 1;
        
        self.front_sus_data = Vec::<u32>::with_capacity(line_count);
        self.rear_sus_data = Vec::<u32>::with_capacity(line_count);

        (self.rear_brake_data, self.front_brake_data) = match contains_brake_data {
            true => (Some(Vec::<u32>::new()), Some(Vec::<u32>::new())),
            false => (None, None)
        };

        //does not filter out 
        for line in lines {
            let lineHolder = line.unwrap();
            let mut vals = lineHolder.split(',');
            self.rear_sus_data.push(truncate_val(vals.next().unwrap()));
            self.front_sus_data.push(truncate_val(vals.next().unwrap()));

            if contains_brake_data {
                self.rear_brake_data.as_mut().unwrap().push(truncate_val(vals.next().unwrap()));
                self.front_brake_data.as_mut().unwrap().push(truncate_val(vals.next().unwrap()));
            }
        }
    }
}

fn truncate_val(val_str: &str) -> u32 {
    let end_i = val_str.find(".").unwrap();
    val_str.slice(0 as usize..end_i).parse::<u32>().unwrap()
}

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