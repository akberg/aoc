
pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;


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
