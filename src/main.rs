use crate::cli::Cli;
use clap::Parser;
use keys::handle_keys;
use parse::Input;
use run::Builtins;
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

fn main() {
    let cli = Cli::parse();
    let mut history: Vec<String> = vec![];

    let input = cli::handle_args(cli).unwrap();

    if input.len() > 0 {
        shell_cycle(&input);
        exit(0);
    }

    loop {
        let raw_input = match handle_keys(&history) {
            Ok(input) => input,
            Err(_) => continue,
        };
        history.push(raw_input.to_owned());
        shell_cycle(&raw_input);
    }
}

fn shell_cycle(raw_input: &str) {
    let inputs: &Vec<Input> = &parse::parse_input(raw_input);
    for input in inputs {
        match input.builtin {
            Builtins::None => run::execute_command(input),
            _ => run::execute_builtin(input),
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
    proptest!(|(arg in "\\PC+")| {
        let mut cmd = Command::cargo_bin("crust").unwrap();
        let output = cmd.arg("-c").arg(format!("echo '{}'", &arg)).output().unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, format!("{}\n", arg).as_bytes());
    });
}
