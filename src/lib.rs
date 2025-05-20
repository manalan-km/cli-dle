use std::io;

mod dailies;
mod infinite;
mod help;
mod data;
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

fn display(guess_words: &Vec<String>) {
    for words in guess_words {
        println!("{} {} {} {} {}",words.chars().nth(0).unwrap(),
                                  words.chars().nth(1).unwrap(),
                                  words.chars().nth(2).unwrap(),
                                  words.chars().nth(3).unwrap(),
                                  words.chars().nth(4).unwrap(),);
        println!("- - - - -");
    }
}

pub fn run_game(wordle:String) { 
    let mut guess: u8 = 0;
    let mut guesses: Vec<String> = Vec::new();
    while guess < 6 {
        let mut guess_word = String::new();
        println!("Your guess:");
        io::stdin().read_line(&mut guess_word).expect("Failed to read input");
        guess_word = guess_word.trim().to_string();

        if wordle == guess_word {
            guesses.push(guess_word);
            print!("\x1B[2J\x1B[1;1H");
            display(&guesses);
            println!("You win!");
            return
        }

        guesses.push(guess_word);
        print!("\x1B[2J\x1B[1;1H");
        display(&guesses);

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