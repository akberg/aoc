use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::aoc::print_img;

use super::YEAR;
static DAY: usize = 08;

fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
    //.lines()
    //.map(|ls| ls.parse::<_>().unwrap())
    //.collect()
}

type Vec2 = nalgebra_glm::TVec2<isize>;

/// Collect positions of all antennas
fn parse_antennas(smap: &str) -> HashMap<char, Vec<Vec2>> {
    let mut map = HashMap::<char, Vec<Vec2>>::new();
    smap.trim().lines().enumerate().for_each(|(y, line)| {
        line.trim().chars().enumerate().for_each(|(x, c)| {
            if c.is_alphanumeric() {
                map.entry(c)
                    .and_modify(|e| {
                        (*e).push(Vec2::new(x as isize, y as isize));
                    })
                    .or_insert(vec![Vec2::new(x as isize, y as isize)]);
            }
        })
    });
    map
}

/// (Solved, >1h) Given a map of antennas, represented by alphanumeric
/// characters, count the number of unique positions for "antinodes":
/// points on the line of two equal chars, at the same distance to the
/// nearest one as the distance between the two points.
fn part1(inputs: &str) -> usize {
    let height = inputs.lines().count() as isize;
    let width = inputs.lines().next().unwrap().len() as isize;
    let map = parse_antennas(inputs);
    let mut antinodes = HashSet::<Vec2>::new();
    for (_freq, antennas) in map.iter() {
        for a in antennas.iter() {
            for b in antennas.iter() {
                let d = a - b;
                for an in [a - d, a + d].iter() {
                    if an != b && an.x >= 0 && an.y >= 0 && an.x < width && an.y < height {
                        antinodes.insert(*an);
                    }
                }
            }
        }
    }
    antinodes.len()
}

/// (Solved, >1h)
fn part2(inputs: &str) -> usize {
    println!("part 2");
    let height = inputs.lines().count() as isize;
    let width = inputs.lines().next().unwrap().len() as isize;
    println!("H/W: {}/{}", height, width);
    let map = parse_antennas(inputs);
    let mut antinodes = HashSet::<Vec2>::new();
    for (_freq, antennas) in map.iter() {
        println!("Frequency {}", _freq);
        for (i, a) in antennas.iter().enumerate() {
            for b in antennas.iter().skip(i + 1) {
                let d = a - b;
                let mut an = *a;
                while an.x >= 0 && an.y >= 0 && an.x < width && an.y < height {
                    antinodes.insert(an);
                    an += d;
                }
                let mut an = *a;
                while an.x >= 0 && an.y >= 0 && an.x < width && an.y < height {
                    antinodes.insert(an);
                    an -= d;
                }
            }
        }
    }
    let mut map = inputs
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    antinodes
        .iter()
        .for_each(|n| map[n.y as usize][n.x as usize] = '#');
    print_img(&map);
    antinodes.len()
}

#[allow(unused)]
static TEST_INPUTS: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[test]
fn test_2024_day8_part1() {
    assert_eq!(part1(TEST_INPUTS), 14);
}

#[test]
fn test_2024_day8_part2() {
    println!("Test part 2");
    assert_eq!(part2(TEST_INPUTS), 34);
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
