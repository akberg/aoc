static DAY: i32 = 13;

use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum Axis { X, Y }


pub fn input() -> (Vec<(i32, i32)>, Vec<(Axis, i32)>) {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(DAY).unwrap();
    let mut f = BufReader::new(f).lines();
    let mut points = Vec::new();
    let mut line = f.next().unwrap().unwrap();

    while !(&line).trim().is_empty() {
        println!("{}", line);
        let mut s = line.split(",");
        points.push((s.next().unwrap().parse::<i32>().unwrap(), s.next().unwrap().parse::<i32>().unwrap()));
        line = f.next().unwrap().unwrap();
    }

    let instr = f.map(|line| {
        let line = line.unwrap();
        let mut s = line.split("=");
        (if 'x'==s.next().unwrap().chars().last().unwrap() { Axis::X } else { Axis::Y }, s.next().unwrap().parse::<i32>().unwrap())
    }).collect::<Vec<_>>();
    (points, instr)
}

/**Fold according to first instruction and count visible points */
pub fn part1(points: &Vec<(i32, i32)>, instr: &Vec<(Axis, i32)>) -> u64 {
    // fold along y=7: all y>7 -> y = 2*7 - y

    points.iter().map(|pt| {
        let (mut x, mut y) = pt;
        instr.iter().take(1).for_each(|&(axis, n)| match axis {
            Axis::X => if x > n { x = 2*n - x }
            Axis::Y => if y > n { y = 2*n - y }
        });
        (x, y)
    }).collect::<HashSet<_>>().len() as u64
}

/**Find the first step where all octopuses flash simultaneously */
pub fn part2(points: &Vec<(i32, i32)>, instr: &Vec<(Axis, i32)>) {
    
    /* Do all folds */
    let res = points.iter().map(|pt| {
        let (mut x, mut y) = pt;
        instr.iter().for_each(|&(axis, n)| match axis {
            Axis::X => if x > n { x = 2*n - x }
            Axis::Y => if y > n { y = 2*n - y }
        });
        (x, y)
    }).collect::<HashSet<_>>();

    /* Get text boundaries */
    let sx = *res.iter().map(|(x,_y)| x).max().unwrap() as usize + 1;
    let sy = *res.iter().map(|(_x,y)| y).max().unwrap() as usize + 1;

    /* Print resulting ASCII art */
    println!("");
    for j in 0..sy {
        for i in 0..sx {
            if res.contains(&(i as i32,j as i32)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
}

/* TESTS */
#[allow(unused)]
static TEST_INPUT: (&[(i32, i32)], &[(Axis, i32)]) = (&[
    (6,10),
    (0,14),
    (9,10),
    (0,3),
    (10,4),
    (4,11),
    (6,0),
    (6,12),
    (4,1),
    (0,13),
    (10,12),
    (3,4),
    (3,0),
    (8,4),
    (1,10),
    (2,14),
    (8,10),
    (9,0),
    ]
    , &[
    (Axis::Y, 7),
    (Axis::X, 5),
    ]
);

#[test]
fn test_day12_part1() {
    assert_eq!(part1(&Vec::from(TEST_INPUT.0), &Vec::from(TEST_INPUT.1)), 17);
}

#[test]
fn test_day12_part2() {
    assert_eq!(part2(&Vec::from(TEST_INPUT.0), &Vec::from(TEST_INPUT.1)), ());
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 1: ", DAY);
    println!("{}", part1(&inputs.0, &inputs.1));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 2: ", DAY);
    part2(&inputs.0, &inputs.1);
    //println!("{}", part2(&inputs.0, &inputs.1));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
