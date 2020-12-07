
#[allow(unused)]
pub fn input() -> Vec<String> {
    use std::fs;
    fs::read_to_string("inputs/day6.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

pub fn part1(inputs: &Vec<String>) -> usize {
    inputs.iter()
        .map(|grp| grp.split_ascii_whitespace()
            .fold(String::new(), |acc, line| format!("{}{}", &acc, &line.chars().filter(|c| !acc.contains(*c)).collect::<String>()))
            .len()
        )
        .sum()
}

pub fn part2(inputs: &Vec<String>) -> usize {
    inputs.iter()
        .map(|grp| grp.split_ascii_whitespace()
            .fold(String::from("abcdefghijklmnopqrstuvwxyz"), |acc, line| line.chars().filter(|c| acc.contains(*c)).collect::<String>())
            .len()
        )
        .sum()
}

// Better solution:
// Treat characters as index
// Count every character and lines
// Any (p1): sum all non-zero values
// All (p2): count all values equal to line count

#[test]
fn test_day6_part1() {
    let inputs = String::from("abc

a
b
c

ab
ac

a
a
a
a

b").split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>();
    assert_eq!(11, part1(&inputs));
}

#[test]
fn test_day6_part2() {
    let inputs = String::from("abc

a
b
c

ab
ac

a
a
a
a

b").split("\n\n").map(|s| s.to_string()).collect::<Vec<String>>();
    assert_eq!(6, part2(&inputs));
}

#[test]
fn run_day6() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 6 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 6 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}