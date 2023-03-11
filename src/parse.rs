use crate::{Builtins, Input};

use pest::Parser;

#[derive(Parser)]
#[grammar = "syntax.pest"]
struct ShellParser;

pub fn parse_input(raw_input: &str) -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];

    let lines = ShellParser::parse(Rule::command_list, &raw_input)
        .unwrap_or_else(|e| panic!("command parsing failed: {}", e));

    for command in lines {
        let command_parts = command
            .into_inner()
            .map(|pair| pair.as_str())
            .collect::<Vec<_>>();
        if command_parts.len() == 0 {
            continue;
        }

        let mut input = Input {
            command: command_parts[0].to_owned(),
            args: command_parts[1..]
                .iter()
                .map(|str| str.to_string())
                .collect(),
            // TODO
            bg: false,
            builtin: Builtins::None,
        };

        match input.command.as_ref() {
            "exit" => input.builtin = Builtins::Exit,
            "echo" => input.builtin = Builtins::Echo,
            "cd" => input.builtin = Builtins::CD,
            _ => {}
        }

        inputs.push(input);
    }

    return inputs;
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
    let inputs: Vec<Input> = parse_input("ls -a -l; cat README.md; cowsay Hello World");
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
    assert_eq!(input.command, "cowsay");
    assert_eq!(input.args, ["Hello", "World"]);
    assert_eq!(input.bg, false);
    assert_eq!(input.builtin, Builtins::None);
}
