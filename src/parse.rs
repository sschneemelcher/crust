use crate::{Builtins, Input};

pub fn parse_input(raw_input: String) -> Vec<Input> {
    let mut inputs: Vec<Input> = vec![];

    // Parse input line by line
    for mut line in raw_input.split(['\n', ';']) {
        if line.len() < 1 {
            continue;
        };

        let mut parsed_input: Input = Default::default();

        // handle running command in background
        if &line[(line.len() - 1)..] == "&" {
            parsed_input.bg = true;
            line = &line[..(line.len() - 1)]
        }

        let mut words = line.split_whitespace();

        // parse the first word of the input
        match words.next() {
            // match the first word of the input
            None => inputs.push(parsed_input.clone()),
            Some("exit") => {
                parsed_input.builtin = Builtins::Exit;
                inputs.push(parsed_input.clone());
            }
            Some("cd") => parsed_input.builtin = Builtins::CD,
            Some("echo") => parsed_input.builtin = Builtins::Echo,
            Some("alias") => parsed_input.builtin = Builtins::Alias,
            Some(command) => {
                parsed_input.command = command.to_string();
            }
        }

        for word in words {
            match word {
                "&" => parsed_input.bg = true,
                arg => parsed_input.args.push(arg.to_string()),
            }
        }
        inputs.push(parsed_input);
    }
    return inputs;
}

#[test]
fn test_ls() {
    let inputs: Vec<Input> = parse_input("ls -a -l".to_string());
    assert_eq!(inputs.len(), 1);
    let input: &Input = &inputs[0];
    assert_eq!(input.command, "ls");
    assert_eq!(input.args, ["-a", "-l"]);
    assert_eq!(input.bg, false);
    assert_eq!(input.builtin, Builtins::None);
}

#[test]
fn test_bg() {
    let inputs: Vec<Input> = parse_input("ls -a -l&".to_string());
    assert_eq!(inputs.len(), 1);
    let input: &Input = &inputs[0];
    assert_eq!(input.command, "ls");
    assert_eq!(input.args, ["-a", "-l"]);
    assert_eq!(input.bg, true);
    assert_eq!(input.builtin, Builtins::None);
}

#[test]
fn test_chaining() {
    let inputs: Vec<Input> = parse_input("ls -a -l; cat README.md; echo Hello World".to_string());
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
