static DAY: i32 = 22;

use std::ops::RangeInclusive;

#[derive(Copy, Clone, PartialEq)]
pub enum State { On, Off }
#[derive(Clone)]
pub struct Instr {
    state: State,
    x: RangeInclusive<i32>,
    y: RangeInclusive<i32>,
    z: RangeInclusive<i32>,
}

pub fn input(test: i32) -> Vec<Instr> {
    use std::io::{prelude::*, BufReader};
    let f = match test { 
        0 => crate::aoc::_test_input_file(DAY, 0),
        1 => crate::aoc::_test_input_file(DAY, 1),
        _ => crate::aoc::input_file(DAY) 
    };
    let f = BufReader::new(f.unwrap());
    f.lines().map(parse_line).collect::<Vec<_>>()
}

fn parse_line(line: Result<String, std::io::Error>) -> Instr {
    let line = line.unwrap();
    let mut line = line.split_ascii_whitespace();
    let state = match line.next().unwrap() {
        "on" => State::On,
        "off" => State::Off,
        &_ => unreachable!(),
    };
    let (x,y,z) = match sscanf::scanf!(line.next().unwrap(), "x={}..{},y={}..{},z={}..{}", i32,i32,i32,i32,i32,i32) {
        None => panic!(),
        Some((x0,x1,y0,y1,z0,z1)) => (x0..=x1, y0..=y1, z0..=z1)
    };
    Instr { state, x, y, z }
}

pub fn part1(inputs: &Vec<Instr>) -> u64 {
    let mut boot = vec![vec![vec![State::Off; 101]; 101]; 101];
    
    for i in inputs {
        for x in i.x.clone().filter(|n| (-50..=50).contains(n)).map(|n| n+50) {
            for y in i.y.clone().filter(|n| (-50..=50).contains(n)).map(|n| n+50) {
                for z in i.z.clone().filter(|n| (-50..=50).contains(n)).map(|n| n+50) {
                    boot[x as usize][y as usize][z as usize] = i.state;
                }
            }
        }
    }
    boot.iter()
    .map(|grid| grid.iter()
        .map(|line| line.iter().filter(|&&s| s == State::On).count() as u64).sum::<u64>()
    )
    .sum::<u64>()
}


pub fn part2(inputs: &Vec<Instr>) -> u64 {
    0
}

/* TESTS */

#[test]
fn test_day22_part1() {
    let inputs = input(0);
    assert_eq!(part1(&inputs), 39);
    let inputs = input(1);
    assert_eq!(part1(&inputs), 590784);
}

#[test]
fn test_day22_part2() {
    let inputs = input(2);
    assert_eq!(part2(&inputs), 2_758_514_936_282_235_u64);
}


#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input(-1);
    println!("{:?}", start.elapsed().unwrap());
    // let pt_start = SystemTime::now();
    // print!("Day {} part 1: ", DAY);
    // println!("{}", part1(&inputs));
    // println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 2: ", DAY);
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
