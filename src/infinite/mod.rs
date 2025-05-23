use crate::{data::{self, get_wordles}, run_game};
use rand::prelude::*;
use std::io;

fn set_wordle () -> &'static String {

    let wordle = data::WORDLES.get_or_init(get_wordles);
    let mut rng = rand::rng();
    let index = rng.random_range(0..wordle.len() + 1);
    &wordle[index]


}



pub fn run() {
    println!("Running infinite mode");

    loop{
        let wordle = set_wordle().to_string();
        println!("Wordle for this round: {wordle}");
        run_game(&wordle);
        println!("Wordle for this round: {wordle}");

        println!("Press Enter to continue!");
        let mut dummy: String = String::new();
        io::stdin().read_line(&mut dummy).expect("Failed to read input!");
    }

    

}