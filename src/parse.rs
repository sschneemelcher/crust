use crate::Builtins;

use pest::Parser;

#[derive(Parser)]
#[grammar = "syntax.pest"]
struct ShellParser;

#[derive(Clone, Default)]
pub struct Input {
    pub command: String,
    pub args: Vec<String>,
    pub bg: bool,
    pub builtin: Builtins,
}

pub fn parse_input(raw_input: &str) -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];

    let lines = ShellParser::parse(Rule::command_list, &raw_input)
        .unwrap_or_else(|e| panic!("command parsing failed: {}", e));

    // println!("{:#?}", lines);
    for command in lines {
        let mut input = Input::default();
        for command_parts in command.to_owned().into_inner() {
            match command_parts.as_rule() {
                Rule::command_name => input.command = command_parts.as_str().to_owned(),
                Rule::bg_indicator => input.bg = true,
                Rule::argument => input.args.push(command_parts.as_str().to_owned()),
                _ => {}
            }
        }

        match input.command.as_ref() {
            "exit" => input.builtin = Builtins::Exit,
            // "echo" => input.builtin = Builtins::Echo,
            "cd" => input.builtin = Builtins::CD,
            _ => {}
        }

        if input.command.len() > 0 {
            inputs.push(input.to_owned());
        }

        // inputs.push(input);
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
