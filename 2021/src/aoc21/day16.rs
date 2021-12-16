static DAY: i32 = 16;

#[derive(Clone, PartialEq, Debug)]
struct Packet {
    version: u8,
    typeid: u8,
    payload: Vec<u8>,
}

/// Parse hex line input a bitstream
fn parse_line(line: &str) -> Vec<char> {
    line.chars()
    .flat_map(
        |c| format!("{:04b}", u8::from_str_radix(&c.to_string(), 16).unwrap())
        .chars()
        .collect::<Vec<_>>()
    )
    .collect::<_>()
}

pub fn input() -> Vec<char> {
    parse_line(crate::aoc::input_raw(DAY).trim())
}

/// Dijkstra's shortest path to find least risky path
pub fn part1(inputs: &Vec<char>) -> i32 {
    0
}

/**Find the first step where all octopuses flash simultaneously */
pub fn part2(inputs: &Vec<char>) -> i32 {
    0
}

/* TESTS */
#[allow(unused)]
static TEST_INPUT: &str = "D2FE28";

#[test]
fn test_day16_part1() {
    assert_eq!(part1(&parse_line(TEST_INPUT)), 40);
}

#[test]
fn test_day16_part2() {
    assert_eq!(part2(&parse_line(TEST_INPUT)), 315);
}

#[test]
fn test_day16_parse_line() {
    assert_eq!(parse_line(TEST_INPUT), vec![
        '1', '1', '0', '1', '0', '0', '1', '0', 
        '1', '1', '1', '1', '1', '1', '1', '0', 
        '0', '0', '1', '0', '1', '0', '0', '0']
    );
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 1: ", DAY);
    println!("{}", part1(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 2: ", DAY);
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
