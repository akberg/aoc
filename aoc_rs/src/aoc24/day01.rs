use std::collections::{BinaryHeap, HashMap, HashSet};

use super::YEAR;
static DAY: usize = 01;
type Int = i64;

pub fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

/// (Solved) Sort two input lists and sum the difference between elements at
/// each index.
///
/// Input: String with each line containing one element from each list.
pub fn part1(inputs: &str) -> Int {
    let (l, r) = inputs
        .lines()
        // Parse input into 2 parallel iterators
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.trim().parse::<Int>().unwrap())
                .collect::<Vec<Int>>()
        })
        // Insert in binary heap for automatic sorting
        .fold(
            (BinaryHeap::new(), BinaryHeap::new()),
            |(mut l, mut r), line| {
                l.push(line[0]);
                r.push(line[1]);
                (l, r)
            },
        );
    // Convert to vec for indexing
    let l = l.into_sorted_vec();
    let r = r.into_sorted_vec();
    // Find difference for each entry
    (0..l.len()).map(|idx| (l[idx] - r[idx]).abs()).sum::<_>()
}

/// (Solved) For each element in the first list, multiply its value with the
/// number of occurences in the second list. Sum the result.
pub fn part2(inputs: &str) -> Int {
    let (entries, map) = inputs
        .lines()
        // Parse input into 2 parallel iterators
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.trim().parse::<Int>().unwrap())
                .collect::<Vec<Int>>()
        })
        // Store left list entries, and make a map of counts from right list
        .fold(
            (Vec::<Int>::new(), HashMap::<Int, Int>::new()),
            |(mut entries, mut map), line| {
                entries.push(line[0]);
                map.entry(line[1])
                    .and_modify(|n| {
                        *n += 1;
                    })
                    .or_insert(1);
                (entries, map)
            },
        );
    entries
        .iter()
        // Multiply to format result
        .map(|k| {
            // Get right list count for current value
            let v = map.get(k).unwrap_or(&0);
            k * v
        })
        .sum::<Int>()
}

#[allow(unused)]
static TEST_INPUTS: &str = "3   4
    4   3
    2   5
    1   3
    3   9
    3   3";

#[test]
fn test_2024_day1_part1() {
    assert_eq!(part1(TEST_INPUTS), 11);
}

#[test]
fn test_2024_day1_part2() {
    assert_eq!(part2(TEST_INPUTS), 31);
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
