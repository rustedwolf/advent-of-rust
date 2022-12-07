use std::fs::read_to_string;


fn main() {
    println!("--- Day 3: Rucksack Reorganization ---");

    let input = read_to_string("data/input").unwrap();
    let items: Vec<char> = get_reappearing_items(&input);
    let mut sum = sum_priorites(&items);
    println!("The sum of the priorities is {}", sum);
    let group_items = get_reappearing_group_items(&input);
    sum = sum_priorites(&group_items);
    println!("The sum of the group priorities is {}", sum);

}

fn get_reappearing_items(input: &str) -> Vec<char> {
    input.lines()
        .map(|line| get_reappearing_item(&line))
        .collect()
}

fn get_reappearing_item(s: &str) -> char {
    let items: Vec<char> = s.chars().collect();
    let compartments = items.split_at(s.len() / 2);

    let mut found: char = '_';

    for item in compartments.0 {
        if compartments.1.contains(item) {
            found = *item;
            break;
        }
    }

    found
}

fn get_reappearing_group_items(input: &str) -> Vec<char> {
    input.lines()
        .map(|line| {
            let mut chars: Vec<char> = line.chars().collect();
            chars.dedup();
            chars
        })
        .collect::<Vec<Vec<char>>>()
        .chunks(3)
        .map(|group| get_reappearing_group_item(group))
        .collect()
}

fn get_reappearing_group_item(group: &[Vec<char>]) -> char {
    let items = &group[0];
    let mut found: char = '_';
    for item in items {
        if all_contain(&group[1..], &item) {
            found = *item;
            break;
        }
    }

    found
}

fn all_contain(group: &[Vec<char>], target: &char) -> bool {
    let mut contains = true;
    for compartment in group {
        if !compartment.contains(target) {
            contains = false;
            break;
        }
    }

    contains
}


fn to_priority(c: char) -> u32 {
    let number: u32 = c.into();

    if c.is_ascii_lowercase() {
        //a-z is 97 - 122
        number - 96
    } else if c.is_ascii_uppercase() {
        //A-Z is 65 - 90
        number - 38 // -64 + 26
    } else {
        0
    }
}

fn sum_priorites(items: &Vec<char>) -> u32 {
    items.iter().fold(0, |accum, item| accum + to_priority(*item))
}


#[test]
fn test_get_reappering_item() {
    let test_str_0 = String::from("vJrwpWtwJgWrhcsFMMfFFhFp");
    let test_str_1 = String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
    let test_str_2 = String::from("PmmdzqPrVvPwwTWBwg");
    let test_str_3 = String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
    let test_str_4 = String::from("ttgJtRGJQctTZtZT");
    let test_str_5 = String::from("CrZsJsPPZsGzwwsLwLmpwMDw");

    assert_eq!(get_reappearing_item(&test_str_0), 'p');
    assert_eq!(get_reappearing_item(&test_str_1), 'L');
    assert_eq!(get_reappearing_item(&test_str_2), 'P');
    assert_eq!(get_reappearing_item(&test_str_3), 'v');
    assert_eq!(get_reappearing_item(&test_str_4), 't');
    assert_eq!(get_reappearing_item(&test_str_5), 's');
}

#[test]
fn test_get_reappearing_group_item() {
    let group_1 = vec!(
        String::from("vJrwpWtwJgWrhcsFMMfFFhFp").chars().collect(),
        String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").chars().collect(),
        String::from("PmmdzqPrVvPwwTWBwg").chars().collect()
    );

    assert_eq!(get_reappearing_group_item(&group_1), 'r');
}

#[test]
fn test_to_priority() {
    assert_eq!(1, to_priority('a'));
    assert_eq!(26, to_priority('z'));
    assert_eq!(27, to_priority('A'));
    assert_eq!(52, to_priority('Z'));
}