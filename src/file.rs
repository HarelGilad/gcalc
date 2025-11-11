use std::fs::{self, File};
use std::io::{Result, Write};
use std::env;

const PATH: &str = ".local/share/gcalc/gfile";

pub fn append_to_file(word: &str, g_val: u32) -> Result<()> {
    let filepath = get_full_filepath();
    let mut lines: Vec<String> = extract_lines_from_file(filepath.as_str());
    let mut found = false;

    for line in &mut lines {
         if let Some((num_part, words_part)) = line.split_once(' ') {
            if num_part == g_val.to_string() {
                let mut words: Vec<String> = words_part.split(',').map(|s| s.trim().to_string()).collect();

                if !words.contains(&word.to_string()) {
                    words.push(word.to_string());
                }

                *line = format!("{} {}", g_val, words.join(","));
                found = true;
                break;
            }
         }
    }

    if !found {
        lines.push(format!("{} {}", g_val, word));
    }

    write_lines_to_file(filepath.as_str(), &lines)
}

pub fn print_all_words(g_val: u32) {
    let filepath = get_full_filepath();
    let lines: Vec<String> = extract_lines_from_file(filepath.as_str());
    let mut words: Vec<String> = Vec::new();

    for line in &lines {
        if let Some((num_part, words_part)) = line.split_once(' ') {
            if num_part == g_val.to_string() {
                words = words_part.split(',').map(|s| s.trim().to_string()).collect();
                break;
            }
        }
    }

    if !words.is_empty() {
        print_words_in_format(&words);
    } else {
        print!("\x1B[1A\x1B[2K"); 
    }
}

fn get_full_filepath() -> String {
    let filepath = env::home_dir().unwrap().join(PATH);
    filepath.to_str().unwrap().to_string()
}

fn extract_lines_from_file(filepath: &str) -> Vec<String> {
    match fs::read_to_string(filepath) {
        Ok(content) => { content.lines().map(|s| s.to_string()).collect() }
        Err(_) => { Vec::new() }
    }
}

fn write_lines_to_file(filepath: &str, lines: &Vec<String>) -> Result<()> {
    let mut file = File::create(filepath)?;
    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}

fn print_words_in_format(words: &Vec<String>) {
    let mut iter = words.chunks(5);

    while let Some(chunk) = iter.next() {
        println!("{}", chunk.join("     "));
    }
}