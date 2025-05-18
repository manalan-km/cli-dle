use crate::data::{self as data, get_guesses, get_wordles};

pub fn run() {
    let official_wordle = data::WORDLES.get_or_init(get_wordles);
    let official_guesses = data::GUESSES.get_or_init(get_guesses);

    println!("{}", official_wordle.len());
    println!("{}", official_guesses.len());
    println!("Running dailies mode");
}