use std::usize;

use memoize::memoize;
/// Keywords: Unbounded knapsack problem
use nalgebra_glm as glm;

use super::YEAR;
static DAY: usize = 13;

static COST_A: usize = 3;
static COST_B: usize = 1;

type Vec2 = nalgebra_glm::I64Vec2;

#[derive(Debug, Copy, Clone)]
struct Game {
    a: Vec2,
    b: Vec2,
    p: Vec2,
}

fn parse_case(inputs: &str) -> Game {
    let lines = inputs.lines().map(|line|line.split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
    let a = Vec2::new(lines[0][2].trim_matches(',')[2..].parse::<i64>().unwrap(), lines[0][3][2..].parse::<i64>().unwrap());
    let b = Vec2::new(lines[1][2].trim_matches(',')[2..].parse::<i64>().unwrap(), lines[1][3][2..].parse::<i64>().unwrap());
    let p = Vec2::new(lines[2][1].trim_matches(',')[2..].parse::<i64>().unwrap(), lines[2][2][2..].parse::<i64>().unwrap());
    Game { a, b, p }
}

fn input() -> Vec<Game> {
    crate::aoc::input_raw(YEAR, DAY)
        .trim()
        .split("\n\n")
        .map(parse_case)
        .collect::<Vec<_>>()
}

fn knapsack(game: &Game) -> usize {

    #[memoize]
    fn recur(a: Vec2, b: Vec2, p: Vec2) -> Option<usize> {
        if p.x == 0 && p.y == 0 {
            return Some(0);
        }
        let mut res = usize::MAX;
        if p.x >= a.x && p.y >= a.y {
            if let Some(c) = recur(a, b, p - a) {
                res = res.min(c + 3);
            }
        }
        if p.x >= b.x && p.y >= b.y {
            if let Some(c) = recur(a, b, p - b) {
                res = res.min(c + 1)
            }
        }
        if res == usize::MAX { None } else { Some(res) }
    }
    recur(game.a, game.b, game.p).unwrap_or(0)
}

fn knapsack_iter(game: &Game) -> usize {
    0
}

fn part1(inputs: &Vec<Game>) -> usize {
    inputs.iter().map(knapsack).sum()
}

fn part2(inputs: &Vec<Game>) -> usize {
    inputs.clone().iter_mut().map(|game| {
        game.p += Vec2::new(10000000000000, 10000000000000);
        game
    }).map(|game| knapsack(&game)).sum()
}

#[test]
fn test_2024_day13_part1() {
    println!("{:?}", None.min(Some(1)));
    let game = Game { a: Vec2::new(94, 34), b: Vec2::new(22, 67), p: Vec2::new(8400, 5400) };
    assert_eq!(knapsack(&game), 280);
    let game = Game { a: Vec2::new(26, 66), b: Vec2::new(67, 21), p: Vec2::new(12748, 12176) };
    assert_eq!(knapsack(&game), 0);
    let game = Game { a: Vec2::new(17, 86), b: Vec2::new(84, 37), p: Vec2::new(7870, 6450) };
    assert_eq!(knapsack(&game), 200);
    let game = Game { a: Vec2::new(69, 23), b: Vec2::new(27, 71), p: Vec2::new(18641, 10279) };
    assert_eq!(knapsack(&game), 0);
}

#[test]
fn test_2024_day13_part2() {
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
    // let res = part1(&inputs);
    // print!("{} Day {} part 1: ", YEAR, DAY);
    // println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part2(&inputs);
    print!("{} Day {} part 2: ", YEAR, DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}


