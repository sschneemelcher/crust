use crate::Builtins;

use pest::{iterators::Pair, Parser};

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

fn parse_command_name(command_name: Pair<Rule>, input: &mut Input) {
    let command_type = match command_name.to_owned().into_inner().next() {
        Some(ct) => ct,
        None => return,
    };
    match command_type.as_rule() {
        Rule::builtin_command => {
            let builtin = match command_type.into_inner().next() {
                Some(b) => b,
                None => return,
            };
            match builtin.as_rule() {
                Rule::exit => {
                    input.builtin = Builtins::Exit;
                    input.command = "exit".to_owned();
                }
                Rule::cd => {
                    input.builtin = Builtins::CD;
                    input.command = "cd".to_owned();
                }
                _ => panic!("should not be reached"),
            }
        }
        Rule::external_command => input.command = command_name.as_str().to_owned(),
        _ => panic!("should not be reached"),
    }
}

pub fn parse_input(raw_input: &str) -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];

    let commands = ShellParser::parse(Rule::lines, &raw_input)
        .unwrap_or_else(|e| panic!("command parsing failed: {}", e));

    let mut input = Input::default();
    for command in commands {
        match command.as_rule() {
            Rule::command_name => parse_command_name(command, &mut input),
            Rule::EOI | Rule::line_sep => {
                if input.command.len() > 0 {
                    inputs.push(input.to_owned());
                }
                input = Input::default();
            }
            Rule::bg_indicator => {
                input.bg = true;
                inputs.push(input.to_owned())
            }
            Rule::arg => input.args.push(command.as_str().to_owned()),
            _ => {
                println!("{:#?}", command);
            }
        }
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
