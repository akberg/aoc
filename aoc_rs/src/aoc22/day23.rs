use std::collections::{HashMap, VecDeque};

use colored::Colorize;

static DAY: usize = 23;

pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, DAY)
}

fn cover_map(elves: &HashMap<(isize,isize), (isize,isize)>, dbg: bool) -> isize {
    let (xmin,xmax,ymin,ymax) = elves.keys().fold((0,0,0,0), |a, (x,y)| {
        (a.0.min(*x), a.1.max(*x), a.2.min(*y), a.3.max(*y))
    });

    let mut free = 0;
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if !elves.contains_key(&(x,y)) {
                free += 1;
                if dbg { print!("{}", format!(".").green()); }
            } else {
                if dbg { print!("{}", format!("#").red()); }
            }
        }
        if dbg { println!(""); }
    }
    free
}

pub fn part1(inputs: &str) -> isize {
    // Hold elf position and proposed new position
    let mut elves = HashMap::new();
    // Hold proposed position and number of elves proposing it
    let mut moves = HashMap::new();

    #[derive(Debug, Copy, Clone)]
    enum Direction { N, S, W, E }
    use Direction::*;
    let mut directions = VecDeque::from([N, S, W, E]);

    inputs.lines()
    .enumerate()
    .for_each(|(y, line)| {
        line.char_indices()
        .for_each(|(x, c)| {
            if c == '#' {
                elves.insert((x as isize,y as isize), (x as isize,y as isize));
            }
        });
    });

    for _round in 0..10 {
        // Part 1, check surroundings and propose move
        'elves: for (x,y) in elves.clone().into_keys() {
            // No surrounding elves, do nothing
            if !elves.contains_key(&(x, y-1))
            && !elves.contains_key(&(x, y+1))
            && !elves.contains_key(&(x-1, y+1))
            && !elves.contains_key(&(x+1, y+1))
            && !elves.contains_key(&(x-1, y-1))
            && !elves.contains_key(&(x+1, y-1))
            && !elves.contains_key(&(x+1, y))
            && !elves.contains_key(&(x-1, y)) {
                *moves.entry((x,y)).or_insert(0) += 1;
                elves.entry((x,y)).and_modify(|e| { *e = (x,y); });
                continue;
            }
            // Find a direction to move
            for d in &directions {
                match d {
                    N => {
                        if !elves.contains_key(&(x, y-1))
                        && !elves.contains_key(&(x-1, y-1))
                        && !elves.contains_key(&(x+1, y-1)) {
                            *moves.entry((x,y-1)).or_insert(0) += 1;
                            elves.entry((x,y)).and_modify(|e| { *e = (x,y-1); });
                            continue 'elves;
                        }
                    },
                    S => {
                        if !elves.contains_key(&(x, y+1))
                        && !elves.contains_key(&(x-1, y+1))
                        && !elves.contains_key(&(x+1, y+1)) {
                            *moves.entry((x,y+1)).or_insert(0) += 1;
                            elves.entry((x,y)).and_modify(|e| { *e = (x,y+1); });
                            continue 'elves;
                        }
                    },
                    W => {
                        if !elves.contains_key(&(x-1, y))
                        && !elves.contains_key(&(x-1, y+1))
                        && !elves.contains_key(&(x-1, y-1)) {
                            *moves.entry((x-1,y)).or_insert(0) += 1;
                            elves.entry((x,y)).and_modify(|e| { *e = (x-1,y); });
                            continue 'elves;
                        }
                    },
                    E => {
                        if !elves.contains_key(&(x+1, y))
                        && !elves.contains_key(&(x+1, y+1))
                        && !elves.contains_key(&(x+1, y-1)) {
                            *moves.entry((x+1,y)).or_insert(0) += 1;
                            elves.entry((x,y)).and_modify(|e| { *e = (x+1,y); });
                            continue 'elves;
                        }
                    },
                }
            }
            // Stuck, do nothing, I guess
            *moves.entry((x,y)).or_insert(0) += 1;
            elves.entry((x,y)).and_modify(|e| { *e = (x,y); });
        }
        let mut elves_nxt = HashMap::new();
        // Part 2, check if proposed move can be executed
        for (x,y) in elves.clone().into_keys() {
            if moves[&elves[&(x,y)]] > 1 {
                // Multiple elves, keep position
                elves_nxt.insert((x,y), (x,y));
            } else {
                // Insert new position
                elves_nxt.insert(elves[&(x,y)], elves[&(x,y)]);
            }
        }
        // Cycle direction list, clear moves
        let d = directions.pop_front().unwrap();
        directions.push_back(d);
        elves = elves_nxt;
        moves.clear();
    }
    cover_map(&elves, false)
}

pub fn part2(inputs: &str) -> usize {
    // Hold elf position and proposed new position
    let mut elves = HashMap::new();
    // Hold proposed position and number of elves proposing it
    let mut moves = HashMap::new();
    // Hold next iteration of elf positions
    let mut elves_nxt = HashMap::new();

    #[derive(Debug, Copy, Clone)]
    enum Direction { N, S, W, E }
    use Direction::*;
    let mut directions = VecDeque::from([N, S, W, E]);

    inputs.lines()
    .enumerate()
    .for_each(|(y, line)| {
        line.char_indices()
        .for_each(|(x, c)| {
            if c == '#' {
                elves.insert((x as isize,y as isize), (x as isize,y as isize));
            }
        });
    });

    let mut round = 0;
    loop {
        round += 1;
        // cover_map(&elves, true);
        // Part 1, check surroundings and propose move
        'elves: for (x,y) in elves.clone().into_keys() {
            // No surrounding elves, do nothing
            if !elves.contains_key(&(x, y-1))
            && !elves.contains_key(&(x, y+1))
            && !elves.contains_key(&(x-1, y+1))
            && !elves.contains_key(&(x+1, y+1))
            && !elves.contains_key(&(x-1, y-1))
            && !elves.contains_key(&(x+1, y-1))
            && !elves.contains_key(&(x+1, y))
            && !elves.contains_key(&(x-1, y)) {
                *moves.entry((x,y)).or_insert(0) += 1;
                elves.entry((x,y)).and_modify(|e| { *e = (x,y); });
                continue;
            }
            // Find a direction to move
            for d in &directions {
                match d {
                    N => {
                        if !elves.contains_key(&(x, y-1))
                        && !elves.contains_key(&(x-1, y-1))
                        && !elves.contains_key(&(x+1, y-1)) {
                            *moves.entry((x,y-1)).or_insert(0) += 1;
                            elves.entry((x,y)).and_modify(|e| { *e = (x,y-1); });
                            continue 'elves;
                        }
                    },
                    S => {
                        if !elves.contains_key(&(x, y+1))
                        && !elves.contains_key(&(x-1, y+1))
                        && !elves.contains_key(&(x+1, y+1)) {
                            *moves.entry((x,y+1)).or_insert(0) += 1;
                            elves.entry((x,y)).and_modify(|e| { *e = (x,y+1); });
                            continue 'elves;
                        }
                    },
                    W => {
                        if !elves.contains_key(&(x-1, y))
                        && !elves.contains_key(&(x-1, y+1))
                        && !elves.contains_key(&(x-1, y-1)) {
                            *moves.entry((x-1,y)).or_insert(0) += 1;
                            elves.entry((x,y)).and_modify(|e| { *e = (x-1,y); });
                            continue 'elves;
                        }
                    },
                    E => {
                        if !elves.contains_key(&(x+1, y))
                        && !elves.contains_key(&(x+1, y+1))
                        && !elves.contains_key(&(x+1, y-1)) {
                            *moves.entry((x+1,y)).or_insert(0) += 1;
                            elves.entry((x,y)).and_modify(|e| { *e = (x+1,y); });
                            continue 'elves;
                        }
                    },
                }
            }
            // Stuck, do nothing, I guess
            *moves.entry((x,y)).or_insert(0) += 1;
            elves.entry((x,y)).and_modify(|e| { *e = (x,y); });
        }
        // Part 2, check if proposed move can be executed
        for (x,y) in elves.clone().into_keys() {
            if moves[&elves[&(x,y)]] > 1 {
                // Multiple elves, keep position
                elves_nxt.insert((x,y), (x,y));
            } else {
                // Insert new position
                elves_nxt.insert(elves[&(x,y)], elves[&(x,y)]);
            }
        }
        // Check end condition
        if elves == elves_nxt {
            break;
        }
        // Cycle direction list, clear moves
        let d = directions.pop_front().unwrap();
        directions.push_back(d);
        elves = elves_nxt.clone();
        elves_nxt.clear();
        moves.clear();
    }
    round
}

#[allow(unused, non_upper_case_globals)]
static test_inputs: &'static str =
"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

#[test]
fn test_day23_part1() {
    assert_eq!(part1(test_inputs), 110);
}

#[test]
fn test_day23_part2() {
    assert_eq!(part2(test_inputs), 20);
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


