use std::ops::Index;

use super::YEAR;
static DAY: usize = 03;

fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

fn part1(inputs: &str) -> u32 {
    inputs.lines().map(|line| {
        let line = line.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>();
        let a = line[0..line.len()-1].iter().max().unwrap();
        let idx: usize = line.iter().position(|c| c==a).unwrap();
        let b = line[idx+1..].iter().max().unwrap();
        println!("{} {}", a, b);
        a * 10 + b
    }).sum()
}

fn part2(inputs: &str) -> u64 {
    inputs.lines().map(|line| {
        let line = line.trim().chars().map(|c| c.to_digit(10).unwrap() as u64).collect::<Vec<_>>();
        let mut res = 0;
        let mut prev = 0;
        for i in line.len()-12 .. line.len() {
            let a = line[prev..=i].iter().max().unwrap();
            prev = line.iter().skip(prev).position(|c| c==a).unwrap() + 1 + prev;
            res = res * 10 + a;
        }
        res
    }).sum()
}

#[test]
fn test_2025_day3_part1() {
    let inputs = "987654321111111
                811111111111119
                234234234234278
                818181911112111";
    assert_eq!(part1(inputs), 357);
}

#[test]
fn test_2025_day3_part2() {
    let inputs = "987654321111111
                811111111111119
                234234234234278
                818181911112111";
    assert_eq!(part2(inputs), 3121910778619);
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


