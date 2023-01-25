use std::env::{current_dir, home_dir, set_current_dir, var};
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::process::{exit, Command};

const SHELL_NAME: &str = "crust";

struct Input {
    command: String,
    args: Vec<String>,
    bg: bool,
    builtin: Builtins,
}

enum Builtins {
    None,
    Exit,
    CD,
    Echo,
    Alias,
}

fn main() {
    loop {
        print_prompt();

        let mut input_buf = String::new();
        stdin().read_line(&mut input_buf).expect("expected a line");
        let input: Input = parse_input(input_buf);

        match input.builtin {
            Builtins::None => execute_command(input),
            _ => execute_builtin(input),
        }
    }
}

fn print_prompt() {
    let prompt = match var("PS2") {
        Ok(val) => val,
        Err(_) => "$ ".to_string(),
    };

    let mut lock = stdout().lock();
    write!(lock, "{}", prompt).unwrap();

    match lock.flush() {
        Ok(_) => {}
        Err(e) => println!("{:#?}", e),
    }
}

fn parse_input(line: String) -> Input {
    let mut parsed_input = Input {
        command: "".to_string(),
        args: vec![],
        builtin: Builtins::None,
        bg: false,
    };

    let mut words = line.split_whitespace();

    // parse the first word of the input
    match words.next() {
        // match the first word of the input
        None => return parsed_input,
        Some("exit") => {
            parsed_input.builtin = Builtins::Exit;
            return parsed_input;
        }
        Some("cd") => parsed_input.builtin = Builtins::CD,
        Some(command) => {
            parsed_input.command = command.to_string();
        }
    }

    for word in words {
        match word {
            "&" => parsed_input.bg = true,
            arg => parsed_input.args.push(arg.to_string()),
        }
    }
    return parsed_input;
}

fn execute_command(input: Input) {
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
            match child.stdout.take() {
                None => {}
                Some(output) => println!("{}", stringify!(output)),
            }
        }
        // if spawning failed, print message
        Err(_) => println! {"{}: command not found", input.command},
    }
}

fn execute_builtin(input: Input) {
    match input.builtin {
        Builtins::Exit => exit(0),
        Builtins::Echo => println! {"{}", input.args.join(" ")},
        Builtins::Alias => { /* TODO use hash map to define aliases */ }
        Builtins::CD => match current_dir() {
            Ok(mut path) => {
                // Check if the user has given 0 (~) or 1 arg
                match input.args.len() {
                    0 => match home_dir() {
                        Some(home) => match set_current_dir(home) {
                            Ok(_) => {}
                            Err(_) => {}
                        },
                        None => println! {"{}: cd: home not set", SHELL_NAME},
                    },
                    1 => {
                        <PathBuf as Extend<String>>::extend::<Vec<String>>(&mut path, input.args);
                        match set_current_dir(path) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                    _ => println! {"{}: cd: too many arguments", SHELL_NAME},
                }
            }
            Err(e) => println! {"{:#?}", e},
        },
        _ => {}
    }
}
