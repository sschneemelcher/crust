use crossterm::{cursor::MoveToColumn, execute, queue, style::Print, terminal::Clear};
use std::{
    convert::TryInto,
    env::var,
    io::{stdout, Write},
};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::Prompt;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum PromptMode {
    #[default]
    Shell,
    Ask,
    WaitingForAskCrust,
    Submit,
    Break,
    Exit,
}

pub async fn print_prompt(mut rx: Receiver<Prompt>) {
    let mut stdout = stdout();

    while let Some(msg) = rx.recv().await {
        let ps2 = handle_ps2(&msg.mode);

        match msg.mode {
            PromptMode::Submit => {
                execute!(stdout, Print('\n'), MoveToColumn(0)).ok().unwrap();
                continue;
            }
            PromptMode::Break => {
                queue!(stdout, Print("^C\n"), MoveToColumn(0)).ok().unwrap();
            }
            _ => {}
        }

        // move to start of line and print prompt
        queue!(
            stdout,
            MoveToColumn(0),
            Clear(crossterm::terminal::ClearType::FromCursorDown),
            Print(ps2.clone()),
        )
        .ok();

        if msg.mode != PromptMode::Break {
            print_input(&msg, &ps2.len());
        }

        stdout.flush().ok();
    }
}

fn handle_ps2(mode: &PromptMode) -> String {
    match mode {
        PromptMode::Ask => "[ask-crust] ".to_string(),
        PromptMode::WaitingForAskCrust => "[waiting...] ".to_string(),
        _ => match var("PS2") {
            Ok(val) => val,
            Err(_) => "$ ".to_string(),
        },
    }
}

fn print_input(msg: &Prompt, ps2_len: &usize) {
    let mut stdout = stdout();
    queue!(
        stdout,
        Print(msg.input.clone()),
        MoveToColumn((ps2_len + msg.position).try_into().unwrap())
    )
    .ok();
}
