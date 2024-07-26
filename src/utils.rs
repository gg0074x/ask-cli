use crate::command_execute::command;
use crate::config_parser::{self, ConfigFile};
use crate::errors;
use config::{Config, File, FileSourceFile};
use directories::BaseDirs;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs::{create_dir, File as StdFile};
use std::io::{self, BufRead};
use std::path::Path;

pub fn parse_data(
    prompt: String,
    system: Option<String>,
) -> HashMap<String, HashMap<String, HashMap<String, String>>> {
    let mut root_map: HashMap<String, HashMap<String, HashMap<String, String>>> = HashMap::new();
    let mut parts: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut text: HashMap<String, String> = HashMap::new();
    let mut sys_parts: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut sys_text: HashMap<String, String> = HashMap::new();

    text.insert("text".to_string(), prompt);
    parts.insert("parts".to_string(), text);
    if let Some(system_prompt) = system {
        sys_text.insert("text".to_string(), system_prompt);
        sys_parts.insert("parts".to_string(), sys_text);
        root_map.insert("system_instruction".to_string(), sys_parts);
    }
    root_map.insert("contents".to_string(), parts);

    root_map
}

pub async fn send_request(
    client: Client,
    data: std::collections::HashMap<
        String,
        std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    >,
    api_token: String,
) -> Result<String, reqwest::Error> {
    match client
        .post(format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash-latest:generateContent?key={api_token}"))
        .header(CONTENT_TYPE, "application/json")
        .json(&data)
        .send()
        .await{
            Ok(res) => {
                match res.text().await{
                    Ok(response_text) => {
                        Ok(response_text)
                    }
                    Err(err) => {
                        Err(err)
                    }
                }
            },
            Err(err) => {
                Err(err)
            }
        }
}

pub fn create_config_dir() -> Option<File<config::FileSourceFile, config_parser::ConfigFile>> {
    if let Some(base_dirs) = BaseDirs::new() {
        if !Path::new(&base_dirs.config_dir().join("ask_config")).exists() {
            let path = base_dirs.config_dir().join("ask_config");
            match create_dir(path) {
                Ok(()) => println!(
                    "I: config folder was created at {:?}",
                    base_dirs.config_dir()
                ),
                Err(err) => println!("{err:?}"),
            }
            match StdFile::create(base_dirs.config_dir().join("ask_config/config.toml")) {
                Ok(_) => println!(
                    "I: config.toml file was created at {:?}",
                    base_dirs.config_dir().join("ask_config")
                ),
                Err(err) => println!("W: config.toml file couldn't be created: {err}"),
            }
        }
        if let Some(config_dir) = base_dirs
            .config_dir()
            .join("ask_config/config.toml")
            .to_str()
        {
            return Some(File::new(config_dir, config_parser::ConfigFile));
        }
        return None;
    }
    None
}

#[derive(Debug, Deserialize)]
pub struct ApiSettings {
    pub token: String,
}

fn make_app_config(
    config_file: File<FileSourceFile, ConfigFile>,
) -> Result<ApiSettings, Box<dyn std::error::Error>> {
    if let Ok(config) = Config::builder()
        .add_source(config_file.required(false))
        .build()
    {
        if let Ok(app) = config.try_deserialize::<ApiSettings>() {
            Ok(app)
        } else {
            Err(errors::EmptyKey.into())
        }
    } else {
        Err(errors::EmptyKey.into())
    }
}

pub fn get_api_key(
    config_file: Option<File<config::FileSourceFile, config_parser::ConfigFile>>,
) -> Result<String, Box<dyn std::error::Error>> {
    if config_file.is_none() {
        if let Ok(env_var) = env::var("GEMINI_TOKEN") {
            Ok(env_var)
        } else {
            Err(errors::EmptyKey.into())
        }
    } else if let Ok(app) = make_app_config(config_file.unwrap()) {
        if app.token.is_empty() {
            if let Ok(env_var) = env::var("GEMINI_TOKEN") {
                Ok(env_var)
            } else {
                Err(errors::EmptyKey.into())
            }
        } else {
            Ok(app.token)
        }
    } else if let Ok(env_var) = env::var("GEMINI_TOKEN") {
        return Ok(env_var);
    } else {
        return Err(errors::EmptyKey.into());
    }
}

pub fn find_command(text: &str, shell: Option<String>) {
    println!("{text}");
    let start_bytes = text.find('>').unwrap_or(0);

    let it = text.split_at(start_bytes + 1).1.trim();

    print!("Do you want to execute the following command? \"{it}\" [Y/N] (N): ");

    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    if handle.read_line(&mut buffer).is_ok() {
        buffer.pop();
        if buffer.to_uppercase().as_str() == "Y" {
            command::execute(it, shell);
        } else {
            println!("Nothing has been done.");
        }
    }
}
