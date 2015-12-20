extern crate regex;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

fn main () {
    
    println!("--- Day 5: Doesn't He Have Intern-Elves For This? ---");

    let mut first_nice = 0;
    let mut second_nice = 0;

    let vowels = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    let bad_strings = Regex::new(r"ab|cd|pq|xy").unwrap();

    let path = Path::new("input/day5.txt");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open the input file: {}", reason),
    };

    let child_list = BufReader::new(&input);

    for line in child_list.lines() {
        let child = line.unwrap();

        let vowel_count = vowels.is_match(&child);
        let has_doubles = child.as_bytes().windows(2).any(|w| w[0] == w[1]);
        let has_bad_strings = bad_strings.is_match(&child);

        if vowel_count && has_doubles && !has_bad_strings {
            first_nice += 1;
        };

        let first_rule = child.as_bytes().windows(2).any(|w| {
            let bv = w.to_vec();
            let s = String::from_utf8(bv).unwrap();
            let r = s.clone() + &".*" + &s;
            Regex::new(&r).unwrap().is_match(&child)
        });
        let second_rule = child.as_bytes().windows(3).any(|w| w[0] == w[2]);

        if first_rule && second_rule {
            second_nice += 1;
        }
    }

    println!("The are {} nice children according to the first method", first_nice);
    println!("The are {} nice children according to the second method", second_nice);
}

#[test]
fn test_vovels() {
    let vowels = Regex::new(r"[aeiou].*[aeiou].*[aeiou]").unwrap();
    assert!(vowels.is_match("ugknbfddgicrmopn"));
}

#[test]
fn test_doubles() {
    let text = "ugknbfddgicrmopn";
    assert!(text.as_bytes().windows(2).any(|w| w[0] == w[1]));
}

#[test]
fn test_bad_strings() {
    let bad_strings = Regex::new(r"ab|cd|pq|xy").unwrap();
    assert!(!bad_strings.is_match("ugknbfddgicrmopn"));
    assert!(bad_strings.is_match("woofab"));
}

#[test]
fn test_first_rule() {
    let text = "ugknbfddgicrmddopn";
    let first_rule = text.as_bytes().windows(2).any(|w| {
        let bv = w.to_vec();
        let s = String::from_utf8(bv).unwrap();
        let r = s.clone() + &".*" + &s;
        Regex::new(&r).unwrap().is_match(&text)
    });
    assert!(first_rule);
}