use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::cmp;

struct Ingridient {
    properties: Vec<i64>
}

type Ingridients = Vec<Ingridient>;
type IngridientMix = Vec<i64>;

fn main() {
    println!("--- Day 15: Science for Hungry People ---");

    let ingridients = gather_ingridients();
    let ingridien_count = ingridients.len();
    let teaspoon_count = 100;
    let mix: IngridientMix = vec![0; ingridien_count];
    let target_calories: i64 = 500;

    let mut max_score = get_max_mix_score(ingridien_count - 1, teaspoon_count, &mix, &ingridients, ingridien_count);

    println!("The maximum score for a cookie can be {}", max_score);

    max_score = get_max_mix_score_by_calories(ingridien_count - 1, teaspoon_count, &mix, &ingridients, ingridien_count, target_calories);

    println!("The maximum score with {} calories for a cookie can be {}", target_calories, max_score);
}

fn gather_ingridients() -> Ingridients {
    let path = Path::new("input/day_15");
    let input = match File::open(&path) {
        Ok(file) => file,
        Err(reason) => panic!("Coul not open input file: {}", reason),
    };

    let reader = BufReader::new(&input);
    let mut ingridients: Ingridients = Vec::new();

    for line in reader.lines() {
        let info_string = line.unwrap();
        let ingridient_info: Vec<&str> = info_string.split(": ").collect();
        let attribute_infos: Vec<&str> = ingridient_info[1].split(", ").collect();

        let mut properties = Vec::new();
        for info in attribute_infos {
            let property_points: i64 = info.split(' ').collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
            properties.push(property_points);
        }
        let ingridient = Ingridient {
            properties: properties
        };
        ingridients.push(ingridient);
    }

    ingridients
}

fn get_mix_score(
    mix: &IngridientMix,
    ingridients: &Ingridients,
    prop_count: usize,
    ingridien_count: usize,
) -> i64 {
    let mut score: i64 = 0;

    'properties: for x in 0..prop_count {
        let mut prop_score = 0;
        for y in 0..ingridien_count {
            let points = mix[y] * ingridients[y].properties[x];
            prop_score += points;
        }
        if prop_score <= 0 {
            score = 0;
            break 'properties;
        }
        score = if x != 0 { score * prop_score } else { prop_score };
    }

    score
}

fn get_calories( mix: &IngridientMix,
    ingridients: &Ingridients,
    index: usize,
    ingridien_count: usize
) -> i64 {
    let mut calories: i64 = 0;
    
    for y in 0..ingridien_count {
        calories += mix[y] * ingridients[y].properties[index];
    }

    calories
}

fn get_max_mix_score(
    start_from: usize,
    teaspoons: i64,
    mix: &IngridientMix,
    ingridients: &Ingridients,
    ingridien_count: usize
) -> i64 {
    let mut max_score: i64 = 0;
    let mut mix_score: i64;

    let mut ingridient_mix = mix.clone();
    ingridient_mix[start_from] = teaspoons;

    if start_from != 0 {
        for teaspoons_left in 0..teaspoons {
            ingridient_mix[start_from] = teaspoons - teaspoons_left;
            if teaspoons_left > 0 {
                mix_score = get_max_mix_score(start_from - 1, teaspoons_left, &ingridient_mix, &ingridients, ingridien_count);
            } else {
                mix_score = get_mix_score(&ingridient_mix, &ingridients, 4, ingridien_count);
            }
            max_score = cmp::max(max_score, mix_score);
        }
    } else {
        max_score = get_mix_score(&ingridient_mix, &ingridients, 4, ingridien_count);
    }

    max_score
}

fn get_max_mix_score_by_calories(
    start_from: usize,
    teaspoons: i64,
    mix: &IngridientMix,
    ingridients: &Ingridients,
    ingridien_count: usize,
    target_calories: i64
) -> i64 {
    let mut max_score: i64 = 0;
    let mut mix_score: i64;

    let mut ingridient_mix = mix.clone();
    ingridient_mix[start_from] = teaspoons;

    if start_from != 0 {
        for teaspoons_left in 0..teaspoons {
            ingridient_mix[start_from] = teaspoons - teaspoons_left;
            if teaspoons_left > 0 {
                mix_score = get_max_mix_score_by_calories(start_from - 1, teaspoons_left, &ingridient_mix, &ingridients, ingridien_count, target_calories);
                max_score = cmp::max(max_score, mix_score);
            } else {
                mix_score = get_mix_score(&ingridient_mix, &ingridients, 4, ingridien_count);
                if target_calories == get_calories(&ingridient_mix, &ingridients, 4, ingridien_count){
                    max_score = cmp::max(max_score, mix_score);
                }
            }
        }
    } else {
        mix_score = get_mix_score(&ingridient_mix, &ingridients, 4, ingridien_count);
        if target_calories == get_calories(&ingridient_mix, &ingridients, 4, ingridien_count){
            max_score = mix_score;
        }
    }

    max_score
}


#[test]
fn test_score_counter() {
    let mut ingridients: Ingridients = Vec::new();
    let butterscotch = Ingridient {
        properties: vec![-1, -2, 6, 3, 8]
    };
    let cinnamon = Ingridient {
        properties: vec![2, 3, -2, -1, 3]
    };

    ingridients.push(butterscotch);
    ingridients.push(cinnamon);

    let mut mix: IngridientMix = vec![44,56];

    let mut score = get_mix_score(&mix, &ingridients, 4, ingridients.len());

    assert_eq!(score, 62842880);

    mix = vec![40,60];

    score = get_mix_score(&mix, &ingridients, 4, ingridients.len());
    let calories = get_calories(&mix, &ingridients, 4, ingridients.len());

    assert_eq!(score, 57600000);
    assert_eq!(calories, 500);
}