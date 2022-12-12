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
    let str_0 = &"mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let str_1 = &"bvwbjplbgvbhsrlpgdmjqwftvncz";
    let str_2 = &"nppdvjthqldpwncqszvftbrmjlhg";
    let str_3 = &"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    let str_4 = &"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    assert_eq!(7, find_marker(str_0, 4));
    assert_eq!(5, find_marker(str_1, 4));
    assert_eq!(6, find_marker(str_2, 4));
    assert_eq!(10, find_marker(str_3, 4));
    assert_eq!(11, find_marker(str_4, 4));

    assert_eq!(19, find_marker(str_0, 14));
    assert_eq!(23, find_marker(str_1, 14));
    assert_eq!(23, find_marker(str_2, 14));
    assert_eq!(29, find_marker(str_3, 14));
    assert_eq!(26, find_marker(str_4, 14));
}