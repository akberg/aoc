static DAY: usize = 01;

use regex;

pub fn input() -> String {
    crate::aoc::input_raw(1)
        //.lines()
        //.map(|ls| ls.parse::<_>().unwrap())
        //.collect()
}

pub fn part1(inputs: &str) -> u32 {
    inputs.lines().map(|line| {
        let d = line.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
        d.first().unwrap() * 10 + d.last().unwrap()
    })
    .sum::<u32>()
}

fn text_to_digit(line: &str) -> Vec<u32> {
    let re = regex::Regex::new(r"^((one)|(two)|(three)|(four)|(five)|(six)|(seven)|(eight)|(nine))").unwrap();
    let mut ret = vec![];
    let mut first_re_hit = 0;
    for i in 0..line.len() {
        println!("{}", &line[i..]);
        if let Some(d) = line.chars().nth(i).unwrap().to_digit(10) {
            ret.push(d)
        }
        let m = re.find(&line[i..]);
        if let Some(caps) = m {
            first_re_hit = caps.start() + 1;
            ret.push(match caps.as_str() {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => unreachable!()
            });
            println!("{:?} first hit at {}", ret, first_re_hit  -1);
        }

    }
    ret
}

pub fn part2(inputs: &str) -> u32 {
    inputs.lines().map(|line| {
        let d = text_to_digit(line);
        println!("{:?} {}", d, d.first().unwrap() * 10 + d.last().unwrap());
        d.first().unwrap() * 10 + d.last().unwrap()
    })
    .sum::<u32>()
}

const TEST_INPUT: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[test]
fn test_day1_part1() {
    assert_eq!(part1(TEST_INPUT), 142);
}

#[test]
fn test_day1_part2() {
    let inputs = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(part2(inputs), 281);
}
// 53900 too high
// 53646 too low
// 53735 too low

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part1(&inputs);
    print!("Day {} part 1: ", DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part2(&inputs);
    print!("Day {} part 2: ", DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}


