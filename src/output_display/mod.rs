use colored::{Colorize, CustomColor};

#[derive(Clone,PartialEq)]
enum TextColor {
    White,
    Green,
    Yellow
}

pub fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
    let ascii_art = r#"
 _____  _     _____     ______ _      _____ 
/  __ \| |   |_   _|    |  _  \ |    |  ___|
| /  \/| |     | |______| | | | |    | |__  
| |    | |     | |______| | | | |    |  __| 
| \__/\| |_____| |_     | |/ /| |____| |___ 
 \____/\_____/\___/     |___/ \_____/\____/                                             
"#;
    println!("{}",ascii_art.to_string().magenta());
}

fn replace(target: Vec<char>, find_char: char, replacement_char:char) -> Vec<char> {

    let mut target = target;

    let index = target.iter().position(|&x| x == find_char).unwrap();

    target[index] = replacement_char;

    target

}

fn display_word(word: &String, wordle: &String) {
    let guess:Vec<char> = word.chars().collect();
    let mut output: Vec<TextColor> = vec![TextColor::White; guess.len()];
    let mut target: Vec<char> = wordle.chars().collect();

    let gold_colour = CustomColor::new(249, 156, 20);

    for i in 0..target.len(){
        if guess[i] == target[i] {
            output[i] = TextColor::Green;
            target = replace(target, guess[i],'-');
            
        }
        if target.contains(&guess[i]) && output[i] == TextColor::White {
            output[i] = TextColor::Yellow;
            target = replace(target, guess[i], '-');
        }  
    }

    print!("|");

    for i in 0..guess.len() {
        if output[i] == TextColor::Green {
            print!(" {} |", guess[i].to_string().to_uppercase().green().bold());
        }
        else if output[i] == TextColor::Yellow {
            print!(" {} |", guess[i].to_string().to_uppercase().custom_color(gold_colour).bold());
        }
        else {
            print!(" {} |", guess[i].to_string().to_uppercase().white().bold());

        }
    }

    println!("");
}

pub fn display_boxes(number : u8) {
    for _i in 0..number {
        println!("---------------------");
        println!("|   |   |   |   |   |");
        println!("---------------------");
    }
}

pub fn display(guess_words: &Vec<String>, wordle: &String) {
    clear_screen();
    
    println!("---------------------");
    for word in guess_words {
        display_word(word, wordle);
        println!("---------------------");
    }
}