static YEAR: usize = 2025;
/* Days */
pub static RUNS: [fn(); 25] = [
    day01::run,
    day02::run,
    day03::run,
    day04::run,
    day05::run,
    day06::run,
    day07::run,
    day08::run,
    day09::run,
    day10::run,
    day11::run,
    day12::run,
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
    || { println!("No puzzle this day.") },
];

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
