use std::io::stdin;
use std::process::{Child, Command};

fn main() {
    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).expect("expected a line");
        let mut words = input.split_whitespace();

        match words.next() {
            // match the first word of the input
            None => {}
            Some("exit") => {
                return;
            }
            Some(command) => {
                let mut cmd = Command::new(command);
                for word in words {
                    // iterate over the arguments
                    cmd.arg(word);
                }
                match cmd.spawn() {
                    // spawn the processed command
                    Ok(Child {
                        stdin: _,
                        mut stdout,
                        stderr: _,
                        ..
                    }) => match stdout.take() {
                        None => {}
                        Some(output) => println!("{:#?}", output),
                    },

                    // if spawning failed, print message
                    Err(e) => println!("{:#?}", e),
                }
            }
        }
    }
}
