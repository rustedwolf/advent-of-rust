extern crate crypto;

use crypto::md5::Md5;
use crypto::digest::Digest;

fn get_smallest_number(input: &str, start_text: &str) -> i32 {

    let mut smallest_number = 0i32;
    let mut md = Md5::new();

    'search: loop {
        let input_str = input.to_string() + &smallest_number.to_string();
        md.input_str(&input_str);
        let out_str = md.result_str();

        if out_str.starts_with(&start_text) {
            break 'search;
        } else { 
            md.reset();
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
}
