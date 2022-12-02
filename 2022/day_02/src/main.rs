use std::fs::read_to_string;


fn main() {
    println!("--- Day 2: Rock Paper Scissors ---");

    let strategy = read_to_string("data/input").unwrap();
    let first_instuctions = |m| { match m {
        'X' => (1, 3, 0, 6),
        'Y' => (2, 6, 3, 0),
        'Z' => (3, 0, 6, 3),
        _ => panic!("Unknown move"),
    }};
    let mut total_score = process_strategy(&first_instuctions, &strategy);
    println!("Your total score acroding first strategy will be: {}", total_score);

    let new_instuctions = |m| { match m {
        'X' => (0, 3, 1, 2),
        'Y' => (3, 1, 2, 3),
        'Z' => (6, 2, 3, 1),
        _ => panic!("Unknown move"),
    }};
    total_score = process_strategy(&new_instuctions, &strategy);
    println!("Your total score acroding new strategy will be: {}", total_score);
}

fn process_strategy(f: &dyn Fn(char) -> (i32, i32, i32, i32), strategy: &str) -> i32 {
    strategy.split("\n")
        .fold(0, |accum: i32, line: &str| {
            let m: Vec<char> = line.chars().collect();
            let instr = f(m[2]);

            let score = instr.0 + match m[0] {
                'A' => instr.1,
                'B' => instr.2,
                'C' => instr.3,
                _ => panic!("Unknown move"),
            };

            accum + score
        })
}


#[test]
fn test_process_strategy() {
    let input = String::from("A Y\nB X\nC Z");
    let first_instuctions = |m| { match m {
        'X' => (1, 3, 0, 6),
        'Y' => (2, 6, 3, 0),
        'Z' => (3, 0, 6, 3),
        _ => panic!("Unknown move"),
    }};
    let new_instuctions = |m| { match m {
        'X' => (0, 3, 1, 2),
        'Y' => (3, 1, 2, 3),
        'Z' => (6, 2, 3, 1),
        _ => panic!("Unknown move"),
    }};

    assert_eq!(15, process_strategy(&first_instuctions, &input));
    assert_eq!(12, process_strategy(&new_instuctions, &input));
}
