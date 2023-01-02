use std::{fmt::Debug, collections::VecDeque, thread::sleep_ms};

static DAY: usize = 24;

pub fn input() -> String {
    crate::aoc::input_raw(24)
}
#[derive(Copy, Clone, PartialEq, Eq)]
enum Tile {
    Up, Down, Left, Right, Open, Wall, Agent, Multi
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Tile::Up => f.write_str("^"),
            &Tile::Down => f.write_str("v"),
            &Tile::Left => f.write_str("<"),
            &Tile::Right => f.write_str(">"),
            &Tile::Open => f.write_str("."),
            &Tile::Wall => f.write_str("#"),
            &Tile::Agent => f.write_str("o"),
            &Tile::Multi => f.write_str("+"),
        }
    }
}
/// For each single round, only the position of each blizzard, forming a maze,
/// matters, so all can be represented as a wall. No up or down moving blizzards
/// in columns of entry and exit, removes that edge case.
fn evolved_map(initial_map: &Vec<Vec<Tile>>, n: usize) -> Vec<Vec<Tile>> {
    let h = initial_map.len();
    let w = initial_map[0].len();
    let mut out = vec![vec![Tile::Open; w]; h];
    for y in 0..initial_map.len() {
        for x in 0..initial_map[0].len() {
            match initial_map[y][x] {
                Tile::Right => {
                    let x_ = (((x-1)+n)%(w-2))+1;
                    out[y][x_] = if out[y][x_] == Tile::Open { Tile::Right } else { Tile::Multi };
                },
                Tile::Left => {
                    let x_ = (((x as isize-1)-n as isize).rem_euclid(w as isize-2)) as usize +1;
                    out[y][x_] = if out[y][x_] == Tile::Open { Tile::Left } else { Tile::Multi };
                }
                Tile::Down => {
                    let y_ = (((y-1)+n)%(h-2))+1;
                    out[y_][x] = if out[y_][x] == Tile::Open { Tile::Down } else { Tile::Multi };
                },
                Tile::Up => {
                    let y_ = (((y as isize-1)-n as isize).rem_euclid(h as isize-2)) as usize +1;
                    out[y_][x] = if out[y_][x] == Tile::Open { Tile::Up } else { Tile::Multi };
                },
                Tile::Wall => out[y][x] = Tile::Wall,
                _ => ()
            }
        }
    }
    out
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
struct ScoredPos {
    pos: (usize, usize),
    time: usize,
    steps: usize,
    start: (usize, usize),
    end: (usize, usize),
}
impl Ord for ScoredPos {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_dist = self.start.0.abs_diff(self.pos.0) + self.start.1.abs_diff(self.pos.1);
        let other_dist = other.start.0.abs_diff(other.pos.0) + other.start.1.abs_diff(other.pos.1);
        (usize::MAX/2 - self.time - self_dist).cmp(&(usize::MAX/2 - other.time - other_dist))
    }
}

fn shortest_path(
initial_map: &Vec<Vec<Tile>>,
initial_time: usize,
start: (usize, usize),
end: (usize, usize)) -> usize {

    let mut queue = VecDeque::new();
    queue.push_back(ScoredPos { pos: start, time: initial_time, steps: 0, start, end});

    let mut _i = 0;
    while let Some(mut current) = queue.pop_front() {
        let (x, y) = current.pos;

        // Return number of steps if end is reached
        if x == end.0 && y == end.1 {
            return current.time;
        }

        current.time += 1;
        let map = evolved_map(&initial_map, current.time);
        // Option to stay put
        if map[y][x] == Tile::Open {
            queue.push_back(current);
        }
        if map[y][x+1] == Tile::Open && !queue.iter().any(|s|s.pos==(x+1,y)) {
            let mut next = current;
            next.pos = (x+1, y);
            next.steps += 1;
            queue.push_back(next);
        }
        if map[y][x-1] == Tile::Open && !queue.iter().any(|s|s.pos==(x-1,y)) {
            let mut next = current;
            next.pos = (x-1, y);
            next.steps += 1;
            queue.push_back(next);
        }
        if y < map.len()-1 && map[y+1][x] == Tile::Open && !queue.iter().any(|s|s.pos==(x,y+1)) {
            let mut next = current;
            next.pos = (x, y+1);
            next.steps += 1;
            queue.push_back(next);
        }
        if y > 0 && map[y-1][x] == Tile::Open && !queue.iter().any(|s|s.pos==(x,y-1)) {
            let mut next = current;
            next.pos = (x, y-1);
            next.steps += 1;
            queue.push_back(next);
        }
    }
    0
}

fn init(inputs: &str) -> (Vec<Vec<Tile>>, (usize, usize), (usize, usize)) {
    // Build initial map
    let initial_map = inputs.lines()
    .map(|line| {
        line.trim().chars()
        .map(|c| match c {
            '#' => Tile::Wall,
            '.' => Tile::Open,
            '<' => Tile::Left,
            '>' => Tile::Right,
            '^' => Tile::Up,
            'v' => Tile::Down,
            _ => unreachable!()
        })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    // Start and end positions
    let posy = 0;
    let posx = (&initial_map[0]).into_iter()
        .enumerate()
        .find(|&(_, &c)| c==Tile::Open)
        .unwrap().0;
    let endy = initial_map.len()-1;
    let endx = initial_map.iter()
        .last().unwrap()
        .into_iter()
        .enumerate()
        .find(|(_, c)| **c==Tile::Open)
        .unwrap().0;
    (initial_map, (posx, posy), (endx, endy))
}

/// A* (or Dijkstra) shortest path with an evolving map
pub fn part1(inputs: &str) -> usize {
    let (initial_map, start, end) = init(inputs);

    shortest_path(&initial_map, 0, start, end)
}

pub fn part2(inputs: &str) -> usize {
    let (initial_map, start, end) = init(inputs);
    let n = shortest_path(&initial_map, 0, start, end);
    // println!("{}", n);
    let n = shortest_path(&initial_map, n, end, start);
    // println!("{}", n);
    let n = shortest_path(&initial_map, n, start, end);
    // println!("{}", n);
    n
}

#[test]
fn test_day24_part1() {
    let inputs = "#.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#";
    assert_eq!(part1(inputs), 18);
}

#[test]
fn test_day24_part2() {
    let inputs = "#.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#";
    assert_eq!(part2(inputs), 54);
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


