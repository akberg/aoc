static DAY: usize = 03;

pub fn input() -> String {
    crate::aoc::input_raw(DAY as i32)
}

fn parse_line(line: &str) -> Vec<u32> {
    line.chars()
    .map(|c| {
        let c = c as u8;
        if c <= 'Z' as u8 {
            (c - 'A' as u8 + 27) as u32
        } else {
            (c - 'a' as u8 + 1) as u32
        }
    })
    .collect::<_>()
}

pub fn part1(inputs: &str) -> u64 {
    inputs.lines()
    .map(|line| line.split_at(line.len() / 2))
    .map(|(s1, s2)| (parse_line(s1), parse_line(s2)))
    .map(|(s1, s2)| {
        s1.iter()
        .filter(|c| s2.contains(c))
        .map(|c|*c as u64).next().unwrap()
    })
    .sum::<_>()
}

pub fn part2(inputs: &str) -> u64 {
    let inputs = inputs.lines()
    .map(|line| parse_line(line))
    .collect::<Vec<_>>();

    inputs.chunks(3)
    .map(|tri| {
        tri[0].iter()
        .filter(|c| tri[1].contains(c) && tri[2].contains(c))
        .map(|c|*c as u64).next().unwrap()
    })
    .sum::<_>()
}

#[test]
fn test_day03_part1() {
    let inputs = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part1(&inputs), 157);
}

#[test]
fn test_day03_part2() {
    let inputs = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
    assert_eq!(part2(&inputs), 70);
}

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


