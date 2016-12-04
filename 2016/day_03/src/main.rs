use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

fn main() {
    println!("--- Day 3: Squares With Three Sides ---");

    let input = get_input().unwrap();

    println!("There are only {} of right triangles", count_triangles(&input));
    println!("But if to count differently, ther can can be {}", count_triangles_differently(&input));
}

fn get_input() -> Result<String, Error> {
    let mut f = File::open("data/input")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn count_triangles(input: &str) -> usize {
    let mut right_triangle_count = 0;

    for data in input.lines() {
        let mut borders: Vec<u32> = get_triangle_columns(&data);
        borders.sort();
        if borders[0] + borders[1] > borders[2] {
            right_triangle_count += 1;
        }
    }

    right_triangle_count
}

fn count_triangles_differently(input: &str) -> usize {
    let mut triangles = 0;
    let mut lines = input.lines();
    let row_count = input.lines().count();
    let mut row = 2;

    loop {
        let line1 = get_triangle_columns(&lines.next().unwrap());
        let line2 = get_triangle_columns(&lines.next().unwrap());
        let line3 = get_triangle_columns(&lines.next().unwrap());

        for i in 0..3 {
            let mut borders = vec![line1[i], line2[i], line3[i]];
            borders.sort();
            if borders[0] + borders[1] > borders[2] {
                triangles += 1;
            }
        }

        if row < row_count - 3 {
            row += 3;
        } else {
            break;
        }
    }


    triangles
}

fn get_triangle_columns(line: &str) -> Vec<u32> {
    line.split_whitespace().map(|s| s.parse().unwrap()).collect()
}