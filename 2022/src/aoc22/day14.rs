static DAY: usize = 14;

use itertools::{Itertools};
use std::collections::HashSet;

pub fn input() -> String {
    crate::aoc::input_raw(14)
}

/// The stupidly naive solution, trace every sand particle
pub fn part1(inputs: &str) -> usize {
    let source = (500, 0);
    let mut height = 1;
    let mut width_min = 500;
    let mut width = 501;
    let mut cave = HashSet::new();
    let inputs = inputs.lines()
    .map(|line| line.trim().split(" -> ")
        .tuple_windows::<(_,_)>()
        .map(|(a, b)| (
            sscanf::scanf!(a, "{},{}", isize, isize).unwrap(),
            sscanf::scanf!(b, "{},{}", isize, isize).unwrap(),
        ))
    );
    for line in inputs {
        for (a, b) in line {
            height = height.max(a.1+1).max(b.1+1);
            width = width.max(a.0+1).max(b.0+1);
            width_min = width_min.min(a.0).min(b.0);
            // Add obstacles
            let dist = (a.0-b.0).abs().max((a.1-b.1).abs()) as usize;
            let delta = (
                ((a.0-b.0).abs() != 0) as usize,
                ((a.1-b.1).abs() != 0) as usize
            );
            // Smallest number as starting point
            let start = if a.0 < b.0 || a.1 < b.1 { a } else { b };
            for i in 0..=dist {
                cave.insert((start.0 as usize + delta.0*i, start.1 as usize + delta.1*i));
            }
        }
    }
    let empty_cave = cave.len();

    'outer: loop {
        let mut p = (0,0);
        let mut p_nxt = source;
        while p != p_nxt {
            p = p_nxt;
            if p.1 == width as usize {
                break 'outer
            }
            if !cave.contains(&(p.0, p.1+1)) {
                p_nxt = (p.0, p.1+1);
            }
            else if !cave.contains(&(p.0-1, p.1+1)) {
                p_nxt = (p.0-1, p.1+1);
            }
            else if !cave.contains(&(p.0+1, p.1+1)) {
                p_nxt = (p.0+1, p.1+1);
            }
            else {
                cave.insert(p);
            }
        }
    }

    cave.len() - empty_cave
}

// Flood fill breadth-first
pub fn part2(inputs: &str) -> usize {
    let source = (500, 0);
    let mut height = 1;
    let mut width_min = 500;
    let mut width = 501;
    let mut cave = HashSet::new();
    let inputs = inputs.lines()
    .map(|line| line.trim().split(" -> ")
        .tuple_windows::<(_,_)>()
        .map(|(a, b)| (
            sscanf::scanf!(a, "{},{}", isize, isize).unwrap(),
            sscanf::scanf!(b, "{},{}", isize, isize).unwrap(),
        ))
    );
    for line in inputs {
        for (a, b) in line {
            height = height.max(a.1+1).max(b.1+1);
            width = width.max(a.0+1).max(b.0+1);
            width_min = width_min.min(a.0).min(b.0);
            // Add obstacles
            let dist = (a.0-b.0).abs().max((a.1-b.1).abs()) as usize;
            let delta = (
                ((a.0-b.0).abs() != 0) as usize,
                ((a.1-b.1).abs() != 0) as usize
            );
            // Smallest number as starting point
            let start = if a.0 < b.0 || a.1 < b.1 { a } else { b };
            for i in 0..=dist {
                cave.insert((start.0 as usize + delta.0*i, start.1 as usize + delta.1*i));
            }
        }
    }
    let empty_cave = cave.len();

    let mut queue = std::collections::VecDeque::from([source]);
    cave.insert(source);

    while let Some(p) = queue.pop_front() {

        if p.1 == height as usize {
            continue;
        }
        if !cave.contains(&(p.0, p.1+1)) {
            cave.insert((p.0, p.1+1));
            queue.push_back((p.0, p.1+1));
        }
        if !cave.contains(&(p.0-1, p.1+1)) {
            cave.insert((p.0-1, p.1+1));
            queue.push_back((p.0-1, p.1+1));
        }
        if !cave.contains(&(p.0+1, p.1+1)) {
            cave.insert((p.0+1, p.1+1));
            queue.push_back((p.0+1, p.1+1));
        }
    }

    cave.len() - empty_cave
}

#[test]
fn test_day14_part1() {
    let inputs = "498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_eq!(part1(inputs), 24);
}

#[test]
fn test_day14_part2() {
    let inputs = "498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9";
    assert_eq!(part2(inputs), 93);
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
    print!("Day {} part 1: ", DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part2(&inputs);
    print!("Day {} part 2: ", DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}


