pub mod models;
pub mod persistence;

use anyhow::{Context, Result};
use magic_crypt::{MagicCrypt, MagicCrypt256, MagicCryptTrait};
use std::{error::Error, fs::File, io::Read};

use clap::{Parser, Subcommand};
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use std::io::{self};
use uuid::Uuid;

use crate::models::{Config, PwEntry};

pub fn find_matches(pattern: &str, pw_entry: PwEntry, result: &mut Vec<PwEntry>) {
    if matches_to_entry(pattern, &pw_entry) {
        result.push(pw_entry);
    }
}

fn matches_to_entry(pattern: &str, pw_entry: &PwEntry) -> bool {
    pw_entry.key.contains(pattern) || pw_entry.username.contains(pattern) || pw_entry.id == pattern
}

pub fn load_config(config_path: &std::path::PathBuf) -> Result<Config, Box<dyn Error>> {
    println!(
        "Attempting to load configuration from: {:?}",
        config_path.display()
    );

    let mut file = File::open(config_path)
        .with_context(|| format!("could not read file `{}`", config_path.display()))?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents)?;

    Ok(config)
}

pub fn is_valid_input(input: &str) -> bool {
    let trimmed = input.trim();
    !trimmed.is_empty() && trimmed.parse::<usize>().is_ok()
}

pub fn copy_to_clipboard(val: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(val.to_owned())?;
    std::thread::sleep(std::time::Duration::from_millis(50));
    Ok(())
}

#[derive(Parser)]
pub struct Cli {
    #[arg(long, default_value = "dev-config.toml", alias = "c")]
    pub config_path: std::path::PathBuf,
    #[command(subcommand)]
    pub cmd: Command,
    // replace dev-config.toml with ~/.pwtool/config.toml before builds
}

#[derive(Subcommand)]
pub enum Command {
    Set {
        #[arg(long, alias = "k")]
        key: String,
        #[arg(long, default_value = "")]
        url: String,
        #[arg(long, alias = "u")]
        username: String,
        #[arg(long, alias = "p")]
        password: String,
        #[arg(long, default_value = "", alias = "n")]
        note: String,
    },
    Get {
        #[arg(long, alias = "P")]
        pattern: String,
    },
}

pub fn get_pw(
    pattern: &str,
    pws: Vec<PwEntry>,
    magic_crypt: MagicCrypt256,
) -> Result<(), Box<dyn Error>> {
    let mut result: Vec<PwEntry> = vec![];
    for pw_entry in pws {
        find_matches(pattern, pw_entry, &mut result);
    }

    let list_len = result.len();

    if list_len == 0 {
        println!("No entries found with given search argument: {}", pattern);
    } else if list_len == 1 {
        println!("Found one entry for search argument: {}", pattern);
        let found_entry: &PwEntry = result.get(0).unwrap();
        let decrypted_pw = magic_crypt.decrypt_base64_to_string(&found_entry.password)?;
        copy_to_clipboard(&decrypted_pw)?;
    } else {
        println!(
            "Found multiple entries for search argument: {}. Please choose preferred entry.",
            pattern
        );
        for i in 0..list_len {
            println!("\t{}. key:{}", i, result.get(i).unwrap().key)
        }
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match input.trim() {
            number if is_valid_input(&number) && number.parse::<usize>().unwrap() < list_len => {
                let input_as_integer = number.trim().parse::<usize>().unwrap();
                let selected_entry = result.get(input_as_integer).unwrap();
                let decrypted_pw =
                    magic_crypt.decrypt_base64_to_string(&selected_entry.password)?;
                copy_to_clipboard(&decrypted_pw)?;
            }
            _ => println!("No option with index {} available", input.trim()),
        }
    }

    std::thread::sleep(std::time::Duration::from_millis(50));
    Ok(())
}

pub fn set_pw(
    key: &str,
    url: &str,
    username: &str,
    password: &str,
    note: &str,
    mut pws: Vec<PwEntry>,
    magic_crypt: MagicCrypt256,
) -> Result<Vec<PwEntry>, String> {
    if pws.iter().any(|entry| entry.key == key) {
        let msg = format!("Entry with key:{} already exists", key);
        println!("{}", &msg);
        return Err(msg);
    }

    let new_entry = PwEntry {
        id: Uuid::new_v4().to_string(),
        key: key.to_owned(),
        url: url.to_owned(),
        username: username.to_owned(),
        password: magic_crypt.encrypt_str_to_base64(password),
        note: note.to_owned(),
    };

    pws.push(new_entry);
    Ok(pws)
}
