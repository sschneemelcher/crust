use clap::Parser;

use crate::{errors, PathBuf};

use std::fs;

#[derive(PartialEq)]
pub enum CLIReturnCode {
    None,
    Success,
    Error,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Optional file to run
    input_file: Option<PathBuf>,

    /// runs the given command string directly
    #[arg(short, long)]
    command: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

pub fn handle_args(cli: Cli) -> (String, CLIReturnCode) {
    match cli.debug {
        0 => {}
        _ => println!("Debug mode is on"),
    }

    if let Some(command) = cli.command.as_deref() {
        return (command.to_owned(), CLIReturnCode::Success);
    }

    if let Some(path) = cli.input_file.as_deref() {
        if cli.debug > 0 {
            println!("operating on file {}", path.display());
        }
        let file_contents = fs::read_to_string(path);
        match file_contents {
            Ok(content) => {
                if cli.debug > 1 {
                    println!("{content}");
                }
                return (content.to_owned(), CLIReturnCode::Success);
            }
            Err(_) => {
                println!(
                    "{}",
                    errors::get_error_message(errors::Errors::FileNotFound)
                );
                return ("".to_owned(), CLIReturnCode::Error);
            }
        };
    } else {
        return ("".to_owned(), CLIReturnCode::None);
    }
}
