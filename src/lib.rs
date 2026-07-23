use std::{fs::File, io::Read, path::PathBuf};
use anyhow::{Error, Result, Context};
use serde::Deserialize;

pub fn find_matches(pattern: &str, pw_entry: PwEntry, result: &mut Vec<PwEntry>) {
	if matches_to_entry(pattern, &pw_entry) {
		result.push(pw_entry);
	}
}

fn matches_to_entry(pattern: &str, pw_entry: &PwEntry) -> bool {
	pw_entry.key.contains(pattern) || pw_entry.username.contains(pattern) || pw_entry.id == pattern
}

#[derive(Deserialize)]
pub struct Config {
	pub search_file: PathBuf
}

pub fn load_config(config_path: &std::path::PathBuf) -> Result<Config, Error> {
    println!("Attempting to load configuration from: {:?}", config_path.display());

	let mut file = File::open(config_path).with_context(|| format!("could not read file `{}`", config_path.display()))?;
	let mut contents = String::new();

	file.read_to_string(&mut contents)?;

	let config: Config = toml::from_str(&contents)?;

	Ok(config)
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct PwEntry {
	pub id: String,
	pub key: String,
	pub username: String,
	pub password: String,
}


#[test]
pub fn check_found_matches() {
	let pw_entry: PwEntry = serde_json::from_str("{
        \"id\": \"a1\",
        \"key\": \"my-school\",
        \"username\": \"user1\",
        \"password\": \"pass1\"
    }").unwrap();
	let mut result = vec![];
	crate::find_matches("school", pw_entry, &mut result);

	assert_eq!(result.first().unwrap().id, "a1");
}

#[test]
pub fn check_default_config() {
	let path = PathBuf::from("test-config.toml");
	let config: Config = crate::load_config(&path).unwrap();
	assert_eq!(config.search_file.to_str(), Some("test-sample.json"))
}