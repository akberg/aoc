use std::collections::{BinaryHeap, HashMap, HashSet};

use super::YEAR;
static DAY: usize = 16;

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
struct State {
    position: Vec2,
    rotation: Direction,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct QueueElement {
    cost: usize,
    state: State,
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

fn input() -> (HashSet<Vec2>, Vec2, Vec2, usize, usize) {
    let inputs = crate::aoc::input_raw(YEAR, DAY);
    parse_map(&inputs)
}

fn backtrack_path(
    links: &HashMap<State, Vec<State>>,
    start: State,
    end: State,
    path: &mut Vec<State>,
) {
    if end != start {
        let prev_v = &links[&end];
        for prev in prev_v {
            if !path.contains(prev) {
                path.push(*prev);
                backtrack_path(links, start, *prev, path);
            }
        }
    }
}

fn shortest_path(
    map: HashSet<Vec2>,
    start_position: Vec2,
    end_position: Vec2,
) -> Vec<(usize, Vec<State>)> {
    // For tracking multiple paths, make map value a vector, storing every
    // previous state with an equally short path.
    let mut links: HashMap<State, Vec<State>> = HashMap::new();

    let start_state = State {
        position: start_position,
        rotation: Direction::Right,
    };

    let mut best_length = None;
    let mut paths = Vec::new();
    // Dist map holds a position/rotation state and its cost. Non-existing
    // entry means MAX cost.
    let mut dist = HashMap::new();
    dist.insert(start_state, 0);

    // Queue holds states to be visited, using cost for priority.
    let mut queue = BinaryHeap::new();
    queue.push(QueueElement {
        cost: 0,
        state: start_state,
    });

    while let Some(QueueElement { cost, state }) = queue.pop() {
        let position = state.position;
        let rotation = state.rotation;
        println!("p={:?} r={:?} c={}", position, rotation, cost);

        if position == end_position {
            // Found shortest path. Store the length
            //return Some(cost);
            if best_length.is_none() {
                best_length = Some(cost);
            }
            // For each shortest path (there may be multiple), backtrace path.
            if Some(cost) == best_length {
                let mut path = Vec::from([state]);
                backtrack_path(&links, start_state, state, &mut path);
                println!("New shortest path ({}): {:?}", path.len(), path);
                paths.push((cost, path));
            }
            continue;
        }
        // Check movement in current direction
        let next_state = State {
            position: rotation.delta(position, 1),
            rotation,
        };
        let next_cost = cost + 1;
        // Next position is legal
        if map.contains(&next_state.position) {
            let prev_cost = dist.entry(next_state).or_insert(usize::MAX);
            if next_cost < *prev_cost {
                // Path through current state is shorter than other paths to
                // next state.
                links.insert(next_state, Vec::from([state]));
                *prev_cost = next_cost;
                queue.push(QueueElement {
                    cost: next_cost,
                    state: next_state,
                });
            } else if next_cost == *prev_cost {
                // Path through current state is as short as other shortest
                // paths already found.
                links.entry(next_state).or_default().push(state);
            }
        }
        // Check rotation left and right
        let next_cost = cost + 1000;
        let next_states = [
            State {
                position,
                rotation: rotation.rot_left(),
            },
            State {
                position,
                rotation: rotation.rot_right(),
            },
        ];
        for next_state in next_states {
            let prev_cost = dist.entry(next_state).or_insert(usize::MAX);

            if next_cost < *prev_cost {
                links.insert(next_state, Vec::from([state]));
                *prev_cost = next_cost;
                queue.push(QueueElement {
                    cost: next_cost,
                    state: next_state,
                });
            } else if next_cost == *prev_cost {
                // Path through current state is as short as other shortest
                // paths already found.
                links.entry(next_state).or_default().push(state);
            }
        }
    }
    return paths;
}

/// (Solved) Find shortest path in a maze, considering the cost of turning.
fn part1(inputs: &(HashSet<Vec2>, Vec2, Vec2, usize, usize)) -> usize {
    shortest_path(inputs.0.clone(), inputs.1, inputs.2)[0].0
}

/// (Solved, >1h) Find the number of tiles covered by all shortest paths.
fn part2(inputs: &(HashSet<Vec2>, Vec2, Vec2, usize, usize)) -> usize {
    let paths = shortest_path(inputs.0.clone(), inputs.1, inputs.2);
    // Remove duplicates
    let mut visits = HashSet::new();
    paths.iter().for_each(|(_, path)| {
        path.iter().for_each(|state| {
            visits.insert(state.position);
        });
    });
    // Display for debug and visualization.
    for y in 0..inputs.3 {
        for x in 0..inputs.4 {
            let pos = Vec2::new(x as i64, y as i64);
            if visits.contains(&pos) {
                print!("O");
            } else if inputs.0.contains(&pos) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!("");
    }

    visits.len()
}

#[allow(unused)]
static TEST_INPUTS0: &str = "###############
 #.......#....E#
 #.#.###.#.###.#
 #.....#.#...#.#
 #.###.#####.#.#
 #.#.#.......#.#
 #.#.#####.###.#
 #...........#.#
 ###.#.#####.#.#
 #...#.....#.#.#
 #.#.#.###.#.#.#
 #.....#...#.#.#
 #.###.#.#.#.#.#
 #S..#.....#...#
 ###############";
#[allow(unused)]
static TEST_INPUTS1: &str = "#################
 #...#...#...#..E#
 #.#.#.#.#.#.#.#.#
 #.#.#.#...#...#.#
 #.#.#.#.###.#.#.#
 #...#.#.#.....#.#
 #.#.#.#.#.#####.#
 #.#...#.#.#.....#
 #.#.#####.#.###.#
 #.#.#.......#...#
 #.#.###.#####.###
 #.#.#...#.....#.#
 #.#.#.#####.###.#
 #.#.#.........#.#
 #.#.#.#########.#
 #S#.............#
 #################";

#[test]
fn test_2024_day16_part1() {
    assert_eq!(part1(&parse_map(TEST_INPUTS0)), 7036);
    assert_eq!(part1(&parse_map(TEST_INPUTS1)), 11048);
}

#[test]
fn test_2024_day16_part2() {
    assert_eq!(part2(&parse_map(TEST_INPUTS0)), 45);
    assert_eq!(part2(&parse_map(TEST_INPUTS1)), 64);
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
