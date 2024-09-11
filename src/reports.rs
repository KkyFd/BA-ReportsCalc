use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Default)]
pub struct Reports {
    pub quantities: [f32; 4],
    pub purple_reports: Option<f32>,
    pub exp: Option<f32>,
}

impl Reports {
    pub fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open(file_path)?;
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let mut reports: Reports = serde_json::from_str(&data)?;
        reports.purple_reports = None;
        reports.exp = None;
        Ok(reports)
    }

    pub fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(self)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}
