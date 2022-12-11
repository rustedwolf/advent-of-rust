use std::fs::read_to_string;


#[derive(Debug)]
#[derive(PartialEq)]
struct Pairs((u32, u32), (u32, u32));

fn main() {
    println!("--- Day 4: Camp Cleanup ---");

    let input = read_to_string("data/input").unwrap();
    let pairs_list: Vec<Pairs> = parse_input(&input);
    let containing = count_containing(&pairs_list);
    println!("Total amount of assignment pairs that does one range fully contain the other is: {}", containing);
    let overlapping = count_overlapping(&pairs_list);
    println!("Total amount of assignment pairs overlap is: {}", overlapping);
}

fn parse_input(input: &str) -> Vec<Pairs> {
    input.lines().map(parse_line).collect()
}

fn parse_line(line: &str) -> Pairs {
    let d: Vec<u32> = line.split(|c| c == '-' || c == ',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect();
    Pairs((d[0],d[1]), (d[2],d[3]))
}

fn count_containing(l: &Vec<Pairs>) -> u32 {
    l.iter().fold(0, |accum, pairs| if is_fully_containing(pairs) { accum + 1 } else { accum })
}

fn is_fully_containing(ps: &Pairs) -> bool {
    (ps.0.0 <= ps.1.0 && ps.1.1 <= ps.0.1)  ||   // 1st overlaps 2nd
    (ps.1.0 <= ps.0.0 && ps.0.1 <= ps.1.1)      // 2nd overlaps 1st
}

fn count_overlapping(l: &Vec<Pairs>) -> u32 {
    l.iter().fold(0, |accum, pairs| if are_overlapping(pairs) { accum + 1 } else { accum })
}

fn are_overlapping(ps: &Pairs) -> bool {
    (ps.0.0 <= ps.1.1 && ps.1.0 <= ps.0.1) ||
    (ps.1.0 <= ps.0.1 && ps.0.0 <= ps.1.1)
}

#[test]
fn test_parse_line() {
    assert_eq!(Pairs((2,4), (6,8)), parse_line(&"2-4,6-8"));
}

#[test]
fn test_is_fully_containing() {
    assert_eq!(false, is_fully_containing(&Pairs((2,4), (6,8))));
    assert_eq!(false, is_fully_containing(&Pairs((2,3), (4,5))));
    assert_eq!(false, is_fully_containing(&Pairs((5,7), (7,9))));
    assert_eq!(true, is_fully_containing(&Pairs((2,8), (3,7))));
    assert_eq!(true, is_fully_containing(&Pairs((6,6), (4,6))));
    assert_eq!(false, is_fully_containing(&Pairs((2,6), (4,8))));
}

#[test]
fn test_are_overlapping() {
    assert_eq!(false, are_overlapping(&Pairs((2,4), (6,8))));
    assert_eq!(false, are_overlapping(&Pairs((2,3), (4,5))));
    assert_eq!(true, are_overlapping(&Pairs((5,7), (7,9))));
    assert_eq!(true, are_overlapping(&Pairs((2,8), (3,7))));
    assert_eq!(true, are_overlapping(&Pairs((6,6), (4,6))));
    assert_eq!(true, are_overlapping(&Pairs((2,6), (4,8))));
}