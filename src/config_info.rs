use std::{collections::HashMap, fs::{File, OpenOptions}, io::{BufReader, BufWriter}};
use serde::{Deserialize, Serialize};

pub const DEFAULT_SUS_MIN: f32 = 0.0;
pub const DEFAULT_SUS_MAX: f32 = 1024.0;
pub const DEFAULT_SUS_DIFF: f32 = DEFAULT_SUS_MAX - DEFAULT_SUS_MIN;
pub const MAPPED_MAX: f32 = 100.0;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct SuspensionRemapInfo {
    pub stroke_len: f32,
    pub scale: f32,
    pub offset: f32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ConfigInfo {
    pub sus_remap_info: HashMap<String, SuspensionRemapInfo>,
}

impl Default for SuspensionRemapInfo {
    fn default() -> SuspensionRemapInfo {
        SuspensionRemapInfo {
            stroke_len: 100.0,
            scale: 1.0,
            offset: 0.0,
        }
    }
}  

impl SuspensionRemapInfo {
    pub fn calc_vals_from_min_and_max(&mut self, new_min: f32, new_max: f32) {
        let diff = new_max - new_min;
        self.scale = DEFAULT_SUS_DIFF / diff;
        self.offset = DEFAULT_SUS_MIN - new_min * self.scale;
    }

    pub fn remap(&self, val: f32) -> f32 {
        (val * self.scale + self.offset) * (MAPPED_MAX / DEFAULT_SUS_DIFF)
    }

    pub fn min(&self) -> f32 {
        self.remap(DEFAULT_SUS_MIN)
    }

    pub fn max(&self) -> f32 {
        self.remap(DEFAULT_SUS_MAX)
    }

    pub fn inverse(&self, val: f32) -> f32 {
        ((val / (MAPPED_MAX / DEFAULT_SUS_DIFF)) - self.offset) / self.scale
    }

    pub fn inverse_without_stroke_len_scale(&self, val: f32) -> f32 {
        (val - self.offset) / self.scale
    }
}

impl ConfigInfo {
    pub fn load_blank() -> ConfigInfo {
        ConfigInfo {
            sus_remap_info: HashMap::new(),
        }
    }

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
        serde_json::to_writer(buf_writer, &self).expect("Error saving to config file");
    }

    pub fn add_sus_remap_info(&mut self, key: String, info: SuspensionRemapInfo) {
        self.sus_remap_info.insert(key, info);
    }

    pub fn set_sus_remap_info(&mut self, key: String, info: SuspensionRemapInfo) {
        self.sus_remap_info.insert(key, info).expect("Error: Suspension remap info not found");
    }

    pub fn get_sus_remap_info(&self, key: String) -> SuspensionRemapInfo {
        *self.sus_remap_info.get(&key).expect("Error: Suspension remap info not found")
    }
}