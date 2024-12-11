use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nalgebra_glm as glm;

use crate::aoc::print_img;

use super::YEAR;
static DAY: usize = 06;

type Vec2 = glm::TVec2<usize>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
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
    pub fn delta(&self, v: &Vec2) -> Vec2 {
        let mut v = v.clone();
        match self {
            Direction::Up => v.y -= 1,
            Direction::Right => v.x += 1,
            Direction::Down => v.y += 1,
            Direction::Left => v.x -= 1,
        };
        v
    }
    pub fn get_char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Obstacle,
    Unvisited,
    Visited(Direction),
}

pub fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
    //.lines()
    //.map(|ls| ls.parse::<_>().unwrap())
    //.collect()
}

fn track_guard(map: &mut Vec<Vec<char>>) -> bool {
    let height = map.len();
    let width = map[0].len();
    let mut pos = Vec2::new(0, 0);
    let mut direction = Direction::Up;
    let mut visited = HashMap::<Vec2, HashSet<Direction>>::new();
    'dir: for y in 0..height {
        for x in 0..width {
            match map[y][x] {
                '^' => {
                    direction = Direction::Up;
                }
                '>' => {
                    direction = Direction::Right;
                }
                'v' => {
                    direction = Direction::Down;
                }
                '<' => {
                    direction = Direction::Left;
                }
                _ => continue,
            }
            pos = Vec2::new(x, y);
            break 'dir;
        }
    }

    loop {
        // Store footprint.
        visited
            .entry(pos)
            .and_modify(|e| {
                (*e).insert(direction);
            })
            .or_insert(HashSet::new());
        map[pos.y][pos.x] = direction.get_char();
        // Check if leaving area.
        match direction {
            Direction::Up => {
                if pos.y == 0 {
                    return false;
                }
            }
            Direction::Right => {
                if pos.x + 1 == width {
                    return false;
                }
            }
            Direction::Down => {
                if pos.y + 1 == height {
                    return false;
                }
            }
            Direction::Left => {
                if pos.x == 0 {
                    return false;
                }
            }
        }
        let nxt = direction.delta(&pos);
        // Check if hitting obstacle, in that case turn right.
        if map[nxt.y][nxt.x] == '#' {
            direction = direction.rot_right();
        } else {
            // Move one step.
            pos = nxt;
        }
        // Check if a loop is closed.
        if visited
            .get(&pos)
            .and_then(|e| Some((*e).contains(&direction)))
            .unwrap_or(false)
        {
            return true;
        }
    }
}

/// (Solved, 20min) Track the movement of an agent, following the rule of turning right at each
/// obstacle met, and count the number of distinct tiles on the map which the agent steps on
/// before exiting the map.
pub fn part1(inputs: &str) -> usize {
    let mut map = inputs
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    track_guard(&mut map);
    print_img(&map);
    map.iter()
        .flatten()
        .map(|c| {
            if ['<', '>', 'v', '^'].contains(c) {
                'X'
            } else {
                *c
            }
        })
        .counts()[&'X']
}

/// (Solved, 2-3h) Find how many different tiles one obstacle can be added to,
/// in order to make the agent stuck in a loop.
///
/// Ended up rewriting the guard tracker to both track until leaving the area
/// and store historical positions.
pub fn part2(inputs: &str) -> u32 {
    let map = inputs
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let mut o = map.clone();
    track_guard(&mut o);
    let mut count = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            // Only test obstacles in the guard's route
            if ['#', '.'].contains(&o[y][x]) {
                continue;
            }
            // println!("{} {}", x, y);
            let mut m = map.clone();
            m[y][x] = '#';
            count += track_guard(&mut m) as u32;
        }
    }
    count
}

#[allow(unused)]
static TEST_INPUTS: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[test]
fn test_2024_day6_part1() {
    assert_eq!(part1(TEST_INPUTS), 41);
}

#[test]
fn test_2024_day6_part2() {
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
