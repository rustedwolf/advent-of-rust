extern crate crypto;
extern crate ansi_escapes;

use crypto::md5::Md5;
use crypto::digest::Digest;

use std::u64;

fn main() {
    println!("--- Day 5: How About a Nice Game of Chess? ---");

    let input = "ojvtpuvg";

    println!("The password is {:?}", decrypt_password(&input));
    awesome_password_decryption(&input);
}

fn decrypt_password(input: &str) -> String {
    let mut password = "".to_string();
    let mut md = Md5::new();

    for i in 0..u64::MAX {
        let input_str = input.to_string() + &i.to_string();
        md.input_str(&input_str);
        let out_str = md.result_str();

        if out_str.starts_with(&"00000") {
            password.push(out_str.chars().nth(5).unwrap());
            if password.len() == 8 {
                break;
            }
        }

        md.reset();
    }

    password
}

fn awesome_password_decryption(input: &str) {
    let mut password: Vec<char> = vec!['_'; 8];
    let mut md = Md5::new();
    let mut char_count = 0;

    println!("Decrypting the next password:");
    print!("{}", ansi_escapes::CursorHide);
    println!("Password: ________");

    for i in 0..u64::MAX {
        let input_str = input.to_string() + &i.to_string();
        md.input_str(&input_str);
        let out_str = md.result_str();

        if out_str.starts_with(&"00000") {
            let mut hash_chars = out_str.chars();
            let character = hash_chars.nth(5).unwrap();

            if character.is_digit(10) {
                let position = character.to_digit(10).unwrap();

                if position < 8 {
                    if password[position as usize] == '_' {
                        password[position as usize] = out_str.chars().nth(6).unwrap();
                        if char_count == 7 {
                            show_password(&password, &out_str);
                            break;
                        } else { char_count += 1; }
                    }
                }
            }
        }

        show_password(&password, &out_str);

        md.reset();
    }

    print!("{}", ansi_escapes::CursorShow);
}

fn show_password(password: &Vec<char>, out_str: &str) {
    let mut password_string = "".to_string();
    for (index, character) in password.iter().enumerate() {
        password_string.push( if character == &'_' {
            out_str.chars().nth(index).unwrap()
        } else {
            character.clone()
        });
    }

    print!("{}", ansi_escapes::EraseLines(2));
    println!("{}", password_string);
}