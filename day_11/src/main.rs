extern crate regex;

use regex::Regex;

fn main() {
    println!("--- Day 11: Corporate Policy ---");

    let mut password = "cqjxjnds".to_string();

    'running: loop {
        password = increment_password(&password);
        if is_valid(&password) {
            break 'running;
        }
    }

    println!("The closest valid password is {:?}", password);
}

fn increment_password(input: &str) -> String {
    let mut input_chars: Vec<char> = input.chars().collect();
    let mut index = input_chars.len() - 1;

    'running: loop {
        let (new_char, wrapped) = increment_char(&input_chars[index]);
        input_chars[index] = new_char;
        if wrapped && index != 0 {
            index -= 1;
        } else {
            break 'running;
        }
    }

    input_chars.into_iter().collect()
}

fn increment_char(input: &char) -> (char, bool) {
    let char_list: Vec<char> = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 
        'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 
        'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let new_char_index = (char_list.binary_search(input).unwrap() + 1) % char_list.len();
    let new_char = char_list[new_char_index];
    (new_char, new_char_index == 0)
}

fn is_valid(input: &str) -> bool {
    has_increasing_straigs_of_three(input) &&
    has_different_double_letters(input) &&
    !has_forbidden_chars(input)
}

fn has_forbidden_chars(input: &str) -> bool {
    let bad_chars = Regex::new(r"i|l|o").unwrap();
    bad_chars.is_match(input)
}

fn has_increasing_straigs_of_three(inpur: &str) -> bool {
    unimplemented!();
}

fn has_different_double_letters(input: &str) -> bool {
    unimplemented!();
}
