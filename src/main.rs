use std::{fs::File, io::{BufRead, BufReader}};

use anyhow::{Context, Result};
use clap::{Parser};

#[derive(Parser)]
struct Cli {
	pattern: String,
	path: std::path::PathBuf,
}

fn main() -> Result<()> {
	let args = Cli::parse();

	let file = File::open(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
	let reader = BufReader::new(file);

	for line_result in reader.lines() {
		let line = line_result?;
		grep_tool::find_matches(&args.pattern, &line, &mut std::io::stdout());
	}
	
	Ok(())
}