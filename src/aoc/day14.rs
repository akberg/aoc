extern crate regex;
use regex::Regex;
use std::collections::HashMap;

fn input() -> String {
    crate::aoc::input_raw(14)
        
}

pub fn part1(inputs: &str) -> u64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"mem\[(?P<addr>[0-9]+)\] = (?P<val>[0-9]+)"
        ).unwrap();
    }
    let mut mem = HashMap::new();
    inputs.split("mask = ")
        .skip(1)
        .for_each(|s| {
            let mut split = s.lines();
            let mask = split.next().unwrap();
            for line in split {
                let cap = RE.captures(line).unwrap();
                let (addr, mut val) = (cap["addr"].parse::<u64>().unwrap(), cap["val"].parse::<u64>().unwrap());
                //println!("{}", val);
                for (i, c) in mask.chars().enumerate() {
                    match c {
                        '1' => val |= 1<<(36-i-1),
                        '0' => val &= 0xfffffffff ^ 1<<(36-i-1),
                        _ => continue,
                    }
                }
                mem.insert(addr, val);
            }
        });
    mem.values().sum()
}

pub fn part2(inputs: &str) -> u64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"mem\[(?P<addr>[0-9]+)\] = (?P<val>[0-9]+)"
        ).unwrap();
    }
    let mut mem = HashMap::new();
    inputs.split("mask = ")
        .skip(1)
        .for_each(|s| {
            let mut split = s.lines();
            let mask = split.next().unwrap();
            for line in split {
                let cap = RE.captures(line).unwrap();
                let (a, val) = (cap["addr"].parse::<u64>().unwrap(), cap["val"].parse::<u64>().unwrap());
                let mut addr = vec![a];
                for (i, c) in mask.chars().enumerate().filter(|(_, c)| *c != '0') {
                    let m = 1<<(36-i-1);
                    match c {
                        '1' => {addr.iter_mut().for_each(|a| *a |= m);},
                        _ => {
                            addr = addr.iter().map(|a| a & !m).chain(addr.iter().map(|a| a | m)).collect();
                        },
                    }
                }
                for a in addr {
                    mem.insert(a, val);
                }
                
            }
        });
    mem.values().sum()
}

#[test]
fn test_day14_part1() {
    let inputs = String::from("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\n\
        mem[8] = 11\n\
        mem[7] = 101\n\
        mem[8] = 0");
    assert_eq!(165, part1(&inputs));
}

#[test]
fn test_day14_part2() {
    let inputs = String::from("mask = 000000000000000000000000000000X1001X\n\
        mem[42] = 100\n\
        mask = 00000000000000000000000000000000X0XX\n\
        mem[26] = 1");
    assert_eq!(208, part2(&inputs));
}

#[test]
fn run_day14() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 14 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 14 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}
