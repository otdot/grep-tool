use crate::{models::PwEntry, persistence::PersistenceLayer};
use anyhow::{Result, anyhow};
use csv::{ReaderBuilder, WriterBuilder};
use std::{error::Error, path::PathBuf};

pub struct CsvPersistence;

impl PersistenceLayer for CsvPersistence {
    fn load(&self, path: &PathBuf) -> Result<Vec<PwEntry>, Box<dyn Error>> {
        let mut reader = ReaderBuilder::new()
            .has_headers(true) // Assumes the first row is headers
            .from_path(path)?;

        let mut credentials = Vec::new();

        // Read records row by row
        for result in reader.records() {
            let record = result.map_err(|e| anyhow!("CSV Read Error: {}", e))?;

            // Assuming the order in CSV is: id, username, password_hash
            if record.len() >= 4 {
                let cred = PwEntry {
                    id: record.get(0).ok_or("Missing ID")?.to_string(),
                    key: record.get(1).ok_or("Missing key")?.to_string(),
                    username: record.get(2).ok_or("Missing Username")?.to_string(),
                    password: record.get(3).ok_or("Missing Password Hash")?.to_string(),
                };
                credentials.push(cred);
            } else {
                let id = record.get(0).ok_or("Missing ID")?.to_string();
                let key = record.get(1).ok_or("Missing key")?.to_string();
                println!("record with missing fields found. id: {}, key: {}", id, key);
            }
        }

        Ok(credentials)
    }

    fn save(&self, path: &PathBuf, pws: &Vec<PwEntry>) -> Result<()> {
        let file = std::fs::File::create(path).map_err(|e| {
            anyhow!(
                "Failed to open file: {}, please make sure the file has been created in {}",
                e,
                &path.display()
            )
        })?;
        let mut wtr = WriterBuilder::new()
            .has_headers(true) // Write headers
            .from_writer(file);

        // Write headers (important for schema definition)
        wtr.write_record(&["id", "key", "username", "password"])?;

        for cred in pws {
            wtr.write_record(&[&cred.id, &cred.key, &cred.username, &cred.password])?;
        }

        wtr.flush()?;
        Ok(())
    }
}
