use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveRight, MoveToColumn, MoveUp, RestorePosition, SavePosition},
    event::{read, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
    Result,
};
use std::{
    convert::TryInto,
    fs,
    io::{stdout, Stdout, Write},
    process::exit,
};

fn exit_raw_mode() {
    match disable_raw_mode() {
        Ok(()) => {}
        Err(_) => panic! {"unable to exit raw mode"},
    };
}

pub fn handle_keys(stdout: &mut Stdout) -> Result<String> {
    let mut position = 0;
    let mut input: String = "".to_string();
    match enable_raw_mode() {
        Ok(()) => {}
        Err(_) => panic! {"unable to enter raw mode"},
    }
    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => {
                    if c == 'd' && event.modifiers == KeyModifiers::CONTROL {
                        exit_raw_mode();
                        exit(0);
                    } else if c == 'c' && event.modifiers == KeyModifiers::CONTROL {
                        input = "".to_string();
                        break;
                    } else if position < input.len() {
                        let line = input.clone();
                        let (head, tail) = line.split_at(position);
                        match execute!(
                            stdout,
                            Print(c),
                            Print(&tail),
                            MoveLeft(tail.len().try_into().unwrap())
                        ) {
                            Ok(()) => {
                                input = format!("{}{}{}", head, c, tail);
                                position += 1;
                            }
                            Err(_) => {}
                        }
                    } else {
                        match execute!(stdout, Print(c)) {
                            Ok(()) => {
                                input.push(c);
                                position += 1;
                            }
                            Err(_) => {}
                        }
                    }
                }
                KeyCode::Backspace => {
                    if input.len() > 0 && position > 0 {
                        if input.len() == position {
                            // delete character at end of line
                            match execute!(stdout, MoveLeft(1), Print(' '), MoveLeft(1)) {
                                Ok(()) => {
                                    input.pop();
                                    position -= 1;
                                }
                                Err(_) => {}
                            }
                        } else {
                            // delete character from inside the line
                            let line = input.clone();
                            let (head, tail) = line.split_at(position - 1);
                            input = head.to_string() + &tail[1..];
                            match execute!(
                                stdout,
                                MoveLeft(position.try_into().unwrap()),
                                Clear(crossterm::terminal::ClearType::FromCursorDown),
                                Print(&head),
                                Print(&tail[1..]),
                                MoveLeft((tail.len() - 1).try_into().unwrap())
                            ) {
                                Ok(()) => position -= 1,
                                Err(_) => {}
                            }
                        }
                    }
                }
                KeyCode::Left => {
                    if position > 0 {
                        position = match execute!(stdout, MoveLeft(1)) {
                            Ok(()) => position - 1,
                            Err(_) => position,
                        };
                    }
                }
                KeyCode::Right => {
                    if position < input.len() {
                        position = match execute!(stdout, MoveRight(1)) {
                            Ok(()) => position + 1,
                            Err(_) => position,
                        };
                    }
                }
                KeyCode::Enter => break,
                KeyCode::Tab => {
                    // Handle completions
                    let completions = get_completion(&input);

                    if completions.len() > 1 {
                        queue!(stdout, Print("\n"), SavePosition, MoveToColumn(0))?;
                        for completion in completions {
                            queue!(stdout, Print(completion + "    "))?;
                        }
                        queue!(stdout, RestorePosition, MoveUp(1))?;
                        stdout.flush()?;
                    } else if completions.len() == 1 {
                        match execute!(stdout, Print(completions[0].clone())) {
                            Ok(()) => {
                                input.push_str(completions[0].as_ref());
                                position += completions[0].len();
                            }
                            Err(_) => {}
                        }
                    }

                    // match execute!(stdout, Print(completion.clone())) {
                    //     Ok(()) => {
                    //         input.push_str(completion.as_ref());
                    //         position += completion.len();
                    //     }
                    //     Err(_) => {}
                    // }
                }
                _ => {}
            },
            _ => {}
        };
    }
    exit_raw_mode();
    execute!(stdout, Print('\n')).ok();
    Ok(input)
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
