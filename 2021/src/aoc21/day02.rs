use std::str::FromStr;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction { Up, Down, Forward } 
#[derive(Copy, Clone, PartialEq)]
pub struct Command {
    dir: Direction,
    len: u64
}

use Direction::*;

impl FromStr for Command {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_ascii_whitespace();
        let dir = s.next();
        let len = s.next();

        let mut ret: Command = Command { dir: Direction::Forward, len: 0 };

        if let Some(len) = len {
            ret.len = len.parse::<u64>().unwrap();
        } else {
            return Err("No length given")
        }

        match dir {
            Some("forward") => ret.dir = Direction::Forward,
            Some("up") => ret.dir = Direction::Up,
            Some("down") => ret.dir = Direction::Down,
            _ => return Err("Invalid direction")
        };

        Ok(ret)
    }
}

pub fn input() -> Vec<Command> {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(2).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|line| line.unwrap().parse::<Command>().unwrap())
        .collect()
}

/**
 * Follow instructions and return the product of resulting horizontal and
 * vertical positions.
 */
pub fn part1(cmds: &Vec<Command>) -> u64 {

    let (x, y) = cmds
        .iter()
        .fold((0, 0), |(x, y), cmd| match cmd.dir {
            Forward =>  (x + cmd.len, y),
            Up =>       (x, y - cmd.len),
            Down =>     (x, y + cmd.len),
        });

    x*y
}

/**
 * Same as part 1, but with updated command interpretation.
 */
pub fn part2(cmds: &Vec<Command>) -> u64 {
    let (x, y, _) = cmds
        .iter()
        .fold((0, 0, 0), |(x, y, a), cmd| match cmd.dir {
            Forward =>  (x + cmd.len, y + a * cmd.len, a),
            Up =>       (x, y, a - cmd.len),
            Down =>     (x, y, a + cmd.len),
        });

    x*y
}

#[test]
fn test_day02_part1() {
    let inputs = vec![
        Command { dir: Forward, len: 5 },
        Command { dir: Down, len: 5 },
        Command { dir: Forward, len: 8 },
        Command { dir: Up, len: 3 },
        Command { dir: Down, len: 8 },
        Command { dir: Forward, len: 2 },
        ];
    assert_eq!(part1(&inputs), 150);
}

#[test]
fn test_day02_part2() {
    let inputs = vec![
        Command { dir: Forward, len: 5 },
        Command { dir: Down, len: 5 },
        Command { dir: Forward, len: 8 },
        Command { dir: Up, len: 3 },
        Command { dir: Down, len: 8 },
        Command { dir: Forward, len: 2 },
        ];
    assert_eq!(part2(&inputs), 900);
}


pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 1 part 1: ");
    println!("{}", part1(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 1 part 2: ");
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
