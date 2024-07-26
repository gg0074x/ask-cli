use clap::Parser;
use config::File;
use termimad::crossterm::style::{
    Attribute::Underlined,
    Color::{Black, Cyan, DarkYellow},
};
use utils::{create_config_dir, find_command, get_api_key, send_request};
use std::env;
mod config_parser;
mod errors;
mod utils;
mod command_execute;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short = 't', long)]
    shell: Option<String>,

    #[arg(short = 'x', long)]
    execute: bool,

    #[arg(short, long)]
    system: Option<String>,

    #[arg(short, long)]
    prompt: String,
}

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

    let args = Args::parse();

    let data: std::collections::HashMap<
        String,
        std::collections::HashMap<String, std::collections::HashMap<String, String>>,
    > = if args.execute {
        utils::parse_data(
            args.prompt,
            Some(format!(
"You will only respond to the prompt with just a one line {} terminal command after a > symbol",
            env::consts::OS)),
        )
    } else {
        utils::parse_data(args.prompt, args.system)
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
                    if args.execute {
                        find_command(text, args.shell);
                    } else {
                        let mut skin = termimad::MadSkin::default();
                        skin.bold.set_fg(Cyan);
                        skin.italic.add_attr(Underlined);
                        skin.code_block.set_bg(Black);
                        skin.code_block.set_fg(DarkYellow);

                        skin.print_text(text);
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
