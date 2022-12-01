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
    let mut calories_list: Vec<i32> = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|cal| cal.parse::<i32>().unwrap_or(0))
                .sum()
        }).collect();

    calories_list.sort_by(|a, b| b.cmp(a));

    calories_list
}
