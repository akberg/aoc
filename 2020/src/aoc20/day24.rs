extern crate regex;

use regex::Regex;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};

#[allow(unused)]
pub fn input() -> String {
    crate::aoc::input_raw(20, 24)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Dir { E, W, SE, NE, SW, NW }
impl FromStr for Dir {
    type Err = &'static str;
    fn from_str(dir: &str) -> Result<Self, Self::Err> {
        match dir {
            "e" => Ok(Self::E),
            "w" => Ok(Self::W),
            "se" => Ok(Self::SE),
            "ne" => Ok(Self::NE),
            "sw" => Ok(Self::SW),
            "nw" => Ok(Self::NW),
            _ => Err("Wrong direction"),
        }
    }
}


fn parse_tiles(mut line: &str) -> Vec<Dir> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"e|w|se|sw|ne|nw").unwrap();
    }
    let mut dirs = Vec::new();
    while line.len() > 0 {
        let cap = RE.captures(line).unwrap();
        line = &line[cap[0].len()..];
        dirs.push(cap[0].parse::<Dir>().unwrap());
    }
    dirs
}

#[allow(unused)]
pub fn part1(inputs: &str) -> usize {
    init_tiles(inputs).len()
}

fn init_tiles(inputs: &str) -> HashSet<(i32, i32)> {
    use Dir::*;
    let mut blacks = HashSet::new();
    for line in inputs.lines() {
        let t = parse_tiles(line).iter().fold((0, 0), |(ax, ay), d| match d {
            E => (ax-1, ay),
            W => (ax+1, ay),
            SE => (ax, ay-1),
            SW => (ax+1, ay-1),
            NW => (ax, ay+1),
            NE => (ax-1, ay+1)
        });
        if !blacks.insert(t) {
            blacks.remove(&t);
        }
    }
    blacks
}

fn neighbors(tile: &(i32, i32)) -> Vec<(i32, i32)> {
    let (ax, ay) = *tile;
    return vec![
        (ax+1, ay),
        (ax-1, ay),
        (ax-1, ay+1),
        (ax, ay+1),
        (ax+1, ay-1),
        (ax, ay-1)
    ]
}

fn tiles_next(blacks: HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let mut trans = HashMap::new();
    for t in &blacks {
        for n in neighbors(&t) {
            let e = trans.entry(n).or_insert(0);
            *e += 1
        }
    }
    let mut ret = HashSet::new();
    for (t, n) in trans {
       if blacks.contains(&t) && n > 0 && n <= 2 
       || !blacks.contains(&t) && n == 2 {
            ret.insert(t);
       }
    }
    ret
}

#[allow(unused)]
pub fn part2(inputs: &str) -> usize {
    let mut tiles = init_tiles(inputs);
    for _ in 0..100 {
        tiles = tiles_next(tiles);
    }
    tiles.len()
}

#[test]
fn test_parse_tile() {
    use Dir::*;
    assert_eq!(vec![E, SE, NE, E, E], parse_tiles("eseneee"));
}

#[test]
fn test_day24_part1() {
    let inputs = "sesenwnenenewseeswwswswwnenewsewsw\n\
    neeenesenwnwwswnenewnwwsewnenwseswesw\n\
    seswneswswsenwwnwse\n\
    nwnwneseeswswnenewneswwnewseswneseene\n\
    swweswneswnenwsewnwneneseenw\n\
    eesenwseswswnenwswnwnwsewwnwsene\n\
    sewnenenenesenwsewnenwwwse\n\
    wenwwweseeeweswwwnwwe\n\
    wsweesenenewnwwnwsenewsenwwsesesenwne\n\
    neeswseenwwswnwswswnw\n\
    nenwswwsewswnenenewsenwsenwnesesenew\n\
    enewnwewneswsewnwswenweswnenwsenwsw\n\
    sweneswneswneneenwnewenewwneswswnese\n\
    swwesenesewenwneswnwwneseswwne\n\
    enesenwswwswneneswsenwnewswseenwsese\n\
    wnwnesenesenenwwnenwsewesewsesesew\n\
    nenewswnwewswnenesenwnesewesw\n\
    eneswnwswnwsenenwnwnwwseeswneewsenese\n\
    neswnwewnwnwseenwseesewsenwsweewe\n\
    wseweeenwnesenwwwswnew";
    assert_eq!(10, part1(inputs));
}


#[test]
fn test_day24_part2() {
    let inputs = "sesenwnenenewseeswwswswwnenewsewsw\n\
    neeenesenwnwwswnenewnwwsewnenwseswesw\n\
    seswneswswsenwwnwse\n\
    nwnwneseeswswnenewneswwnewseswneseene\n\
    swweswneswnenwsewnwneneseenw\n\
    eesenwseswswnenwswnwnwsewwnwsene\n\
    sewnenenenesenwsewnenwwwse\n\
    wenwwweseeeweswwwnwwe\n\
    wsweesenenewnwwnwsenewsenwwsesesenwne\n\
    neeswseenwwswnwswswnw\n\
    nenwswwsewswnenenewsenwsenwnesesenew\n\
    enewnwewneswsewnwswenweswnenwsenwsw\n\
    sweneswneswneneenwnewenewwneswswnese\n\
    swwesenesewenwneswnwwneseswwne\n\
    enesenwswwswneneswsenwnewswseenwsese\n\
    wnwnesenesenenwwnenwsewesewsesesew\n\
    nenewswnwewswnenesenwnesewesw\n\
    eneswnwswnwsenenwnwnwwseeswneewsenese\n\
    neswnwewnwnwseenwseesewsenwsweewe\n\
    wseweeenwnesenwwwswnew";
    let mut tiles = init_tiles(inputs);
    tiles = tiles_next(tiles);
    assert_eq!(15, tiles.len());
    tiles = tiles_next(tiles);
    assert_eq!(12, tiles.len());
    tiles = tiles_next(tiles);
    assert_eq!(25, tiles.len());

    assert_eq!(2208, part2(inputs));
}


#[test]
fn run_day24() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 24 part 1: ");
    println!("{:?} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 24 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}