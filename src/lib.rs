extern crate colored;

use std::io;


mod dailies;
mod infinite;
mod help;
mod data;
mod guess_validation;
mod output_display;

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


pub fn run_game(wordle:&String) { 
    let mut guess: u8 = 0;
    let mut guesses: Vec<String> = Vec::new();
    output_display::clear_screen();
    while guess < 6 {
        println!("#{}/6", guess +1);
        output_display::display_boxes(6 - guess);
        let mut guess_word = String::new();
        println!("Your guess:");
        io::stdin().read_line(&mut guess_word).expect("Failed to read input");
        guess_word = guess_word.trim().to_string();

        if let Err(e) = guess_validation::validate_guess(&guess_word,&guesses) {
            println!("Error: {}",e);
            println!("Press Enter to continue!");
            let mut dummy: String = String::new();
            io::stdin().read_line(&mut dummy).expect("Failed to read input!");
            output_display::display(&guesses, &wordle);
            continue;
        }

        if wordle == &guess_word {
            
            guesses.push(guess_word);
            output_display::display(&guesses, &wordle);
            println!("You win!");
            return
        }

        guesses.push(guess_word);
        
        output_display::display(&guesses, &wordle);
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