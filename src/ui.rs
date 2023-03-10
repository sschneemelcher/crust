use crossterm::{
    cursor::{MoveToColumn, RestorePosition, SavePosition},
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType::FromCursorDown},
    Result,
};
use std::{
    convert::TryInto,
    env::var,
    fs,
    io::{Stdout, Write},
    process::exit,
};

fn exit_raw_mode() {
    match disable_raw_mode() {
        Ok(()) => {}
        Err(_) => panic! {"unable to exit raw mode"},
    };
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Mode {
    #[default]
    Input,
    Submit,
    Break,
    Exit,
}

#[derive(Clone, Default, Debug)]
pub struct Prompt {
    position: usize,
    input: String,
    mode: Mode,
    completions: Vec<String>,
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
                let line = prompt.input.clone();
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
                let line = prompt.input.clone();
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
    print_prompt(stdout, &prompt);

    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => handle_keypressed(event, &mut prompt),
            _ => {}
        }

        print_prompt(stdout, &prompt);
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
