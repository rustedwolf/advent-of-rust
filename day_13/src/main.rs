use std::cmp;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Relation {
    person: String,
    related_person: String,
    relation: i32
}

type PeopleList = Vec<String>;
type RelationList = Vec<Relation>;

trait Relations {
    fn get_happiness_change(&self, person: &str, related_person: &str) -> i32;
    fn add_me(&mut self, me: &str, guests: &PeopleList);
}

impl Relations for RelationList {
    fn get_happiness_change(&self, person: &str, related_person: &str) -> i32 {
        let relation = self.iter().find(|&r| {
            r.person == person && r.related_person == related_person
        }).unwrap();
        relation.relation
    }
    fn add_me(&mut self, me: &str, guests: &PeopleList){
        for guest in guests {
            let relation_to_me = Relation {
                person: guest.to_string(),
                related_person: me.to_string(),
                relation: 0
            };
            self.push(relation_to_me);

            let relation_to_person = Relation {
                person: me.to_string(),
                related_person: guest.to_string(),
                relation: 0
            };
            self.push(relation_to_person);
        }        
    }
}

fn main() {
    println!("--- Day 13: Knights of the Dinner Table ---");

    let data = get_data();
    let mut people = make_guestlist(&data);
    let mut relations = get_relations(&data);
    let mut guest_count = people.len();

    let mut optimal_change = get_optimal_change(&mut people, &relations, guest_count);

    println!("The optimal change of happiness is {}", optimal_change);

    let me = "Me";
    relations.add_me(me, &people);
    people.push(me.to_string());
    guest_count = people.len();

    optimal_change = get_optimal_change(&mut people, &relations, guest_count);

    println!("The optimal change of happiness with me is {}", optimal_change);
}

fn get_data() -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    let path = Path::new("input/day_13");
    let input_file = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Could not open input file: {}", reason),
    };

    let reader = BufReader::new(&input_file);

    for line in reader.lines() {
        data.push(line.unwrap());
    }

    data
}

fn make_guestlist(data: &Vec<String>) -> PeopleList {
    let mut people: PeopleList = Vec::new();

    for raw_line in data {
        let info: Vec<&str> = raw_line.split(" ").collect();
        let name = info[0].to_string();
        if !people.contains(&name) {
            people.push(name);
        }
    }

    people
}

fn get_relations(data: &Vec<String>) -> RelationList {
    let mut relations: RelationList = Vec::new();

    for raw_line in data {
        let info: Vec<&str> = raw_line.split(" ").collect();
        let relation = Relation {
            person: info[0].to_string(),
            related_person: info[10].trim_right_matches('.').to_string(),
            relation: match info[2] {
                "gain" => info[3].parse::<i32>().unwrap(),
                "lose" => - info[3].parse::<i32>().unwrap(),
                _ => 0
            }
        };
        relations.push(relation);
    }

    relations
}

fn get_optimal_change(people: &mut PeopleList, relations: &RelationList, length: usize) -> i32 {
    let mut optimal_change = 0;

    for x in 0..length {
        optimal_change = cmp::max(optimal_change, count_happiness(&people, &relations));
        if length > 2 {
            optimal_change = cmp::max(optimal_change, get_optimal_change(people, &relations, length - 1));
        }
        people.swap(x, (x + 1) % length);
    }

    optimal_change
}

fn count_happiness(people: &PeopleList, relations: &RelationList) -> i32 {
    let mut happiness_lvl = 0;

    for g in 0..people.len() {
        happiness_lvl += relations.get_happiness_change(&people[g], &people[if g != 0 { g - 1 } else { people.len() -1 }]);
        happiness_lvl += relations.get_happiness_change(&people[g], &people[(g + 1) % people.len()]);
    }

    happiness_lvl
}
