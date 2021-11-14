
static EYE_COLOR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[allow(unused)]
pub fn input() -> Vec<String> {
    use std::fs;
    let filename = format!("inputs/day4.txt");
    fs::read_to_string(filename)
        .unwrap()
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

#[allow(unused)]
pub fn part1(inputs: &Vec<String>) -> usize {
    let fields = ["byr", "iyr","eyr", "hgt", "hcl", "ecl", "pid" ];
    inputs.iter()
        .filter(|psp| psp.split(|c| c==':' || c==' ' || c=='\n')
            .filter(|field| fields.contains(field))
            .count() == 7 // Passport contains all required fields
        )
        .count()
}


#[allow(unused)]
pub fn part2(inputs: &Vec<String>) -> usize {
    let fields = ["byr", "iyr","eyr", "hgt", "hcl", "ecl", "pid" ];
    inputs.iter()
        .filter(|psp| psp.split(|c| c==':' || c==' ' || c=='\n')
            .filter(|field| fields.contains(field))
            .count() == 7 // Passport contains all required fields
        )
        .filter(|psp| psp.split_ascii_whitespace()
            .map(|s| s.to_string())
            .all(|field| {
                let (key, val) = field.split_at(4);
                let key = &key[..3];
                match key {
                    "byr" | "iyr" | "eyr" => {
                        let val = val.parse::<usize>().unwrap();
                        match key {
                            "byr" => val >= 1920 && 2002 >= val,
                            "iyr" => val >= 2010 && 2020 >= val,
                            "eyr" => val >= 2020 && 2030 >= val,                   
                            &_ => false,         
                        }
                    }
                    "hgt" => {
                        if val.contains("cm") {
                            let val = val.split_at(val.len()-2).0.parse::<usize>().unwrap();
                            val >= 150 && 193 >= val
                        } else if val.contains("in") {
                            let val = val.split_at(val.len()-2).0.parse::<usize>().unwrap();
                            val >= 59 && 76 >= val
                        } else { false }
                    }, 
                    "hcl" => val.len() == 7 
                        && &val[0..1] == "#" 
                        && val[1..].chars().all(|c| "0123456789abcdef".contains(c)), 
                    "ecl" => EYE_COLOR.contains(&val), 
                    "pid" => val.len() == 9 && val.chars().all(|c| "0123456789".contains(c)),
                    _ => true
                }
            })
        ).count()
}

#[test]
fn test_day4_part1() {
    let inputs = String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in")
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    assert_eq!(2, part1(&inputs));
}

#[test]
fn test_day4_part2() {
    let inputs = String::from("eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007")
        .split("\n\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    assert_eq!(4, part2(&inputs));
}

#[test]
fn run_day4() {

    println!("Parsing input . . .");
    let inputs = input();
    println!("{}", inputs.len());
    println!("Day 4 part 1:");
    println!("{}", part1(&inputs));
    
    println!("Day 4 part 2:");
    println!("{}", part2(&inputs));
}