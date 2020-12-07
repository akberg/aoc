#[macro_use] extern crate lazy_static;

mod aoc;


fn main() {
    let i = aoc::day2::input();
    println!("Day 2 part 1: {}", aoc::day2::part1(&i).unwrap());
    println!("Day 2 part 2: {}", aoc::day2::part2(&i).unwrap());
}
