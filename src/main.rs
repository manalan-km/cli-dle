use cli_dle;
use std::{env, process};

fn main() {

    let args: Vec<String> = env::args().collect();

    let game_mode = cli_dle::Config::build_config(args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    cli_dle::run(game_mode.mode);    
    
}
