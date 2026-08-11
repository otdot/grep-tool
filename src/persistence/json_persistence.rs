use crate::{models::PwEntry, persistence::PersistenceLayer};
use anyhow::{Result, anyhow};
use magic_crypt::MagicCrypt256;
use serde_json;
use std::{
    error::Error,
    path::{PathBuf},
};

pub struct JsonPersistence;

impl PersistenceLayer for JsonPersistence {
    fn load(&self, path: &PathBuf) -> Result<Vec<PwEntry>, Box<dyn Error>> {
        let file_content = std::fs::read_to_string(path)?;
        let creds: Vec<PwEntry> = serde_json::from_str(&file_content)
            .map_err(|e| anyhow!("Failed to deserialize JSON: {}", e))?;
        Ok(creds)
    }

    fn save(&self, path: &PathBuf, pws: &Vec<PwEntry>) -> Result<()> {
        let file_content = serde_json::to_string_pretty(pws)
            .map_err(|e| anyhow!("Failed to serialize to JSON: {}", e))?;
        std::fs::write(path, file_content)?;
        Ok(())
    }
}
