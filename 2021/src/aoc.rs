use std::fs;
use std::io::{prelude::*, BufReader};

pub fn input_file(day: i32) -> std::io::Result<fs::File> {
    let filename = format!("inputs/day{}.txt", day);
    fs::File::open(filename)
}

pub fn input(day: i32) -> Vec<String> {
    let filename = format!("inputs/day{}.txt", day);
    let f = fs::File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|s| s.unwrap())//to_string())
        .collect::<Vec<String>>()
}

pub fn input_raw(day: i32) -> String {
    let filename = format!("inputs/day{}.txt", day);
    fs::read_to_string(filename)
        .unwrap()
}

pub fn _test_input_file(day: i32, test: i32) -> std::io::Result<fs::File> {
    let filename = format!("inputs/day{}test{}.txt", day, test);
    fs::File::open(filename)
}

pub fn _test_input(day: i32, test: i32) -> Vec<String> {
    let filename = format!("inputs/day{}test{}.txt", day, test);
    let f = fs::File::open(filename).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|s| s.unwrap())//to_string())
        .collect::<Vec<String>>()
}

pub fn _test_input_raw(day: i32, test: i32) -> String {
    let filename = format!("inputs/day{}test{}.txt", day, test);
    fs::read_to_string(filename)
        .unwrap()
}

#[allow(unused)]
pub fn print_img<T: std::fmt::Display>(img: &Vec<Vec<T>>) {
    for i in 0..img.len(){
        for j in 0..img[0].len() {
            print!("{}", img[i][j]);
        }
        println!("");
    }
}
