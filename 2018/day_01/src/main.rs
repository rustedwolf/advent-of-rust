use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() {
    println!("--- Day 1: Chronal Calibration ---");

    let input = get_input().unwrap();
    let mut freaq = 0;


}

fn get_input() -> Result<String, Error> {
    let mut f = File::open("data/input")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

