extern crate unescape;

use std::path::Path;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use unescape::unescape;

fn count_escaped(input: &str) -> usize {
    let mut char_count = 2;
    for symbol in input.chars() {
        char_count += symbol.escape_default().count();
    }
    char_count
}

fn main() {
    println!("--- Day 8: Matchsticks ---"); 

    let path = Path::new("input/day_8.txt");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open input file: {}", reason)
    };

    let reader = BufReader::new(&input);

    let mut unescaped_char_sum = 0;
    let mut escaped_char_sum = 0;

    for line in reader.lines() {
        let line_text = line.unwrap();
        let chars: Vec<char> = line_text.chars().collect();

        let enescaped_chars: Vec<char> = unescape(&line_text).unwrap().chars().collect();

        unescaped_char_sum += chars.len() - (enescaped_chars.len() - 2);
        escaped_char_sum += count_escaped(&line_text) - chars.len();
    }

    println!("The number of characters of code for string literals 
minus the number of characters in memory for the values of 
the strings is: {} \n", unescaped_char_sum);
    println!("The total number of characters to represent the newly 
encoded strings minus the number of characters of code 
in each original string literal is: {}", escaped_char_sum);
}
