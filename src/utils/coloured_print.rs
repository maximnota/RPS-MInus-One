use colored::*;
use std::io::{self, Write};

pub fn coloured_print(text: &str, colour: (u8, u8, u8)) {
    let colored_text = text.truecolor(colour.0, colour.1, colour.2);
    println!("{}", colored_text);
}

pub fn print_colored_word(word: &str, colour: (u8, u8, u8)) {
    let colored_word = word.truecolor(colour.0, colour.1, colour.2);
    print!("{}", colored_word);
    io::stdout().flush().unwrap();
}
