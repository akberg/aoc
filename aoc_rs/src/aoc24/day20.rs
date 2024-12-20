use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;

use super::YEAR;
static DAY: usize = 20;

// Cloned from day16, remove direction from state.

type Vec2 = nalgebra_glm::TVec2<usize>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}
impl Direction {
    pub fn delta(&self, v: Vec2, scale: usize) -> Vec2 {
        let mut v = v.clone();
        match self {
            Direction::Up => v.y -= scale as usize,
            Direction::Right => v.x += scale as usize,
            Direction::Down => v.y += scale as usize,
            Direction::Left => v.x -= scale as usize,
        };
        v
    }
    pub fn iterator() -> impl Iterator<Item = Direction> {
        use Direction::*;
        [Up, Right, Down, Left].iter().copied()
    }
}
impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Down => 'v',
                Direction::Left => '<',
            }
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct QueueElement {
    cost: usize,
    position: Vec2,
}
impl Ord for QueueElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl PartialOrd for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Parse map and start/end positions. Map is encoded as a set containing
/// available tiles.
fn parse_map(inputs: &str) -> (HashSet<Vec2>, Vec2, Vec2, usize, usize) {
    let height = inputs.trim().lines().count();
    let width = inputs.trim().lines().next().unwrap().len();
    let mut start = Vec2::new(0, 0);
    let mut end = Vec2::new(0, 0);
    let mut map = HashSet::new();
    inputs.trim().lines().enumerate().for_each(|(y, line)| {
        line.trim().char_indices().for_each(|(x, c)| {
            match c {
                'S' | 'E' | '.' => {
                    if c == 'S' {
                        start.x = x as usize;
                        start.y = y as usize;
                    } else if c == 'E' {
                        end.x = x as usize;
                        end.y = y as usize;
                    }
                    // Add open space
                    map.insert(Vec2::new(x as usize, y as usize));
                }
                '#' => {
                    // Closes space
                    ()
                }
                _ => unreachable!(),
            }
        });
    });
    (map, start, end, height, width)
}

/// Dijkstra shortest path, modified from day 16. Now find shortest path to
/// all.
fn shortest_path_all(map: &HashSet<Vec2>, start_position: Vec2) -> HashMap<Vec2, usize> {
    // Dist map holds a position/rotation state and its cost. Non-existing
    // entry means MAX cost.
    let mut dist: HashMap<Vec2, usize> = HashMap::new();
    dist.insert(start_position, 0);

    // Queue holds states to be visited, using cost for priority.
    let mut queue = BinaryHeap::new();
    queue.push(QueueElement {
        cost: 0,
        position: start_position,
    });

    while let Some(QueueElement { cost, position }) = queue.pop() {
        for d in Direction::iterator() {
            // Check all directions
            let next_position = d.delta(position, 1);
            let next_cost = cost + 1;

            if map.contains(&next_position) {
                // Open path this direction.
                let prev_cost = dist.entry(next_position).or_insert(usize::MAX);
                if next_cost < *prev_cost {
                    // This path is shorter, update cost.
                    *prev_cost = next_cost;
                    queue.push(QueueElement {
                        cost: next_cost,
                        position: next_position,
                    });
                }
            }
        }
    }
    return dist;
}

fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

/// 1. Find shortest path from S to all, and shortes distance E to all.
/// 2. For each position N, and for each position M with manhattan
/// distance <= radius from M, the cost will be dist(S:N) + dist(M:E) + man(N:M)
/// Count the number of occurences which give a cost <= dist(S:E) - 100.
fn available_shortcuts(input: &str, short: usize, impact: usize) -> usize {
    let (map, start, end, height, width) = parse_map(input);

    let from_s = shortest_path_all(&map, start);
    let from_e = shortest_path_all(&map, end);
    let s_to_e = from_s[&end];

    let mut jumps_set = HashSet::new();

    (0..height).cartesian_product(0..width).for_each(|(y, x)| {
        // From each available tile to jump from
        if let Some(s) = map.get(&Vec2::new(x, y)) {
            for j in 0..=short {
                for i in 0..=(short - j) {
                    let mut jumps = Vec::from([Vec2::new(x + i, y + j)]);
                    let dx = x >= i;
                    let dy = y >= j;
                    if dx {
                        jumps.push(Vec2::new(x - i, y + j));
                    }
                    if dy {
                        jumps.push(Vec2::new(x + i, y - j));
                    }
                    if dx && dy {
                        jumps.push(Vec2::new(x - i, y - j));
                    }
                    for pos_j in jumps {
                        // For each available landing tile within
                        if let Some(e) = map.get(&pos_j) {
                            let cost = from_s[s] + from_e[e] + i + j;
                            if cost <= s_to_e - impact {
                                jumps_set.insert((s, e));
                            }
                        }
                    }
                }
            }
        }
    });
    jumps_set.len()
}

/// (Solved, >1h) Replay of shortest path, adding a post-processing to find
/// walls to go through.
fn part1(inputs: &str) -> usize {
    available_shortcuts(inputs, 2, 100)
}

/// (Solved, >2h) Modified constraints required complete rewrite to final
/// solution.
fn part2(inputs: &str) -> usize {
    available_shortcuts(inputs, 20, 100)
}

#[allow(unused)]
static TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

#[test]
fn test_2024_day20_part1() {
    println!("Too cool for testing");
    assert_eq!(available_shortcuts(TEST_INPUT, 2, 64), 1);
    // assert_eq!(available_shortcuts(TEST_INPUT, 4), 28);
    // assert_eq!(available_shortcuts(TEST_INPUT, 4), 28);
}

#[test]
fn test_2024_day20_part2() {
    // TODO
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
