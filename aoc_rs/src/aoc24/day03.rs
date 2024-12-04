use regex::Regex;

use super::YEAR;
static DAY: usize = 03;

pub fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

/// (Solved, 15min) Find all occurences of uncorrupted `mul` instructions in a
/// corrupted program, multiply their arguments and sum the results.
///
/// Input: One string (a program)
pub fn part1(inputs: &str) -> i64 {
    // Match mul(a, b) where a and b a 1-3 digit integers.
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(inputs)
        .map(|cap| {
            cap.get(1).unwrap().as_str().parse::<i64>().unwrap()
                * cap.get(2).unwrap().as_str().parse::<i64>().unwrap()
        })
        .sum::<_>()
}

/// (Solved, 15min) Extends from part 1 by adding occurences of `do()` to
/// include mul instructions, and `don't()` to ignore mul instructions.
pub fn part2(inputs: &str) -> i64 {
    let re = Regex::new(
        r"(?<mul>mul\((?<a>\d{1,3}),(?<b>\d{1,3})\))|(?<open>do\(\))|(?<close>don't\(\))",
    )
    .unwrap();
    let mut en = true;
    let mut res = 0;
    for cap in re.captures_iter(inputs) {
        if let Some(_) = cap.name("mul") {
            if en {
                res += &cap["a"].parse::<i64>().unwrap() * &cap["b"].parse::<i64>().unwrap();
            }
        }
        if let Some(_) = cap.name("open") {
            en = true;
        }
        if let Some(_) = cap.name("close") {
            en = false;
        }
    }
    return res;
}

#[test]
fn test_2024_day3_part1() {
    let test_inputs = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part1(test_inputs), 161);
}

#[test]
fn test_2024_day3_part2() {
    let test_inputs = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part2(test_inputs), 48);
}

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
