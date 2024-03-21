use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::ops::{Bound, RangeBounds};

pub struct RawPotData {
    pub remap_ref: String,
    pub polling_rate: u32,
    pub data: Vec<u32>,
}

pub struct Loader {
    pub raw_pot_datas: HashMap<String, RawPotData>,
}

impl Loader {
    pub fn new() -> Loader {
        Loader {
            raw_pot_datas: HashMap::new(),
        }
    }

    pub fn get_raw_pot_data(&self, key: String) -> &RawPotData {
        self.raw_pot_datas.get(&key).expect("Error: Data not found")
    }

    /// takes a String path [path] and returns instace of bufReader
    /// - [x] Load all data
    /// - [ ]  Save time data to allow easier referencing
    /// - [ ]  Implement rolling loading 
    pub fn load(&mut self, path: String) {
        self.raw_pot_datas.clear();

        let file = File::open(path.trim()).unwrap();
        let mut lines = io::BufReader::new(&file).lines();
        let first_line = lines.next().unwrap().unwrap();

        let mut pot_data_is = Vec::<String>::new();
        let metadata_iter = first_line.split(',');

        for md in metadata_iter {
            let mut tag_rate_iter = md.split(':');
            let tag = tag_rate_iter.next().expect("Error: Invalid metadata");
            let rate = tag_rate_iter.next().expect("Error: Invalid metadata").parse::<u32>().expect("Error: Invalid polling rate in metadata");
            let remap_ref = tag_rate_iter.next().expect("Error: Invalid metadata").to_string();

            pot_data_is.push(tag.to_string());
            self.raw_pot_datas.insert(tag.to_owned(), RawPotData {
                remap_ref, polling_rate: rate, data: Vec::new()
            });
        }

        //does not filter out 
        for line in lines {
            
            let lineHolder = line.unwrap();
            let vals = lineHolder.split(',');
            
            for (i, val) in vals.enumerate() {
                if i >= 6 {
                    self.raw_pot_datas.get_mut(&pot_data_is[i - 6]).unwrap().data.push(truncate_val(val));
                }
            }
        }
    }
}

fn truncate_val(val_str: &str) -> u32 {
    let end_i = val_str.find(".").unwrap_or(val_str.len());
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