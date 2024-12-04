/// Yet another of those damned map puzzles, I should really just make a
/// reusable utility for handling these tasks.
use itertools::Itertools;

use super::YEAR;
static DAY: usize = 04;

fn parse_input(inputs: String) -> Vec<Vec<char>> {
    inputs
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect_vec())
        .collect_vec()
}

pub fn input() -> Vec<Vec<char>> {
    parse_input(crate::aoc::input_raw(YEAR, DAY))
}

pub fn part1(inputs: &Vec<Vec<char>>) -> i64 {
    let mut count = 0;
    let height = inputs.len();
    let width = inputs[0].len();
    let word = "XMAS".chars().collect_vec();
    println!("{:?}", inputs);

    for y in 0..height {
        for x in 0..width {
            if inputs[y][x] != 'X' {
                // Early return when no X is hit.
                continue;
            }
            // Check for "XMAS" in all directions.
            count += ((1..=3).all(|i| inputs[y].get(x + i) == Some(&word[i]))) as i64;
            count += ((1..=3)
                .all(|i| inputs.get(y + i).and_then(|row| row.get(x + i)) == Some(&word[i])))
                as i64;
            count += ((1..=3)
                .all(|i| inputs.get(y + i).and_then(|row| row.get(x)) == Some(&word[i])))
                as i64;
            if x > 2 {
                count += ((1..=3).all(|i| inputs[y].get(x - i) == Some(&word[i]))) as i64;
                count += ((1..=3)
                    .all(|i| inputs.get(y + i).and_then(|row| row.get(x - i)) == Some(&word[i])))
                    as i64;
            }
            if y > 2 {
                count += ((1..=3)
                    .all(|i| inputs.get(y - i).and_then(|row| row.get(x + i)) == Some(&word[i])))
                    as i64;
                count += ((1..=3)
                    .all(|i| inputs.get(y - i).and_then(|row| row.get(x)) == Some(&word[i])))
                    as i64;
                if x > 2 {
                    count += ((1..=3).all(|i| {
                        inputs.get(y - i).and_then(|row| row.get(x - i)) == Some(&word[i])
                    })) as i64;
                }
            }
        }
    }
    count
}

pub fn part2(inputs: &Vec<Vec<char>>) -> i64 {
    let mut count = 0;
    let height = inputs.len();
    let width = inputs[0].len();

    for y in 0..(height - 2) {
        for x in 0..(width - 2) {
            if inputs[y + 1][x + 1] == 'A'
                && ((inputs[y][x] == 'M' && inputs[y + 2][x + 2] == 'S')
                    || inputs[y][x] == 'S' && inputs[y + 2][x + 2] == 'M')
                && ((inputs[y][x + 2] == 'M' && inputs[y + 2][x] == 'S')
                    || inputs[y][x + 2] == 'S' && inputs[y + 2][x] == 'M')
            {
                count += 1;
            }
        }
    }
    count
}

lazy_static::lazy_static! {
    static ref TEST_INPUTS: Vec<Vec<char>> = parse_input(
        String::from("MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX")
    );
}
#[test]
fn test_2024_day4_part1() {
    println!("{}", true as i64);
    assert_eq!(part1(&TEST_INPUTS), 18);
}

#[test]
fn test_2024_day4_part2() {
    assert_eq!(part2(&TEST_INPUTS), 9);
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
