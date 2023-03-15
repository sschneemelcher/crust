use clap::Parser;

use crate::{errors, PathBuf};

use std::{fs, process::exit};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional file to run
    input_file: Option<PathBuf>,

    /// runs the given command string directly
    #[arg(short, long)]
    command: Option<String>,
}

pub fn handle_args(cli: Cli) -> Option<String> {
    if let Some(command) = cli.command.as_deref() {
        if command.len() == 0 {
            exit(0);
        }
        return Some(command.to_owned());
    }

    if let Some(path) = cli.input_file.as_deref() {
        let file_contents = fs::read_to_string(path);
        match file_contents {
            Ok(content) => {
                return Some(content.to_owned());
            }
            Err(_) => {
                println!(
                    "{}",
                    errors::get_error_message(errors::Errors::FileNotFound)
                );
                exit(1);
            }
        };
    } else {
        return Some("".to_owned());
    }
}
