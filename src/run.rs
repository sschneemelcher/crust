use std::{
    env::{current_dir, home_dir, set_current_dir},
    path::PathBuf,
    process::{exit, Command},
};

use crate::{
    errors::{get_error_message, Errors},
    Input, SHELL_NAME,
};

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Builtins {
    #[default]
    None,
    Exit,
    CD,
    Echo,
    Alias,
}

pub fn execute_command(input: &Input) {
    let mut cmd = Command::new(&input.command);
    cmd.args(&input.args);

    match cmd.spawn() {
        Ok(mut child) => {
            if !input.bg {
                match child.wait() {
                    Ok(_) => {}
                    Err(e) => println!("{:#?}", e),
                }
            }
        }
        // if spawning failed, print message
        Err(_) => println!(
            "{}: {}",
            input.command,
            get_error_message(Errors::CommandNotFound)
        ),
    }
}

pub fn execute_builtin(input: &Input) {
    match input.builtin {
        Builtins::Exit => exit(0),
        Builtins::Echo => println! {"{}", input.args.join(" ")},
        Builtins::Alias => { /* TODO use hash map to define aliases */ }
        Builtins::CD => change_dir(input),
        _ => {}
    }
}

fn change_dir(input: &Input) {
    match current_dir() {
        Ok(mut path) => {
            // Check if the user has given 0 (~) or 1 arg
            match input.args.len() {
                0 => match home_dir() {
                    Some(home) => match set_current_dir(home) {
                        Ok(_) => {}
                        Err(_) => {}
                    },
                    None => println! {"-{}: cd: home not set", SHELL_NAME},
                },
                1 => {
                    let arg: &str = &input.args[0].clone();
                    let extension = PathBuf::from(input.args[0].clone());
                    path.push(extension);
                    match set_current_dir(path) {
                        Ok(_) => {}
                        Err(_) => {
                            println! {"-{}: cd: {}: {}", SHELL_NAME, arg, get_error_message(Errors::FileNotFound)}
                        }
                    }
                }
                _ => println! {"-{}: cd: too many arguments", SHELL_NAME},
            }
        }
        Err(e) => println! {"{:#?}", e},
    }
}
