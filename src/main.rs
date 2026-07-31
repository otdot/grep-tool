use std::{error::Error, fs::File, io::Read};

use anyhow::{Context, Result};
use clap::Parser;
use pwtool::{
    Cli, Command,
    models::{Config, PwEntry},
    persistence::get_persistence_layer,
};

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let config: Config = pwtool::load_config(&args.config_path)?;
    let persistence = get_persistence_layer(&config.vault_file_type);
    let pws: Vec<PwEntry> = persistence.load(&config.password_vault_path)?;

    match args.cmd {
        Command::Get { pattern } => {
            pwtool::get_pw(&pattern, pws)?;
        }
        Command::Set {
            key,
            username,
            password,
        } => {
            let new_pws = pwtool::set_pw(&key, &username, &password, pws)?;
            persistence.save(&config.password_vault_path, &new_pws)?;
            println!("Successfully saved new credentials for key '{}'.", &key);
        }
    }

    Ok(())
}
