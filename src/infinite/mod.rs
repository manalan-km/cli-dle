use crate::{data::{self, get_wordles}, run_game};
use rand::prelude::*;

fn set_wordle () -> &'static String {

    let wordle = data::WORDLES.get_or_init(get_wordles);
    let mut rng = rand::rng();
    let index = rng.random_range(0..wordle.len() + 1);
    &wordle[index]


}



pub fn run() {
    println!("Running infinite mode");

    let wordle = set_wordle().to_string();
    println!("{wordle}");
    run_game(wordle);

}