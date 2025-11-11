mod gematria;
mod file;

use std::io::{self, Write};
use std::env;

fn main() {
    let word = if let Some(s) = env::args().nth(1) {
        s
    } else {
        println!("Usage: gcalc <word>");
        return;
    };

    let g_val = if let Some(val) = gematria::calc(&word) {
        val
    } else {
        println!("Hebrew Characters Only!");
        return;
    };
    
    println!("The Gematria Value Of \"{}\" Is {}.", word, g_val);
    println!("All Words With Gematria Value Of {} =>", g_val);
    file::print_all_words(g_val);

    add_new_value(word.as_str(), g_val);
}

fn add_new_value(word: &str, g_val: u32) {
    print!("Save Word to List? (y/n): ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "y" {
        file::append_to_file(&word, g_val).unwrap();
        println!("Added Successfully.")
    }
}