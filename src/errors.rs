// use rand::{thread_rng, Rng};

extern crate rand;

pub enum Errors {
    CommandNotFound,
    PermissionDenied,
    FileNotFound,
    InvalidArgument,
}

pub fn get_error_message(err: Errors) -> String {
    // let mut rng = thread_rng();
    // let y: u8 = rng.gen_range(0, 2);

    match err {
        Errors::CommandNotFound => return "Rodeo! Couldn't find that command 🤠.".to_string(),
        Errors::PermissionDenied => return "Permission denied 🚫. Looks like you're not the sheriff around here.".to_string(),
        Errors::FileNotFound => return "File not found 🙁, it's probably out there somewhere. Keep searching partner.".to_string(),
        Errors::InvalidArgument => return "Uh oh, that argument won't work 🤔. Time to try a different trail.".to_string(),
    }
}
