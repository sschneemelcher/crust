use async_openai::{types::CreateCompletionRequestArgs, Client};
use crossterm::{
    cursor::MoveToColumn,
    event::{read, Event, KeyCode, KeyModifiers},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear},
};
use std::{convert::TryInto, env::var, error::Error, fs, io::stdout, process::exit};
use tokio::sync::mpsc::{Receiver, Sender};

fn exit_raw_mode() {
    match disable_raw_mode() {
        Ok(()) => {}
        Err(_) => panic! {"unable to exit raw mode"},
    };
}

#[derive(Debug, Default, Clone)]
pub struct Prompt {
    input: String,
    position: usize,
    mode: Mode,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Mode {
    #[default]
    Shell,
    Ask,
    Enter,
    Break,
    Exit,
}

pub async fn handle_keys(prompt_tx: Sender<Prompt>) -> Result<String, Box<dyn Error>> {
    let mut prompt = Prompt::default();

    prompt_tx.send(Prompt::default()).await.ok();

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
                        prompt.mode = Mode::Exit;
                    } else if c == 'c' && event.modifiers == KeyModifiers::CONTROL {
                        match prompt.mode {
                            Mode::Ask => prompt.mode = Mode::Shell,
                            _ => {
                                prompt.mode = Mode::Break;
                            }
                        }
                    } else if c == 'a' && event.modifiers == KeyModifiers::CONTROL {
                        prompt.mode = Mode::Ask;
                    } else if prompt.position < prompt.input.len() {
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
                    if prompt.input.len() > 0 && prompt.position > 0 {
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
                KeyCode::Enter => match prompt.mode {
                    Mode::Ask => match get_openai_completion(&prompt.input).await {
                        Ok(completion) => {
                            prompt.input = completion.clone();
                            prompt.position = prompt.input.len();
                            prompt.mode = Mode::Shell;
                        }
                        Err(_) => {}
                    },

                    _ => {
                        prompt.input.push('\n');
                        prompt.mode = Mode::Enter;
                    }
                },
                // KeyCode::Tab => {
                //     // Handle completions
                //     let completions = get_completion(&prompt.input);

                //     if completions.len() > 1 {
                //         queue!(
                //             stdout,
                //             Saveprompt.position,
                //             MoveToColumn(0),
                //             Clear(crossterm::terminal::ClearType::FromCursorDown)
                //         )?;

                //         for completion in completions {
                //             queue!(stdout, Print(completion + "    "))?;
                //         }

                //         execute!(stdout, Print("\n"), MoveToColumn(0))?;
                //         // print_prompt(stdout, &mode, &prompt.input, &prompt.position);
                //         execute!(stdout, Print(prompt.input.clone()), Restoreprompt.position)?;
                //     } else if completions.len() == 1 {
                //         match execute!(stdout, Print(completions[0].clone())) {
                //             Ok(()) => {
                //                 prompt.input.push_str(completions[0].as_ref());
                //                 prompt.position += completions[0].len();
                //             }
                //             Err(_) => {}
                //         }
                //     }
                // }
                _ => {}
            },
            _ => {}
        };

        prompt_tx.send(prompt.clone()).await.ok();

        match prompt.mode {
            Mode::Exit => {
                exit_raw_mode();
                exit(0);
            }
            Mode::Enter => {
                exit_raw_mode();
                return Ok(prompt.input);
            }
            Mode::Break => {
                prompt = Prompt::default();
                prompt_tx.send(prompt.clone()).await.ok();
            }
            _ => {}
        }
    }
}

// fn get_completion(_line: &str) -> Vec<String> {
//     let mut completions: Vec<String> = vec![];

//     match fs::read_dir(".") {
//         Ok(paths) => {
//             for path in paths {
//                 completions.push(path.unwrap().path().display().to_string());
//             }
//         }
//         Err(_) => {}
//     }
//     return completions;
// }

pub async fn print_prompt(mut rx: Receiver<Prompt>) {
    let mut stdout = stdout();
    while let Some(prompt) = rx.recv().await {
        let ps2 = match prompt.mode {
            Mode::Ask => "[ask-crust]: ".to_string(),
            _ => match var("PS2") {
                Ok(val) => val,
                Err(_) => "$ ".to_string(),
            },
        };

        execute!(
            stdout,
            MoveToColumn(0),
            Clear(crossterm::terminal::ClearType::FromCursorDown),
            Print(format! {"{}{}", ps2.clone(), prompt.input}),
            MoveToColumn((ps2.len() + prompt.position).try_into().unwrap())
        )
        .ok();

        if prompt.mode == Mode::Enter {
            execute!(stdout, MoveToColumn(0),).ok();
        } else if prompt.mode == Mode::Break {
            execute!(stdout, Print("^C\n")).ok();
        }
    }
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
