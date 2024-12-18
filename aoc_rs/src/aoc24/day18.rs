use std::collections::{BinaryHeap, HashMap, HashSet};

/// Keywords: Shortest Path
///
use super::YEAR;
static DAY: usize = 18;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct State {
    cost: usize,
    position: (isize, isize),
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn input() -> Vec<(isize, isize)> {
    crate::aoc::input_raw(YEAR, DAY)
        .lines()
        .map(|line| {
            let (a, b) = line.trim().split_once(",").unwrap();
            (a.parse::<_>().unwrap(), b.parse::<_>().unwrap())
        })
        .collect()
}

fn build_map(blocks: &Vec<(isize, isize)>, n: usize) -> HashSet<(isize, isize)> {
    let mut map = HashSet::new();
    for i in 0..n {
        map.insert(blocks[i]);
    }
    map
}

/// Find the shortest path from (0, 0) to (size, size) on a grid where blocked
/// cells are given by map. Dijkstra.
fn shortest_path(map: &HashSet<(isize, isize)>, size: isize) -> Option<usize> {
    let mut queue = BinaryHeap::new();
    let mut dist = HashMap::new();
    (0..=size).for_each(|x| {
        (0..=size).for_each(|y| {
            if !map.contains(&(x, y)) {
                dist.insert((x, y), usize::MAX);
            }
        })
    });
    queue.push(State {
        cost: 0,
        position: (0, 0),
    });

    while let Some(State { cost, position }) = queue.pop() {
        // Found destination
        if position == (size, size) {
            // // Print final map
            // for y in 0..=size {
            //     for x in 0..=size {
            //         if map.contains(&(x, y)) {
            //             print!("#");
            //         }
            //         else if dist[&(x, y)] < usize::MAX {
            //             print!("O");
            //         }
            //         else {
            //             print!(".");
            //         }
            //     }
            //     println!("");
            // }
            return Some(cost);
        }
        // Better route already found
        if cost > dist[&position] {
            continue;
        }
        for d in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let next = State {
                cost: cost + 1,
                position: (position.0 + d.0, position.1 + d.1),
            };
            // Out of bounds
            if next.position.0 < 0
                || next.position.1 < 0
                || next.position.0 > size
                || next.position.1 > size
            {
                continue;
            }
            // Cell is blocked
            if map.contains(&next.position) {
                continue;
            }
            if next.cost < dist[&next.position] {
                dist.entry(next.position).and_modify(|e| *e = next.cost);
                queue.push(next)
            }
        }
    }
    println!("No path found");
    None
}

/// (Solved, >1h) Find shortest path.
fn part1(inputs: &Vec<(isize, isize)>) -> usize {
    let map = build_map(inputs, 1024);
    shortest_path(&map, 70).unwrap()
}

/// (Solved, 40min) Find first input to cause a path to be unreachable.
fn part2(inputs: &Vec<(isize, isize)>) -> (isize, isize) {
    let mut map = HashSet::new();
    for p in inputs {
        map.insert(*p);
        if shortest_path(&map, 70).is_none() {
            return *p
        }
    }
    // Failure:
    (0, 0)
}

lazy_static::lazy_static! {
    static ref INPUTS: Vec<(isize, isize)> = vec![
        (5, 4),
        (4, 2),
        (4, 5),
        (3, 0),
        (2, 1),
        (6, 3),
        (2, 4),
        (1, 5),
        (0, 6),
        (3, 3),
        (2, 6),
        (5, 1),
        (1, 2),
        (5, 5),
        (2, 5),
        (6, 5),
        (1, 4),
        (0, 4),
        (6, 4),
        (1, 1),
        (6, 1),
        (1, 0),
        (0, 5),
        (1, 6),
        (2, 0),
    ];

}
#[test]
fn test_2024_day18_part1() {
    let map = build_map(&INPUTS, 12);
    assert_eq!(shortest_path(&map, 6), Some(22));
}

#[test]
fn test_2024_day18_part2() {
    let mut map = HashSet::new();
    let mut res = (0, 0);
    for (i, p) in INPUTS.iter().enumerate() {
        map.insert(*p);
        println!("{} ns", i);
        if shortest_path(&map, 6).is_none() {
            println!("First");
            res = *p;
            break;
        }
    }
    assert_eq!(res, (6, 1));
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
    println!("{:?}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
