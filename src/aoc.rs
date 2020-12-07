
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

use std::fs;

pub fn input(day: i32) -> Vec<String> {
    input_raw(day)
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub fn input_raw(day: i32) -> String {
    let filename = format!("inputs/day{}.txt", day);
    fs::read_to_string(filename)
        .unwrap()
}
