use itertools::Itertools;

use super::YEAR;
static DAY: usize = 02;

pub fn input() -> Vec<Vec<i64>> {
    crate::aoc::input_raw(YEAR, DAY)
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

/// Mark report as safe if it is ever increasing or ever decreasing,
/// and has differences in the range 1-3.
fn is_report_safe(report: &&Vec<i64>) -> bool {
    let mut r = report
        .iter()
        .tuple_windows::<(_, _)>()
        // Map to (direction, acceptable difference)
        .map(|(cur, nxt)| ((cur - nxt).clamp(-1, 1), (1..=3).contains(&(cur - nxt).abs())));
    // Check all levels are safe and same direction.
    r.clone().all_equal() && r.all(|(_, diff)| diff)
}

/// (Solved) Count the number of "safe" reports, see `is_report_safe`.
///
/// Input: Each line a report -- A list of numbers (called levels).
pub fn part1(inputs: &Vec<Vec<i64>>) -> i64 {
    inputs
        .iter()
        // Filter out unsafe reports
        .filter(is_report_safe)
        .count() as i64
}

/// (Solved) Count the number of "safe" reports, see `is_report_safe`, with the
/// ability to remove one number (one level) in order to make it safe.
pub fn part2(inputs: &Vec<Vec<i64>>) -> i64 {
    inputs
        .iter()
        // Filter out unsafe reports.
        .filter(|report| {
            // Early return if the report is safe.
            if is_report_safe(report) {
                return true;
            }
            println!("{:?} unsafe", report);
            // Try removing one level at the time.
            for i in 0..report.len() {
                let mut m = (*report).clone();
                m.remove(i);
                if is_report_safe(&&m) { println!("    {:?} safe", m); return true }
            }
            // No one level could be removed to make the report safe.
            return false
        })
        .count() as i64
}

#[test]
fn test_2024_day02_part1() {
    let test_inputs = Vec::from([
        Vec::from([7, 6, 4, 2, 1]),
        Vec::from([1, 2, 7, 8, 9]),
        Vec::from([9, 7, 6, 2, 1]),
        Vec::from([1, 3, 2, 4, 5]),
        Vec::from([8, 6, 4, 4, 1]),
        Vec::from([1, 3, 6, 7, 9]),
    ]);
    assert_eq!(part1(&test_inputs), 2);
}

#[test]
fn test_2024_day02_part2() {
    let test_inputs = Vec::from([
        Vec::from([7, 6, 4, 2, 1]),
        Vec::from([1, 2, 7, 8, 9]),
        Vec::from([9, 7, 6, 2, 1]),
        Vec::from([1, 3, 2, 4, 5]),
        Vec::from([8, 6, 4, 4, 1]),
        Vec::from([1, 3, 6, 7, 9]),
    ]);
    assert_eq!(part2(&test_inputs), 4);
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
