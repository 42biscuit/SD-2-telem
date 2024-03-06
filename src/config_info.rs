use std::{fs::{File, OpenOptions}, io::{BufReader, BufWriter}};
use serde::{Deserialize, Serialize};

pub const DEFAULT_SUS_MIN: f32 = 0.0;
pub const DEFAULT_SUS_MAX: f32 = 1024.0;
pub const DEFAULT_SUS_DIFF: f32 = DEFAULT_SUS_MAX - DEFAULT_SUS_MIN;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct SuspensionRemapInfo {
    pub stroke_len: f32,
    pub scale: f32,
    pub offset: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigInfo {
    pub sus_remap_info: Vec<SuspensionRemapInfo>,
    pub current_sus_remap_i: usize,
}  

impl SuspensionRemapInfo {
    pub fn calc_vals_from_min_and_max(&mut self, new_min: f32, new_max: f32) {
        let diff = new_max - new_min;
        self.scale = DEFAULT_SUS_DIFF / diff;
        self.offset = DEFAULT_SUS_MIN - new_min * self.scale;
    }

    pub fn remap(&self, val: f32) -> f32 {
        (val * self.scale + self.offset) * (self.stroke_len / DEFAULT_SUS_DIFF)
    }

    pub fn min(&self) -> f32 {
        self.remap(DEFAULT_SUS_MIN)
    }

    pub fn max(&self) -> f32 {
        self.remap(DEFAULT_SUS_MAX)
    }

    pub fn inverse(&self, val: f32) -> f32 {
        ((val / (self.stroke_len / DEFAULT_SUS_DIFF)) - self.offset) / self.scale
    }

    pub fn inverse_without_stroke_len_scale(&self, val: f32) -> f32 {
        (val - self.offset) / self.scale
    }
}

impl ConfigInfo {
    pub fn load() -> ConfigInfo {
        let file = File::open("config.json").unwrap();
        let buf_reader = BufReader::new(file);
        let config = serde_json::from_reader(buf_reader).unwrap();

        config
    }

    pub fn save(&self) {
        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open("config.json")
            .unwrap();

        let buf_writer = BufWriter::new(file);
        serde_json::to_writer(buf_writer, &self).unwrap();
    }

    pub fn current_sus_remap_info(&mut self) -> &mut SuspensionRemapInfo {
        &mut self.sus_remap_info[self.current_sus_remap_i]
    }
}