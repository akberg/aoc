use std::collections::HashMap;

use super::YEAR;
static DAY: usize = 19;

fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

/// Return the number of matches. Manual memoization implementation, as #memoize
/// could not handle the strings.
fn can_match<'a>(
    rules: &'a Vec<&'a str>,
    line: &'a str,
    mem: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(&m) = mem.get(line) {
        return m;
    }
    let m = rules
        .iter()
        .filter(|&r| line.starts_with(r))
        .map(|&r| {
            if r.len() == line.len() {
                1
            } else {
                can_match(rules, &line[r.len()..], mem)
            }
        })
        .sum();
    mem.insert(line, m);
    m
}

/// (Solved, 30min) DFS matching with memoization.
fn part1(inputs: &str) -> usize {
    let (rules, lines) = inputs.split_once("\n\n").unwrap();
    let rules = rules.trim().split(", ").collect::<Vec<_>>();
    let mut mem = HashMap::new();

    lines
        .trim()
        .lines()
        .filter(|line| can_match(&rules, line, &mut mem) > 0)
        .count()
}

/// (Solved, 10min) Expand to return the number of different matches.
fn part2(inputs: &str) -> usize {
    let (rules, lines) = inputs.split_once("\n\n").unwrap();
    let rules = rules.trim().split(", ").collect::<Vec<_>>();
    let mut mem = HashMap::new();

    lines
        .trim()
        .lines()
        .map(|line| can_match(&rules, line, &mut mem))
        .sum()
}

#[allow(unused)]
static TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

#[test]
fn test_2024_day19_part1() {
    assert_eq!(part1(TEST_INPUT), 6);
}

#[test]
fn test_2024_day19_part2() {
    assert_eq!(part2(TEST_INPUT), 16);
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
