use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use super::YEAR;
static DAY: usize = 05;

fn parse_input(
    inputs: String,
) -> (
    HashMap<usize, (HashSet<usize>, HashSet<usize>)>,
    Vec<Vec<usize>>,
) {
    let (orders_in, lists) = inputs.trim().split_once("\n\n").unwrap();
    // Make a map Value : (Values before, Values after)
    let mut orders = HashMap::<usize, (HashSet<usize>, HashSet<usize>)>::new();
    orders_in.lines().for_each(|line| {
        let (a, b) = line.trim().split_once("|").unwrap();
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();
        let e = orders.entry(a).or_insert((HashSet::new(), HashSet::new()));
        e.1.insert(b);
        let e = orders.entry(b).or_insert((HashSet::new(), HashSet::new()));
        e.0.insert(a);
    });
    // Parse lists
    let lists = lists
        .lines()
        .map(|line| {
            line.trim()
                .split(",")
                .map(|c| c.parse::<usize>().unwrap())
                .collect_vec()
        })
        .collect_vec();
    (orders, lists)
}

pub fn input() -> (
    HashMap<usize, (HashSet<usize>, HashSet<usize>)>,
    Vec<Vec<usize>>,
) {
    parse_input(crate::aoc::input_raw(YEAR, DAY))
}

/// (Solved, 45min) Given list of expected pairwise orderings, filter out the incorrectly sorted
/// lists, and sum the middle element of each kept list.
///
/// Input: Two parts. First, a list of lines on the format `A|B` defining pairwise orderings.
/// Second, each line a comma-separated list of numbers to be checked against the defined ordering.
///
/// * (orders, lists) - `orders` is a hashmap produced from parsing the first part of input, see
/// `parse_input`. `lists` is a parsed Vec of Vec's.
pub fn part1(
    (orders, lists): &(
        HashMap<usize, (HashSet<usize>, HashSet<usize>)>,
        Vec<Vec<usize>>,
    ),
) -> usize {
    lists
        .iter()
        .filter(|list| {
            for i in 0..list.len() {
                for j in 0..i {
                    if orders.get(&list[i]).unwrap().1.contains(&list[j]) {
                        return false;
                    }
                }
                for j in (i + 1)..list.len() {
                    if orders.get(&list[i]).unwrap().0.contains(&list[j]) {
                        return false;
                    }
                }
            }
            true
        })
        .map(|list| list.get(list.len() / 2).unwrap())
        .sum::<usize>()
}

fn sort_list(
    mut list: Vec<usize>,
    orders: &HashMap<usize, (HashSet<usize>, HashSet<usize>)>,
) -> Vec<usize> {
    let mut modified = true;
    while modified {
        modified = false;
        for i in 1..list.len() as usize {
            if orders.get(&list[i - 1]).unwrap().0.contains(&list[i]) {
                list.swap(i - 1, i);
                modified = true;
            }
        }
    }
    list
}

/// (Solved, 45min) Take only the incorrectly sorted lists, correcting the sorting according to
/// defined ordering, and sum the middle elements.
pub fn part2(
    (orders, lists): &(
        HashMap<usize, (HashSet<usize>, HashSet<usize>)>,
        Vec<Vec<usize>>,
    ),
) -> usize {
    lists
        .iter()
        .filter_map(|list| {
            for i in 0..list.len() {
                for j in 0..i {
                    if orders.get(&list[i]).unwrap().1.contains(&list[j]) {
                        return Some(sort_list(list.to_owned(), orders));
                    }
                }
                for j in (i + 1)..list.len() {
                    if orders.get(&list[i]).unwrap().0.contains(&list[j]) {
                        return Some(sort_list(list.to_owned(), orders));
                    }
                }
            }
            None
        })
        .map(|list| list[list.len() / 2])
        .sum::<usize>()
}

#[allow(unused)]
static TEST_INPUTS: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

#[test]
fn test_2024_day5_part1() {
    let test_inputs = parse_input(String::from(TEST_INPUTS));
    assert_eq!(part1(&test_inputs), 143);
}

#[test]
fn test_2024_day5_part2() {
    let test_inputs = parse_input(String::from(TEST_INPUTS));
    assert_eq!(part2(&test_inputs), 123);
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
