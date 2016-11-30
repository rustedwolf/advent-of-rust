use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

type CompoundList = Vec<(String, u32)>;

struct Aunt {
    number: String,
    compounds: CompoundList
}
type AuntList = Vec<Aunt>;

fn main() {
    println!("--- Day 16: Aunt Sue ---");
    
    let aunt_list = get_aunt_list();
    let searched_compounds: CompoundList = vec![
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1)
    ];

    for aunt in &aunt_list {
        if check_aunt_for_compounds(&aunt, &searched_compounds) {
            println!("The number of searched aunt is {}", aunt.number);
            break;
        }
    }
    
    for aunt in &aunt_list {
        if check_aunt_for_compounds_with_ranges(&aunt, &searched_compounds) {
            println!("But the number of searched aunt is {}", aunt.number);
            break;
        }
    }
}

fn get_aunt_list() -> AuntList{
    let path = Path::new("input/day_16");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Coul not open input file: {}", reason),
    };

    let reader = BufReader::new(&input);
    let mut aunt_list: AuntList = Vec::new();

    for line in reader.lines() {
        let info_string = line.unwrap();

        let aunt_info: Vec<&str> = info_string.splitn(2, ": ").collect();
        let compound_infos: Vec<&str> = aunt_info[1].split(", ").collect();
        let mut compound_list: CompoundList = Vec::new();

        for info in compound_infos {
            let compound_info: Vec<&str> = info.split(": ").collect();
            let aunt_compound = (compound_info[0].to_string(), compound_info[1].parse::<u32>().unwrap());
            compound_list.push(aunt_compound);
        }

        let aunt = Aunt {
            number: aunt_info[0].to_string(),
            compounds: compound_list
        };
        aunt_list.push(aunt);
    }

    aunt_list
}

fn check_aunt_for_compounds(aunt: &Aunt, searched_compounds: &CompoundList) -> bool {
    let mut found = true;

    for compound in &aunt.compounds {
        found = searched_compounds.contains(&compound);
        if !found { break; }
    }

    found
}

fn check_aunt_for_compounds_with_ranges(aunt: &Aunt, searched_compounds: &CompoundList) -> bool {
    let mut found = true;

    for compound in &aunt.compounds {
        found = match compound.0.as_str() {
            "cats" => compound.1 > searched_compounds[1].1,
            "trees" => compound.1 > searched_compounds[7].1,
            "pomeranians" => compound.1 < searched_compounds[3].1,
            "goldfish" => compound.1 < searched_compounds[6].1,
            _ => searched_compounds.contains(&compound),
        };
        if !found { break; }
    }

    found
}

#[test]
fn test_contains() {
    let searched_compounds: CompoundList = vec![
        ("children".to_string(), 3),
        ("cats".to_string(), 7),
        ("samoyeds".to_string(), 2),
        ("pomeranians".to_string(), 3),
        ("akitas".to_string(), 0),
        ("vizslas".to_string(), 0),
        ("goldfish".to_string(), 5),
        ("trees".to_string(), 3),
        ("cars".to_string(), 2),
        ("perfumes".to_string(), 1)
    ];
    let compound = ("samoyeds".to_string(), 2);
    let compound2 = ("goldfish".to_string(), 4);

    assert_eq!(true, searched_compounds.contains(&compound));
    assert_eq!(false, searched_compounds.contains(&compound2));
}