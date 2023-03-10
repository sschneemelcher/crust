use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::{fs, io::Stdout, process::exit};

use crate::{ui, Mode, Prompt};

fn exit_raw_mode() {
    match disable_raw_mode() {
        Ok(()) => {}
        Err(_) => panic! {"unable to exit raw mode"},
    };
}

fn handle_keypressed(event: KeyEvent, prompt: &mut Prompt) {
    if event.modifiers == KeyModifiers::CONTROL && prompt.mode == Mode::Input {
        match event.code {
            KeyCode::Char('c') => {
                prompt.mode = Mode::Break;
                return;
            }
            KeyCode::Char('d') => {
                prompt.mode = Mode::Exit;
                return;
            }
            _ => {}
        }
    }

    match event.code {
        KeyCode::Char(c) => {
            if prompt.position < prompt.input.len() {
                let line = &prompt.input;
                let (head, tail) = line.split_at(prompt.position);
                prompt.input = format!("{}{}{}", head, c, tail);
                prompt.position += 1;
            } else {
                prompt.input.push(c);
                prompt.position += 1;
            }
        }
        KeyCode::Backspace => {
            if prompt.input.len() == 0 || prompt.position == 0 {
                return;
            }
            if prompt.input.len() == prompt.position {
                // delete character at end of line
                prompt.input.pop();
                prompt.position -= 1;
            } else {
                // delete character from inside the line
                let line = &prompt.input;
                let (head, tail) = line.split_at(prompt.position - 1);
                prompt.input = head.to_string() + &tail[1..];
                prompt.position -= 1;
            }
        }
        KeyCode::Left => {
            if prompt.position > 0 {
                prompt.position = prompt.position - 1;
            }
        }
        KeyCode::Right => {
            if prompt.position < prompt.input.len() {
                prompt.position = prompt.position + 1;
            }
        }
        KeyCode::Enter => prompt.mode = Mode::Submit,

        KeyCode::Tab => {
            // Handle completions
            prompt.completions = get_completion(&prompt.input);
        }
        _ => {}
    }
}

pub fn handle_keys(stdout: &mut Stdout) -> Result<String> {
    match enable_raw_mode() {
        Ok(()) => {}
        Err(_) => panic! {"unable to enter raw mode"},
    }

    let mut prompt = Prompt::default();
    ui::print_prompt(stdout, &prompt);

    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => handle_keypressed(event, &mut prompt),
            _ => {}
        }

        ui::print_prompt(stdout, &prompt);

        if prompt.completions.len() == 1 {
            prompt.input.push_str(&prompt.completions[0])
        }
        prompt.completions = vec![];

        match prompt.mode {
            Mode::Exit => {
                exit_raw_mode();
                exit(0);
            }
            Mode::Submit => {
                exit_raw_mode();
                return Ok(prompt.input);
            }
            Mode::Break => {
                prompt = Prompt::default();
            }
            _ => {}
        }
    }
}

fn get_completion(_line: &str) -> Vec<String> {
    let mut completions: Vec<String> = vec![];

    match fs::read_dir(".") {
        Ok(paths) => {
            for path in paths {
                completions.push(path.unwrap().path().display().to_string());
            }
        }
        Err(_) => {}
    }
    return completions;
}
