use std::fs::File;
use std::io::{Read, Write, Result};

pub struct FileHandler;

impl FileHandler {
    pub fn read_file(path: &str) -> Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    pub fn save_to_file(path: &str, data: &[u8]) -> Result<()> {
        let mut file = File::create(path)?;
        file.write_all(data)?;
        Ok(())
    }
}
