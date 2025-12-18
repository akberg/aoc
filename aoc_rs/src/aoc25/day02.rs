use std::ops::Range;

use super::YEAR;
static DAY: usize = 02;

fn parse_input(line: String) -> Vec<Range<i64>> {
    line.split(",").map(|rng| {
        let (s, e) = rng.split_once("-").unwrap();
        s.parse::<i64>().unwrap()..(e.parse::<i64>().unwrap() + 1)
    }).collect()
}

fn input() -> Vec<Range<i64>> {
    parse_input(crate::aoc::input_raw(YEAR, DAY))
}

fn magnitude(n: i64) -> u32 {
    let mut m = 0;
    while 10i64.pow(m + 1) < n {
        m += 1;
    }
    m
}

// Find numbers containing a digit sequence repeated twice.
fn part1(inputs: &Vec<Range<i64>>) -> i64 {
    let mut count = 0;
    for rng in inputs.clone() {
        for i in rng {
            for j in 1..(i / 2) {
                let m = magnitude(j);
                for n in 2.. {
                    // j + j * 10.pow(m) + j * 10.pow(2 * m) ...;
                }
            }

            let m = magnitude(i);
            if m % 2 == 0 { continue; }
            let half = (m+1) / 2;
            let lower = i / 10i64.pow(half);
            if i == lower + (lower * 10i64.pow(half)) {
                count += i;
            }
        }
    }
    return count
}

// Find numbers containing a digit sequence repeated twice.
fn part2(inputs: &Vec<Range<i64>>) -> i64 {
    let mut count = 0;
    for rng in inputs.clone() {
        for i in rng {
            let m = magnitude(i);
            let half = (m+1) / 2;
            let lower = i / 10i64.pow(half);
            if i == lower + (lower * 10i64.pow(half)) {
                count += i;
            }
        }
    }
    return count
}

#[test]
fn test_2025_day2_part1() {
    assert_eq!(magnitude(1), 0);
    assert_eq!(magnitude(15), 1);
    assert_eq!(magnitude(88), 1);
    assert_eq!(magnitude(359), 2);
    let inputs = parse_input(String::from("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"));
    assert_eq!(part1(&inputs), 1227775554);
}

#[test]
fn test_2025_day2_part2() {
    // TODO
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


