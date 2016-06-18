extern crate serde_json;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use serde_json::Value;

fn get_sum(value: &Value, count_red: bool) -> i64 {
    let mut total_sum = 0;

    if value.is_number() {
        total_sum += value.as_i64().unwrap();
    }

    if value.is_array() {
        match value.as_array() {
            Some(values) => {
                for val in values {
                    total_sum += get_sum(val, count_red);
                }
            },
            None => {},
        }        
    }

    if value.is_object() {
        if count_red || !has_reds(&value) { 
            for (key, val) in value.as_object().unwrap().iter() {
                total_sum += get_sum(val, count_red);
            }
        }
    }

    total_sum
}

fn has_reds(value: &Value) -> bool {
    let mut haystack = value.as_object().unwrap().iter();

    haystack.any(|(s, val)| if val.is_string() {
        val.as_string().unwrap() == "red"
    } else { false })
}

fn main() {
    println!("--- Day 12: JSAbacusFramework.io ---");

    let path = Path::new("input/day_12");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reson) => panic!("Could not open input file: {}", reson)
    };

    let mut input_string = "".to_string();
    let mut reader = BufReader::new(&input);
    reader.read_line(&mut input_string);

    let data: Value = serde_json::from_str(&input_string).unwrap();

    println!("Total sum of json values is {}", get_sum(&data, true));
    println!("Total sum of json values without red objects is {}", get_sum(&data, false));
}
