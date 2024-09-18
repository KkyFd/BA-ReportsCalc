use crate::state::State;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Default)]
pub struct Character {
    name: Option<String>,
    level: Option<u8>,
    skills: Option<[u8; 4]>,
}

impl State for Character {
    fn load_from_file(&self,
    ) -> Result<Box<dyn State>, Box<dyn std::error::Error>> {
        let mut file = std::fs::File::open("character.json")?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        let mut character: Character = serde_json::from_str(&buffer)?;
        Ok(Box::new(character))
    }

    fn save_to_file(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data = serde_json::to_string_pretty(self)?;
        let mut file = fs::File::create(file_path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}

