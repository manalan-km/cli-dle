use crate::{data::{ get_wordles}, run_game};
use rand::prelude::*;
use std::io;

fn set_wordle () -> String {

    let wordles = get_wordles();
    let mut rng = rand::rng();
    let index = rng.random_range(0..wordles.len() + 1);
    let wordle = wordles[index].clone();

    wordle
}



pub fn run() {
    println!("Running infinite mode");

   loop{
        let wordle = set_wordle().to_string();
        run_game(&wordle);
        println!("Wordle for this round: {wordle}");

        println!("Press Enter to continue!");
        let mut dummy: String = String::new();
        io::stdin().read_line(&mut dummy).expect("Failed to read input!");
    }

    

}