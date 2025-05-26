
pub fn get_wordles() -> Vec<String> {

    let wordles:Vec<String>= include_str!("shuffled_real_wordles.txt").lines().map(|word| word.to_string()).collect();


    wordles

}

pub fn get_guesses() -> Vec<String> {

    let guesses:Vec<String>= include_str!("combined_wordlist.txt").lines().map(|word| word.to_string()).collect();

    guesses

}
