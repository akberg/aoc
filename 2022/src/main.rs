mod aoc22;
pub mod aoc;

static FIRST_YEAR: usize = 2022;
static RUNS: [[fn(); 25]; 1] = [
    aoc22::RUNS
];

use std::env;
fn help() {
    println!("usage:
aoc <year> <day>");
}

fn main() {
    let mut args = env::args();
    let _application = args.next().unwrap();
    let year = args.next().unwrap_or_default().parse::<usize>().unwrap_or(FIRST_YEAR+RUNS.len()-1);
    let day  = args.next().unwrap_or_default().parse::<usize>().unwrap_or(RUNS[year-FIRST_YEAR].len());
    println!("{}, day {}", year, day);
    //aoc22::day02::run();
    RUNS[year - FIRST_YEAR][day - 1]();
}
