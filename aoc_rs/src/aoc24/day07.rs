use itertools::Itertools;

use super::YEAR;
static DAY: usize = 07;

fn parse_input(inputs: String) -> Vec<(i64, Vec<i64>)> {
    inputs
        .lines()
        .map(|line| {
            let (k, v) = line.split_once(":").unwrap();
            let v = v
                .split_whitespace()
                .map(|n| n.trim().parse::<i64>().unwrap())
                .collect_vec();
            (k.trim().parse::<i64>().unwrap(), v)
        })
        .collect_vec()
}

fn input() -> Vec<(i64, Vec<i64>)> {
    parse_input(crate::aoc::input_raw(YEAR, DAY))
}

/// (Solved, 45min) Given a list of numbers, find the operators between each
/// (disregarding precedence) which makes an expression to yield the final
/// result.
fn part1(inputs: &Vec<(i64, Vec<i64>)>) -> i64 {
    fn resolve_r(result: i64, part: i64, operands: &Vec<i64>, i: usize) -> bool {
        if operands.len() == i {
            // All operands consumed, check if end result is correct
            result == part
        } else {
            resolve_r(result, part * operands[i], operands, i + 1)
                || resolve_r(result, part + operands[i], operands, i + 1)
        }
    }
    inputs
        .iter()
        .filter_map(|(res, ops)| {
            if resolve_r(*res, ops[0], ops, 1) {
                Some(res)
            } else {
                None
            }
        })
        .sum::<i64>()
}

/// (Solved, 5min) Add to part 1 the possibility to concatenate operands.
fn part2(inputs: &Vec<(i64, Vec<i64>)>) -> i64 {
    fn resolve_r(result: i64, part: i64, operands: &Vec<i64>, i: usize) -> bool {
        if operands.len() == i {
            // All operands consumed, check if end result is correct
            result == part
        } else {
            resolve_r(result, part * operands[i], operands, i + 1)
                || resolve_r(result, part + operands[i], operands, i + 1)
                || resolve_r(
                    result,
                    part * 10i64.pow(operands[i].to_string().len() as _) + operands[i],
                    operands,
                    i + 1,
                )
        }
    }
    inputs
        .iter()
        .filter_map(|(res, ops)| {
            if resolve_r(*res, ops[0], ops, 1) {
                Some(res)
            } else {
                None
            }
        })
        .sum::<i64>()
}

#[test]
fn test_2024_day7_part1() {
    let test_inputs = parse_input(String::from(
        "190: 10 19
    3267: 81 40 27
    83: 17 5
    156: 15 6
    7290: 6 8 6 15
    161011: 16 10 13
    192: 17 8 14
    21037: 9 7 18 13
    292: 11 6 16 20",
    ));
    assert_eq!(part1(&test_inputs), 3749);
}

#[test]
fn test_2024_day7_part2() {
    let test_inputs = parse_input(String::from(
        "190: 10 19
        3267: 81 40 27
        83: 17 5
        156: 15 6
        7290: 6 8 6 15
        161011: 16 10 13
        192: 17 8 14
        21037: 9 7 18 13
        292: 11 6 16 20",
    ));
    assert_eq!(part2(&test_inputs), 11387);
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
