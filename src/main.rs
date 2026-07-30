use std::{error::Error, fs::File, io::Read};

use anyhow::{Context, Result};
use clap::Parser;
use pwtool::{Cli, Command, Config, PwEntry};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let config: Config = pwtool::load_config(&args.config_path)?;

    let mut file = File::open(&config.search_file)
        .with_context(|| format!("could not read file `{}`", &config.search_file.display()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let pws: Vec<PwEntry> = serde_json::from_str(&contents)?;
    match args.cmd {
        Command::Get { pattern } => {
            pwtool::get_pw(&pattern, pws)?;
        }
        Command::Set {
            key,
            username,
            password,
        } => {
            let _ = pwtool::set_pw(&key, &username, &password, pws, &config.search_file);
        }
    }

    Ok(())
}
