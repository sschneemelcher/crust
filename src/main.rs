use std::env::var;
use std::io::{stdin, stdout, Write};

mod errors;
mod parse;
mod run;

pub const SHELL_NAME: &str = "crust";

#[derive(Clone)]
pub struct Input {
    command: String,
    args: Vec<String>,
    bg: bool,
    builtin: Builtins,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Builtins {
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
        let inputs: &Vec<Input> = &parse::parse_input(input_buf);
        for input in inputs {
            match input.builtin {
                Builtins::None => run::execute_command(input),
                _ => run::execute_builtin(input),
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
