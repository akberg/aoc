static DAY: i32 = 19;

use nalgebra_glm as glm;

fn parse_line(line: &str) -> Option<(i32,i32,i32)> {
    if line.is_empty() {
        None
    } else {
        let mut pts = line.split(',').map(|s| s.parse::<i32>().unwrap());
        Some((pts.next().unwrap(), pts.next().unwrap(), pts.next().unwrap()))
    }
}

pub fn input(test: bool) -> Vec<Vec<(i32, i32, i32)>> {
    let f = if test { crate::aoc::_test_input_raw(DAY, 0) } else { crate::aoc::input_raw(DAY) };
    let mut scanners = Vec::new();
    let mut beacons = Vec::new();
    let mut new_scanner = true;
    for line in f.lines() {
        if new_scanner {
            new_scanner = false;
            scanners.push(beacons);
            beacons = Vec::new();
        } else {
            match parse_line(line) {
                Some(pt) => beacons.push(pt),
                None => new_scanner = true,
            };
        }
    }
    scanners.push(beacons);
    scanners
}

/// 
pub fn part1(inputs: &Vec<Vec<(i32,i32,i32)>>) -> u64 {
    let map = inputs[0].clone();
    
    0
}

/// Parse packet and compute its value
pub fn part2(inputs: &Vec<Vec<(i32,i32,i32)>>) -> u64 {
    0
}

/* TESTS */

#[test]
fn test_day16_part1() {
    let inputs = input(true);
    assert_eq!(part1(&inputs), 79);
}

#[test]
fn test_day16_part2() {
    let inputs = input(true);
    assert_eq!(part2(&inputs), 3);
}


#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input(false);
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 1: ", DAY);
    println!("{}", part1(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 2: ", DAY);
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
