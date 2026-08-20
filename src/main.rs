use std::error::Error;

use anyhow::Result;
use clap::Parser;
use magic_crypt::new_magic_crypt;
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
    let mc = new_magic_crypt!(&config.encryption_key, 256);

    match args.cmd {
        Command::Get { pattern } => {
            pwtool::get_pw(&pattern, pws, mc)?;
        }
        Command::Set {
            key,
            url,
            username,
            password,
            note,
        } => {
            let pw = match &password {
                Some(user_pw) => user_pw,
                _ => &pwtool::generate_password_from_config(&config)?.join(""),
            };
            let new_pws = pwtool::set_pw(&key, &url, &username, &pw, &note, pws, mc)?;
            persistence.save(&config.password_vault_path, &new_pws)?;
            println!("Successfully saved new credentials for key '{}'.", &key);
        }
    }

    Ok(())
}
