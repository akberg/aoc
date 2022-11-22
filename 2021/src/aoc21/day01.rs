
pub fn input() -> Vec<u32> {
    crate::aoc::input_raw(1)
        .lines()
        .map(|x| x.parse::<u32>().unwrap())
        .collect()
}


/**
 * Count the number of increasing differences in a list.
 * Skip first value, then count all values that are higher
 * than the precious one.
 */
pub fn part1(depths: &Vec<u32>) -> u32 {
    depths.iter()
        .enumerate()
        .skip(1)
        .filter(|(i, _)| depths[*i] > depths[i-1])
        .count() as u32
}

/**
 * Count the number of increasing differences in a list, using
 * a three measurement window. Sum windows and apply same counting
 * as in part 1
 */
pub fn part2(depths: &Vec<u32>) -> u32 {
    part1(&(2..depths.len())
        .map(|i| depths[i]+depths[i-1]+depths[i-2])
        .collect()
    )
}

#[test]
fn test_day01_part1() {
    let inputs = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(part1(&inputs), 7);
}

#[test]
fn test_day01_part2() {
    let inputs = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    assert_eq!(part2(&inputs), 5);
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