extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::num::ParseIntError;

#[allow(unused)]
pub fn input() -> String {
    crate::aoc::input_raw(20, 16)
}

fn parse_rules(inputs: &str) -> Result<HashMap<String, Vec<RangeInclusive<usize>>>, ParseIntError> {
    lazy_static! {
        static ref RULE: Regex = Regex::new(r"(?P<key>.*): (?P<low1>\d+)-(?P<up1>\d+) or (?P<low2>\d+)-(?P<up2>\d+)$").unwrap();
    }
    let mut rules: HashMap<_, _> = HashMap::new();
    for line in inputs.lines() {
        let cap = RULE.captures(line).unwrap();
       
        rules.insert(
            str::to_owned(&cap["key"]),
            //String::from(&cap["key"]), 
            vec![
                cap["low1"].parse::<usize>()?..=cap["up1"].parse::<usize>()?, 
                cap["low2"].parse::<usize>()?..=cap["up2"].parse::<usize>()?
                ]
            );
        
    }
    Ok(rules)
}

fn parse_ticket(inputs: &str) -> Vec<usize> {
    inputs.split(",")
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}


#[allow(unused)]
pub fn part1(inputs: &str) -> usize {
    let inputs = inputs.split("\n\n").collect::<Vec<&str>>();
    let rules = parse_rules(inputs[0]).unwrap();
    inputs[2].lines()
        .skip(1)
        .flat_map(|line| line
            .split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .filter(|n| !rules.values().flatten().any(|r| r.contains(n)))
        )
        .sum()
}

fn infer_fields(inputs: &str) -> HashMap<String, usize> {
    let inputs = inputs.split("\n\n").collect::<Vec<&str>>();

    // Constraints
    let rules = parse_rules(inputs[0]).unwrap();   
    println!("{:?}", rules);
    let my_ticket = inputs[1].lines()
        .skip(1)
        .map(|line| parse_ticket(line))
        .last().unwrap();
    
    // Values 
    let mut keys = vec![rules.keys().map(|s| s.as_str()).collect::<Vec<&str>>(); my_ticket.len()];
    
    // Constraints
    inputs[2].lines()
        .skip(1)
        .map(|line| parse_ticket(line))
        .filter(|t| 
            (*t).iter()
                .all(|field| rules.values().flatten().any(|r| r.contains(field)))
        )
        .for_each(|ticket| {
            for (i, field) in ticket.iter().enumerate() {
                // Apply constraints 
                keys[i] = keys[i]
                    .iter()
                    .filter(|&&k| rules[k][0].contains(field) || rules[k][1].contains(field))
                    .map(|s| *s)
                    .collect();
                if keys[i].len() == 1 {
                    (0..my_ticket.len())
                    .filter(|j| *j!=i)
                    .for_each(|j| 
                        keys[j] = keys[j]
                        .iter()
                        .filter(|k| k != &&keys[i][0])
                        .map(|s| s.clone())
                        .collect()
                    );
                }
            }
        });
    // Node consistency
    let mut sieving = true;
    while sieving {
        sieving = false;
        for i in 0..my_ticket.len() {
            if keys[i].len() == 1 {
                (0..my_ticket.len())
                .filter(|j| *j!=i)
                .for_each(|j| 
                    keys[j] = keys[j]
                    .iter()
                    .filter(|k| k != &&keys[i][0])
                    .map(|s| s.clone())
                    .collect()
                );
            } else {
                sieving = true;
            }
        }
    }
    let mut translation = HashMap::new();
    for (i, k) in keys.iter().enumerate() {
        translation.insert(str::to_owned(k[0]), my_ticket[i]);
    }
    translation
}

#[allow(unused)]
pub fn part2(inputs: &str) -> usize {
    let res_keys = [
        "departure location", 
        "departure station",
        "departure platform",
        "departure track",
        "departure date",
        "departure time"];
    let ticket = infer_fields(inputs);
    println!("{:?}", ticket);
    
    res_keys.iter().fold(1, |acc, x| acc * ticket[*x])
}

#[test]
fn test_parse_rules() {
    let _rules = parse_rules(&"seat: 0-5 or 8-10\ncar: 4-7 or 13-15").unwrap();
    let r = 0..=5;
    assert_eq!(0..=5, _rules.get("seat").unwrap()[0]);
    assert_eq!(true, r.contains(&1));
    assert_eq!(1, _rules.values().flatten().filter(|r| {println!("{:?} {}", r, r.contains(&9)); (**r).contains(&9)}).count());
}


#[test]
fn test_ticket_invalid() {
    let rules = parse_rules(&"seat: 0-5 or 8-10\ncar: 4-7 or 13-15").unwrap();
    let mut rules_ans = HashMap::new();
    rules_ans.insert(String::from("seat"), vec![0..=5, 8..=10]);
    rules_ans.insert(String::from("car"), vec![4..=7, 13..=15]);
    assert_eq!(rules_ans, rules);
}

#[test]
fn test_day16_part1() {
    let inputs = "class: 1-3 or 5-7\n\
    row: 6-11 or 33-44\n\
    seat: 13-40 or 45-50\n\
    \n\
    your ticket:\n\
    7,1,14\n\
    \n\
    nearby tickets:\n\
    7,3,47\n\
    40,4,50\n\
    55,2,20\n\
    38,6,12";
    assert_eq!(71, part1(inputs));
}

#[test]
fn test_day16_part2() {
    let inputs = "class: 0-1 or 4-19\n\
    row: 0-5 or 8-19\n\
    seat: 0-13 or 16-19\n\
    \n\
    your ticket:\n\
    11,12,13\n\
    \n\
    nearby tickets:\n\
    6,0,20\n\
    3,9,18\n\
    15,1,5\n\
    5,14,9";
    let res = infer_fields(inputs);
    println!("{:?}", res);
    assert_eq!(12, res["class"]);
    assert_eq!(11, res["row"]);
    assert_eq!(13, res["seat"]);
}

#[test]
fn run_day16() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 16 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 15 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}