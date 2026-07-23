use std::{error::Error, fs::File, io::{Read}};

use anyhow::{Context, Result};
use clap::{Parser};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use pwtool::{Config, PwEntry};


#[derive(Parser)]
struct Cli {
	pattern: String,
	// replace dev-config.toml with ~/.pwtool/config.toml before builds
	#[arg(default_value = "dev-config.toml")]
	config_path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
	let args = Cli::parse();

	let config: Config = pwtool::load_config(&args.config_path)?;

	let mut file = File::open(&config.search_file).with_context(|| format!("could not read file `{}`", &config.search_file.display()))?;

	let mut contents = String::new();
	file.read_to_string(&mut contents)?;

	let pws: Vec<PwEntry> = serde_json::from_str(&contents)?;
	let mut result: Vec<PwEntry> = vec![];
	for pw_entry in pws {
		pwtool::find_matches(&args.pattern, pw_entry, &mut result);
	}

	let list_len = result.len();
	let mut ctx = ClipboardContext::new()?;

	if list_len == 0 {
		println!("no entries found with given search argument: {}", &args.pattern);
	} else if list_len == 1 {
		println!("Found one entry for search argument: {}", &args.pattern);
		let found_entry: &PwEntry = result.get(0).unwrap();
		println!("{}", &found_entry.password);
		ctx.set_contents(found_entry.password.to_owned())?;
		// wait for OS to receive the signal for adding contents to clipboard 
		std::thread::sleep(std::time::Duration::from_millis(50)); 
	} else {
		println!("Found multiple entries for search argument: {}. Please choose preferred entry.", &args.pattern);
	}

	Ok(())
}