extern crate colored;

use std::io;
use colored::{Colorize, CustomColor};

mod dailies;
mod infinite;
mod help;
mod data;
mod guess_validation;

#[derive(Debug)]
pub struct Config {
    pub mode: Mode
}

impl Config {
    pub fn build_config (args: Vec<String>) -> Result<Config,&'static str> {
        
        if args.len() > 2 {
            println!("Multiple arguments were provided. Only the first argument is considered.")
        }

        if args.len() < 2 {
            return Err("Not enough arguments. \n
            Usage: \n
            clirdle  <game_mode> \n");
        }
        
        let mode = &args[1];
        if mode == "dailies" {
            Ok(Config{mode: Mode::Dailies})
        }
        else if mode == "infinite" {
            Ok(Config { mode: Mode::Infinite })
        }
        else if mode == "help" {
            Ok(Config { mode: Mode::Help })
        }
        else {
            Err("Invalid mode")
        }
        
    }
}

#[derive(Debug)]
pub enum Mode {
    Dailies,
    Infinite,
    Help
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn display_word(word: &String, wordle: &String) {
    let word_characters : Vec<char> = word.chars().collect();
    let wordle_characters : Vec<char> = wordle.chars().collect();
    let gold_colour = CustomColor::new(249, 156, 20);
    print!("|");

    for i in 0..word_characters.len() {
        
        if wordle_characters.contains(&word_characters[i]) {
            if wordle_characters[i] == word_characters[i] {
                print!(" {} |", word_characters[i].to_string().to_uppercase()
                .green().bold())
            }else {
                print!(" {} |", word_characters[i].to_string().to_uppercase().custom_color(gold_colour).bold())
            }            
        }
        else {
            print!(" {} |", word_characters[i].to_string().to_uppercase());
        }
    }
    println!("");

}

fn display(guess_words: &Vec<String>, wordle: &String) {
    clear_screen();
    
    println!("---------------------");
    for word in guess_words {
        display_word(word, wordle);
        println!("---------------------");
    }
}

pub fn run_game(wordle:&String) { 
    let mut guess: u8 = 0;
    let mut guesses: Vec<String> = Vec::new();
    clear_screen();
    while guess < 6 {
        println!("#{}/6", guess +1);
        let mut guess_word = String::new();
        println!("Your guess:");
        io::stdin().read_line(&mut guess_word).expect("Failed to read input");
        guess_word = guess_word.trim().to_string();

        if let Err(e) = guess_validation::validate_guess(&guess_word,&guesses) {
            println!("Error: {}",e);
            continue;
        }

        if wordle == &guess_word {
            
            guesses.push(guess_word);
            display(&guesses, &wordle);
            println!("You win!");
            return
        }

        guesses.push(guess_word);
        
        display(&guesses, &wordle);
        guess = guess + 1;

    }
}


pub fn run(mode:Mode) {

    match mode {
        Mode::Dailies => dailies::run(),
        Mode::Infinite => infinite::run(),
        Mode::Help => help::run(), 
    };

}