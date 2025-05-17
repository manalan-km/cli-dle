use wordle_cli;
use std::{env, process};

fn main() {

    let args: Vec<String> = env::args().collect();

    let game_mode = wordle_cli::Config::build_config(args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    wordle_cli::run(game_mode.mode);    
    
}
