use crossterm::{
    cursor::MoveLeft,
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode},
    Result,
};
use std::{io::Stdout, process::exit};

pub fn handle_keys(stdout: &mut Stdout) -> Result<String> {
    let mut input: String = "".to_string();
    enable_raw_mode().ok();
    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => {
                    if (c == 'c' || c == 'd') && event.modifiers == KeyModifiers::CONTROL {
                        disable_raw_mode().ok();
                        exit(0);
                    }
                    execute!(stdout, Print(c)).ok();
                    input.push(c);
                }
                KeyCode::Backspace => {
                    if input.len() > 0 {
                        input.pop();
                        execute!(stdout, MoveLeft(1), Print(' '), MoveLeft(1)).ok();
                    }
                }
                KeyCode::Enter => break,
                _ => {}
            },
            _ => {}
        };
    }
    disable_raw_mode().ok();
    execute!(stdout, Print('\n')).ok();
    Ok(input)
}
