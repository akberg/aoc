use super::YEAR;
static DAY: usize = 25;

fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

const WIDTH: usize = 5;
const HEIGHT: usize = 7;

#[derive(Debug, Copy, Clone)]
enum Scheme {
    Key([usize; WIDTH]),
    Lock([usize; WIDTH]),
}

fn parse_schematic(s: &str) -> Scheme {
    let is_key = s.lines().next().unwrap().chars().all(|c| c == '.');
    let s = s
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut v = [0; WIDTH];
    for x in 0..WIDTH {
        for y in 0..HEIGHT - 1 {
            if s[y][x] != s[y + 1][x] {
                v[x] = if is_key { HEIGHT - y } else { y };
                break;
            }
        }
    }
    if is_key {
        Scheme::Key(v)
    } else {
        Scheme::Lock(v)
    }
}

/// (Solved, 15min) Match locks and keys.
fn part1(inputs: &str) -> u32 {
    let schemes = inputs
        .split("\n\n")
        .map(parse_schematic)
        .collect::<Vec<_>>();
    let mut count = 0;
    for s0 in &schemes {
        if let Scheme::Key(k) = s0 {
            for s1 in &schemes {
                if let Scheme::Lock(l) = s1 {
                    println!("{:?} with {:?}", l, k);
                    if (0..WIDTH).all(|i| k[i] + l[i] <= HEIGHT) {
                        count += 1;
                        println!(" matches");
                    }
                }
            }
        }
    }
    count
}

fn part2(_inputs: &str) -> String {
    String::from("Merry Christmas!")
}

#[allow(unused)]
static TEST_INPUTS: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

#[test]
fn test_2024_day25_part1() {
    assert_eq!(part1(TEST_INPUTS), 3);
}

#[test]
fn test_2024_day25_part2() {
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
