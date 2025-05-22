use colored::{Colorize, CustomColor};

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