use clap::Parser;
use errors::{get_error_message, Errors};
use keys::handle_keys;
use std::fs;
use std::io::stdout;
use std::path::PathBuf;
use std::process::exit;

mod errors;
mod keys;
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

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Mode {
    #[default]
    Input,
    Submit,
    Break,
    Exit,
}

#[derive(Clone, Default, Debug)]
pub struct Prompt {
    position: usize,
    input: String,
    mode: Mode,
    completions: Vec<String>,
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

fn main() {
    let cli = Cli::parse();
    let mut stdout = stdout();

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

    loop {
        let raw_input = match handle_keys(&mut stdout) {
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
