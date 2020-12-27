extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
//use std::hash::Hash;

#[allow(unused)]
pub fn input() -> (
    HashMap<String, usize>,
    HashMap<String, HashMap<String, usize>>,
) {
    parse_input(&crate::aoc::input_raw(21))
}


                                // Allergen     Ingredient      matches
fn parse_input(inputs: &str) -> (HashMap<String, usize>, HashMap<String, HashMap<String, usize>>) {
    let re = Regex::new(r"(?P<ingredients>[^\(]+)\(contains (?P<allergens>[^\(-\)]+)\)").unwrap();
    let mut allergens = HashMap::new();
    let mut count = HashMap::new();

    for line in inputs.lines() {
        let cap = re.captures(line).unwrap();
        let vars = cap["ingredients"].trim().split_ascii_whitespace().collect::<Vec<_>>();
        let vals = cap["allergens"].split(", ").collect::<Vec<_>>();
        for v in vals {
            //variables.insert(v.to_owned());
            let ingredients = allergens.entry(v.to_owned()).or_insert(HashMap::new());
            
            for var in &vars {
                let f = ingredients.entry((*var).to_owned()).or_insert(0);
                *f += 1;
            }
        }
        for var in &vars {
            let e = count.entry((*var).to_owned()).or_insert(0);
            *e += 1;
            //println!("{}: {}", var, *e);
        }
    }
    return (count, allergens)
}

fn assign_all(csp: HashMap<String, HashMap<String, usize>>) -> Vec<(String, String)> {
    let (s, allergen, csp) = assign_allergen(csp);
    if csp.is_empty() {
        return vec![(s, allergen)]
    } else {
        let mut v = assign_all(csp);
        v.push((s, allergen));
        return v
    }
}
fn assign_allergen(mut csp: HashMap<String, HashMap<String, usize>>) -> (String, String, HashMap<String, HashMap<String, usize>>) {
    //let mut csp = csp.clone();
    let mut size = 0;
    let mut list = &HashMap::new();
    let mut rem = &String::new();
    // Select allergen with the most matches
    for (s, a) in &csp {
        if a.values().sum::<usize>() > size {
            size = a.values().sum();
            list = a;
            rem = s;
        }
    }
    let rem = rem.clone();
    let list = list.clone();
    let mut assign = &String::new();
    let mut size = 0;
    // Select ingredient with the most matches with allergen
    for (i, n) in &list {
        if *n > size {
            assign = &i;
            size = *n;
        }
    }
    let assign = assign.clone();
    // Remove allergen entry
    csp.remove_entry(&rem);
    (
    assign.clone(),
    rem,
    csp.iter()
    .map(|(k, map)| (k.to_owned(), map
        .iter()
        .filter(|(k, _)| **k != assign)
        .map(|(k, v)| (k.to_owned(), *v))
        .collect::<HashMap<String, usize>>()))
    .collect::<HashMap<String, HashMap<String, usize>>>()
    )
    
}


#[allow(unused)]
pub fn part1(map: &HashMap<String, HashMap<String, usize>>, count: HashMap<String, usize>) -> i64 {
    let a = assign_all(map.clone()).iter().map(|(s,_)| s.to_owned()).collect::<Vec<_>>();
    println!("{} / {}", a.len(), count.len());
    count.iter().filter(|(k,_)| !a.contains(k)).map(|(k,v)| {/*println!("{} {}", k, v);*/ *v}).sum::<usize>() as i64
}

#[allow(unused)]
pub fn part2(map: &HashMap<String, HashMap<String, usize>>) -> String {
    let mut a = assign_all(map.clone());
    a.sort_by_key(|k| k.1.to_owned());
    println!("{:?}", a);
    a.iter().map(|k|k.0.to_owned()).collect::<Vec<_>>().join(",")
}

#[test]
fn test_day21_part1() {
    let s = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
    trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
    sqjhc fvjkl (contains soy)\n\
    sqjhc mxmxvkd sbzzf (contains fish)";

    let (count, csp) = parse_input(s);
    assert_eq!(5, part1(&csp, count));
}

#[test]
fn test_day21_part2() {
    let s = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
    trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
    sqjhc fvjkl (contains soy)\n\
    sqjhc mxmxvkd sbzzf (contains fish)";

    let (_, csp) = parse_input(s);
    assert_eq!(String::from("mxmxvkd,sqjhc,fvjkl"), part2(&csp));
}

#[test]
fn run_day21() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let (count, csp) = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 21 part 1: ");
    println!("{} - in {:?}", part1(&csp, count), pt_start.elapsed().unwrap());
    print!("Day 21 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&csp), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}