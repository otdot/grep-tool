use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PwEntry {
    pub id: String,
    pub key: String,
    pub url: String,
    pub username: String,
    pub password: String,
    pub note: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum VaultFileType {
    Json,
    Csv,
}

#[derive(Deserialize)]
pub struct Config {
    pub password_vault_path: PathBuf,
    pub vault_file_type: VaultFileType,
    pub encryption_key: String,
    pub auto_generated_pws: AutoGenPwConfig
}

#[derive(Deserialize)]
pub struct AutoGenPwConfig {
    pub word_length: u64,
    pub items: Vec<String>
}
