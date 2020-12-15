extern crate regex;
// use regex::Regex;
use std::collections::HashMap;

pub fn input() -> Vec<usize> {
    crate::aoc::input_raw(15)
        .split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
        
}

pub fn part1(inputs: &[usize]) -> usize {
    let mut log: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut prev: usize = inputs[0];
    for i in 0..2020 {
        if i < inputs.len() {
            log.insert(inputs[i], (0, i+1));  // Remember spoken
            prev = inputs[i];       // Previous number
            continue;
        } 
        let m = log.get(&prev).unwrap();        // Get entry
        prev = if m.0 == 0 { 0 } else { m.1 - m.0 };    // Zero or diff
        let m = log.get_mut(&prev);
        match m {
            None => {log.insert(prev, (0, i+1));},
            Some(t) => {t.0 = t.1; t.1 = i+1; }
        }
    }
    prev
}

pub fn part2(inputs: &[usize]) -> usize {
    let mut log: HashMap<usize, usize> = HashMap::with_capacity(30000000);
    let mut prev: usize = inputs[0];
    for i in 0..inputs.len() {
        log.insert(inputs[i], i+1);    // Remember spoken
        prev = inputs[i];                   // Previous number
    }
    for i in inputs.len()..2020 {
        //prev = log.get(&prev).map(|n| i-n).unwrap_or(0);  
        if let Some(e) = log.get_mut(&prev) {
            prev = i - *e;
            *e = i+1;
        } else {
            log.insert(prev, i+1);
            prev = 0;
        }
        println!("{}", prev);
    }
    prev
}

#[test]
fn test_day15_0() {
    let inputs = [0, 3, 6];
    assert_eq!(436, part1(&inputs));
    println!("Part 1 done");
    //assert_eq!(175594, part2(&inputs));
}

#[test]
fn test_day15_1() {
    let inputs = [1, 3, 2];
    assert_eq!(1, part1(&inputs));
    println!("Part 1 done");
    //assert_eq!(2578, part2(&inputs));
}

#[test]
fn test_day15_2() {
    let inputs = [2, 1, 3];
    assert_eq!(10, part1(&inputs));
    println!("Part 1 done");
    //assert_eq!(3544142, part2(&inputs));
}


#[test]
fn run_day15() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 15 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    // print!("Day 15 part 2: ");
    // let pt_start = SystemTime::now();
    // println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}
