use std::cmp;

fn main() {
    println!("--- Day 20: Infinite Elves and Infinite Houses ---");

    let target_present_count = 34000000;

    let first_house_number = get_first_house_number_with_presents(target_present_count, 10, target_present_count);
    let second_house_number = get_first_house_number_with_presents(target_present_count, 11, 50);

    println!("First house number to receive at least {} presents is {}", target_present_count, first_house_number);
    println!("Next time it will be {}", second_house_number);
}

fn get_first_house_number_with_presents(target_present_count: usize, presents: usize, visit_limit: usize) -> usize {
    let limit = target_present_count / 2;
    let mut house_number = limit;
    let mut houses = vec![1; limit + 1];

    'search: for elf in 2..limit + 1 {
        let mut house = elf;
        let mut visits = 0;

        'visits: while house <= limit && visits < visit_limit {
            houses[house] += elf * presents;
            if houses[house] >= target_present_count {
                house_number = cmp::min(house, house_number);
            }

            house += elf;
            visits += 1;
        }
    }

    house_number
}
