use crate::cli::Cli;
use clap::Parser;
use cli::CLIReturnCode;
use keys::handle_keys;
use std::io::stdout;
use std::path::PathBuf;
use std::process::exit;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod cli;
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

fn main() {
    let cli = Cli::parse();
    let mut stdout = stdout();

    let (input, code) = cli::handle_args(cli);

    match code {
        CLIReturnCode::Success if input.len() > 0 => {
            let inputs: &Vec<Input> = &parse::parse_input(&input);
            for input in inputs {
                match input.builtin {
                    Builtins::None => run::execute_command(&input),
                    _ => run::execute_builtin(input),
                }
            }
            exit(0);
        }
        CLIReturnCode::Success => exit(0),
        CLIReturnCode::Error => exit(1),

        _ => {}
    }

    loop {
        let raw_input = match handle_keys(&mut stdout) {
            Ok(input) => input,
            Err(_) => continue,
        };

        let inputs: &Vec<Input> = &parse::parse_input(&raw_input);
        for input in inputs {
            match input.builtin {
                Builtins::None => run::execute_command(input),
                _ => run::execute_builtin(input),
            }
        }
    }
}

use assert_cmd::Command;
use proptest::proptest;

#[test]
fn test_crust_echo_simple() {
    let mut cmd = Command::cargo_bin("crust").unwrap();
    let output = cmd.arg("-c").arg("echo Hello World").output().unwrap();

    assert!(output.status.success());
    assert_eq!(output.stdout, b"Hello World\n");
}

#[test]
fn test_crust_echo() {
    proptest!(|(arg in "\\PC*")| {
        let mut cmd = Command::cargo_bin("crust").unwrap();
        let output = cmd.arg("-c").arg(format!("echo {}", &arg)).output().unwrap();
        println!("{:#?}", &output);
        println!("{:#?}\n", &arg);

        assert!(output.status.success());
        assert_eq!(output.stdout, format!("{}\n", arg).as_bytes());
    });
}
