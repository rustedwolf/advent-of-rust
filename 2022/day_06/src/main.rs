use std::fs::read_to_string;


fn main() {
    println!("--- Day 6: Tuning Trouble ---");

    let input = read_to_string("data/input").unwrap();
    let packet_maker_pos = find_marker(&input, 4);
    println!("To find first start-of-packet marker you need to process {} characters", packet_maker_pos);

    let message_maker_pos = find_marker(&input, 14);
    println!("to find first start-of-packet marker you need to process {} characters", message_maker_pos);
}

fn find_marker(input: &str, length: usize) -> usize {
    let mut position: usize = length;

    for i in length..(input.len()) {
        let test_slice = &input[(i - length)..i];
        let mut unique_chars: Vec<char> = test_slice.chars().collect();
        unique_chars.sort();
        unique_chars.dedup();
        if unique_chars.len() == length {
            position = i;
            break;
        }
    }

    position
}


#[test]
fn test_find_marker() {
    assert_eq!(7, find_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
    assert_eq!(5, find_marker(&"bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    assert_eq!(6, find_marker(&"nppdvjthqldpwncqszvftbrmjlhg", 4));
    assert_eq!(10, find_marker(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
    assert_eq!(11, find_marker(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));

    assert_eq!(19, find_marker(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
    assert_eq!(23, find_marker(&"bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
    assert_eq!(23, find_marker(&"nppdvjthqldpwncqszvftbrmjlhg", 14));
    assert_eq!(29, find_marker(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
    assert_eq!(26, find_marker(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
}