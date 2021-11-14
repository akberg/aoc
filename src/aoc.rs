
use std::fs;

pub fn input(year: i32, day: i32) -> Vec<String> {
    input_raw(year, day)
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub fn input_raw(year: i32, day: i32) -> String {
    let filename = format!("inputs/{}/day{}.txt", year, day);
    fs::read_to_string(filename)
        .unwrap()
}
