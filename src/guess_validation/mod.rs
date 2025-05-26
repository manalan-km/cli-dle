// use crate::data::{get_guesses, GUESSES};

use crate::data::get_guesses;

pub enum ValidationErrors {
    TooShort,
    TooLong,
    ContainsInvalidChars,
    InvalidGuess,
    GuessAlreadyExists
}


impl std::fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationErrors::TooShort => write!(f,"Your guess is too short!"),
            ValidationErrors::TooLong => write!(f,"Your guess is too long! Maximum number of letters should be 5."),
            ValidationErrors::ContainsInvalidChars => write!(f,"Your guess contains invalid characters. Please use letters only in English Alphabet :)"),
            ValidationErrors::InvalidGuess => write!(f, "Invalid guess! Please use a valid 5 letter word :)"),
            ValidationErrors::GuessAlreadyExists => write!(f,"You have already used this word! Try again with a new one"),
        }
    }
}


pub fn validate_guess<'a>(guess_word: &'a String, guess_words: &'a Vec<String>) -> Result<&'a String,ValidationErrors> {

    let possible_guess = get_guesses();

    if !guess_word.chars().all(|x| x.is_ascii_alphabetic()) {
        return Err(ValidationErrors::ContainsInvalidChars);
    }

    if guess_word.len() > 5 {
        return Err(ValidationErrors::TooLong);
    }

    if guess_word.len() < 5 {
        return Err(ValidationErrors::TooShort);
    }

    if !possible_guess.contains(&guess_word) {
        return Err(ValidationErrors::InvalidGuess);
    }

    if guess_words.contains(&guess_word) {
        return Err(ValidationErrors::GuessAlreadyExists);
    }    

    Ok(guess_word)
}
