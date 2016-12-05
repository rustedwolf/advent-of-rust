extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::Error;
use regex::Regex;

#[derive(Debug)]
struct Room<'a> {
    name: &'a str,
    id: u32,
    checksum: &'a str,
}

fn main() {
    println!("--- Day 4: Security Through Obscurity ---");

    let input = get_input().unwrap();
    let rooms = get_rooms(&input);

    println!("The sum of real rooms id's is {}", get_real_room_id_sum(&rooms));

    for room in rooms {
        println!("{} in room {}", decode_name(&room), room.id);
    }
}

fn get_input() -> Result<String, Error> {
    let mut f = File::open("data/input")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn get_rooms(input: &String) -> Vec<Room> {
    let re = Regex::new(r"([a-z-]*)-(\d*)\[(.*)\]").unwrap();
    let mut rooms: Vec<Room> = Vec::new();

    for room_code in input.lines() {
        let caps = re.captures(&room_code).unwrap();
        let room = Room {
            name: caps.at(1).unwrap(),
            id: caps.at(2).unwrap().parse().unwrap(),
            checksum: caps.at(3).unwrap()
        };
        if is_valid_room(&room) {
            rooms.push(room);
        }
    }

    rooms
}

fn get_real_room_id_sum(rooms: &Vec<Room>) -> u32 {
    let mut sum = 0;

    for room in rooms {
        sum += room.id;
    }

    sum
}

fn is_valid_room(room: &Room) -> bool {
    room.checksum == generate_checksum(room.name)
}

fn generate_checksum(name: &str) -> String {
    let chars = get_chars(name);
    let mut checksum = "".to_string();
    let mut char_counts: Vec<(char, u32)> = Vec::new();
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

    for i in 0..5 {
        checksum.push(char_counts[i].0);
    }

    checksum
}

fn get_chars(name: &str) -> Vec<char> {
    let mut chars: Vec<char> = name.chars()
                                    .filter(|&c| c != '-')
                                    .collect();
    chars.sort();

    chars
}

fn decode_name(room: &Room) -> String {
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    let mut name = "".to_string();
    let letter_len = letters.len() as u32;
    let shift_count = room.id % letter_len;

    for c in room.name.chars() {
        match c {
            '-' => name.push(' '),
            'a' ... 'z' => {
                let char_pos1 = letters.binary_search(&c).unwrap() as u32;
                let real_pos = (char_pos1 + shift_count) % letter_len;
                name.push(letters[real_pos as usize]);
            },
            _ => {},
        }
    }

    name
}

#[test]
fn test_get_chars() {
    assert_eq!(vec!['a','a','a','a','a','b','b','b','x','y','z'], get_chars("aaaaa-bbb-z-y-x"));
}

#[test]
fn test_checksum() {
    assert_eq!("abxyz", generate_checksum("aaaaa-bbb-z-y-x"));
    assert_eq!("abcde", generate_checksum("a-b-c-d-e-f-g-h"));
    assert_eq!("oarel", generate_checksum("not-a-real-room"));
}

#[test]
fn test_decode_name() {
    let room = Room {
        name: "qzmt-zixmtkozy-ivhz",
        id: 343,
        checksum: "zimth"
    };

    assert_eq!("very encrypted name", decode_name(&room));
}