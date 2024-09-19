use crate::state::State;
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

impl State for Reports {
    fn load_from_file(&self) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open("reports.json")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let mut reports: Reports = serde_json::from_str(&buffer)?;
        reports.purple_reports = None;
        reports.exp = None;
        Ok(reports)
    }

    fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(self)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}
