static DAY: usize = 03;

use colored::Colorize;
use std::collections::HashMap;

pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, DAY)
}

pub fn part1(inputs: &str) -> u32 {
    let mut symbols = HashMap::new(); // idx -> symbol
    let mut numbers = Vec::new(); // number -> idx
    let width = inputs.lines().next().unwrap().len() as i32;
    let mut n = 0;
    let mut i = 0;
    let mut parsing = false;
    for (y, line) in inputs.lines().enumerate() {
        for (x, ch) in line.trim().chars().enumerate() {
            if let Some(d) = ch.to_digit(10) {
                parsing = true;
                n = n * 10 + d as i32;
                i = y as i32 * width + x as i32;
            } else {
                if parsing {
                    numbers.push((n, i as i32));
                    parsing = false;
                    n = 0;
                }
                if ch != '.' {
                    symbols.insert(y as i32 * width + x as i32, ch);
                }
            }
        }

        if parsing {
            numbers.push((n, i as i32));
            parsing = false;
            n = 0;
        }
    }

    // debug
    let mut map = inputs
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|ch| (ch, false))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // end debug

    let ret = numbers
        .into_iter()
        .filter_map(|(n, i)| {
            let mag = {
                let mut m = 0;
                while 10i32.pow(m) <= n {
                    m += 1;
                }
                m - 1
            } as i32;
            let mut m = 0;
            while m <= mag {
                for p in [
                    i - m + 1,
                    i - m - 1,
                    i - m + width,
                    i - m - width,
                    i - m + width + 1,
                    i - m + width - 1,
                    i - m - width + 1,
                    i - m - width - 1,
                ] {
                    if let Some(_ch) = symbols.get(&p) {
                        map[(p / width) as usize][(p % width) as usize].1 = true;
                        for m in 0..=mag {
                            map[(i / width) as usize][(i % width - m) as usize].1 = true;
                        }
                        return Some(n);
                    }
                }
                m += 1;
            }
            None
        })
        .sum::<i32>() as u32;
    // debug
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!(
                "{}",
                if map[y][x].1 {
                    map[y][x].0.to_string().bold().green()
                } else {
                    map[y][x].0.to_string().red()
                }
            )
        }
        println!("");
    }
    // end debug
    ret
}

pub fn part2(inputs: &str) -> u32 {
    let mut symbols = HashMap::new(); // idx -> (count, scale, idx0, idx1)
    let mut numbers = Vec::new(); // number -> idx
    let width = inputs.lines().next().unwrap().len() as i32;
    let mut n = 0;
    let mut i = 0;
    let mut parsing = false;
    for (y, line) in inputs.lines().enumerate() {
        for (x, ch) in line.trim().chars().enumerate() {
            if let Some(d) = ch.to_digit(10) {
                // println!("n={} d={}", n, d);
                parsing = true;
                n = n * 10 + d as i32;
                i = y as i32 * width + x as i32;
            } else {
                if parsing {
                    numbers.push((n, i as i32));
                    parsing = false;
                    n = 0;
                }
                if ch == '*' {
                    // println!("{} at {:?}", ch, (x, y));
                    symbols.insert(y as i32 * width + x as i32, (0, 1, 0, 0));
                }
            }
        }

        if parsing {
            numbers.push((n, i as i32));
            parsing = false;
            n = 0;
        }
    }

    // debug
    let mut map = inputs
        .lines()
        .map(|line| line.trim().chars().map(|ch| (ch, 0)).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // end debug

    // Iterate over numbers, but save results in symbols
    numbers.iter().enumerate().for_each(|(idx, (n, i))| {
        let mag = {
            let mut m = 0;
            while 10i32.pow(m) <= *n {
                m += 1;
            }
            m - 1
        } as i32;
        let mut m = 0;
        'num: while m <= mag {
            for p in [
                i - m + 1,
                i - m - 1,
                i - m + width,
                i - m - width,
                i - m + width + 1,
                i - m + width - 1,
                i - m - width + 1,
                i - m - width - 1,
            ] {
                if let Some(tup) = symbols.get_mut(&p) {
                    let (mut count, mut scale, mut idx0, mut idx1) = tup;
                    if count == 0 {
                        idx0 = idx;
                    } else if count == 1 {
                        idx1 = idx;
                    } else {
                        continue;
                    }
                    count += 1;
                    scale *= n;

                    // Update stats
                    *tup = (count, scale, idx0, idx1);

                    map[(p / width) as usize][(p % width) as usize].1 = count;
                    for m in 0..=mag {
                        map[(i / width) as usize][(i % width - m) as usize].1 = 1;
                    }
                    break 'num;
                }
            }
            m += 1;
        }
    });
    let ret = symbols
        .into_iter()
        .filter_map(|(_p, (count, scale, _idx0, _idx1))| if count == 2 { Some(scale) } else { None })
        .sum::<i32>() as u32;
    // debug
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!(
                "{}",
                if map[y][x].1 == 2 {
                    map[y][x].0.to_string().bold().green()
                } else if map[y][x].1 == 0 {
                    map[y][x].0.to_string().red()
                } else {
                    map[y][x].0.to_string().blue()
                }
            )
        }
        println!("");
    }
    // end debug
    ret
}

#[test]
fn test_day3_part1() {
    let inputs = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    ";

    assert_eq!(part1(inputs), 4361);
}
// 549908

#[test]
fn test_day3_part2() {
    let inputs = "467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    ";

    assert_eq!(part2(inputs), 467835);
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
