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
    let choices = match err {
        Errors::CommandNotFound =>  vec!["Rodeo! Couldn't find that command 🤠.",
                                         "Hold on partner, I don't recognize that command 🤔.",
                                         "Whoopsie 🤯, it seems that command done got away."],

        Errors::PermissionDenied =>  vec!["Permission denied 🚫. Looks like you're not the sheriff around here.",
                                          "You don't have the necessary permissions to do that. Time to call in the big guns (sudo) 💪.",
                                          "Uh oh, permission denied 🚨. You might need to ask for help from the root sheriff."],

        Errors::FileNotFound => vec!["File not found 🙁, it's probably out there somewhere. Keep searching partner.",
                                     "Whoops! I couldn't find that file 🤨. Maybe it's hiding in the hills.",
                                     "File not found 🤔, did you check your trail?",
                                     "Looks like a snake in your boot! Couldn't find that file 🐍",
                                     "Galloping ghosts! The file couldn't be found 🐴",
                                     "Well shucks, looks like we hit a snag finding that file 🤠"],
        
        Errors::InvalidArgument => vec!["Uh oh, that argument won't work 🤔. Time to try a different trail.",
                                        "invalid argument 🚫. You need to check your input, partner.",
                                        "Oops! That argument is invalid 🤨. Try a different one."]
    };
    let n: usize = rng.gen_range(0..choices.len());

    return choices[n].to_owned();
}
