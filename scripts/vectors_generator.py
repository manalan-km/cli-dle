
output_file_content = """
//This file is generated from scripts/vectors_generator.py. \n
//Please make changes to that file, if need be :) \n

use std::sync::OnceLock;


pub static WORDLES : OnceLock<Vec<String>> = OnceLock::new();
pub static GUESSES : OnceLock<Vec<String>> = OnceLock::new();

pub fn get_wordles() -> Vec<String> {

    {{WORDLE_VECTOR}}

}
pub fn get_guesses() -> Vec<String> {

    {{GUESSES_VECTOR}}

}

"""

def replace(file_path:str, replacement_string: str):
    with open(file_path,"r") as input_file:
        elements = input_file.read().splitlines()
        vec = "vec!["
        for element in elements:
            if element.startswith("#"):
                continue
            else:
                vec = vec + f"String::from(\"{element}\"),"
        
        vec = vec + "]"
        return output_file_content.replace(replacement_string, vec)

output_file_content = replace("shuffled_real_wordles.txt","{{WORDLE_VECTOR}}")
output_file_content = replace("official_allowed_guesses.txt","{{GUESSES_VECTOR}}")

print(output_file_content)

with open("../src/data/mod.rs","w") as output_file:
    output_file.write(output_file_content)