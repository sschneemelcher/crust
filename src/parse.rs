use crate::{Builtins, Input};

use pest::Parser;

#[derive(Parser)]
#[grammar = "syntax.pest"]
struct ShellParser;

pub fn parse_input(raw_input: &str) -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];

    let parse_result = ShellParser::parse(Rule::command, &raw_input);

    match parse_result {
        Ok(mut result) => {
            let command = result.next().unwrap();
            let command_parts = command
                .into_inner()
                .map(|pair| pair.as_str())
                .collect::<Vec<_>>();
            inputs.push(Input {
                command: command_parts[0].to_owned(),
                args: command_parts[1..]
                    .iter()
                    .map(|str| str.to_string())
                    .collect(),
                bg: false,
                builtin: Builtins::None,
            });
            inputs
        }
        Err(_e) => vec![],
    }

    // Parse input line by line
    // for mut line in raw_input.split(['\n', ';']) {
    //     if line.len() < 1 {
    //         continue;
    //     };

    //     let mut parsed_input: Input = Default::default();

    //     // handle running command in background
    //     if &line[(line.len() - 1)..] == "&" {
    //         parsed_input.bg = true;
    //         line = &line[..(line.len() - 1)]
    //     }

    //     let mut words = line.split_whitespace();

    //     // parse the first word of the input
    //     match words.next() {
    //         // match the first word of the input
    //         None => inputs.push(parsed_input.clone()),
    //         Some("exit") => {
    //             parsed_input.builtin = Builtins::Exit;
    //             inputs.push(parsed_input.clone());
    //         }
    //         Some("cd") => parsed_input.builtin = Builtins::CD,
    //         Some("echo") => parsed_input.builtin = Builtins::Echo,
    //         Some("alias") => parsed_input.builtin = Builtins::Alias,
    //         Some(command) => {
    //             parsed_input.command = command.to_owned();
    //         }
    //     }

    //     for word in words {
    //         match word {
    //             "&" => parsed_input.bg = true,
    //             arg => parsed_input.args.push(arg.to_owned()),
    //         }
    //     }
    //     inputs.push(parsed_input);
    // }
    // return inputs;
}

#[test]
fn test_ls() {
    let inputs: Vec<Input> = parse_input("ls -a -l");
    assert_eq!(inputs.len(), 1);
    let input: &Input = &inputs[0];
    assert_eq!(input.command, "ls");
    assert_eq!(input.args, ["-a", "-l"]);
    assert_eq!(input.bg, false);
    assert_eq!(input.builtin, Builtins::None);
}

#[test]
fn test_bg() {
    let inputs: Vec<Input> = parse_input("ls -a -l&");
    assert_eq!(inputs.len(), 1);
    let input: &Input = &inputs[0];
    assert_eq!(input.command, "ls");
    assert_eq!(input.args, ["-a", "-l"]);
    assert_eq!(input.bg, true);
    assert_eq!(input.builtin, Builtins::None);
}

#[test]
fn test_chaining() {
    let inputs: Vec<Input> = parse_input("ls -a -l; cat README.md; echo Hello World");
    assert_eq!(inputs.len(), 3);

    let mut input: &Input = &inputs[0];
    assert_eq!(input.command, "ls");
    assert_eq!(input.args, ["-a", "-l"]);
    assert_eq!(input.bg, false);
    assert_eq!(input.builtin, Builtins::None);

    input = &inputs[1];
    assert_eq!(input.command, "cat");
    assert_eq!(input.args, ["README.md"]);
    assert_eq!(input.bg, false);
    assert_eq!(input.builtin, Builtins::None);

    input = &inputs[2];
    assert_eq!(input.command, "");
    assert_eq!(input.args, ["Hello", "World"]);
    assert_eq!(input.bg, false);
    assert_eq!(input.builtin, Builtins::Echo);
}
