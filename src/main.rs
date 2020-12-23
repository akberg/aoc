#[macro_use] extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod aoc;


fn main() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = aoc::day23::input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 23 part 1: ");
    println!("{:?} - in {:?}", aoc::day23::part1(inputs.clone()), pt_start.elapsed().unwrap());
    print!("Day 23 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", aoc::day23::part2(inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}
