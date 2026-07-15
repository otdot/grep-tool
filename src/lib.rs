use std::{fs::File, io::Read, path::PathBuf};
use anyhow::{Error, Result, Context};


use serde::Deserialize;

pub fn find_matches(pattern: &str, line: &String, mut writer: impl std::io::Write) {
	let line = line.trim();
	if line.contains(pattern) {
		let _ = writeln!(writer, "{}", line);
	}
}

#[derive(Deserialize)]
pub struct Config {
	pub search_file: PathBuf
}

pub fn load_config(config_path: &std::path::PathBuf) -> Result<Config, Error> {
    println!("Attempting to load configuration from: {:?}", config_path.display());

	let mut file = File::open(config_path).with_context(|| format!("could not read file `{}`", config_path.display()))?;
	// d = default
	let mut contents = String::new();

	file.read_to_string(&mut contents)?;

	let config: Config = toml::from_str(&contents)?;

	Ok(config)
}


#[test]
pub fn check_found_matches() {
	let line = String::from("tests are great\n");
	let mut result = Vec::new();
	crate::find_matches("test", &line, &mut result);
    assert_eq!(result, line.as_bytes())
}

#[test]
pub fn check_default_config() {
	let path = PathBuf::from("dev-config.toml");
	let config: Config = crate::load_config(&path).unwrap();
	assert_eq!(config.search_file.to_str(), Some("src/main.rs"))
}