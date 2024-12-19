use clap::Parser;
use config::File;
use std::env;
use termimad::crossterm::style::{
    Attribute::Underlined,
    Color::{Black, Cyan, DarkYellow},
};
use utils::{create_config_dir, find_command, get_api_key, send_request};
mod args;
mod command_execute;
mod config_parser;
mod errors;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_file: Option<File<config::FileSourceFile, config_parser::ConfigFile>> =
        create_config_dir();

    let api_token = match get_api_key(config_file) {
        Ok(api_key) => api_key,
        Err(err) => {
            return Err(err);
        }
    };

    let args = args::Args::parse();

    let data: std::collections::HashMap<
        String,
        std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    > = match args.cmd {
        args::Commands::Execute { ref prompt } => utils::parse_data(
            prompt.to_string(),
            Some(format!(
"You will only respond to the prompt with just a one line {} terminal command after a > symbol",
            env::consts::OS)),
        ),
        args::Commands::Query { ref prompt } => utils::parse_data(prompt.to_string(), args.system),
    };

    let client = reqwest::Client::new();

    match send_request(client, data, api_token).await {
        Ok(res) => {
            if let Ok(response_json) = serde_json::from_str::<serde_json::Value>(&res) {
                if let Some(text) = response_json
                    .get("error")
                    .and_then(|value| value.get("message"))
                    .and_then(|value| value.as_str())
                {
                    println!("{text}");
                    return Err(errors::DeserializeError.into());
                }
                if let Some(text) = response_json
                    .get("candidates")
                    .and_then(|value| value.get(0))
                    .and_then(|value| value.get("content"))
                    .and_then(|value| value.get("parts"))
                    .and_then(|value| value.get(0))
                    .and_then(|value| value.get("text"))
                    .and_then(|value| value.as_str())
                {
                    match args.cmd {
                        args::Commands::Query { prompt: _ } => {
                            let mut skin = termimad::MadSkin::default();
                            skin.bold.set_fg(Cyan);
                            skin.italic.add_attr(Underlined);
                            skin.code_block.set_bg(Black);
                            skin.code_block.set_fg(DarkYellow);

                            skin.print_text(text);
                        }
                        args::Commands::Execute { prompt: _ } => {
                            find_command(text, args.shell);
                        }
                    }

                    return Ok(());
                } else if let Some(error) = response_json
                    .get("candidates")
                    .and_then(|value| value.get(0))
                    .and_then(|value| value.get("finishReason"))
                    .and_then(|value| value.as_str())
                {
                    match error {
                        "SAFETY" => return Err(errors::CensorError.into()),
                        _ => return Ok(()),
                    }
                }
            }
        }
        Err(err) => {
            return Err(err.into());
        }
    }
    return Err(errors::DeserializeError.into());
}
