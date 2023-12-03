static DAY: usize = 02;

use nalgebra_glm as glm;

struct Game {
    game_id: u32,
    draws: Vec<glm::TVec3<u32>>,
}
// impl Game {
//     pub fn from(s: &str) -> Self {
//         let (game_id, line) = sscanf!(s, "Game {}: {}", u32, str);
//     }
// }

pub fn input() -> Vec<String> {
    crate::aoc::input_raw(2)
        .lines()
        .map(|ls| ls.parse::<_>().unwrap())
        .collect()
}

pub fn part1(inputs: &Vec<String>) -> u32 {
    0
}

pub fn part2(inputs: &Vec<String>) -> u32 {
    0
}

#[test]
fn test_day2_part1() {
    // TODO
}

#[test]
fn test_day2_part2() {
    // TODO
}

// #[test]
// fn test_game_from() {
//     assert_eq!(Game { game_id: 5, draws: vec![glm::TVec3<u32>(6,3,1), glm::TVec3<u32>(1,2,2)]}, Game::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"))
// }

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


