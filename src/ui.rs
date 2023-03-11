use crossterm::{
    cursor::{MoveToColumn, SavePosition},
    execute, queue,
    style::Print,
    terminal::{Clear, ClearType::FromCursorDown},
};
use std::{
    convert::TryInto,
    env::var,
    io::{Stdout, Write},
};

use crate::keys::Mode;

#[derive(Clone, Default, Debug)]
pub struct Prompt {
    pub position: usize,
    pub input: String,
    pub mode: Mode,
    pub completions: Vec<String>,
}

pub fn print_prompt(stdout: &mut Stdout, prompt: &Prompt) {
    match prompt.mode {
        Mode::Submit => {
            queue!(stdout, Print('\n')).ok().unwrap();
        }
        Mode::Break => {
            queue!(stdout, Print("^C\n")).ok().unwrap();
        }
        _ => {}
    }

    print_completions(stdout, &prompt.completions);

    queue!(stdout, MoveToColumn(0), Clear(FromCursorDown)).ok();

    if prompt.mode == Mode::Submit {
        stdout.flush().ok();
        return;
    }

    let ps2 = match var("PS2") {
        Ok(val) => val,
        Err(_) => "$ ".to_owned(),
    };

    queue!(stdout, Print(&ps2)).ok();

    let pos: u16 = match prompt.mode {
        Mode::Input => {
            queue!(stdout, Print(&prompt.input)).ok();

            (ps2.len() + prompt.position).try_into().unwrap()
        }
        _ => ps2.len().try_into().unwrap(),
    };
    execute!(stdout, MoveToColumn(pos)).ok();
}

fn print_completions(stdout: &mut Stdout, completions: &Vec<String>) {
    if completions.len() > 1 {
        queue!(stdout, SavePosition, MoveToColumn(0), Clear(FromCursorDown)).ok();

        for completion in completions {
            queue!(stdout, Print(format!("{}    ", completion))).ok();
        }

        queue!(stdout, Print("\n")).ok();
    }
}
