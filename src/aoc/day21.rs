extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub fn input() -> (
    HashMap<String, usize>,
    HashMap<String, HashMap<String, usize>>,
) {
    parse_input(&crate::aoc::input_raw(21))
}


// Domains: ingredients
// Domains: possible matched allergens
#[derive(Debug)]
struct CSP<T: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq + Hash + Clone> {
    variables: HashSet<T>,      
    domains: HashMap<T, HashSet<V>>,
    constraints: HashMap<T, HashMap<T, Vec<(V, V)>>>,//fn (V, V) -> bool,//
}
impl<T: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq + Hash + Clone> CSP<T, V> {
    pub fn get_all_arcs(&self) -> Vec<(T, T)> {
        let mut ret = Vec::new();
        for x in &self.variables {
            for y in &self.variables {
                if x != y {
                    ret.push((x.clone(), y.clone()));
                }
            }
        }
        ret
    }

    fn select_unassigned_variable(&self, assignments: &HashMap<T, HashSet<V>>) -> Option<V> {
        None
    }

    pub fn backtrack(&self, assignments: HashMap<T, HashSet<V>>) -> Option<HashMap<T, HashSet<V>>> {
        if assignments.values().all(|x| x.len() == 1) {
            return Some(assignments)
        } else {
            let var = self.select_unassigned_variable(&assignments);
            None
        }
    }
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

fn make_csp(inputs: &str) -> CSP<String, String> {
    let re = Regex::new(r"(?P<ingredients>[^\(]+)\(contains (?P<allergens>[^\(-\)]+)\)").unwrap();
    let mut variables = HashSet::new();
    let mut domains = HashMap::new();
    
    for line in inputs.lines() {
        let cap = re.captures(line).unwrap();
        let vars = cap["ingredients"].trim().split_ascii_whitespace();
        let vals = cap["allergens"].split(", ").collect::<Vec<_>>();
        for v in vars {
            variables.insert(v.to_owned());
            let e = domains.entry(v.to_owned()).or_insert(HashSet::new());
            e.insert(String::from("unknown"));
            for val in &vals {
                e.insert((*val).to_owned());
            }
        }
    }
    let mut constraints = HashMap::new();
    // Add constrained value pairs
    for x1 in domains.keys() {
        let mut e = HashMap::new();
        for x2 in domains.keys() {
            if x1 == x2 { continue }
            let mut f = Vec::new();
            for y1 in domains.get(x1).unwrap() {
                for y2 in domains.get(x2).unwrap() {
                    if y1 == "unknown" || y2 == "unknown" || y1 != y2 {
                        f.push((y1.clone(), y2.clone()));
                    }
                }
            }
            e.insert(x2.clone(), f);
        }
        constraints.insert(x1.clone(), e);
    }
    //let constraints: fn(String, String) -> bool =  |y1, y2| y1 == "unknown" || y2 == "unknown" || y1 != y2;
    CSP { domains, variables, constraints }
}


// fn ac3<T: PartialEq + Eq + Hash + Clone, V: PartialEq + Eq + Hash + Clone>(csp: &mut CSP<T, V>) -> HashMap<T, HashSet<V>> {
//     let mut arcs = csp.get_all_arcs();
//     let mut assignments = csp.domains.clone();

//     while !arcs.is_empty() {
//         let (x1, x2) = arcs.pop().unwrap();
//         if ac3_revise(&mut assignments, csp.constraints, x1.clone(), x2.clone()) {
//             for x in &csp.variables {
//                 if *x != x1 && *x != x2 {
//                     arcs.insert(0, (x1.clone(), x.clone()));
//                 }
//             }
//         }
//     }
//     return assignments
// }



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
fn test_day21_make_csp() {
    let s = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n\
    trh fvjkl sbzzf mxmxvkd (contains dairy)\n\
    sqjhc fvjkl (contains soy)\n\
    sqjhc mxmxvkd sbzzf (contains fish)";

    let csp = make_csp(s);
    println!("{:?}", csp);
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