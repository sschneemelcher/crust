use assert_cmd::Command;
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

    /// runs the given command string directly
    #[arg(short, long)]
    command: Option<String>,

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

    let mut input = String::default();

    if let Some(command) = cli.command.as_deref() {
        input = command.to_string();
    }

    if let Some(path) = cli.input_file.as_deref() {
        if cli.debug > 0 {
            println!("operating on file {}", path.display());
        }
        let file_contents = fs::read_to_string(path);
        input = match file_contents {
            Ok(content) => {
                if cli.debug > 1 {
                    println!("{content}");
                }
                content
            }
            Err(_) => {
                println!("{}", get_error_message(Errors::FileNotFound));
                exit(1);
            }
        };
    }

    if input.len() > 0 {
        let inputs: &Vec<Input> = &parse::parse_input(input);
        for input in inputs {
            match input.builtin {
                Builtins::None => run::execute_command(input),
                _ => run::execute_builtin(input),
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

#[test]
fn test_crust_echo() {
    let mut cmd = Command::cargo_bin("crust").unwrap();
    let output = cmd.arg("-c").arg("echo Hello World").output().unwrap();
    assert!(output.status.success());

    assert_eq!(output.stdout, b"Hello World\n");
}
