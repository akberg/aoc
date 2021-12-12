const DAY: i32 = 12;

use std::collections::{HashMap};
use rayon::prelude::*;

const IDSTART: char = '<';
const IDEND: char = '>';
const NOTWICE: char = '-';

fn parse_line(line: &str, graph: &mut HashMap<char,Vec<char>>) {
    let mut line = line.split('-');
    let a = line.next().unwrap();
    let a = match a {
        "start" => IDSTART,
        "end" => IDEND,
        _ => a.chars().next().unwrap() as _,
    };
    let b = line.next().unwrap();
    let b = match b {
        "start" => IDSTART,
        "end" => IDEND,
        _ => b.chars().next().unwrap() as _,
    };
    if let Some(v) = graph.get_mut(&a) {
        (*v).push(b);
    } else {
        graph.insert(a, vec![b]);
    }
    if let Some(v) = graph.get_mut(&b) {
        (*v).push(a);
    } else {
        graph.insert(b, vec![a]);
    }
}

pub fn input() -> HashMap<char, Vec<char>> {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(DAY).unwrap();
    let f = BufReader::new(f);
    let mut graph: HashMap<char, Vec<char>> = HashMap::new();
    f.lines().for_each(|line| parse_line(&line.unwrap(), &mut graph));

    graph
}

#[inline(always)]
fn is_lower(c: &char) -> bool {
    'a' <= *c && *c <= 'z'
}
#[inline(always)]
fn is_upper(c: &char) -> bool {
    'A' <= *c && *c <= 'Z'
}

/// Recursive BFS, finding all paths that visit every small cave (lower case id)
/// at most once.
/// - graph: caves named by first character or start/end character
/// - path: current path
fn bfs_recursive(graph: &HashMap<char, Vec<char>>, path: Vec<char>) -> u64 {
    graph[&path.last().unwrap()].clone()
        .par_iter()
        .filter(|&x| is_upper(x) || !path.contains(x))
        .map(|&x| {
            if x == IDEND {
                1
            } else {
                let mut path = path.clone();
                path.push(x);
                bfs_recursive(graph, path)
            }
        })
        .sum::<_>()
}

/// Recursive BFS, finding all paths that visit every small cave (lower case id)
/// at most once, except for one that may be visited twice
/// - graph: caves named by first character or start/end character
/// - path: current path
/// - twice: the small cave that has been visited twice, or NOTWICE if none have been yet.
fn bfs_recursive2(graph: &HashMap<char, Vec<char>>, path: Vec<char>, twice: char) -> u64 {
    graph[&path.last().unwrap()].clone()
        .par_iter()
        .filter(|&&x| x != IDSTART)
        .map(|&x| {
            if x == IDEND {
                1
            } else if is_lower(&x) && path.contains(&x) && twice == NOTWICE {
                let mut path = path.clone();
                path.push(x);
                bfs_recursive2(graph, path, x)
            } else if is_upper(&x) || !path.contains(&x) {
                let mut path = path.clone();
                path.push(x);
                bfs_recursive2(graph, path, twice)
            } else {
                0
            }
        })
        .sum::<_>()
}

/**Countall paths where all small caves are visited at most once
 * BFS, but not marking large caves as visited. Needs to be recursive
 */
pub fn part1(inputs: &HashMap<char, Vec<char>>) -> u64 {
    return bfs_recursive(inputs, vec![IDSTART])
}

/**Find the first step where all octopuses flash simultaneously */
pub fn part2(inputs: &HashMap<char, Vec<char>>) -> u64 {
    return bfs_recursive2(inputs, vec![IDSTART], NOTWICE)
}

/* TESTS */
#[allow(unused)]
static TEST_INPUT1: &'static [&str] = &[
    "start-A",
    "start-b",
    "A-c",
    "A-b",
    "b-d",
    "A-end",
    "b-end",
];
#[allow(unused)]
static TEST_INPUT2: &'static [&str] = &[
    "fs-end",
    "he-DX",
    "fs-he",
    "start-DX",
    "pj-DX",
    "end-zg",
    "zg-sl",
    "zg-pj",
    "pj-he",
    "RW-he",
    "fs-DX",
    "pj-RW",
    "zg-RW",
    "start-pj",
    "he-WI",
    "zg-he",
    "pj-fs",
    "start-RW",
];

#[test]
fn test_day12_part1() {
    let mut inputs = HashMap::new();
    TEST_INPUT1.iter().for_each(|line|parse_line(*line, &mut inputs));
    eprintln!("graoh 1: {:?}", inputs);
    assert_eq!(part1(&inputs), 10);
    let mut inputs = HashMap::new();
    TEST_INPUT2.iter().for_each(|line|parse_line(*line, &mut inputs));
    assert_eq!(part1(&inputs), 226);
}

#[test]
fn test_day12_part2() {
    let mut inputs = HashMap::new();
    TEST_INPUT1.iter().for_each(|line|parse_line(*line, &mut inputs));
    eprintln!("graoh 1: {:?}", inputs);
    assert_eq!(part2(&inputs), 36);
    let mut inputs = HashMap::new();
    TEST_INPUT2.iter().for_each(|line|parse_line(*line, &mut inputs));
    assert_eq!(part2(&inputs), 3509);
}


#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 1: ", DAY);
    println!("{}", part1(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 2: ", DAY);
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
