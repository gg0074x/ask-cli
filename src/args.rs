use std::path::PathBuf;

use clap::{builder::Str, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Commands,

    #[arg(short = 'S', long)]
    pub shell: Option<String>,

    #[arg(short, long)]
    pub system: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Ask the AI a question.
    #[clap(alias = "ask")]
    Query {
        /// Prompt to send the AI.
        #[arg(value_name = "PROMPT", index = 1)]
        prompt: String,
    },

    /// Ask the AI to execute a command in your terminal.
    #[clap(alias = "run")]
    Execute {
        /// Prompt to send the AI.
        #[arg(value_name = "PROMPT", index = 1)]
        prompt: String,
    },
}
