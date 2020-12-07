
extern crate regex;
use regex::Regex;

fn parse_input(inputs: &String) -> (usize, usize, char, String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<ll>\d+)-(?P<ul>\d+) (?P<chr>[a-z]): (?P<pwd>[a-z]+)"
        ).unwrap();
    }
    let cap = RE.captures(inputs).unwrap();
    (
        cap["ll"].parse::<usize>().unwrap(), 
        cap["ul"].parse::<usize>().unwrap(), 
        cap["chr"].parse::<char>().unwrap(), 
        cap["pwd"].to_string()
    )
}

pub fn input() -> Vec<(usize, usize, char, String)> {
    let mut inputs: Vec<(usize, usize, char, String)> = Vec::new();
    for e in super::input(2).iter() {
        inputs.push(parse_input(&e));
    }
    inputs
}

#[allow(unused)]
pub fn part1(inputs: &Vec<(usize, usize, char, String)>) -> Result<i32, &'static str> {
    Ok(
        inputs
            .iter()
            .filter(|p| {
                let c = p.3.matches(p.2).count();
                c >= p.0 && c <= p.1
            })
            .count() as i32
    )
}

#[allow(unused)]
pub fn part2(inputs: &Vec<(usize, usize, char, String)>) -> Result<i32, &'static str> {
    Ok(
        inputs
            .iter()
            .filter(|p| {
                (p.3.chars().nth(p.0 - 1).unwrap() == p.2) ^ (p.3.chars().nth(p.1 - 1).unwrap() == p.2)
            })
            .count() as i32
    )
}

// TESTS

#[test]
fn test_regex() {
    let cap = parse_input(&String::from("1-2 a: abc"));
    assert_eq!(cap, (1 as usize, 2 as usize, 'a', String::from("abc")));
}

#[test]
fn test_part1() {
    let inputs = String::from("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let content = inputs.iter().map(|e| parse_input(e)).collect::<Vec<(usize, usize, char, String)>>();
    assert_eq!(Ok(2), part1(&content));
}

#[test]
fn test_part2() {
    let inputs = String::from("1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc")
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let content = inputs.iter().map(|e| parse_input(e)).collect::<Vec<(usize, usize, char, String)>>();
    assert_eq!(Ok(1), part2(&content));
}

#[test]
fn run_day2() {
    println!("Parsing input . . .");
    let inputs = input();
    println!("Day 2 part 1:");
    println!("{}", part1(&inputs).unwrap());
    println!("Day 2 part 2:");
    println!("{}", part2(&inputs).unwrap());
}
