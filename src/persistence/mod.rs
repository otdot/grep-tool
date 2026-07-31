mod csv_persistence;
mod json_persistence;

use anyhow::Result;
use std::{
    error::Error,
    path::{PathBuf},
};

use crate::models::{PwEntry, VaultFileType};

// Function to select the right persistence layer based on extension
pub fn get_persistence_layer(f_type: &VaultFileType) -> Box<dyn PersistenceLayer> {
    match f_type {
        VaultFileType::Json => Box::new(json_persistence::JsonPersistence),
        VaultFileType::Csv => Box::new(csv_persistence::CsvPersistence),
    }
}

pub trait PersistenceLayer {
    /// Loads credentials from the given path.
    fn load(&self, path: &PathBuf) -> Result<Vec<PwEntry>, Box<dyn Error>>;
    /// Saves credentials to the given path.
    fn save(&self, path: &PathBuf, pws: &Vec<PwEntry>) -> Result<()>;
}
