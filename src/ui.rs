use async_openai::{types::CreateCompletionRequestArgs, Client};
use crossterm::{
    cursor::{MoveToColumn, RestorePosition, SavePosition},
    event::{read, Event, KeyCode, KeyModifiers},
    execute, queue,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
};
use std::{convert::TryInto, env::var, error::Error, fs, io::Stdout, process::exit};

fn exit_raw_mode() {
    match disable_raw_mode() {
        Ok(()) => {}
        Err(_) => panic! {"unable to exit raw mode"},
    };
}

#[derive(Clone)]
pub enum Mode {
    Shell,
    Ask,
}

pub async fn handle_keys(stdout: &mut Stdout) -> Result<String, Box<dyn Error>> {
    let mut position = 0;
    let mut input: String = "".to_string();

    match enable_raw_mode() {
        Ok(()) => {}
        Err(_) => panic! {"unable to enter raw mode"},
    }
    let mut mode = Mode::Shell;

    loop {
        print_prompt(stdout, &mode, &input, &position);

        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(event) => match event.code {
                KeyCode::Char(c) => {
                    if c == 'd' && event.modifiers == KeyModifiers::CONTROL {
                        exit_raw_mode();
                        exit(0);
                    } else if c == 'c' && event.modifiers == KeyModifiers::CONTROL {
                        match mode {
                            Mode::Shell => {
                                input = "".to_string();
                                break;
                            }
                            Mode::Ask => mode = Mode::Shell,
                        }
                    } else if c == 'a' && event.modifiers == KeyModifiers::CONTROL {
                        mode = Mode::Ask;
                    } else if position < input.len() {
                        let line = input.clone();
                        let (head, tail) = line.split_at(position);
                        input = format!("{}{}{}", head, c, tail);
                        position += 1;
                    } else {
                        input.push(c);
                        position += 1;
                    }
                }
                KeyCode::Backspace => {
                    if input.len() > 0 && position > 0 {
                        if input.len() == position {
                            // delete character at end of line
                            input.pop();
                            position -= 1;
                        } else {
                            // delete character from inside the line
                            let line = input.clone();
                            let (head, tail) = line.split_at(position - 1);
                            input = head.to_string() + &tail[1..];
                            position -= 1;
                        }
                    }
                }
                KeyCode::Left => {
                    if position > 0 {
                        position = position - 1;
                    }
                }
                KeyCode::Right => {
                    if position < input.len() {
                        position = position + 1;
                    }
                }
                KeyCode::Enter => match mode {
                    Mode::Shell => break,
                    Mode::Ask => match get_openai_completion(&input).await {
                        Ok(completion) => {
                            input = completion.clone();
                            position = input.len();
                            mode = Mode::Shell;
                        }
                        Err(_) => {}
                    },
                },
                KeyCode::Tab => {
                    // Handle completions
                    let completions = get_completion(&input);

                    if completions.len() > 1 {
                        queue!(
                            stdout,
                            SavePosition,
                            MoveToColumn(0),
                            Clear(crossterm::terminal::ClearType::FromCursorDown)
                        )?;

                        for completion in completions {
                            queue!(stdout, Print(completion + "    "))?;
                        }

                        execute!(stdout, Print("\n"), MoveToColumn(0))?;
                        print_prompt(stdout, &mode, &input, &position);
                        execute!(stdout, Print(input.clone()), RestorePosition)?;
                    } else if completions.len() == 1 {
                        match execute!(stdout, Print(completions[0].clone())) {
                            Ok(()) => {
                                input.push_str(completions[0].as_ref());
                                position += completions[0].len();
                            }
                            Err(_) => {}
                        }
                    }
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

pub fn print_prompt(stdout: &mut Stdout, mode: &Mode, input: &str, position: &usize) {
    let prompt = match mode {
        Mode::Shell => match var("PS2") {
            Ok(val) => val,
            Err(_) => "$ ".to_string(),
        },
        Mode::Ask => "[ask-crust]: ".to_string(),
    };

    execute!(
        stdout,
        MoveToColumn(0),
        Clear(crossterm::terminal::ClearType::FromCursorDown),
        Print(prompt.clone() + input),
        MoveToColumn((prompt.len() + position).try_into().unwrap())
    )
    .ok();
}

async fn get_openai_completion(input: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let request = CreateCompletionRequestArgs::default()
            .model("text-davinci-003")
            .prompt(format!{"Provide a command line snippet for achieving the following task. Only answer with the code, nothing more.
Task: {}?
Snippet: `", input})
            .max_tokens(40_u16)
            .build()?;

    let response = client.completions().create(request).await?;

    if response.choices.len() == 1 {
        let choice = response.choices[0].text.clone();
        return Ok(choice[..choice.len() - 1].to_string());
    }

    Ok("".to_string())
}
