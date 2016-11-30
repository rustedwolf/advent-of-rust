#[macro_use] extern crate lazy_static;
extern crate regex;

use regex::Regex;

fn main() {
    println!("--- Day 11: Corporate Policy ---");

    let mut password = "cqjxjnds".to_string();
    password = get_next_password(&password);    
    println!("The closest valid password is {:?}", password);
    password = get_next_password(&password);    
    println!("Next password is {:?}", password);
}

fn get_next_password(input: &str) -> String {
    let mut password = input.to_string();
    'running: loop {
        password = increment_password(&password);
        if is_valid(&password) {
            break 'running;
        }
    }
    password
}

fn increment_password(input: &str) -> String {
    let mut input_chars: Vec<char> = input.chars().collect();
    let mut index = input_chars.len() - 1;

    'running: loop {
        let (new_char, wrapped) = increment_char(&input_chars[index]);
        input_chars[index] = new_char;
        if !wrapped {
            break 'running;
        } else {
            index = if index != 0 { index - 1 } else { input_chars.len() - 1 };
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

fn has_increasing_straigs_of_three(input: &str) -> bool {
    let abc = "abcdefghijklmnopqrstuvwxyz";
    for x in 0..abc.len() - 2 {
        if input.contains(&abc[x..x+3]) {
            return true;
        }
    } 
    false
}

fn has_forbidden_chars(input: &str) -> bool {
    lazy_static! {
        static ref BAD_CHARS: Regex = Regex::new(r"[ilo]").unwrap();
    }
    BAD_CHARS.is_match(input)
}

fn has_different_double_letters(input: &str) -> bool {
    let charsset: Vec<char> = input.chars().collect();
    let mut matches: Vec<char> = vec![];
    for x in 0..charsset.len() - 1 {
        if charsset[x] == charsset[x+1] {
            matches.push(charsset[x]);
        }
    }
    matches.dedup();
    matches.len() > 1
}

#[test]
fn test_is_valid() {
    assert_eq!(true, is_valid("abcdffaa"));
    assert_eq!(true, is_valid("ghjaabcc"));
}

#[test]
fn test_for_increasing_straigs_of_three() {
    assert_eq!(true, has_increasing_straigs_of_three("hijklmmn"));
    assert_eq!(false, has_increasing_straigs_of_three("abbceffg"));
    assert_eq!(false, has_increasing_straigs_of_three("abbcegjk"));
    assert_eq!(true, has_increasing_straigs_of_three("abcdffaa"));
    assert_eq!(true, has_increasing_straigs_of_three("ghjaabcc"));
}

#[test]
fn test_for_forbidden_chars() {
    assert_eq!(true, has_forbidden_chars("hijklmmn"));
    assert_eq!(false, has_forbidden_chars("abbceffg"));
    assert_eq!(false, has_forbidden_chars("abbcegjk"));
}

#[test]
fn test_diffrent_double_letters() {
    assert_eq!(false, has_different_double_letters("hijklmmn"));
    assert_eq!(true, has_different_double_letters("abbceffg"));
    assert_eq!(false, has_different_double_letters("abbcegjk"));
    assert_eq!(true, has_different_double_letters("abcdffaa"));
    assert_eq!(true, has_different_double_letters("ghjaabcc"));
}