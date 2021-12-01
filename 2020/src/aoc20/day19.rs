extern crate regex;
extern crate either;

use regex::Regex;

use std::collections::HashMap;


#[allow(unused)]
pub fn input() -> (Regex, Regex, String) {
    let inputs = crate::aoc::input_raw(20, 19);
    let mut split = inputs.split("\n\n");
    let rules_str = split.next().unwrap();
    let re = parse_rules(rules_str);
    let re2 = parse_rules2(rules_str);
    let messages = split.next().unwrap().to_string();
    (re, re2, messages)
}

fn parse_rules(inputs: &str) -> Regex {

    fn build_regex(i: usize, rules: &HashMap<usize, String>) -> String {
        let r = rules.get(&i).unwrap();
        if "ab".chars().any(|c| (*r).contains(c)) {
            // Already a regex
            String::from(r)
        } else {
            // Rule links to other rules: 2 3 | 3 2
            let group = r.split("|")
                .map(|s|
                    s.split_whitespace()
                    .map(|link| build_regex(link.parse::<usize>().unwrap(), rules))
                    .collect::<String>()
                )
                .collect::<Vec<String>>()
                .join("|");

            if i == 0 {
                String::from(["^(", group.as_str(), ")$"].join(""))
            } else {
                String::from(["(", group.as_str(), ")"].join(""))
            }
        }
    }

    let mut rules = HashMap::new();
    // make map
    for line in inputs.lines() {
        let mut split = line.split(": ");
        let i = split.next().unwrap().parse::<usize>().unwrap();
        rules.insert(i, split.next().unwrap().trim().trim_matches('"').to_owned());
    }
    Regex::new(build_regex(0, &rules).as_str()).unwrap()
}

fn parse_rules2(inputs: &str) -> Regex {
    fn build_regex2(i: usize, rules: &HashMap<usize, String>, r8: &mut usize, r11: &mut usize) -> String {
        let r = rules.get(&i).unwrap();
        if "ab".chars().any(|c| (*r).contains(c)) {
            // Already a regex
            String::from(r)
        } else {
            if i == 11 {
                *r11 += 1;
                if *r11 > 5 { return String::new() }
            }
            if i == 8 {
                *r8 += 1;
                if *r8 > 5 { return String::new() }
            }
            
            // Rule links to other rules: 2 3 | 3 2
            let group = r.split("|")
                .map(|s|
                    s.split_whitespace()
                    .map(|link| build_regex2(link.parse::<usize>().unwrap(), rules, r8, r11))
                    .collect::<String>()
                )
                .collect::<Vec<String>>()
                .join("|");

            if i == 0 {
                String::from(["^(", group.as_str(), ")$"].join(""))
            } else {
                String::from(["(", group.as_str(), ")"].join(""))
            }
        }
    }

    let mut rules = HashMap::new();
    // make map
    for line in inputs.lines() {
        let mut split = line.split(": ");
        let i = split.next().unwrap().parse::<usize>().unwrap();
        rules.insert(i, split.next().unwrap().trim().trim_matches('"').to_owned());
    }
    *rules.entry(8).or_default() = String::from("42 | 42 8");
    *rules.entry(11).or_default() = String::from("42 31 | 42 11 31");
    Regex::new(build_regex2(0, &rules, &mut 0, &mut 0).as_str()).unwrap()
}



#[allow(unused)]
pub fn part1(rules: &Regex, sentences: &str) -> i64 {
    sentences.lines()
    .filter(|x| rules.is_match(x))
    .count() as i64
}

#[allow(unused)]
pub fn part2(rules: &Regex, sentences: &str) -> i64 {
    sentences.lines()
    .filter(|x| rules.is_match(x))
    .count() as i64
}


#[test]
fn test_day19_parse_rules() {
    let re: Regex = Regex::new(r"^(a((aa|bb)(ab|ba)|(ab|ba)(aa|bb))b)$").unwrap();
    let rules = "0: 4 1 5\n\
    1: 2 3 | 3 2\n\
    2: 4 4 | 5 5\n\
    3: 4 5 | 5 4\n\
    4: \"a\"\n\
    5: \"b\"";
    assert_eq!(re.as_str(), parse_rules(rules).as_str());
}

#[test]
fn test_day19_part1() {
    let rules = parse_rules("0: 4 1 5\n\
    1: 2 3 | 3 2\n\
    2: 4 4 | 5 5\n\
    3: 4 5 | 5 4\n\
    4: \"a\"\n\
    5: \"b\"");
    let sentences = "ababbb\n\
    bababa\n\
    abbbab\n\
    aaabbb\n\
    aaaabbb";
    assert_eq!(2, part1(&rules, sentences));
}


#[test]
fn test_day19_part2() {
    let _rules = parse_rules2("42: 9 14 | 10 1\n\
        9: 14 27 | 1 26\n\
        10: 23 14 | 28 1\n\
        1: \"a\"\n\
        11: 42 31\n\
        5: 1 14 | 15 1\n\
        19: 14 1 | 14 14\n\
        12: 24 14 | 19 1\n\
        16: 15 1 | 14 14\n\
        31: 14 17 | 1 13\n\
        6: 14 14 | 1 14\n\
        2: 1 24 | 14 4\n\
        0: 8 11\n\
        13: 14 3 | 1 12\n\
        15: 1 | 14\n\
        17: 14 2 | 1 7\n\
        23: 25 1 | 22 14\n\
        28: 16 1\n\
        4: 1 1\n\
        20: 14 14 | 1 15\n\
        3: 5 14 | 16 1\n\
        27: 1 6 | 14 18\n\
        14: \"b\"\n\
        21: 14 1 | 1 14\n\
        25: 1 1 | 1 14\n\
        22: 14 14\n\
        8: 42\n\
        26: 14 22 | 1 20\n\
        18: 15 15\n\
        7: 14 5 | 1 21\n\
        24: 14 1");

    
    let inputs = "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa\n\
        bbabbbbaabaabba\n\
        babbbbaabbbbbabbbbbbaabaaabaaa\n\
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa\n\
        bbbbbbbaaaabbbbaaabbabaaa\n\
        bbbababbbbaaaaaaaabbababaaababaabab\n\
        ababaaaaaabaaab\n\
        ababaaaaabbbaba\n\
        baabbaaaabbaaaababbaababb\n\
        abbbbabbbbaaaababbbbbbaaaababb\n\
        aaaaabbaabaaaaababaa\n\
        aaaabbaaaabbaaa\n\
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa\n\
        babaaabbbaaabaababbaabababaaab\n\
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    //let inputs = "aaaaabbaabaaaaababaa";
    assert_eq!(12, part2(&_rules, inputs));
}


#[test]
fn test_day19() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let (re1, re2, sentences) = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 19 part 1: ");
    println!("{} - in {:?}", part1(&re1, &sentences), pt_start.elapsed().unwrap());
    print!("Day 19 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&re2, &sentences), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}