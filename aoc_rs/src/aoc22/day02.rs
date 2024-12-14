pub fn input() -> Vec<(String, String)> {
    crate::aoc::input_raw(super::YEAR, 2)
    .lines()
    .map(|s| {
        let mut sp = s.split(" ");
        (sp.next().unwrap().to_string(), sp.next().unwrap().to_string())
    })
    .collect::<_>()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Rps { Rock = 1, Paper = 2, Scissor = 3 }
impl From<&str> for Rps {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissor,
            _ => unreachable!()
        }
    }
}
impl PartialOrd for Rps {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let a = if let (Rps::Scissor, Rps::Rock) = (self, other) {
            0
        } else { *self as u8 };
        let b = if let (Rps::Scissor, Rps::Rock) = (other, self) {
            0
        } else { *other as u8 };
        a.partial_cmp(&b)
    }
}

pub fn part1(inputs: &Vec<(String, String)>) -> u64 {
    inputs.iter()
    .map(|(a, b)| (Rps::from(a.as_str()), Rps::from(b.as_str())))
    .map(|(a,b)| {
        b as u64 + match b.partial_cmp(&a).unwrap() {
            std::cmp::Ordering::Less => 0,
            std::cmp::Ordering::Equal => 3,
            std::cmp::Ordering::Greater => 6
        }
    }).sum::<_>()
}

pub fn part2(inputs: &Vec<(String, String)>) -> u64 {
    inputs.iter()
    .map(|(a, b)| {
        let a = Rps::from(a.as_str()) as i64;
        match b.as_str() {
            "X" => (0 + ((a - 1) - 1 + 3) % 3 + 1) as u64,
            "Y" => (3 + a) as u64,
            "Z" => (6 + ((a - 1) + 1 + 3) % 3 + 1) as u64,
            _ => unreachable!()
        }
    })
    .sum::<_>()
}

#[test]
fn test_day02_part1() {
    let inputs = vec![
        (String::from("A"), String::from("Y")),
        (String::from("B"), String::from("X")),
        (String::from("C"), String::from("Z"))
    ];
    assert_eq!(part1(&inputs), 15);
}

#[test]
fn test_day02_part2() {
    let inputs = vec![
        (String::from("A"), String::from("Y")),
        (String::from("B"), String::from("X")),
        (String::from("C"), String::from("Z"))
    ];
    println!("{}", ((1 - 1) - 1 + 3) % 3 + 1);
    println!("{}", ((2 - 1) - 1 + 3) % 3 + 1);
    println!("{}", ((3 - 1) - 1 + 3) % 3 + 1);
    assert_eq!(part2(&inputs), 12);
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 2 part 1: ");
    println!("{}", part1(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 2 part 2: ");
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}
