use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() {
    println!("--- Day 6: Signals and Noise ---");

    let input = get_input().unwrap();
    let charsets = get_charsets(&input);

    println!("The message is: {}", get_message(&charsets));
    println!("But the real message is: {}", get_real_message(&charsets));
}

fn get_input() -> Result<String, Error> {
    let mut f = File::open("data/input")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn get_message(charsets: &Vec<Vec<char>>) -> String {
    let mut message = "".to_string();

    for charset in charsets {
        message.push(get_most_common_letter(&charset));
    }

    message
}

fn get_real_message(charsets: &Vec<Vec<char>>) -> String {
    let mut message = "".to_string();

    for charset in charsets {
        message.push(get_less_common_letter(&charset));
    }

    message
}

fn get_charsets(input: &str) -> Vec<Vec<char>> {
    let mut charsets: Vec<Vec<char>> = vec![Vec::new(); 8];

    for line in input.lines() {
        for (index, character) in line.chars().enumerate() {
            charsets[index].push(character);
        }
    }

    charsets
}

fn get_most_common_letter(charset: &Vec<char>) -> char {
    let char_counts = get_character_counts(charset);

    char_counts[0].0
}

fn get_less_common_letter(charset: &Vec<char>) -> char {
    let char_counts = get_character_counts(charset);

    char_counts[char_counts.len() - 1].0
}

fn get_character_counts(charset: &Vec<char>) -> Vec<(char, u32)> {
    let mut char_counts: Vec<(char, u32)> = Vec::new();
    let mut chars: Vec<char> = charset.clone();
    chars.sort();

    let mut current_char = chars[0];
    let mut char_count = 0;

    for c in chars {
        if current_char == c {
            char_count += 1;
        } else {
            char_counts.push((current_char, char_count));
            current_char = c;
            char_count = 1;
        }
    }
    char_counts.push((current_char, char_count));
    char_counts.sort_by(|&ca, &cb| if cb.1 != ca.1 { cb.1.cmp(&ca.1) } else { ca.0.cmp(&cb.0) });

    char_counts
}