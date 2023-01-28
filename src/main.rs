use std::env::{current_dir, home_dir, set_current_dir, var};
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use std::process::{exit, Command};

const SHELL_NAME: &str = "crust";

#[derive(Clone)]
struct Input {
    command: String,
    args: Vec<String>,
    bg: bool,
    builtin: Builtins,
}

#[derive(Clone)]
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
        let inputs: &Vec<Input> = &parse_input(input_buf);
        for input in inputs {
            match input.builtin {
                Builtins::None => execute_command(input),
                _ => execute_builtin(input),
            }
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

fn parse_input(raw_input: String) -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];

    // Parse input line by line
    for line in raw_input.split(['\n', ';']) {
        if line.len() < 1 {
            continue;
        };

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
            None => inputs.push(parsed_input.clone()),
            Some("exit") => {
                parsed_input.builtin = Builtins::Exit;
                inputs.push(parsed_input.clone());
            }
            Some("cd") => parsed_input.builtin = Builtins::CD,
            Some("echo") => parsed_input.builtin = Builtins::Echo,
            Some("alias") => parsed_input.builtin = Builtins::Alias,
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
        inputs.push(parsed_input);
    }
    return inputs;
}

fn execute_command(input: &Input) {
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
                Some(output) => println!("{:?}", output),
            }
        }
        // if spawning failed, print message
        Err(_) => println! {"{}: command not found", input.command},
    }
}

fn execute_builtin(input: &Input) {
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
                    <PathBuf as Extend<String>>::extend::<Vec<String>>(
                        &mut path,
                        input.args.clone(),
                    );
                    match set_current_dir(path) {
                        Ok(_) => {}
                        Err(_) => {
                            println! {"-{}: cd: {}: No such file or directory", SHELL_NAME, arg}
                        }
                    }
                }
                _ => println! {"-{}: cd: too many arguments", SHELL_NAME},
            }
        }
        Err(e) => println! {"{:#?}", e},
    }
}
