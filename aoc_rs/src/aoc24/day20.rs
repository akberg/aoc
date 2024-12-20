use std::collections::{BinaryHeap, HashMap, HashSet};

use super::YEAR;
static DAY: usize = 20;

// Cloned from day16, remove direction from state.

type Vec2 = nalgebra_glm::TVec2<i64>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    Up = 0,
    Right,
    Down,
    Left,
}
impl Direction {
    pub fn rot_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
    pub fn rot_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
    pub fn delta(&self, v: Vec2, scale: usize) -> Vec2 {
        let mut v = v.clone();
        match self {
            Direction::Up => v.y -= scale as i64,
            Direction::Right => v.x += scale as i64,
            Direction::Down => v.y += scale as i64,
            Direction::Left => v.x -= scale as i64,
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
        // .then_with(|| self.position.cmp(&other.position))
        // .then_with(|| self.rotation.cmp(&other.rotation))
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
                        start.x = x as i64;
                        start.y = y as i64;
                    } else if c == 'E' {
                        end.x = x as i64;
                        end.y = y as i64;
                    }
                    // Add open space
                    map.insert(Vec2::new(x as i64, y as i64));
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

/// Backtracking path, modified from day 16.
fn backtrack_path(
    links: &HashMap<Vec2, Vec2>,
    start: Vec2,
    mut end: Vec2,
    path: &mut Vec<Vec2>,
) {
    while end != start {
        let prev = &links[&end];
        path.insert(0, *prev);
        end = *prev;
        //backtrack_path(links, start, *prev, path);
    }
}

/// Dijkstra shortest path, modified from day 16. There is now one single path.
fn shortest_path(
    map: HashSet<Vec2>,
    start_position: Vec2,
    end_position: Vec2,
) -> Option<(usize, Vec<Vec2>)> {
    // Track shortest path to any point.
    let mut links: HashMap<Vec2, Vec2> = HashMap::new();

    // let mut best_length = None;
    //let mut paths = Vec::new();
    // Dist map holds a position/rotation state and its cost. Non-existing
    // entry means MAX cost.
    let mut dist = HashMap::new();
    dist.insert(start_position, 0);

    // Queue holds states to be visited, using cost for priority.
    let mut queue = BinaryHeap::new();
    queue.push(QueueElement {
        cost: 0,
        position: start_position,
    });

    while let Some(QueueElement { cost, position }) = queue.pop() {
        println!("p={:?} c={}", position, cost);

        if position == end_position {
            // Found shortest path. Store the length
            let mut path = Vec::with_capacity(cost);
            path.push(position);
            backtrack_path(&links, start_position, position, &mut path);
            println!("Initial shortest path ({}): {:?}", path.len(), path);
            return Some((cost, path));
        }

        for d in Direction::iterator() {
            // Check all directions
            let next_position = d.delta(position, 1);
            let next_cost = cost + 1;

            if map.contains(&next_position) {
                let prev_cost = dist.entry(next_position).or_insert(usize::MAX);
                if next_cost < *prev_cost {
                    // Path through current state is shorter than other paths to
                    // next state.
                    links.insert(next_position, position);
                    *prev_cost = next_cost;
                    queue.push(QueueElement {
                        cost: next_cost,
                        position: next_position,
                    });
                }
            }
        }
    }
    return None;
}

fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

fn part1(inputs: &str) -> u32 {
    let (map, start, end, height, width) = parse_map(inputs);
    let (cost, path) = shortest_path(map, start, end).unwrap();

    for (i, position) in path.iter().enumerate() {
        for d in Direction::iterator() {
            // TODO: For any available shortcut, run shortest_path again
        }
    }
    0
}

fn part2(_inputs: &str) -> u32 {
    todo!();
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
    // TODO
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


