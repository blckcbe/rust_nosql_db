use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

// use std::io::{self, Read, Write};

#[derive(Serialize, Deserialize)]
pub struct Db {
    storage: HashMap<String, String>,
}

impl Db {
    pub fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.storage.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.storage.get(key)
    }

    pub fn delete(&mut self, key: &str) {
        self.storage.remove(key);
    }

    pub fn save(&self) -> io::Result<()> {
        let serialized = serde_json::to_string(&self.storage)?;
        std::fs::write("db.json", serialized)?;
        Ok(())
    }

    pub fn load() -> io::Result<Self> {
        let file = File::open("db.json");
        match file {
            Ok(mut file) => {
                let mut contents = String::new();
                file.read_to_string(&mut contents)?;
                let storage: HashMap<String, String> = serde_json::from_str(&contents)?;
                Ok(Self { storage })
            }
            Err(_) => Err(io::Error::new(io::ErrorKind::NotFound, "Database file not found")),
        }
    }
}
