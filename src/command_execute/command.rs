use std::env;
use std::process::Command;

pub fn execute(command: &str, shell: Option<String>) {
    match env::consts::OS {
        "macos" | "linux" => parse_unix(shell, command),
        "windows" => parse_windows(command),
        _ => (),
    }
}

fn parse_windows(command: &str) {
    if Command::new("cmd").arg("/C").arg(command).status().is_err() {
        println!("Command failed to execute!");
    }
}

fn parse_unix(shell: Option<String>, command: &str) {
    if let Some(sh) = shell {
        if Command::new(sh).arg("-c").arg(command).status().is_err() {
            println!("Command failed to execute!");
        }
    } else if let Ok(sh) = std::env::var("SHELL") {
        if Command::new(sh).arg("-c").arg(command).status().is_err() {
            println!("Command failed to execute!");
        }
    }
}
