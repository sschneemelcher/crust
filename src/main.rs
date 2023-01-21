use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
    loop {
        let mut lock = stdout().lock();
        write!(lock, "$ ").unwrap();
        match lock.flush() {
            Ok(_) => {}
            Err(e) => println!("{:#?}", e),
        }

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
                    Ok(mut child) => {
                        match child.wait() {
                            Ok(_) => {}
                            Err(e) => println!("{:#?}", e),
                        }
                        match child.stdout.take() {
                            None => {}
                            Some(output) => println!("{:#?}", output),
                        }
                    }
                    // if spawning failed, print message
                    Err(e) => println!("{:#?}", e),
                }
            }
        }
    }
}
