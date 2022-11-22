
pub fn input() -> u32 {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(2).unwrap();
    let f = BufReader::new(f);
    // f.lines()
    //     .map(|line| line.unwrap().parse::<Command>().unwrap())
    //     .collect()
    0
}

/// TODO: part 1
pub fn part1(_: &u32) -> u64 {
    0
}

/// TODO: part 2
pub fn part2(_: &u32) -> u64 {
    0
}

#[test]
fn test_day02_part1() {
}

#[test]
fn test_day02_part2() {
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 1 part 1: ");
    println!("{}", part1(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 1 part 2: ");
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
