use crate::state::State;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub name: Option<String>,
    pub level: u8,
    pub skills: [u8; 4],
}

impl Default for Character {
    fn default() -> Self {
        Character {
            name: None,
            level: 1,
            skills: [1, 1, 1, 1],
        }
    }
}

impl State for Character {
    fn load_from_file(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open(file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let character: Character = serde_json::from_str(&buffer)?;
        Ok(character)
    }

    fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(self)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}
