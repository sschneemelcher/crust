use crate::Builtins;

use pest::{error::InputLocation, iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "syntax.pest"]
struct CrustParser;

#[derive(Clone, Default)]
pub struct Input {
    pub command: String,
    pub args: Vec<String>,
    pub bg: bool,
    pub builtin: Builtins,
}

pub fn parse_input(raw_input: &str) -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];

    let lines = match CrustParser::parse(Rule::script, &raw_input) {
        Ok(results) => results,
        Err(e) => {
            if let InputLocation::Pos(i) = e.location {
                println!(
                    "command parsing failed - unexpected symbol {:#?}",
                    String::from_utf8(vec![raw_input.as_bytes()[i]])
                        .ok()
                        .unwrap()
                )
            }
            return vec![];
        }
    };

    let mut input = Input::default();
    for command in lines {
        match command.as_rule() {
            Rule::command_name => command_name(&command, &mut input),
            Rule::bg_indicator => {
                input.bg = true;
                inputs.push(input.to_owned());
            }
            Rule::EOI | Rule::line_sep => {
                if input.command.len() > 0 && !input.bg {
                    inputs.push(input.to_owned());
                }
                input = Input::default();
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

fn command_name(command_name: &Pair<Rule>, input: &mut Input) {
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
