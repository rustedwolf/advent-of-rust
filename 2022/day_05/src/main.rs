use std::fs::read_to_string;

type CrateStack = Vec<char>;

#[derive(Debug)]
#[derive(PartialEq)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    println!("--- Day 5: Supply Stacks ---");

    let input = read_to_string("data/input").unwrap();
    let (crate_stacks, instructions) = parse_input(&input);
    let first_stack = rearrange_crates(&crate_stacks, &instructions);
    let mut crates = top_crates(&first_stack);
    println!("Old arrangement top crates are: {}", crates);

    let second_stack = rearrange_crates_2(&crate_stacks, &instructions);
    crates = top_crates(&second_stack);
    println!("New arrangement top crates are: {}", crates);
}

fn parse_input(input: &str) -> (Vec<CrateStack>, Vec<Instruction>) {
    let lines = input.lines().collect();
    let split_pos: usize = find_end_empty_line_position(&lines);
    let crate_stacks: Vec<CrateStack> = generate_crate_stack(&lines[..split_pos]);
    let instructions: Vec<Instruction> = generate_instructions(&lines[(split_pos + 1)..]);

    (crate_stacks, instructions)
}

fn find_end_empty_line_position(lines: &Vec<&str>) -> usize {
    let mut counter: usize = 0;
    for line in lines {
        if line.is_empty() {
            break;
        } else {
            counter += 1;
        }
    }

    counter
}

fn generate_crate_stack(data: &[&str]) -> Vec<CrateStack> {
    let mut lines: Vec<Vec<char>> = data.iter()
        .map(|l| l.chars().collect())
        .collect();
    let last_row: Vec<char> = lines.pop().unwrap();
    let list_size = lines.len();
    // stacks are not zero-indexes, so in order to avoid cosntant additions, just add empty stack at zero position
    let mut stacks: Vec<CrateStack> = vec![CrateStack::new()];

    for (index, character) in last_row.iter().enumerate() {
        if *character != ' ' {
            let mut new_stack: CrateStack = Vec::new();
            for line in (0..list_size).rev() {
                let c = lines[line][index];
                if c != ' ' {
                    new_stack.push(c);
                } else {
                    break;
                }
            }
            stacks.push(new_stack);
        }
    }

    stacks
}

fn generate_instructions(lines: &[&str]) -> Vec<Instruction> {
    lines.iter().map(|l| get_insctuction(l)).collect()
}

fn get_insctuction(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split_whitespace().collect();
    Instruction {
        amount: parts[1].parse::<usize>().unwrap(),
        from: parts[3].parse::<usize>().unwrap(),
        to: parts[5].parse::<usize>().unwrap()
    }
}

fn rearrange_crates(stack: &Vec<CrateStack>, instructions: &Vec<Instruction>) -> Vec<CrateStack> {
    let mut new_stack = stack.clone();
    for instruction in instructions {
        for _ in 0..instruction.amount {
            let sypply_crate = new_stack[instruction.from].pop().unwrap();
            new_stack[instruction.to].push(sypply_crate);
        }
    }

    new_stack
}

fn rearrange_crates_2(stack: &Vec<CrateStack>, instructions: &Vec<Instruction>) -> Vec<CrateStack> {
    let mut new_stack = stack.clone();
    for instruction in instructions {
        let from = new_stack[instruction.from].len() - instruction.amount;
        let mut moved: Vec<char> = new_stack[instruction.from].drain(from..).collect();
        new_stack[instruction.to].append(&mut moved);
    }

    new_stack
}

fn top_crates(crate_stacks: &Vec<CrateStack>) -> String {
    let mut top_crates: Vec<char> = Vec::new();

    for stack in crate_stacks {
        if ! stack.is_empty(){
            top_crates.push(*stack.last().unwrap());
        }
    }

    top_crates.iter().cloned().collect::<String>()
}


#[test]
fn test_find_end_empty_line_position() {
    let lines = "first line\nsecond\nthird\n\nfitht\n".lines().collect();

    assert_eq!(3, find_end_empty_line_position(&lines));
}

#[test]
fn test_get_insctuction() {
    assert_eq!(Instruction{amount: 1, from: 2, to:1}, get_insctuction(&"move 1 from 2 to 1"));
    assert_eq!(Instruction{amount: 3, from: 1, to:3}, get_insctuction(&"move 3 from 1 to 3"));
    assert_eq!(Instruction{amount: 2, from: 2, to:1}, get_insctuction(&"move 2 from 2 to 1"));
    assert_eq!(Instruction{amount: 1, from: 1, to:2}, get_insctuction(&"move 1 from 1 to 2"));
}

#[test]
fn test_top_crates() {
    let crate_stacks: Vec<CrateStack> =  vec!(vec!['C'], vec!['M'], vec!['P', 'D', 'N', 'Z']);

    assert_eq!("CMZ", top_crates(&crate_stacks));
}