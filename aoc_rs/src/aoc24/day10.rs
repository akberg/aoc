use std::collections::{HashSet, VecDeque};

use itertools::Itertools;

use super::YEAR;
static DAY: usize = 10;

type Vec2 = nalgebra_glm::TVec2<usize>;

fn parse_input(input: &str) -> Vec<Vec<i8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(64) as i8)
                .collect_vec()
        })
        .collect_vec()
}

fn input() -> Vec<Vec<i8>> {
    parse_input(&crate::aoc::input_raw(YEAR, DAY))
}

/// (Solved, 40min) Find trailheads (positions of value 0), and by only ever
/// moving to adjacent positions of exactly 1 higher value, find the number of
/// 9's that can be reached from each head (BFS with counting each 9 only once
/// per head).
fn part1(inputs: &Vec<Vec<i8>>) -> usize {
    let height = inputs.len();
    let width = inputs[0].len();
    // Find starting positions
    let heads = inputs
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, col)| if *col == 0 { Some(Vec2::new(x, y)) } else { None })
        })
        .collect::<VecDeque<_>>();

    println!("Trailheads: {:?}", heads);
    let mut total_peaks = 0;
    for head in heads.iter() {
        let mut peaks = HashSet::<Vec2>::new();
        let mut positions = VecDeque::new();
        positions.push_back(*head);
        while let Some(p) = positions.pop_front() {
            // println!("{}", inputs[p.y][p.x]);
            if inputs[p.y][p.x] == 9 {
                peaks.insert(p);
                continue;
            }
            if p.x > 0 && inputs[p.y][p.x - 1] - inputs[p.y][p.x] == 1 {
                positions.push_back(p - Vec2::new(1, 0));
            }
            if p.y > 0 && inputs[p.y - 1][p.x] - inputs[p.y][p.x] == 1 {
                positions.push_back(p - Vec2::new(0, 1));
            }
            if p.x < width-1 && inputs[p.y][p.x + 1] - inputs[p.y][p.x] == 1 {
                positions.push_back(p + Vec2::new(1, 0));
            }
            if p.y < height-1 && inputs[p.y + 1][p.x] - inputs[p.y][p.x] == 1 {
                positions.push_back(p + Vec2::new(0, 1));
            }
            // println!("{:?}", positions);
        }
        println!("Peaks from {:?}: {}", head, peaks.len());
        total_peaks += peaks.len();
    }
    total_peaks
}

/// (Solved, 1min) Part 2 was what I incorrectly implemented for part 1 at
/// first, so this was as simple as copying and reverting those final changes.
/// Find the number of distinct paths from a 0 to a 9, basic BFS.
fn part2(inputs: &Vec<Vec<i8>>) -> usize {
    let height = inputs.len();
    let width = inputs[0].len();
    // Find starting positions
    let heads = inputs
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, col)| if *col == 0 { Some(Vec2::new(x, y)) } else { None })
        })
        .collect::<VecDeque<_>>();

    println!("Trailheads: {:?}", heads);
    let mut total_peaks = 0;
    for head in heads.iter() {
        let mut peaks = 0;
        let mut positions = VecDeque::new();
        positions.push_back(*head);
        while let Some(p) = positions.pop_front() {
            // println!("{}", inputs[p.y][p.x]);
            if inputs[p.y][p.x] == 9 {
                peaks += 1;
                continue;
            }
            if p.x > 0 && inputs[p.y][p.x - 1] - inputs[p.y][p.x] == 1 {
                positions.push_back(p - Vec2::new(1, 0));
            }
            if p.y > 0 && inputs[p.y - 1][p.x] - inputs[p.y][p.x] == 1 {
                positions.push_back(p - Vec2::new(0, 1));
            }
            if p.x < width-1 && inputs[p.y][p.x + 1] - inputs[p.y][p.x] == 1 {
                positions.push_back(p + Vec2::new(1, 0));
            }
            if p.y < height-1 && inputs[p.y + 1][p.x] - inputs[p.y][p.x] == 1 {
                positions.push_back(p + Vec2::new(0, 1));
            }
            // println!("{:?}", positions);
        }
        println!("Peaks from {:?}: {}", head, peaks);
        total_peaks += peaks;
    }
    total_peaks
}

#[test]
fn test_2024_day10_part1() {
    let test_inputs = "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";
    assert_eq!(part1(&parse_input(test_inputs)), 2);
    let test_inputs = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    assert_eq!(part1(&parse_input(test_inputs)), 36);
}

#[test]
fn test_2024_day10_part2() {
    let test_inputs = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
    assert_eq!(part1(&parse_input(test_inputs)), 81);
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
