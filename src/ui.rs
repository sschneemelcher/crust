use crossterm::{
    cursor::{MoveLeft, MoveToColumn},
    queue,
    style::Print,
    terminal::{Clear, ClearType::FromCursorDown},
};
use std::{
    convert::TryInto,
    env::var,
    io::{stdout, Stdout, Write},
};

use crate::keys::Mode;

#[derive(Clone, Default, Debug)]
pub struct Prompt {
    pub position: usize,
    pub input: String,
    pub mode: Mode,
    pub completions: Vec<String>,
    pub history_idx: usize,
    pub prev_position: usize,
    pub saved_input: String,
}

/* Print the prompt to the screen 
 * 
 * This function is responsible for printing the prompt to the screen. It will
 * print the prompt, the input, and any completions. It will also handle
 * printing the prompt in different modes.
 */

pub fn print_prompt(prompt: &Prompt) {
    let mut stdout = stdout();

    match prompt.mode {
        Mode::Submit => queue!(stdout, Print('\n'), MoveToColumn(0)).ok().unwrap(),
        Mode::Break => queue!(stdout, Print("^C\n"), MoveToColumn(0)).ok().unwrap(),
        _ => {}
    }

    let ps2 = match var("PS2") {
        Ok(val) => val,
        Err(_) => "$ ".to_owned(),
    };

    queue!(
        stdout,
        MoveLeft((ps2.len() + prompt.prev_position).try_into().unwrap()),
        Clear(FromCursorDown),
    )
    .ok();
    print_completions(&mut stdout, &prompt.completions);

    if prompt.mode == Mode::Submit {
        stdout.flush().ok();
        return;
    }

    queue!(stdout, Print(&ps2), Print(&prompt.input)).ok();

    if prompt.input.len() != prompt.position {
        queue!(
            stdout,
            MoveLeft((prompt.input.len() - prompt.position).try_into().unwrap())
        )
        .ok();
    }
    stdout.flush().ok();
}

fn print_completions(stdout: &mut Stdout, completions: &Vec<String>) {
    if completions.len() > 1 {
        for completion in completions {
            queue!(stdout, Print(format!("{}    ", completion))).ok();
        }

        queue!(stdout, Print("\n"), MoveToColumn(0)).ok();
    }
}
