extern crate regex;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

type Molecule = String;
type Molecules = Vec<Molecule>;

struct Replacement {
    molecule: Molecule,
    replacement: Molecule,
}
type Replacements = Vec<Replacement>;

fn main() {
    println!("--- Day 19: Medicine for Rudolph ---");

    let (replacements, molecule) = get_data();

    let molecule_list: Molecules = get_distinct_molecule_list(&replacements, &molecule);
    println!("There are {} of distinct molecules can be created ", molecule_list.len());

    let real_molecule = real_formula(&molecule);
    println!("To fabricate this molecule you need {} steps", count_stepts(&real_molecule));
}

fn get_data() -> (Replacements, Molecule) {
    let mut replacements: Replacements = Vec::new();
    let mut molecule = "".to_string();

    let input = get_input();
    let reader = BufReader::new(&input);

    for line in reader.lines() {
        let line_info = line.unwrap();

        if line_info.contains(" => ") {
            replacements.push(parse_replacement(&line_info));
        } else if line_info.len() > 0 {
            molecule = line_info;
        }
    }

    (replacements, molecule)
}

fn get_input() -> File {
    let path = Path::new("data/input");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open input file: {}", reason),
    };

    input
}

fn parse_replacement(info: &str) -> Replacement {
    let replacement_info: Vec<&str> = info.split(" => ").collect();
    let replacement = Replacement {
        molecule: replacement_info[0].to_string(),
        replacement: replacement_info[1].to_string()
    };

    replacement
}

fn get_distinct_molecule_list(replacements: &Replacements, molecule: &str) -> Molecules {
    let mut molecule_list: Molecules = generate_molecule_list(replacements, molecule);
    molecule_list.sort();
    molecule_list.dedup();

    molecule_list
}

fn generate_molecule_list(replacements: &Replacements, molecule: &str) -> Molecules {
    let mut molecule_list: Molecules = Vec::new();

    for replacement in replacements {
        let mut replacement_molecules = generate_replacement_molecules(&replacement, molecule);
        molecule_list.append(&mut replacement_molecules);
    }

    molecule_list
}

fn generate_replacement_molecules(replacement: &Replacement, molecule: &str) -> Molecules {
    let mut molecule_list: Molecules = Vec::new();
    let target_molecules: Vec<_> = molecule.match_indices(&replacement.molecule).collect();

    for target_molecule in target_molecules {
        let new_molecule = replace_molecule(replacement, &target_molecule, molecule);
        molecule_list.push(new_molecule);
    }

    molecule_list
}

fn replace_molecule(replacement: &Replacement, target_molecule: &(usize, &str), molecule: &str) -> Molecule {
    let mut molecule_clone = molecule.to_string();
    let (first, second) = molecule_clone.split_at_mut(target_molecule.0);
    let (_, rest) = second.split_at(target_molecule.1.len());
    let new_molecule = first.to_string() + &replacement.replacement + &rest;

    new_molecule
}

// Now it appears, that Eric Walt's wanted to be a dick by misleading us with a "story".
// So now we have to pay attention is for the goal, not the fluff.

fn count_stepts(molecule: &str) -> usize {
    let re_a = Regex::new(r"[A-Z]").unwrap();
    let re_c = Regex::new(r",").unwrap();

    // I disagree with this "- 1", cuz I just don't understand since we're starting from "e"
    re_a.captures_iter(molecule).count() - re_c.captures_iter(molecule).count() - 1
}

fn real_formula(fake_formula: &str) -> String {
    let mut real_formula = fake_formula.to_string();

    real_formula = real_formula.replace("Rn", "(");
    real_formula = real_formula.replace("Ar", ")");
    real_formula = real_formula.replace("Y", ",");

    real_formula
}

#[test]
fn test_replacement() {
    let molecule = "HOH".to_string();
    let replacements = vec![
        Replacement { molecule: "H".to_string(), replacement: "HO".to_string() },
        Replacement { molecule: "H".to_string(), replacement: "OH".to_string() },
        Replacement { molecule: "O".to_string(), replacement: "HH".to_string() }
    ];

    let mut molecule_list = generate_molecule_list(&replacements, &molecule);

    assert_eq!("HOOH", &molecule_list[0]);
    assert_eq!("HOHO", &molecule_list[1]);
    assert_eq!("OHOH", &molecule_list[2]);
    assert_eq!("HOOH", &molecule_list[3]);
    assert_eq!("HHHH", &molecule_list[4]);

    molecule_list.sort();
    molecule_list.dedup();

    assert_eq!(4, molecule_list.len());
}

#[test]
fn test_fabrication() {
    let molecule = "HOH".to_string();
    let santas_molecule = "HOHOHO".to_string();
    let starting_molecule = "e".to_string();
    let replacements = vec![
        Replacement { molecule: "e".to_string(), replacement: "H".to_string() },
        Replacement { molecule: "e".to_string(), replacement: "O".to_string() },
        Replacement { molecule: "H".to_string(), replacement: "HO".to_string() },
        Replacement { molecule: "H".to_string(), replacement: "OH".to_string() },
        Replacement { molecule: "O".to_string(), replacement: "HH".to_string() },
    ];

    assert_eq!(Some(3), get_min_fabrication_steps(&starting_molecule, &molecule, 1, &replacements));
    assert_eq!(Some(6), get_min_fabrication_steps(&starting_molecule, &santas_molecule, 1, &replacements));
}

#[test]
fn test_formula_readability() {
    assert_eq!("C(F,Al)", real_formula(&"CRnFYAlAr"));
}

#[test]
fn test_regex() {
    let re = Regex::new(r"[A-Z][a-z]?").unwrap();

    assert_eq!(3, re.captures_iter(&"HOH").count());
}

#[test]
fn test_count_fewest_steps() {
    assert_eq!(2, count_stepts(&"HOH"));
    assert_eq!(5, count_stepts(&"HOHOHO"));
    assert_eq!(11, count_stepts(&"HOHOH(Al, MgMg)OH(F)Al(C)"));
}