use memoize::memoize;
use rayon::prelude::*;

use super::YEAR;
static DAY: usize = 11;

fn input() -> Vec<usize> {
    crate::aoc::input_raw(YEAR, DAY)
        .split_whitespace()
        .map(|ls| ls.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

#[memoize]
fn evolve(n: usize, depth: usize) -> usize {
    // End case
    if depth == 0 {
        return 1;
    }
    // Recursive rules
    // 1.
    if n == 0 {
        return evolve(1, depth - 1);
    }
    // 2.
    if n.to_string().len() % 2 == 0 {
        let s = 10usize.pow(n.to_string().len() as u32 / 2);
        let a = n / s;
        let b = n - a * s;
        return evolve(a, depth - 1) + evolve(b, depth - 1);
    }
    // 3.
    evolve(n * 2024, depth - 1)
}

/// (Solved 20min) Evolve an array of numbers following these rules, for 25 iterations.
/// 1. If the stone is engraved with the number 0, it is replaced by a stone engraved with the
/// number 1.
/// 2. If the stone is engraved with a number that has an even number of digits, it is replaced by
/// two stones. The left half of the digits are engraved on the new left stone, and the right half
/// of the digits are engraved on the new right stone. (The new numbers don't keep extra leading
/// zeroes: 1000 would become stones 10 and 0.)
/// 3. If none of the other rules apply, the stone is replaced by a new stone; the old stone's
/// number multiplied by 2024 is engraved on the new stone.
fn part1(inputs: &Vec<usize>) -> usize {
    inputs.iter().map(|&n| evolve(n, 25)).sum()
}

/// (Solved, 10min) Evolve an array of numbers following the same rules as before, for 75
/// iterations. Initial solution returned vectors, which is nice for debugging logic, but consumes
/// too much memory. It's really only the size of each subtree that matters in the end.
///
/// Added parallel iterator for a 20% speed-up. Allowing 1-2 more recursive levels to spawn
/// additional thread gives another tiny increase, but not enough to account for messier code.
fn part2(inputs: &Vec<usize>) -> usize {
    inputs.par_iter().map(|&n| evolve(n, 75)).sum()
}

#[test]
fn test_2024_day11_part1() {
    let test_inputs = vec![125, 17];
    assert_eq!(part1(&test_inputs), 5312);
}

#[test]
fn test_2024_day11_part2() {}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part1(&inputs);
    print!("{} Day {} part 1: ", YEAR, DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part2(&inputs);
    print!("{} Day {} part 2: ", YEAR, DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
