
use std::fs;
use std::io::{prelude::*, BufReader};

pub fn input(year: i32, day: i32) -> Vec<String> {
    let filename = format!("inputs/day{}.txt", day);
    let f = fs::File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|s| s.unwrap())//to_string())
        .collect::<Vec<String>>()
}

pub fn input_raw(year: i32, day: i32) -> String {
    let filename = format!("inputs/day{}.txt", day);
    fs::read_to_string(filename)
        .unwrap()
}
