use std::{fs::File, io::{BufRead, BufReader}};

use anyhow::{Context, Result};
use clap::{Parser};
use grep_tool::Config;

#[derive(Parser)]
struct Cli {
	pattern: String,
	#[arg(default_value = "dev-config.toml")]
	config_path: std::path::PathBuf,
}

fn main() -> Result<()> {
	let args = Cli::parse();

	let config: Config = grep_tool::load_config(&args.config_path)?;

	let file = File::open(&config.search_file).with_context(|| format!("could not read file `{}`", &config.search_file.display()))?;
	let reader = BufReader::new(file);

	for line_result in reader.lines() {
		let line = line_result?;
		grep_tool::find_matches(&args.pattern, &line, &mut std::io::stdout());
	}
	
	Ok(())
}