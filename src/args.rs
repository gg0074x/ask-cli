use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Option<Commands>,

    #[arg(
        index = 1,
        help = "Postional prompt argument used when no command is specified (ask by default)"
    )]
    pub prompt: Option<String>,

    #[arg(short = 'S', long, help = "Shell to use for command execution")]
    pub shell: Option<String>,

    #[arg(short, long, help = "System prompt")]
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
