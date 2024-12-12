use std::collections::{HashMap, HashSet};

use super::YEAR;
static DAY: usize = 12;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Tile {
    c: char,
    i: usize,
}

/// Floodfill parsing of contiguous area in map.
fn floodfill(
    x: isize,
    y: isize,
    charmap: &Vec<Vec<char>>,
    map: &mut HashMap<(isize, isize), Tile>,
) -> (usize, usize) {
    if x < 0 || y < 0 || x >= charmap[0].len() as isize || y >= charmap.len() as isize {
        return (0, 0);
    }
    let mut area = 1;
    let mut circ = 0;
    let tile = map[&(x, y)];
    // Iterate neighbouring positions
    for pos in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)].iter() {
        if let Some(t) = map.get(pos) {
            println!(" = {:?}", t);
            // Neighbour already visited, check if is border
            if *t != tile {
                circ += 1;
            }
        } else {
            // Neighbour not visited
            if pos.0 >= 0
                && pos.1 >= 0
                && pos.0 < charmap[0].len() as isize
                && pos.1 < charmap.len() as isize
                && charmap[pos.1 as usize][pos.0 as usize] == tile.c
            {
                map.insert(*pos, tile);
                // Same type, continue floodfill
                let (a, c) = floodfill(pos.0, pos.1, charmap, map);
                area += a;
                circ += c;
            } else {
                circ += 1;
            }
        }
    }
    (area, circ)
}

fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

/// (Solved, >2h30) Identify distinct regions of a map, and sum the product of
/// each region's area and circumference.
fn part1(inputs: &str) -> usize {
    let mut map = HashMap::new();
    let mut i = 0;
    // Map to char 2D array
    let inputs = inputs
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut area_circ = Vec::new();
    let mut tiles = Vec::new();

    for (y, line) in inputs.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            let (x, y) = (x as isize, y as isize);
            if !map.contains_key(&(x, y)) {
                map.insert((x, y), Tile { c, i });
                tiles.push(Tile { c, i });
                i += 1;
                // Floodfill and compute area of plot. This is the first time this tile is seen.
                area_circ.push(floodfill(x, y, &inputs, &mut map));
            }
        }
    }
    area_circ.into_iter().map(|(a, c)| a * c).sum::<usize>()
}

/// Now, instead of circumference, use the number of sides.
fn part2(inputs: &str) -> usize {
    let mut map = HashMap::new();
    let mut i = 0;
    // Map to char 2D array
    let inputs = inputs
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut area_circ = Vec::new();
    let mut tiles = Vec::new();

    for (y, line) in inputs.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            let (x, y) = (x as isize, y as isize);
            if !map.contains_key(&(x, y)) {
                map.insert((x, y), Tile { c, i });
                tiles.push(Tile { c, i });
                i += 1;
                // Floodfill and compute area of plot. This is the first time this tile is seen.
                area_circ.push(floodfill(x, y, &inputs, &mut map));
            }
        }
    }
    area_circ.into_iter().map(|(a, c)| a * c).sum::<usize>()
}

#[test]
fn test_2024_day12_part1() {
    let test_inputs = "AAAA
                       BBCD
                       BBCC
                       EEEC";
    assert_eq!(part1(test_inputs), 140);
    let test_inputs = "OOOOO
                       OXOXO
                       OOOOO
                       OXOXO
                       OOOOO";
    assert_eq!(part1(test_inputs), 772);
    let test_inputs = "RRRRIICCFF
                       RRRRIICCCF
                       VVRRRCCFFF
                       VVRCCCJFFF
                       VVVVCJJCFE
                       VVIVCCJJEE
                       VVIIICJJEE
                       MIIIIIJJEE
                       MIIISIJEEE
                       MMMISSJEEE";
    assert_eq!(part1(test_inputs), 1930);
}

#[test]
fn test_2024_day12_part2() {
    // TODO
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
