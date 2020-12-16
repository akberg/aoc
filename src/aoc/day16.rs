extern crate regex;
use regex::Regex;

pub fn input() -> String {
    let re = Regex::new(r"(?P<field>(?P))(your ticket:\n(?P<my>)")
    crate::aoc::input_raw(16)
}

fn is_valid(ticket: &[u32]) -> bool {
    ticket.iter()
        .all(|&f| 
            f > 31 && f < 201 ||
            f > 227 && f < 951 ||
            f > 49 && f < 885 ||
            f > 36 && f < 248 ||
            f > 258 && f < 974 ||
            f > 37 && f < 507 || 
            f > 527 && f < 965 ||
            f > 37 && f < 331 || 
            f > 351 && f < 970 ||
            f > 38 && f < 370 || 
            f > 382 && f < 970 ||
            f > 33 && f < 686 || 
            f > 711 && f < 960 ||
            f > 46 && f < 753 || 
            f > 775 && f < 953 ||
            f > 34 && f < 138 || 
            f > 154 && f < 959 ||
            f > 26 && f < 167 || 
            f > 181 && f < 961 ||
            f > 43 && f < 664 || 
            f > 675 && f < 968 ||
            f > 47 && f < 603 || 
            f > 620 && f < 954 ||
            f > 40 && f < 290 || 
            f > 313 && f < 972 ||
            f > 37 && f < 792 || 
            f > 799 && f < 972 ||
            f > 32 && f < 97  || 
            f > 115 && f < 954 ||
            f > 25 && f < 916 || 
            f > 942 && f < 966 ||
            f > 39 && f < 572 || 
            f > 587 && f < 966 ||
            f > 25 && f < 834 || 
            f > 858 && f < 953 ||
            f > 48 && f < 534 || 
            f > 544 && f < 959 ||
            f > 47 && f < 442 || 
            f > 463 && f < 969
        )
}


pub fn count_invalid(tickets: &[&str]) -> usize {
    tickets.iter()
        .map(|line| line.split(",").map(|c| {println!("{}", c); c.parse::<u32>().unwrap()}).collect::<Vec<u32>>())
        .filter(|t| !is_valid(&t))
        .count()
}


pub fn part1(inputs: &str) -> usize {
    println!("{:?}", inputs.split("tickets:").collect::<Vec<&str>>().last());
    let tickets = inputs.split("tickets:").collect::<Vec<&str>>().last().unwrap().lines().collect::<Vec<&str>>();
    println!("{:?}", tickets);
    println!("{:?}", inputs.split("tickets:").collect::<Vec<&str>>().len());
    count_invalid(&tickets)
}


#[test]
fn run_day16() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    //println!("{:?}", inputs.split("tickets:\n").collect::<Vec<&str>>().last());
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 16 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    // print!("Day 15 part 2: ");
    // let pt_start = SystemTime::now();
    // println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}