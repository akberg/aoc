use std::collections::HashMap;

fn parse_line(line: &String) -> ((usize, usize), (usize, usize)) {
    let mut line = line.trim().split(" -> ");
    let mut start = line.next().unwrap().split(',');
    let mut stop = line.next().unwrap().split(',');
    (
        (start.next().unwrap().parse::<_>().unwrap(), start.next().unwrap().parse::<_>().unwrap()),
        (stop.next().unwrap().parse::<_>().unwrap(), stop.next().unwrap().parse::<_>().unwrap()),
    )
}

pub fn input() -> Vec<((usize, usize), (usize, usize))> {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(5).unwrap();
    let f = BufReader::new(f);
    f.lines()
        .map(|line| parse_line(&line.unwrap()))
        .collect()
}

pub fn part1(inputs: &Vec<((usize, usize), (usize, usize))>) -> usize {
    let mut tiles = HashMap::new();
    let mut dangerous_tiles = 0;
    for i in inputs.iter().filter(|i| i.0.0 == i.1.0 || i.0.1 == i.1.1) {

        let (line, sel, vert) = if i.0.0 != i.1.0 { 
            if i.1.0 > i.0.0 {
                (i.0.0..=i.1.0, i.0.1, false) 
            } else {
                (i.1.0..=i.0.0, i.0.1, false)
            }
        } else { 
            if i.1.1 > i.0.1 {
                (i.0.1..=i.1.1, i.0.0, true) 
            } else {
                (i.1.1..=i.0.1, i.0.0, true)
            }
        };

        for p in line {
            let key = if vert { (sel, p) } else { (p, sel) };
            match tiles.get_mut(&key) {
                Some(n) => {
                    *n += 1;
                    if *n == 2 {
                        dangerous_tiles += 1;
                    }
                },
                None    => {
                    tiles.insert(key, 1);
                },
            }
        }
    }
    dangerous_tiles
}

pub fn part2(inputs: &Vec<((usize, usize), (usize, usize))>) -> usize {
    let mut tiles = HashMap::new();
    let mut dangerous_tiles = 0;
    for i in inputs {
        let line = if i.0.1 == i.1.1 { 
            // Horizontal line
            if i.1.0 > i.0.0 {
                (i.0.0..=i.1.0).map(|p| (p, i.0.1)).collect::<Vec<_>>()
            } else {
                (i.1.0..=i.0.0).map(|p| (p, i.0.1)).collect::<Vec<_>>()
            }
        } else if i.0.0 == i.1.0 {
            // Vertical line
            if i.1.1 > i.0.1 {
                (i.0.1..=i.1.1).map(|p| (i.0.0, p)).collect::<Vec<_>>()
            } else {
                (i.1.1..=i.0.1).map(|p| (i.0.0, p)).collect::<Vec<_>>()
            }
        } else {
            // Diagonal line
            if i.1.0 > i.0.0 {
                // left to right
                let dy = if i.1.1 > i.0.1 { 1 } else { -1 };

                (0..=i.1.0-i.0.0)
                    .map(|idx| (
                        i.0.0+idx,
                        (i.0.1 as isize + dy * idx as isize) as usize
                    ))
                    .collect::<Vec<_>>()
            } else {
                // right to left
                let dy = if i.0.1 > i.1.1 { 1 } else { -1 };

                (0..=i.0.0-i.1.0)
                    .map(|idx| (
                        i.1.0+idx,
                        (i.1.1 as isize + dy * idx as isize) as usize
                    ))
                    .collect::<Vec<_>>()

            }
        };

        for key in line {
            match tiles.get_mut(&key) {
                Some(n) => {
                    *n += 1;
                    if *n == 2 {
                        dangerous_tiles += 1;
                    }
                },
                None    => {
                    tiles.insert(key, 1);
                },
            }
        }
    }
    dangerous_tiles
}


/* TESTS */
#[allow(unused)]
static TEST_BOARDS: &'static [&'static str] = &[
    "0,9 -> 5,9",
    "8,0 -> 0,8",
    "9,4 -> 3,4",
    "2,2 -> 2,1",
    "7,0 -> 7,4",
    "6,4 -> 2,0",
    "0,9 -> 2,9",
    "3,4 -> 1,4",
    "0,0 -> 8,8",
    "5,5 -> 8,2",
];

#[test]
fn test_day05_part1() {
    let inputs = TEST_BOARDS.iter()
        .map(|&line|String::from(line))
        .map(|line| parse_line(&line))
        .collect::<Vec<_>>();
    assert_eq!(part1(&inputs), 5);
}

#[test]
fn test_day05_part2() {
    let inputs = TEST_BOARDS.iter()
        .map(|&line|String::from(line))
        .map(|line| parse_line(&line))
        .collect::<Vec<_>>();
    assert_eq!(part2(&inputs), 12);
}

#[test]
fn test_day05_parse_line() {
    assert_eq!(parse_line(&String::from(TEST_BOARDS[0])), ((0,9),(5,9)))
}

#[allow(unused)]
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
