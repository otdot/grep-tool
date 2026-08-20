pub mod models;
pub mod persistence;

use anyhow::{Context, Result};
use magic_crypt::{MagicCrypt256, MagicCryptTrait};
use std::{
    error::Error,
    fs::File,
    io::Read,
    time::{SystemTime, UNIX_EPOCH},
};

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
    #[arg(short, long, default_value = "dev-config.toml", alias = "c")]
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
        password: Option<String>,
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

pub fn transform_varying_case(string: String) {}

pub fn generate_password_from_config(config: &Config) -> Result<Vec<String>, Box<dyn Error>> {
    let auto_generated_pw_conf = &config.auto_generated_pws;

    // used for getting a "random" number
    let system_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let max_index = u64::try_from(auto_generated_pw_conf.items.len())?;
    let mut items_vec_index = system_time % &max_index;
    let mut password_items: Vec<String> = vec![];

    for _ in 0..auto_generated_pw_conf.word_length {
        let index = items_vec_index.try_into().unwrap_or(0);
        let item_to_add = auto_generated_pw_conf.items.clone().into_iter().nth(index);
        if let Some(item) = item_to_add {
            items_vec_index = u64::try_from(item.len())? % &max_index;
            password_items.push(item);
        } else {
            panic!("item with index {} was not found", index)
        }
    }

    Ok(password_items)
}
