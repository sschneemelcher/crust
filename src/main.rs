use std::io::stdin;
use std::process::{Command, Child};

fn main() {
    loop {
        let mut input = String::new();

        stdin()
        .read_line(&mut input)
        .expect("expected a line");
        let mut iter = input.split_whitespace();

        match iter.next() {                             // iterate over the input word by word

            None => {},                                 // if the input is empty, do nothing 
            word => {
                let command = word.unwrap();
                if command == "exit" {
                    return;
                } else {
                    let mut cmd = Command::new(command);
                    for s in iter {
                        cmd.arg(s);                         // iterate over the arguments
                    }
                    match cmd.spawn() {                     // spawn the processed command
                        Ok(Child{stdin: _, mut stdout, stderr: _, ..}) => {
                            match stdout.take() {  
                                None => {},                 // if there is no output, do nothing
                                x => println!("{:#?}", x.unwrap()) // else print the output
                            }
                        },
                        Err(e) => println!("{:#?}", e)      // if spawning failed, print message
                    } 
                }
            }
        }
    }
}
