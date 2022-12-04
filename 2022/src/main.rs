mod aoc22;
pub mod aoc;

static YEAR: usize = 2022;
static RUNS: [[fn(); 25]; 1] = [
    aoc22::RUNS
];

use std::env;
use chrono::{self, Datelike};
#[allow(unused)]
fn help() {
    println!("usage:
aoc2022 <day>");
}

fn main() {
    let mut args = env::args();
    let _application = args.next().unwrap();
    //let year = args.next().unwrap_or_default().parse::<usize>().unwrap_or(FIRST_YEAR+RUNS.len()-1);
    let day  = args.next().unwrap_or_default()
        .parse::<usize>()
        .unwrap_or(chrono::Utc::now().date_naive().day() as usize);
    println!("{}, day {}", YEAR, day);
    //aoc22::day02::run();
    RUNS[0][day - 1]();
}
