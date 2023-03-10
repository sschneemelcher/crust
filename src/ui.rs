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

use crate::{Mode, Prompt};

pub fn print_prompt(stdout: &mut Stdout, prompt: &Prompt) {
    match prompt.mode {
        Mode::Submit => queue!(stdout, Print('\n')).ok().unwrap(),
        Mode::Break => queue!(stdout, Print("^C\n")).ok().unwrap(),
        _ => {}
    }

    if prompt.completions.len() > 1 {
        queue!(stdout, SavePosition, MoveToColumn(0), Clear(FromCursorDown)).ok();

        for completion in prompt.completions.clone() {
            queue!(stdout, Print(completion + "    ")).ok();
        }

        queue!(stdout, Print("\n")).ok();
    }

    queue!(stdout, MoveToColumn(0), Clear(FromCursorDown)).ok();

    if prompt.mode == Mode::Submit {
        stdout.flush().ok();
        return;
    }

    let ps2 = match var("PS2") {
        Ok(val) => val,
        Err(_) => "$ ".to_string(),
    };

    queue!(stdout, Print(ps2.clone())).ok();

    if prompt.mode == Mode::Input {
        queue!(stdout, Print(prompt.input.clone())).ok();
    }

    let pos: u16 = (ps2.len() + prompt.position).try_into().unwrap();
    execute!(stdout, MoveToColumn(pos)).ok();
}
