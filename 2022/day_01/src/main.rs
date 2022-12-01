use std::fs::read_to_string;

fn main() {
    println!("--- Day 1: Calorie Counting ---");

    let content = read_to_string("data/input").unwrap();
    let calories_list: Vec<i32> = process_input(&content);
    println!("The biggest amount of calories is: {}", calories_list[0]);

    let top_3_total: i32 = calories_list[..3].iter().sum();
    println!("The calories are three top Elves carrying is {}", top_3_total);
}


fn process_input(input: &str) -> Vec<i32> {
    let mut calories = 0;
    let mut calories_list: Vec<i32> = vec![];

    for line in input.lines() {
        if !line.is_empty() {
            calories += line.parse::<i32>().unwrap_or(0);
        } else {
            calories_list.push(calories);
            calories = 0;
        }
    }

    calories_list.sort_by(|a, b| b.cmp(a));

    calories_list
}
