use clap::Parser;
use errors::{get_error_message, Errors};
use std::fs;
use std::io::stdout;
use std::path::PathBuf;
use std::process::exit;
use tokio::sync::mpsc;

mod errors;
mod parse;
mod run;
mod ui;

pub const SHELL_NAME: &str = "crust";

#[derive(Clone, Default)]
pub struct Input {
    command: String,
    args: Vec<String>,
    bg: bool,
    builtin: Builtins,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Builtins {
    #[default]
    None,
    Exit,
    CD,
    Echo,
    Alias,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional file to run
    input_file: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.debug {
        0 => {}
        _ => println!("Debug mode is on"),
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
                let inputs: &Vec<Input> = &parse::parse_input(content);
                for input in inputs {
                    match input.builtin {
                        Builtins::None => run::execute_command(input),
                        _ => run::execute_builtin(input),
                    }
                }
            }
            Err(_) => {
                println!("{}", get_error_message(Errors::FileNotFound));
                exit(1);
            }
        }
        exit(0);
    }

    let (prompt_tx, prompt_rx) = mpsc::channel(32);

    tokio::spawn(async move { ui::print_prompt(prompt_rx).await });

    loop {
        let raw_input = match ui::handle_keys(prompt_tx.clone()).await {
            Ok(input) => input,
            Err(_) => continue,
        };

        let inputs: &Vec<Input> = &parse::parse_input(raw_input);
        for input in inputs {
            match input.builtin {
                Builtins::None => run::execute_command(input),
                _ => run::execute_builtin(input),
            }
        }
    }
}
