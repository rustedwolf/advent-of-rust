fn main() {
    println!("--- Day 10: Elves Look, Elves Say ---");
    
    let mut sequence = "1113222113".to_string();

    for _ in 0..40 {
        sequence = look_and_say(&sequence);
    }
    println!("The first length on the sequence is {}", sequence.len());

    for _ in 40..50 {
        sequence = look_and_say(&sequence);
    }
    println!("The nex length on the sequence is {}", sequence.len());
}

fn look_and_say(sequence: &str) -> String {
    let mut new_sequence = "".to_string();
    let mut current_number: char = '_';
    let mut number_count = 0;

    for number in sequence.chars() {
        if current_number != number {
            if number_count > 0 {
                new_sequence.push_str(&number_count.to_string());
                new_sequence.push(current_number);
            }
            current_number = number;
            number_count = 1;
        } else {
            number_count += 1;
        }
    }
    new_sequence.push_str(&number_count.to_string());
    new_sequence.push(current_number);

    new_sequence
}