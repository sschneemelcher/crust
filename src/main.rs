use clap::Parser;
use errors::{get_error_message, Errors};
use std::env::var;
use std::fs;
use std::io::{stdout, Write};
use std::path::PathBuf;
use ui::handle_keys;

mod errors;
mod parse;
mod run;
mod ui;

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
                println!("{}", get_error_message(Errors::FileOpenError));
            }
        }
    }

    loop {
        print_prompt();

        let raw_input = match handle_keys(&mut stdout) {
            Ok(input) => input,
            Err(_) => continue,
        };
        // let mut input_buf = String::new();
        // stdin().read_line(&mut input_buf).expect("expected a line");
        let inputs: &Vec<Input> = &parse::parse_input(raw_input);
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
