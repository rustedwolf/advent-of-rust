extern crate md5;

use md5::{Md5, Digest};

fn get_smallest_number(input: &str, start_text: &str) -> i32 {
    let mut hasher = Md5::new();
    let mut smallest_number: i32 = 0;

    'search: loop {
        hasher.update(input.to_string() + &smallest_number.to_string());
        let string_digest = format!("{:x}", hasher.finalize_reset());

        if string_digest.starts_with(start_text) {
            break 'search;
        } else {
            smallest_number += 1;
        }
    }

    smallest_number
}

fn main() {
    println!("--- Day 4: The Ideal Stocking Stuffer ---");

    let input = "iwrupvqb";

    let mut smallest_number = get_smallest_number(&input, "00000");
    println!("The smallest number for 5 leading zero hash is: {:?}", smallest_number);

    smallest_number = get_smallest_number(&input, "000000");
    println!("The smallest number for 6 leading zero hash is: {:?}", smallest_number);
    // Yay! we sort of made sort of a miner
}
