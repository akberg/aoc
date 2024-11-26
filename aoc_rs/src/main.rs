// #![feature(int_abs_diff)]
// #![feature(int_roundings)]
pub mod aoc;
mod aoc23;
mod aoc24;

static FIRST_YEAR: usize = 2023;
static YEAR: usize = 2023;
static RUNS: [[fn(); 25]; 2] = [aoc23::RUNS, aoc24::RUNS];

use chrono::{self, Datelike};
use std::env;
#[allow(unused)]
fn help() {
    println!("usage: aoc <day> [year=current-year]");
}

fn main() {
    let mut args = env::args();
    let _application = args.next().unwrap();
    // TODO: Parse year
    let day = args
        .next()
        .unwrap_or_default()
        .parse::<usize>()
        .unwrap_or(chrono::Utc::now().date_naive().day() as usize);
    let year = args
        .next()
        .unwrap_or_default()
        .parse::<usize>()
        .unwrap_or(FIRST_YEAR + RUNS.len() - 1);
    println!("{}, day {}", year, day);
    //aoc22::day02::run();
    RUNS[year - FIRST_YEAR][day - 1]();
}