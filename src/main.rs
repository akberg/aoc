#[macro_use] extern crate lazy_static;

mod aoc;


fn main() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = aoc::day15::input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 15 part 1: ");
    println!("{} - in {:?}", aoc::day15::part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 15 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", aoc::day15::part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}
