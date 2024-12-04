static DAY: usize = 02;

use glm::IVec3 as Vec3;
use nalgebra_glm as glm;

fn parse_input(inputs: String) -> Vec<Vec<Vec3>> {
    inputs
        .lines()
        .map(|line| {
            // Get rid of Game tag
            let line = line.split_once(":").unwrap().1;
            // Split sets
            line.split(";")
                .map(|set| {
                    set.split(",")
                        .map(|color| {
                            // Translate to RGB vec3
                            let (n, c) = color.trim().split_once(" ").unwrap();
                            match c {
                                "red" => Vec3::new(n.parse::<i32>().unwrap(), 0, 0),
                                "green" => Vec3::new(0, n.parse::<i32>().unwrap(), 0),
                                "blue" => Vec3::new(0, 0, n.parse::<i32>().unwrap()),
                                &_ => unreachable!(),
                            }
                        })
                        .sum::<Vec3>() // Collect colors of the set to one vec3
                })
                .collect::<Vec<Vec3>>()
        })
        .collect::<Vec<_>>()
}

pub fn input() -> Vec<Vec<Vec3>> {
    parse_input(crate::aoc::input_raw(super::YEAR, DAY))
}

pub fn part1(inputs: &Vec<Vec<Vec3>>) -> u32 {
    let content = Vec3::new(12, 13, 14);
    inputs
        .iter()
        // Add ID to the stream
        .enumerate()
        // Componentwise LE comparison, ALL operator on componentwise results.
        .filter(|&(_i, set)| {
            glm::all(&glm::less_than_equal(
                &set.iter().fold(Vec3::new(0, 0, 0), |acc, c| {
                    Vec3::new(acc.x.max(c.x), acc.y.max(c.y), acc.z.max(c.z))
                }),
                &content,
            ))
        })
        // Map to IDs for final sum and answer generation
        .map(|(i, _set)| i + 1)
        .sum::<usize>() as u32
}

pub fn part2(inputs: &Vec<Vec<Vec3>>) -> u32 {
    inputs
        .iter()
        // Componentwise LE comparison, componentwise MUL of result
        .map(|set| {
            glm::comp_mul(&set.iter().fold(Vec3::new(0, 0, 0), |acc, c| {
                Vec3::new(acc.x.max(c.x), acc.y.max(c.y), acc.z.max(c.z))
            }))
        })
        .sum::<i32>() as u32
}

#[test]
fn test_2023_day2_part1() {
    let inputs = parse_input(String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ));
    assert_eq!(part1(&inputs), 8);
}

#[test]
fn test_2023_day2_part2() {
    let inputs = parse_input(String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ));
    assert_eq!(part2(&inputs), 2286);
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
