use rand::{thread_rng, Rng};

extern crate rand;

pub enum Errors {
    CommandNotFound,
    PermissionDenied,
    FileNotFound,
    InvalidArgument,
}

pub fn get_error_message(err: Errors) -> String {
    let mut rng = thread_rng();

    let message: &str = match err {
        Errors::CommandNotFound => {
            let n: u8 = rng.gen_range(0..3);
            match n {
                0 => "Rodeo! Couldn't find that command 🤠.",
                1 => "Hold on partner, I don't recognize that command 🤔.",
                _ => "Whoopsie 🤯, it seems that command done got away.",
            }
        }
        Errors::PermissionDenied => {
            let n: u8 = rng.gen_range(0..3);
            match n {
                0 => "Permission denied 🚫. Looks like you're not the sheriff around here.",
                1 => "You don't have the necessary permissions to do that. Time to call in the big guns (sudo) 💪.",
                _ => "Uh oh, permission denied 🚨. You might need to ask for help from the root sheriff.",
            }
        }
        Errors::FileNotFound => {
            let n: u8 = rng.gen_range(0..3);
            match n {
                0 => "File not found 🙁, it's probably out there somewhere. Keep searching partner.",
                1 => "Whoops! I couldn't find that file 🤨. Maybe it's hiding in the hills.",
                _ => "File not found 🤔, did you check your trail?",
            }
        }
        Errors::InvalidArgument => {
            let n: u8 = rng.gen_range(0..3);
            match n {
                0 => "Uh oh, that argument won't work 🤔. Time to try a different trail.",
                1 => "invalid argument 🚫. You need to check your input, partner.",
                _ => "Oops! That argument is invalid 🤨. Try a different one.",
            }
        }
    };

    return message.to_string();
}
